use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;

register_structs! {
    PmcRegisters {
        (0x00 => lvdsc1: ReadWrite<u8, LVDSC1::Register>),
        (0x01 => lvdsc2: ReadWrite<u8, LVDSC2::Register>),
        (0x02 => regsc: ReadWrite<u8, REGSC::Register>),
        (0x03 => _reserved0),
        (0x04 => lpotrim: ReadWrite<u8, LPOTRIM::Register>),
        (0x05 => @END),
    }
}

register_structs! {
    SmcRegisters {
        (0x00 => verid: ReadWrite<u32, VERID::Register>),
        (0x04 => param: ReadWrite<u32, PARAM::Register>),
        (0x08 => pmprot: ReadWrite<u32, PMPROT::Register>),
        (0x0C => pmctrl: ReadWrite<u32, PMCTRL::Register>),
        (0x10 => stopctrl: ReadWrite<u32, STOPCTRL::Register>),
        (0x14 => pmstat: ReadWrite<u32, PMSTAT::Register>),
        (0x18 => @END),
    }
}

const PMC_BASE: StaticRef<PmcRegisters> =
    unsafe { StaticRef::new(0x4007D000 as *const PmcRegisters) };

const SMC_BASE: StaticRef<SmcRegisters> =
    unsafe { StaticRef::new(0x4007E000 as *const SmcRegisters) };

//pmc.rs
register_bitfields![u8,
    LVDSC1 [
        LVDV OFFSET(0) NUMBITS(2) [],
        LVDRE OFFSET(4) NUMBITS(1) [],
        LVDIE OFFSET(5) NUMBITS(1) [],
        LVDACK OFFSET(6) NUMBITS(1) [],
        LVDF OFFSET(7) NUMBITS(1) [
            NoLowVoltage = 0,
            LowVoltageDetected = 1]
    ],
    LVDSC2 [
        LVWV OFFSET(0) NUMBITS(2) [
            LowTripPoint = 0b00,
            Mid1TripPoint = 0b01,
            Mid2TripPoint = 0b10,
            HighTripPoint = 0b11
        ],
        LVWIE OFFSET(5) NUMBITS(1) [
            InterruptDisabled = 0,
            InterruptEnabled = 1
        ],
        LVWACK OFFSET(6) NUMBITS(1) [
            NoAcknowledge = 0,
            Acknowledge = 1
        ],
        LVWF OFFSET(7) NUMBITS(1) [
            NoWarning = 0,
            WarningDetected = 1
        ]
    ],
    REGSC [
        BIASEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CLKBIASDIS OFFSET(1) NUMBITS(1) [
            NoEffect = 0,
            DisabledInVLPS = 1
        ],
        REGFPM OFFSET(2) NUMBITS(1) [
            LowPowerMode = 0,
            FullPerformanceMode = 1
        ],
        REGONS OFFSET(3) NUMBITS(1) [
            RegulatorOff = 0,
            RegulatorOn = 1
        ],
        ACKISO OFFSET(4) NUMBITS(1) [
            PeripheralsIsolated = 0,
            PeripheralsNotIsolated = 1
        ],
        BGEN OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        LPOSTAT OFFSET(6) NUMBITS(1) [
            LowPhase = 0,
            HighPhase = 1
        ],
        LPODIS OFFSET(7) NUMBITS(1) [
            Enabled = 0,
            Disabled = 1
        ]
    ],
    LPOTRIM [
        LPOTRIM OFFSET(0) NUMBITS(5) []
    ]
];

//smc.rs
register_bitfields![u32,
    VERID [
        FEATURE OFFSET(0) NUMBITS(16) [], // Feature Specification Number
        MINOR OFFSET(16) NUMBITS(8) [],   // Minor Version Number
        MAJOR OFFSET(24) NUMBITS(8) []    // Major Version Number
    ],
    PARAM [
        EHSRUN OFFSET(0) NUMBITS(1) [
            NotAvailable = 0,
            Available = 1
        ],
        ELLS OFFSET(3) NUMBITS(1) [
            NotAvailable = 0,
            Available = 1
        ],
        ELLS2 OFFSET(5) NUMBITS(1) [
            NotAvailable = 0,
            Available = 1
        ],
        EVLLS0 OFFSET(6) NUMBITS(1) [
            NotAvailable = 0,
            Available = 1
        ]
    ],
    PMPROT [
        AVLP OFFSET(5) NUMBITS(1) [
            NotAllowed = 0,
            Allowed = 1
        ],
        AHSRUN OFFSET(7) NUMBITS(1) [
            NotAllowed = 0,
            Allowed = 1
        ]
    ],
    PMCTRL [
        STOPM OFFSET(0) NUMBITS(3) [
            NormalStop = 0b000,
            VeryLowPowerStop = 0b010,
            Reserved = 0b110
        ],
        VLPSA OFFSET(3) NUMBITS(1) [
            StopSuccessful = 0,
            StopAborted = 1
        ],
        RUNM OFFSET(5) NUMBITS(2) [
            NormalRun = 0b00,
            VeryLowPowerRun = 0b10,
            HighSpeedRun = 0b11
        ]
    ],
    STOPCTRL [
        STOPO OFFSET(6) NUMBITS(2) [
            Stop1 = 0b01, // Stop with both system and bus clocks disabled
            Stop2 = 0b10  // Stop with system clock disabled and bus clock enabled
        ]
    ],
    PMSTAT [
        PMSTAT OFFSET(0) NUMBITS(8) [] // Power Mode Status
    ]
];

pub enum PowerEvent {
    LowVoltageDetected,
    LowVoltageWarning,
    EnteredLowPowerMode,
    ExitedLowPowerMode,
}
/// 전력 관리 이벤트를 수신할 클라이언트 트레이트
pub trait PowerClient {
    fn handle_power_event(&self, event: PowerEvent);
}

/// PMC + SMC를 관리하는 Power 구조체
pub struct Power<'a> {
    pmc: StaticRef<PmcRegisters>,
    smc: StaticRef<SmcRegisters>,
    client: OptionalCell<&'a dyn PowerClient>,
}

impl<'a> Power<'a> {
    pub const fn new() -> Self {
        Power {
            pmc: PMC_BASE,
            smc: SMC_BASE,
            client: OptionalCell::empty(),
        }
    }

    /// 클라이언트 설정
    pub fn set_client(&self, client: &'a dyn PowerClient) {
        self.client.set(client);
    }

    /// 전력 관리 인터럽트 핸들러
    pub fn handle_interrupt(&self) {
        // Low Voltage Detect (LVDF) 발생 여부 확인
        if self.pmc.lvdsc1.is_set(LVDSC1::LVDF) {
            self.pmc.lvdsc1.modify(LVDSC1::LVDF::CLEAR);
            self.client
                .map(|client| client.handle_power_event(PowerEvent::LowVoltageDetected));
        }

        // Low Voltage Warning (LVWF) 발생 여부 확인
        if self.pmc.lvdsc2.is_set(LVDSC2::LVWF) {
            self.pmc.lvdsc2.modify(LVDSC2::LVWF::CLEAR);
            self.client
                .map(|client| client.handle_power_event(PowerEvent::LowVoltageWarning));
        }
    }

    /// 저전력 모드로 전환 (VLPR 모드)
    pub fn enter_low_power_mode(&self) {
        self.smc.pmctrl.modify(PMCTRL::RUNM::VeryLowPowerRun);
        self.client
            .map(|client| client.handle_power_event(PowerEvent::EnteredLowPowerMode));
    }

    /// 현재 전력 모드 상태 확인
    pub fn get_power_mode_status(&self) -> u8 {
        self.smc.pmstat.read(PMSTAT::PMSTAT) as u8
    }
}
