// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

#![no_std]
#![crate_name = "s32k144"]
#![crate_type = "rlib"]

//pub mod can0;
pub mod chip;
pub mod clock;
pub mod ftm0;
pub mod interrupt_service;
pub mod lpit0;
pub mod lptmr0;
pub mod lpuart0;
pub mod nvmc;
pub mod peripheral_interrupts;
pub mod pinmux;
//pub mod portc;
//pub mod portd;
pub mod power;
pub mod rtc;
//pub mod wdog;
pub mod gpio;
