// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024.

use crate::chip::S32K144DefaultPeripherals;

/// This struct initializes all peripheral drivers for the S32K144.
/// If a board wishes to use only a subset of these peripherals, this
/// should be modified accordingly in main.rs.
pub struct S32K144InterruptService<'a> {
    pub peripherals: &'a S32K144DefaultPeripherals<'a>,
}

impl<'a> S32K144InterruptService<'a> {
    pub unsafe fn new(peripherals: &'a S32K144DefaultPeripherals<'a>) -> Self {
        Self { peripherals }
    }

    /// Necessary for setting up circular dependencies.
    pub fn init(&'static self) {
        // Register deferred call clients for necessary peripherals
        kernel::deferred_call::DeferredCallClient::register(&self.peripherals.nvmc);
        kernel::deferred_call::DeferredCallClient::register(&self.peripherals.timer0);
        kernel::deferred_call::DeferredCallClient::register(&self.peripherals.timer1);
        kernel::deferred_call::DeferredCallClient::register(&self.peripherals.timer2);

        // Initialize peripherals
        self.peripherals.init();
    }
}

impl kernel::platform::chip::InterruptService for S32K144InterruptService<'_> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            crate::peripheral_interrupts::POWER_CLOCK => {
                self.peripherals.pwr_clk.handle_interrupt()
            }
            crate::peripheral_interrupts::RTC => self.peripherals.rtc.handle_interrupt(),
            crate::peripheral_interrupts::TIMER0 => self.peripherals.timer0.handle_interrupt(),
            crate::peripheral_interrupts::TIMER1 => self.peripherals.timer1.handle_interrupt(),
            crate::peripheral_interrupts::TIMER2 => self.peripherals.timer2.handle_interrupt(),
            crate::peripheral_interrupts::LPUART0 => self.peripherals.lpuart0.handle_interrupt(),
            crate::peripheral_interrupts::FTM0 => self.peripherals.pwm0.handle_interrupt(),
            crate::peripheral_interrupts::FTFC => self.peripherals.nvmc.handle_interrupt(),
            _ => return false,
        }
        true
    }
}
