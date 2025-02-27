// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! RTC driver for S32K144

use core::cell::Cell;
use kernel::hil::time::{self, Alarm, Ticks, Time};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

const RTC_BASE: StaticRef<RtcRegisters> =
    unsafe { StaticRef::new(0x4003D000 as *const RtcRegisters) };

#[repr(C)]
struct RtcRegisters {
    /// RTC Time Seconds Register
    tsr: ReadWrite<u32, TSR::Register>,
    /// RTC Time Prescaler Register
    tpr: ReadWrite<u32, TPR::Register>,
    /// RTC Time Alarm Register
    tar: ReadWrite<u32, TAR::Register>,
    /// RTC Time Compensation Register
    tcr: ReadWrite<u32, TCR::Register>,
    /// RTC Control Register
    cr: ReadWrite<u32, CR::Register>,
    /// RTC Status Register
    sr: ReadWrite<u32, SR::Register>,
    /// RTC Lock Register
    lr: ReadWrite<u32, LR::Register>,
    /// RTC Interrupt Enable Register
    ier: ReadWrite<u32, IER::Register>,
}

register_bitfields![u32,
    TSR [
        TSR OFFSET(0) NUMBITS(32) []
    ],
    TPR [
        TPR OFFSET(0) NUMBITS(16) []
    ],
    TAR [
        TAR OFFSET(0) NUMBITS(32) []
    ],
    TCR [
        TCR OFFSET(0) NUMBITS(8) [],
        CIR OFFSET(8) NUMBITS(8) [],
        TCV OFFSET(16) NUMBITS(8) [],
        CIC OFFSET(24) NUMBITS(8) []
    ],
    CR [
        SWR OFFSET(0) NUMBITS(1) [
            NoEffect = 0
        ],
        SUP OFFSET(2) NUMBITS(1) [
            NonSupervisor = 0,
            Supervisor = 1
        ],
        UM OFFSET(3) NUMBITS(1) [
            Locked = 0,
            Unlocked = 1
        ],
        CPS OFFSET(5) NUMBITS(1) [
            PrescalerOutput = 0,
            RTC32KHz = 1
        ],
        LPOS OFFSET(7) NUMBITS(1) [
            RTC32KHz = 0,
            RTC1KHz = 1
        ],

        CLKO OFFSET(9) NUMBITS(1)[
            OutputToPeripherals = 0,
            NotOutputToPeripherals = 1
        ],

        CPE OFFSET(24) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    SR[
        TIF OFFSET(0) NUMBITS(1)[],
        TOF OFFSET(1) NUMBITS(1)[],
        TAF OFFSET(2) NUMBITS(1)[],
        TCE OFFSET(4) NUMBITS(1)[],
    ],
    LR[
        TCL OFFSET(3) NUMBITS(1)[],
        CRL OFFSET(4) NUMBITS(1)[],
        SRL OFFSET(5) NUMBITS(1)[],
        LRL OFFSET(6) NUMBITS(1)[],
    ],
    IER[
        TIIE OFFSET(0) NUMBITS(1)[],
        TOIE OFFSET(1) NUMBITS(1)[],
        TAIE OFFSET(2) NUMBITS(1)[],
        TSIE OFFSET(4) NUMBITS(1)[],
        TSIC OFFSET(16) NUMBITS(3)[]
    ]
];

pub struct Rtc<'a> {
    registers: StaticRef<RtcRegisters>,
    overflow_client: OptionalCell<&'a dyn time::OverflowClient>,
    alarm_client: OptionalCell<&'a dyn time::AlarmClient>,
    enabled: Cell<bool>,
}

impl Rtc<'_> {
    pub const fn new() -> Self {
        Self {
            registers: RTC_BASE,
            overflow_client: OptionalCell::empty(),
            alarm_client: OptionalCell::empty(),
            enabled: Cell::new(false),
        }
    }

    pub fn handle_interrupt(&self) {
        let regs = &*self.registers;

        if regs.sr.is_set(SR::TIF) {
            regs.sr.modify(SR::TIF::CLEAR);
            self.overflow_client.map(|client| client.overflow());
        }
        if regs.sr.is_set(SR::TAF) {
            regs.sr.modify(SR::TAF::CLEAR);
            self.alarm_client.map(|client| client.alarm());
        }
    }
}

impl Time for Rtc<'_> {
    type Frequency = time::Freq32KHz;
    type Ticks = time::Ticks32;

    fn now(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.tsr.read(TSR::TSR))
    }
}

impl<'a> time::Counter<'a> for Rtc<'a> {
    fn set_overflow_client(&self, client: &'a dyn time::OverflowClient) {
        self.overflow_client.set(client);
        self.registers.ier.modify(IER::TOIE::SET);
    }

    fn start(&self) -> Result<(), ErrorCode> {
        self.registers.tpr.write(TPR::TPR.val(0));
        self.registers.sr.modify(SR::TCE::SET);
        self.enabled.set(true);
        Ok(())
    }

    fn stop(&self) -> Result<(), ErrorCode> {
        self.registers.sr.modify(SR::TCE::CLEAR);
        self.enabled.set(false);
        Ok(())
    }

    fn reset(&self) -> Result<(), ErrorCode> {
        self.registers.tsr.write(TSR::TSR.val(0));
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.enabled.get()
    }
}

impl<'a> Alarm<'a> for Rtc<'a> {
    fn set_alarm_client(&self, client: &'a dyn time::AlarmClient) {
        self.alarm_client.set(client);
    }

    fn set_alarm(&self, reference: Self::Ticks, dt: Self::Ticks) {
        let regs = &*self.registers;
        let expire = reference.wrapping_add(dt);
        regs.tar.write(TAR::TAR.val(expire.into_u32()));
        regs.ier.modify(IER::TAIE::SET);
    }

    fn get_alarm(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.tar.read(TAR::TAR))
    }

    fn disarm(&self) -> Result<(), ErrorCode> {
        self.registers.ier.modify(IER::TAIE::CLEAR);
        Ok(())
    }

    fn is_armed(&self) -> bool {
        self.registers.ier.is_set(IER::TAIE)
    }

    fn minimum_dt(&self) -> Self::Ticks {
        Self::Ticks::from(1)
    }
}
