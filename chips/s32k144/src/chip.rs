// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use core::fmt::Write;
use cortexm4f::{nvic, CortexM4F, CortexMVariant};
use kernel::platform::chip::InterruptService;

pub struct S32K144<'a, I: InterruptService + 'a> {
    mpu: cortexm4f::mpu::MPU,
    userspace_kernel_boundary: cortexm4f::syscall::SysCall,
    interrupt_service: &'a I,
}

impl<'a, I: InterruptService + 'a> S32K144<'a, I> {
    pub unsafe fn new(interrupt_service: &'a I) -> Self {
        Self {
            mpu: cortexm4f::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm4f::syscall::SysCall::new(),
            interrupt_service,
        }
    }
}

/// This struct, when initialized, instantiates all peripheral drivers for the nrf52.
///
/// If a board wishes to use only a subset of these peripherals, this
/// should not be used or imported, and a modified version should be
/// constructed manually in main.rs.
pub struct S32K144DefaultPeripherals<'a> {
    pub pwr_clk: crate::power::Power<'a>,
    pub rtc: crate::rtc::Rtc<'a>,
    pub timer0: crate::lpit0::TimerAlarm<'a>,
    pub timer1: crate::lptmr0::TimerAlarm<'a>,
    pub timer2: crate::lpit0::Lpit0Timer<'a>,
    pub lpuart0: crate::lpuart0::Lpuart0<'a>,
    pub nvmc: crate::nvmc::Ftfc,
    pub clock: crate::clock::Clock,
    pub pwm0: crate::ftm0::Pwm,
    pub gpio_port: crate::gpio::Port<'a, 48>,
}

impl S32K144DefaultPeripherals<'_> {
    pub fn new() -> Self {
        Self {
            pwr_clk: crate::power::Power::new(),
            rtc: crate::rtc::Rtc::new(),
            timer0: crate::lpit0::TimerAlarm::new(),
            timer1: crate::lptmr0::TimerAlarm::new(),
            timer2: crate::lpit0::Lpit0Timer::new(),
            lpuart0: crate::lpuart0::Lpuart0::new(crate::lpuart0::LPUART0_BASE),
            nvmc: crate::nvmc::Ftfc::new(),
            clock: crate::clock::Clock::new(),
            pwm0: crate::ftm0::Pwm::new(),
            gpio_port: crate::gpio::s32k144_gpio_create(),
        }
    }
    // Necessary for setting up circular dependencies
    pub fn init(&'static self) {
        kernel::deferred_call::DeferredCallClient::register(&self.nvmc);
    }
}
impl kernel::platform::chip::InterruptService for S32K144DefaultPeripherals<'_> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            crate::peripheral_interrupts::POWER_CLOCK => self.pwr_clk.handle_interrupt(),
            crate::peripheral_interrupts::RTC => self.rtc.handle_interrupt(),
            crate::peripheral_interrupts::TIMER0 => self.timer0.handle_interrupt(),
            crate::peripheral_interrupts::TIMER1 => self.timer1.handle_interrupt(),
            crate::peripheral_interrupts::TIMER2 => self.timer2.handle_interrupt(),
            crate::peripheral_interrupts::LPUART0 => self.lpuart0.handle_interrupt(),
            _ => return false,
        }
        true
    }
}

impl<'a, I: InterruptService + 'a> kernel::platform::chip::Chip for S32K144<'a, I> {
    type MPU = cortexm4f::mpu::MPU;
    type UserspaceKernelBoundary = cortexm4f::syscall::SysCall;

    fn mpu(&self) -> &Self::MPU {
        &self.mpu
    }

    fn userspace_kernel_boundary(&self) -> &Self::UserspaceKernelBoundary {
        &self.userspace_kernel_boundary
    }

    fn service_pending_interrupts(&self) {
        unsafe {
            loop {
                if let Some(interrupt) = nvic::next_pending() {
                    if !self.interrupt_service.service_interrupt(interrupt) {
                        panic!("unhandled interrupt {}", interrupt);
                    }
                    let n = nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { nvic::has_pending() }
    }

    fn sleep(&self) {
        unsafe {
            cortexm4f::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm4f::support::atomic(f)
    }

    unsafe fn print_state(&self, write: &mut dyn Write) {
        CortexM4F::print_cortexm_state(write);
    }
}
