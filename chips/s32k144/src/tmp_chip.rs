// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::fmt::Write;
use cortexm4f::{nvic, CortexM4F, CortexMVariant};
use kernel::platform::chip::InterruptService;

use crate::{can0, lpi2c0, lpit0, lpspi0, lpuart0, portc, portd, ptd, rtc, wdog};

pub const LPUART0_BASE: usize = 0x4006A000;
pub const LPSPI0_BASE: usize = 0x4002C000;
pub const LPIT_BASE: usize = 0x40037000;
pub const LPI2C0_BASE: usize = 0x40066000;
pub const PORTD_BASE: usize = 0x4004C000;
pub const PORTC_BASE: usize = 0x4004B000;
pub const CAN0_BASE: usize = 0x40024000;
pub const PTD_BASE: usize = 0x40024000;
pub const WDOG_BASE: usize = 0x40052000;

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

pub struct S32K144DefaultPeripherals {
    pub timer0: &'static lpit0::LPIT,
    pub lpuart0: &'static lpuart0::LPUART,
    pub lpspi0: &'static lpspi0::LPSPI,
    pub lpi2c0: &'static lpi2c0::LPI2C,
    pub can0: &'static can0::CAN,
    pub portd: &'static portd::PORTD,
    pub portc: &'static portc::PORTC,
    pub ptd: &'static ptd::PTD,
    pub wdog: &'static wdog::WDOG,
    pub rtc: &'static rtc::Rtc,
}

impl S32K144DefaultPeripherals {
    pub fn new() -> Self {
        Self {
            timer0: unsafe { &*(LPIT_BASE as *const lpit0::LPIT) },
            lpuart0: unsafe { &*(LPUART0_BASE as *const lpuart0::LPUART) },
            lpspi0: unsafe { &*(LPSPI0_BASE as *const lpspi0::LPSPI) },
            lpi2c0: unsafe { &*(LPI2C0_BASE as *const lpi2c0::LPI2C) },
            can0: unsafe { &*(CAN0_BASE as *const can0::CAN) },
            portd: unsafe { &*(PORTD_BASE as *const portd::PORTD) },
            portc: unsafe { &*(PORTC_BASE as *const portc::PORTC) },
            ptd: unsafe { &*(PORTD_BASE as *const ptd::PTD) },
            wdog: unsafe { &*(WDOG_BASE as *const &wdog::WDOG) },
            rtc: unsafe { &*(RTC_BASE as *const &rtc::RTC) },
        }
    }

    pub fn init(&'static self) {
        self.lpuart0.init(115200, 48_000_000); //clock_freq = 8_0000_0000
        self.lpuart0.enable_tx();
        self.lpspi0.init(1_000_000, 48_000_000);
        self.lpi2c0.init(400_000, 48_000_000); //400kbps (Fast Mode)
        self.can0.init();
    }
}

impl kernel::platform::chip::InterruptService for S32K144DefaultPeripherals {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            crate::peripheral_interrupts::LPUART0 => self.lpuart0.handle_interrupt(),
            crate::peripheral_interrupts::LPSPI0 => self.lpspi0.handle_interrupt(),
            crate::peripheral_interrupts::LPI2C0 => self.lpi2c0.handle_interrupt(),
            crate::peripheral_interrupts::CAN0_ORED => self.can0.handle_interrupt(),
            crate::peripheral_interrupts::TIMER0 => self.timer0.handle_interrupt(),
            crate::peripheral_interrupts::PORTD => self.portd.handle_interrupt(),
            crate::peripheral_interrupts::PORTC => self.portc.handle_interrupt(),
            crate::peripheral_interrupts::PTD => self.ptd.handle_interrupt(),
            crate::peripheral_interrupts::WDOG => self.wdog.handle_interrupt(),
            crate::peripheral_interrupts::RTC => self.rtc.handle_interrupt(),
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
                        panic!("Unhandled interrupt {}", interrupt);
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
