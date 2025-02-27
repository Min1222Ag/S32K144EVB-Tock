use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::time::{Alarm, Ticks, Time};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const LPIT0_BASE: StaticRef<Lpit0Registers> =
    unsafe { StaticRef::new(0x40037000 as *const Lpit0Registers) };

#[repr(C)]
pub struct Lpit0Registers {
    pub mcr: ReadWrite<u32, MCR::Register>,
    pub mier: ReadWrite<u32, MIER::Register>,
    pub setten: ReadWrite<u32>,
    pub clrten: ReadWrite<u32>,
    pub tval: [ReadWrite<u32>; 4],
    pub cval: [ReadWrite<u32>; 4],
    pub tctrl: [ReadWrite<u32, TCTRL::Register>; 4],
}

register_bitfields![u32,
    MCR[
        M_CEN OFFSET(0) NUMBITS(1) [Disabled = 0, Enabled = 1],
        DBG_EN OFFSET(1) NUMBITS(1) [Disabled = 0, Enabled = 1]
    ],
    MIER[
        TIE0 OFFSET(0) NUMBITS(1) [Disabled = 0, Enabled = 1],
        TIE1 OFFSET(1) NUMBITS(1) [Disabled = 0, Enabled = 1]
    ],
    TCTRL[
        T_EN OFFSET(0) NUMBITS(1) [Disabled = 0, Enabled = 1],
        MODE OFFSET(1) NUMBITS(1) [Periodic = 0, OneShot = 1]
    ]
];

/// **Lpit0Timer 구조체 정의**
/// - LPIT 타이머의 기본적인 인터페이스를 제공하는 구조체
/// - AlarmClient를 설정할 수 있음
pub struct Lpit0Timer<'a> {
    registers: StaticRef<Lpit0Registers>,
    client: OptionalCell<&'a dyn kernel::hil::time::AlarmClient>,
    deferred_call: OptionalCell<DeferredCall>,
}

impl<'a> Lpit0Timer<'a> {
    pub const fn new() -> Lpit0Timer<'a> {
        Lpit0Timer {
            registers: LPIT0_BASE,
            client: OptionalCell::empty(),
            deferred_call: OptionalCell::empty(),
        }
    }

    pub fn initialize(&self) {
        self.deferred_call.set(DeferredCall::new());
    }

    pub fn handle_interrupt(&self) {
        self.registers.mier.modify(MIER::TIE0::Disabled);
        self.client.map(|client| client.alarm());
    }
}

impl Time for Lpit0Timer<'_> {
    type Frequency = kernel::hil::time::Freq16KHz;
    type Ticks = kernel::hil::time::Ticks32;

    fn now(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.cval[0].get())
    }
}

impl<'a> Alarm<'a> for Lpit0Timer<'a> {
    fn set_alarm_client(&self, client: &'a dyn kernel::hil::time::AlarmClient) {
        self.client.set(client);
    }

    fn set_alarm(&self, reference: Self::Ticks, dt: Self::Ticks) {
        let expire = reference.wrapping_add(dt);
        self.registers.tval[0].set(expire.into_u32());
        self.registers.tctrl[0].modify(TCTRL::T_EN::Enabled);
        self.registers.mier.modify(MIER::TIE0::Enabled);
    }

    fn get_alarm(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.tval[0].get())
    }

    fn disarm(&self) -> Result<(), ErrorCode> {
        self.registers.tctrl[0].modify(TCTRL::T_EN::Disabled);
        Ok(())
    }

    fn is_armed(&self) -> bool {
        self.registers.tctrl[0].is_set(TCTRL::T_EN)
    }

    fn minimum_dt(&self) -> Self::Ticks {
        Self::Ticks::from(10)
    }
}

impl<'a> DeferredCallClient for Lpit0Timer<'a> {
    fn handle_deferred_call(&self) {
        self.client.map(|client| client.alarm());
    }

    fn register(&'static self) {
        if let Some(dc) = self.deferred_call.take() {
            dc.register(self); // 실제 DeferredCall에 register 호출
            self.deferred_call.set(dc); // 다시 OptionalCell에 저장
        }
    }
}

/// **TimerAlarm 구조체 정의**
/// - `Lpit0Timer`와 유사하지만 `DeferredCallClient` 트레이트를 구현하여
///   소프트웨어 인터럽트(지연 호출)를 지원
pub struct TimerAlarm<'a> {
    registers: StaticRef<Lpit0Registers>,
    client: OptionalCell<&'a dyn kernel::hil::time::AlarmClient>,
    deferred_call: OptionalCell<DeferredCall>,
}

impl<'a> TimerAlarm<'a> {
    pub const fn new() -> TimerAlarm<'a> {
        TimerAlarm {
            registers: LPIT0_BASE,
            client: OptionalCell::empty(),
            deferred_call: OptionalCell::empty(), // DeferredCall을 나중에 초기화
        }
    }

    pub fn initialize(&self) {
        self.deferred_call.set(DeferredCall::new());
    }

    pub fn handle_interrupt(&self) {
        self.registers.mier.modify(MIER::TIE0::Disabled);

        if let Some(dc) = self.deferred_call.take() {
            dc.set();
            self.deferred_call.set(dc);
        }
    }
}

impl<'a> Time for TimerAlarm<'a> {
    type Frequency = kernel::hil::time::Freq16KHz;
    type Ticks = kernel::hil::time::Ticks32;

    fn now(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.cval[0].get())
    }
}

impl<'a> Alarm<'a> for TimerAlarm<'a> {
    fn set_alarm_client(&self, client: &'a dyn kernel::hil::time::AlarmClient) {
        self.client.set(client);
    }

    fn set_alarm(&self, reference: Self::Ticks, dt: Self::Ticks) {
        let expire = reference.wrapping_add(dt);
        self.registers.tval[0].set(expire.into_u32());
        self.registers.tctrl[0].modify(TCTRL::T_EN::Enabled);
        self.registers.mier.modify(MIER::TIE0::Enabled);
    }

    fn get_alarm(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.tval[0].get())
    }

    fn disarm(&self) -> Result<(), ErrorCode> {
        self.registers.tctrl[0].modify(TCTRL::T_EN::Disabled);
        Ok(())
    }

    fn is_armed(&self) -> bool {
        self.registers.tctrl[0].is_set(TCTRL::T_EN)
    }

    fn minimum_dt(&self) -> Self::Ticks {
        Self::Ticks::from(10)
    }
}

impl<'a> DeferredCallClient for TimerAlarm<'a> {
    fn handle_deferred_call(&self) {
        self.client.map(|client| client.alarm());
    }

    fn register(&'static self) {
        if let Some(dc) = self.deferred_call.take() {
            dc.register(self); // 실제 DeferredCall에 register 호출
            self.deferred_call.set(dc); // 다시 OptionalCell에 저장
        }
    }
}
