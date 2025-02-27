// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Tock kernel for the S32K144EVB
//!
//! It is based on 32K144EVB SoC (Cortex M4 core).

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]

use core::ptr::{addr_of, addr_of_mut};

use kernel::capabilities;
use kernel::component::Component;
use kernel::hil::time::Counter;
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::scheduler::round_robin::RoundRobinSched;

#[allow(unused_imports)]
use kernel::{create_capability, debug, debug_gpio, debug_verbose, static_init};

use s32k144::chip::S32K144DefaultPeripherals;
use s32k144::gpio::Pin;

// Kernel LED (same as microphone LED)
const LED_KERNEL_PIN: Pin = Pin::PTD0; //PTD15 : blue pin

//UART Pins
//const UART_TX_PIN: Pin = Pin::PTC6;
//const UART_RX_PIN: Pin = Pin::PTC7;

/// UART Writer for panic!()s.
pub mod panic;

// State for loading and holding applications.
// How should the kernel respond when a process faults.
const FAULT_RESPONSE: capsules_system::process_policies::PanicFaultPolicy =
    capsules_system::process_policies::PanicFaultPolicy {};

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 4;

static mut PROCESSES: [Option<&'static dyn kernel::process::Process>; NUM_PROCS] =
    [None; NUM_PROCS];

static mut CHIP: Option<&'static s32k144::chip::S32K144<S32K144DefaultPeripherals>> = None;
static mut PROCESS_PRINTER: Option<&'static capsules_system::process_printer::ProcessPrinterText> =
    None;

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];
// debug mode requires more stack space
// pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

#[no_mangle]
#[inline(never)]
pub extern "C" fn _start() -> ! {
    unsafe {
        main();
    }
    loop {}
}

/// Supported drivers by the platform
pub struct S32K144EVB<'a> {
    console: &'static capsules_core::console::Console<'static>,
    gpio: &'a capsules_core::gpio::GPIO<'a, s32k144::gpio::GPIOPin<'a>>,
    alarm: &'static capsules_core::alarm::AlarmDriver<
        'static,
        capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm<
            'static,
            s32k144::rtc::Rtc<'static>,
        >,
    >,
    scheduler: &'static RoundRobinSched<'static>,
    ipc: kernel::ipc::IPC<{ NUM_PROCS as u8 }>,
    systick: cortexm4::systick::SysTick,
    //pwm: &'static capsules_extra::pwm::Pwm<'static, 1>,
    //led: &'a capsules_core::led::LedDriver<'a, s32k144::gpio::GPIOPin<'a>, 1>,
}

impl SyscallDriverLookup for S32K144EVB<'_> {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            capsules_core::console::DRIVER_NUM => f(Some(self.console)),
            capsules_core::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules_core::alarm::DRIVER_NUM => f(Some(self.alarm)),
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            //capsules_core::led::DRIVER_NUM => f(Some(self.led)),
            //capsules_extra::pwm::DRIVER_NUM => f(Some(self.pwm)),
            _ => f(None),
        }
    }
}

impl KernelResources<s32k144::chip::S32K144<'static, S32K144DefaultPeripherals<'static>>>
    for S32K144EVB<'_>
{
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type Scheduler = RoundRobinSched<'static>;
    type SchedulerTimer = cortexm4::systick::SysTick;
    type WatchDog = ();
    type ContextSwitchCallback = ();

    fn syscall_driver_lookup(&self) -> &Self::SyscallDriverLookup {
        self
    }
    fn syscall_filter(&self) -> &Self::SyscallFilter {
        &()
    }
    fn process_fault(&self) -> &Self::ProcessFault {
        &()
    }
    fn scheduler(&self) -> &Self::Scheduler {
        self.scheduler
    }
    fn scheduler_timer(&self) -> &Self::SchedulerTimer {
        &self.systick
    }
    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }
    fn context_switch_callback(&self) -> &Self::ContextSwitchCallback {
        &()
    }
}

/// This is in a separate, inline(never) function so that its stack frame is
/// removed when this function returns. Otherwise, the stack space used for
/// these static_inits is wasted.
#[inline(never)]
unsafe fn start() -> (
    &'static kernel::Kernel,
    S32K144EVB<'static>,
    &'static s32k144::chip::S32K144<'static, S32K144DefaultPeripherals<'static>>,
) {
    // Initialize chip peripheral drivers
    let s32k144_peripherals =
        static_init!(S32K144DefaultPeripherals, S32K144DefaultPeripherals::new());

    // set up circular peripheral dependencies
    s32k144_peripherals.init();

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&*addr_of!(PROCESSES)));

    //--------------------------------------------------------------------------
    // CAPABILITIES
    //--------------------------------------------------------------------------

    // Create capabilities that the board needs to call certain protected kernel
    // functions.
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);

    //--------------------------------------------------------------------------
    // DEBUG GPIO
    //--------------------------------------------------------------------------

    // Configure kernel debug GPIOs as early as possible. These are used by the
    // `debug_gpio!(0, toggle)` macro. We uconfigure these early so that the
    // macro is available during most of the setup code and kernel exection.

    const LED_KERNEL_PIN_INDEX: usize = LED_KERNEL_PIN as usize;

    kernel::debug::assign_gpios(
        Some(&s32k144_peripherals.gpio_port.pins[LED_KERNEL_PIN_INDEX]),
        None,
        None,
    );

    //--------------------------------------------------------------------------
    // GPIO
    //--------------------------------------------------------------------------

    let gpio = components::gpio::GpioComponent::new(
        board_kernel,
        capsules_core::gpio::DRIVER_NUM,
        components::gpio_component_helper!(
            s32k144::gpio::GPIOPin,
            9 => &s32k144_peripherals.gpio_port.pins[Pin::PTC1 as usize],
            16 => &s32k144_peripherals.gpio_port.pins[Pin::PTD0 as usize],
        ),
    )
    .finalize(components::gpio_component_static!(s32k144::gpio::GPIOPin));

    //--------------------------------------------------------------------------
    // ALARM & TIMER
    //--------------------------------------------------------------------------

    let rtc = &s32k144_peripherals.rtc;
    let _ = rtc.start();

    let mux_alarm = components::alarm::AlarmMuxComponent::new(rtc)
        .finalize(components::alarm_mux_component_static!(s32k144::rtc::Rtc));
    let alarm = components::alarm::AlarmDriverComponent::new(
        board_kernel,
        capsules_core::alarm::DRIVER_NUM,
        mux_alarm,
    )
    .finalize(components::alarm_component_static!(s32k144::rtc::Rtc));

    //--------------------------------------------------------------------------
    // UART & CONSOLE & DEBUG
    //--------------------------------------------------------------------------

    use kernel::hil::uart::Configure;

    s32k144_peripherals.lpuart0.enable();

    match s32k144_peripherals
        .lpuart0
        .configure(kernel::hil::uart::Parameters {
            baud_rate: 115200,
            stop_bits: kernel::hil::uart::StopBits::One,
            parity: kernel::hil::uart::Parity::None,
            hw_flow_control: false,
            width: kernel::hil::uart::Width::Eight, // 8-bit
        }) {
        Ok(_) => debug!("UART configured successfully"),
        Err(e) => debug!("UART configuration failed: {:?}", e),
    }

    // Create a shared UART channel for the console and for kernel debug.
    let uart_mux = components::console::UartMuxComponent::new(&s32k144_peripherals.lpuart0, 115200)
        .finalize(components::uart_mux_component_static!());

    // Setup the console.
    let console = components::console::ConsoleComponent::new(
        board_kernel,
        capsules_core::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::console_component_static!());
    // Create the debugger object that handles calls to `debug!()`.
    components::debug_writer::DebugWriterComponent::new(uart_mux)
        .finalize(components::debug_writer_component_static!());

    //--------------------------------------------------------------------------
    // Process Console
    //--------------------------------------------------------------------------
    let process_printer = components::process_printer::ProcessPrinterTextComponent::new()
        .finalize(components::process_printer_text_component_static!());
    PROCESS_PRINTER = Some(process_printer);

    let _process_console = components::process_console::ProcessConsoleComponent::new(
        board_kernel,
        uart_mux,
        mux_alarm,
        process_printer,
        Some(cortexm4f::support::reset),
    )
    .finalize(components::process_console_component_static!(
        s32k144::rtc::Rtc
    ));
    let _ = _process_console.start();

    //--------------------------------------------------------------------------
    // FINAL SETUP AND BOARD BOOT
    //--------------------------------------------------------------------------

    // it seems that microbit v2 has no external clock
    s32k144_peripherals.clock.low_stop();
    s32k144_peripherals.clock.high_stop();
    s32k144_peripherals.clock.low_start();
    s32k144_peripherals.clock.high_start();
    while !s32k144_peripherals.clock.low_started() {}
    while !s32k144_peripherals.clock.high_started() {}

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(&*addr_of!(PROCESSES))
        .finalize(components::round_robin_component_static!(NUM_PROCS));

    let s32k144evb = S32K144EVB {
        console,
        gpio,
        alarm,
        scheduler,
        systick: cortexm4::systick::SysTick::new_with_calibration(64000000),
        ipc: kernel::ipc::IPC::new(
            board_kernel,
            kernel::ipc::DRIVER_NUM,
            &memory_allocation_capability,
        ),
    };

    let chip = static_init!(
        s32k144::chip::S32K144<S32K144DefaultPeripherals>,
        s32k144::chip::S32K144::new(s32k144_peripherals)
    );
    CHIP = Some(chip);

    debug!("Initialization complete. Entering main loop.");

    //--------------------------------------------------------------------------
    // PROCESSES AND MAIN LOOP
    //--------------------------------------------------------------------------

    // These symbols are defined in the linker script.
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
        /// End of the ROM region containing app images.
        static _eapps: u8;
        /// Beginning of the RAM region for app memory.
        static mut _sappmem: u8;
        /// End of the RAM region for app memory.
        static _eappmem: u8;
    }

    kernel::process::load_processes(
        board_kernel,
        chip,
        core::slice::from_raw_parts(
            core::ptr::addr_of!(_sapps),
            core::ptr::addr_of!(_eapps) as usize - core::ptr::addr_of!(_sapps) as usize,
        ),
        core::slice::from_raw_parts_mut(
            core::ptr::addr_of_mut!(_sappmem),
            core::ptr::addr_of!(_eappmem) as usize - core::ptr::addr_of!(_sappmem) as usize,
        ),
        &mut *addr_of_mut!(PROCESSES),
        &FAULT_RESPONSE,
        &process_management_capability,
    )
    .unwrap_or_else(|err| {
        debug!("Error loading processes!");
        debug!("{:?}", err);
    });

    (board_kernel, s32k144evb, chip)
}

/// Main function called after RAM initialized.
#[no_mangle]
pub unsafe fn main() {
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);

    let (board_kernel, board, chip) = start();
    board_kernel.kernel_loop(&board, chip, Some(&board.ipc), &main_loop_capability);
}
