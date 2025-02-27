// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024.

// S32K144 Interrupt Vector Mapping

pub const PORTA: u32 = 59;
pub const PORTB: u32 = 60;
pub const PORTC: u32 = 61;
pub const PORTD: u32 = 62;
pub const PORTE: u32 = 63;
pub const LPUART0: u32 = 31;
pub const RTC: u32 = 46;
//pub const RTC_ALARM: u32 = 46;
//pub const RTC_SEC: u32 = 47;
pub const FTM0: u32 = 99;
//pub const FTM0_CH0: u32 = 99;
//pub const FTM0_CH1: u32 = 99;
//pub const FTM0_CH2: u32 = 100;
//pub const FTM0_CH3: u32 = 100;
//pub const FTM0_CH4: u32 = 101;
//pub const FTM0_CH5: u32 = 101;
//pub const FTM0_CH6: u32 = 102;
//pub const FTM0_CH7: u32 = 102;
//pub const FTM0_FAULT: u32 = 103;
//pub const FTM0_OF: u32 = 104;
//pub const FTM0_RELOAD: u32 = 104;
pub const TIMER0: u32 = 48; // Low Power Periodic Timer
pub const TIMER1: u32 = 49; // Low Power Periodic Timer
pub const TIMER2: u32 = 50; // Low Power Periodic Timer
pub const POWER_CLOCK: u32 = 20; //pmc:20
pub const FTFC: u32 = 18; //19,21
