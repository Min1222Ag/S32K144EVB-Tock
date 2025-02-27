// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![no_std]
#![cfg_attr(not(doc), no_main)]
#![allow(missing_docs)]

use capsules_core::virtualizers::virtual_uart::{MuxUart, UartDevice};
use cortexm4::systick::SysTick;
use critical_section::{Impl, RawRestoreState};
use kernel::capabilities;
use kernel::component::Component;
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{create_capability, debug, static_init};
use rtt_target::{rprintln, rtt_init_print};

// Chip-specific imports
use s32k144::chip::{S32K144DefaultPeripherals, S32K144};
use s32k144::interrupt_service::S32K144InterruptService;
use s32k144::lpuart0::LPUART;
use s32k144::portd::RegisterBlock as PORTD;
use s32k144::ptd::RegisterBlock as PTD;

pub mod panic;

const NUM_PROCS: usize = 4;

pub static mut PROCESSES: [Option<&'static dyn kernel::process::Process>; NUM_PROCS] =
    [None; NUM_PROCS];

/// Chip 구조체 인스턴스
pub static mut CHIP: Option<&'static S32K144<'static, S32K144InterruptService<'static>>> = None;

static mut PROCESS_PRINTER: Option<&'static capsules_system::process_printer::ProcessPrinterText> =
    None;

/// 프로세스가 크래시할 경우 패닉 발생
///const FAULT_RESPONSE: capsules_system::process_policies::PanicFaultPolicy =
///    capsules_system::process_policies::PanicFaultPolicy {};

struct CriticalSectionImpl;
critical_section::set_impl!(CriticalSectionImpl);

unsafe impl Impl for CriticalSectionImpl {
    unsafe fn acquire() -> RawRestoreState {
        cortex_m::interrupt::disable();
        RawRestoreState::default()
    }

    unsafe fn release(_state: RawRestoreState) {
        cortex_m::interrupt::enable();
    }
}

pub struct RgbLed<'a> {
    ptd: &'a PTD,
    portd: &'a PORTD,
}

impl<'a> RgbLed<'a> {
    const RED_PIN: u32 = 15;
    const BLUE_PIN: u32 = 0;

    pub fn init(
        ptd: &'a s32k144::ptd::RegisterBlock,
        portd: &'a s32k144::portd::RegisterBlock,
    ) -> Self {
        // GPIO 모드 설정
        portd.pcr0.modify(|_, w| w.mux().bits(0b001)); // BLUE LED
        portd.pcr15.modify(|_, w| w.mux().bits(0b001)); // RED LED

        // 출력 핀으로 설정
        ptd.pddr.modify(|r, w| unsafe {
            w.bits(r.bits() | (1 << Self::RED_PIN) | (1 << Self::BLUE_PIN))
        });

        RgbLed { ptd, portd }
    }

    pub fn set_red(&self, state: bool) {
        if state {
            self.ptd
                .pcor
                .write(|w| unsafe { w.ptco().bits(1 << Self::RED_PIN) });
        } else {
            self.ptd
                .psor
                .write(|w| unsafe { w.ptso().bits(1 << Self::RED_PIN) });
        }
    }

    pub fn set_blue(&self, state: bool) {
        if state {
            self.ptd
                .pcor
                .write(|w| unsafe { w.ptco().bits(1 << Self::BLUE_PIN) });
        } else {
            self.ptd
                .psor
                .write(|w| unsafe { w.ptso().bits(1 << Self::BLUE_PIN) });
        }
    }
}

pub struct S32K144EVB {
    console: &'static capsules_core::console::Console<'static>,
    scheduler: &'static RoundRobinSched<'static>,
    systick: SysTick,
}

impl SyscallDriverLookup for S32K144EVB {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            capsules_core::console::DRIVER_NUM => f(Some(self.console)),
            _ => f(None),
        }
    }
}
struct LpuartWrapper<'a> {
    lpuart: &'a LPUART,
}
impl<'a> kernel::hil::uart::Configure for LpuartWrapper<'a> {
    fn configure(&self, _params: kernel::hil::uart::Parameters) -> Result<(), kernel::ErrorCode> {
        Ok(())
    }
}

impl<'a> kernel::hil::uart::Transmit<'a> for LpuartWrapper<'a> {
    fn set_transmit_client(&self, _client: &'a dyn kernel::hil::uart::TransmitClient) {}

    fn transmit_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (kernel::ErrorCode, &'static mut [u8])> {
        for i in 0..len {
            self.lpuart.transmit(buffer[i]).unwrap();
        }
        Ok(())
    }

    fn transmit_word(&self, word: u32) -> Result<(), kernel::ErrorCode> {
        self.lpuart.transmit(word as u8).unwrap();
        Ok(())
    }

    fn transmit_abort(&self) -> Result<(), kernel::ErrorCode> {
        Err(kernel::ErrorCode::FAIL)
    }
}

impl<'a> kernel::hil::uart::Receive<'a> for LpuartWrapper<'a> {
    fn set_receive_client(&self, _client: &'a dyn kernel::hil::uart::ReceiveClient) {}

    fn receive_buffer(
        &self,
        buffer: &'static mut [u8],
        _len: usize,
    ) -> Result<(), (kernel::ErrorCode, &'static mut [u8])> {
        buffer[0] = self.lpuart.receive().unwrap();
        Ok(())
    }

    fn receive_word(&self) -> Result<(), kernel::ErrorCode> {
        self.lpuart.receive().unwrap();
        Ok(())
    }

    fn receive_abort(&self) -> Result<(), kernel::ErrorCode> {
        Err(kernel::ErrorCode::FAIL)
    }
}

impl KernelResources<S32K144<'static, S32K144InterruptService<'static>>> for S32K144EVB {
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type Scheduler = RoundRobinSched<'static>;
    type SchedulerTimer = SysTick;
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

#[inline(never)]
unsafe fn start() -> (
    &'static kernel::Kernel,
    S32K144EVB,
    &'static S32K144<'static, S32K144InterruptService<'static>>,
    RgbLed<'static>,
    &'static LpuartWrapper<'static>,
) {
    rtt_init_print!();
    rprintln!("RTT Ready!");
    debug!("READY");

    let peripherals = static_init!(S32K144DefaultPeripherals, S32K144DefaultPeripherals::new());
    let interrupt_service = static_init!(
        S32K144InterruptService,
        S32K144InterruptService::new(peripherals)
    );

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));
    let chip = static_init!(
        S32K144<S32K144InterruptService>,
        S32K144::new(interrupt_service)
    );

    peripherals.init();

    peripherals.wdog.disable();

    interrupt_service.init();

    let led = RgbLed::init(
        peripherals.ptd.get_registers(),
        peripherals.portd.get_registers(),
    );

    static mut UART_BUFFER: [u8; 1024] = [0; 1024];

    let lpuart_wrapper = static_init!(
        LpuartWrapper,
        LpuartWrapper {
            lpuart: &peripherals.lpuart0
        }
    );

    let uart_mux = static_init!(
        MuxUart<'static>,
        MuxUart::new(
            lpuart_wrapper as &dyn kernel::hil::uart::Uart,
            unsafe { &mut UART_BUFFER },
            115200
        )
    );

    lpuart_wrapper.lpuart.set_client(uart_mux, uart_mux);

    let uart_device = static_init!(UartDevice<'static>, UartDevice::new(uart_mux, true));

    //--------------------------------------------------------------------------
    // ALARM & TIMER
    //--------------------------------------------------------------------------

    let rtc = peripherals.rtc;
    let _ = rtc.start();

    let mux_alarm = components::alarm::AlarmMuxComponent::new(rtc)
        .finalize(components::alarm_mux_component_static!(s32k144::rtc::RTC));
    let alarm = components::alarm::AlarmDriverComponent::new(
        board_kernel,
        capsules_core::alarm::DRIVER_NUM,
        mux_alarm,
    )
    .finalize(components::alarm_component_static!(s32k144::rtc::RTC));

    //--------------------------------------------------------------------------
    // PROCESS CONSOLE
    //--------------------------------------------------------------------------

    let process_printer = components::process_printer::ProcessPrinterTextComponent::new()
        .finalize(components::process_printer_text_component_static!());
    PROCESS_PRINTER = Some(process_printer);

    let _process_console = components::process_console::ProcessConsoleComponent::new(
        board_kernel,
        uart_mux,
        mux_alarm,
        process_printer,
        Some(cortexm4::support::reset),
    )
    .finalize(components::process_console_component_static!(
        s32k144::rtc::RTC
    ));

    let _ = _process_console.start();

    let scheduler = static_init!(RoundRobinSched, RoundRobinSched::new());

    let board = S32K144EVB {
        console: static_init!(
            capsules_core::console::Console<'static>,
            capsules_core::console::Console::new(
                uart_device,
                static_init!([u8; 1024], [0; 1024]),
                static_init!([u8; 1024], [0; 1024]),
                board_kernel.create_grant(
                    capsules_core::console::DRIVER_NUM,
                    &create_capability!(capabilities::MemoryAllocationCapability),
                ),
            )
        ),
        scheduler,
        systick: SysTick::new(),
    };

    (board_kernel, board, chip, led, lpuart_wrapper)
}

#[no_mangle]
pub unsafe fn main() {
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);

    let (board_kernel, board, chip, led, uart) = start();

    debug!("S32K144EVB booting...");

    // LED test
    led.set_red(true);
    for _ in 0..10_000_000 {
        cortex_m::asm::nop();
    } // delay
    led.set_blue(true);

    uart.lpuart.transmit(b'H').unwrap();
    uart.lpuart.transmit(b'E').unwrap();
    uart.lpuart.transmit(b'L').unwrap();
    uart.lpuart.transmit(b'L').unwrap();
    uart.lpuart.transmit(b'O').unwrap();

    board_kernel.kernel_loop(
        &board,
        chip,
        None::<&kernel::ipc::IPC<{ NUM_PROCS as u8 }>>,
        &main_loop_capability,
    );
}
