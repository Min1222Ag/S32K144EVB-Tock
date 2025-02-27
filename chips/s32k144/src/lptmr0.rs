use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::time::{Alarm, Ticks, Time};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const LPTMR0_BASE: StaticRef<Lptmr0Registers> =
    unsafe { StaticRef::new(0x40040000 as *const Lptmr0Registers) };

#[repr(C)]
pub struct Lptmr0Registers {
    pub csr: ReadWrite<u32, CSR::Register>,
    pub psr: ReadWrite<u32, PSR::Register>,
    pub cmr: ReadWrite<u32, CMR::Register>,
    pub cnr: ReadWrite<u32>,
}

register_bitfields![u32,
    CSR [
        TEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TMS OFFSET(1) NUMBITS(1) [
            TimeCounter = 0,
            PulseCounter = 1
        ],
        TFC OFFSET(2) NUMBITS(1) [
            ResetOnTCF = 0,
            ResetOnOverflow = 1
        ],
        TPP OFFSET(3) NUMBITS(1) [
            ActiveHigh = 0,
            ActiveLow = 1
        ],
        TPS OFFSET(4) NUMBITS(2) [
            Input0 = 0b00,
            Input1 = 0b01,
            Input2 = 0b10,
            Input3 = 0b11
        ],
        TIE OFFSET(6) NUMBITS(1) [
            InterruptDisabled = 0,
            InterruptEnabled = 1
        ],
        TCF OFFSET(7) NUMBITS(1) [
            NotEqual = 0,
            Equal = 1
        ],
        TDRE OFFSET(8) NUMBITS(1) [
            DmaDisabled = 0,
            DmaEnabled = 1
        ]
    ],
    PSR [
        PCS OFFSET(0) NUMBITS(2) [
            Clock0 = 0,
            Clock1 = 1,
            Clock2 = 2,
            Clock3 = 3
        ],
        PBYP OFFSET(2) NUMBITS(1) [
            Enabled = 0,
            Bypassed = 1
        ],
        PRESCALE OFFSET(3) NUMBITS(4) [
            DivideBy2 = 0,
            DivideBy4 = 1,
            DivideBy8 = 2,
            DivideBy16 = 3,
            DivideBy32 = 4,
            DivideBy64 = 5,
            DivideBy128 = 6,
            DivideBy256 = 7,
            DivideBy512 = 8,
            DivideBy1024 = 9,
            DivideBy2048 = 10,
            DivideBy4096 = 11,
            DivideBy8192 = 12,
            DivideBy16384 = 13,
            DivideBy32768 = 14,
            DivideBy65536 = 15
        ]
    ],
    CMR [
        COMPARE OFFSET(0) NUMBITS(16) []
    ],
    CNR [
        COUNTER OFFSET(0) NUMBITS(16) []
    ]
];

pub struct TimerAlarm<'a> {
    registers: StaticRef<Lptmr0Registers>,
    client: OptionalCell<&'a dyn kernel::hil::time::AlarmClient>,
    deferred_call: OptionalCell<DeferredCall>,
}

impl<'a> TimerAlarm<'a> {
    pub const fn new() -> TimerAlarm<'a> {
        TimerAlarm {
            registers: LPTMR0_BASE,
            client: OptionalCell::empty(),
            deferred_call: OptionalCell::empty(), // DeferredCall을 나중에 초기화
        }
    }

    pub fn initialize(&self) {
        self.deferred_call.set(DeferredCall::new());
    }

    pub fn handle_interrupt(&self) {
        // Clear interrupt flag
        self.registers.csr.modify(CSR::TCF::NotEqual);

        // Notify the alarm client
        self.client.map(|client| {
            client.alarm();
        });
    }
}

impl Time for TimerAlarm<'_> {
    type Frequency = kernel::hil::time::Freq16KHz;
    type Ticks = kernel::hil::time::Ticks32;

    fn now(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.cnr.get())
    }
}

impl<'a> Alarm<'a> for TimerAlarm<'a> {
    fn set_alarm_client(&self, client: &'a dyn kernel::hil::time::AlarmClient) {
        self.client.set(client);
    }

    fn set_alarm(&self, reference: Self::Ticks, dt: Self::Ticks) {
        let expire = reference.wrapping_add(dt);

        self.registers
            .cmr
            .write(CMR::COMPARE.val(expire.into_u32()));

        // Enable interrupt
        self.registers.csr.modify(CSR::TIE::InterruptEnabled);
    }

    fn get_alarm(&self) -> Self::Ticks {
        Self::Ticks::from(self.registers.cmr.read(CMR::COMPARE))
    }

    fn disarm(&self) -> Result<(), ErrorCode> {
        self.registers.csr.modify(CSR::TIE::InterruptDisabled);
        Ok(())
    }

    fn is_armed(&self) -> bool {
        self.registers.csr.is_set(CSR::TIE)
    }

    fn minimum_dt(&self) -> Self::Ticks {
        Self::Ticks::from(1)
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
