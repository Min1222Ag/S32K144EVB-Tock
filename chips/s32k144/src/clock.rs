use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{register_bitfields, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;

#[repr(C)]
pub struct ScgRegisters {
    pub csr: ReadOnly<u32, CSR::Register>,
    pub rccr: ReadWrite<u32, RCCR::Register>,
    pub vccr: ReadWrite<u32, VCCR::Register>,
    pub hccr: ReadWrite<u32, HCCR::Register>,
    pub clkoutcnfg: ReadWrite<u32, CLKOUTCNFG::Register>,
    pub sosccsr: ReadWrite<u32, SOSCCSR::Register>,
    pub soscdiv: ReadWrite<u32, SOSCDIV::Register>,
    pub sosccfg: ReadWrite<u32, SOSCCFG::Register>,
    pub sirccsr: ReadWrite<u32, SIRCCSR::Register>,
    pub sircdiv: ReadWrite<u32, SIRCDIV::Register>,
    pub sirccfg: ReadWrite<u32, SIRCCFG::Register>,
    pub firccsr: ReadWrite<u32, FIRCCSR::Register>,
    pub fircdiv: ReadWrite<u32, FIRCDIV::Register>,
    pub firccfg: ReadWrite<u32, FIRCCFG::Register>,
    pub spllcsr: ReadWrite<u32, SPLLCSR::Register>,
    pub splldiv: ReadWrite<u32, SPLLDIV::Register>,
    pub spllcfg: ReadWrite<u32, SPLLCFG::Register>,
}
register_bitfields![u32,
    CSR [
        /// Slow Clock Divide Ratio (DIVSLOW)
        DIVSLOW OFFSET(0) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7
        ],

        /// Bus Clock Divide Ratio (DIVBUS)
        DIVBUS OFFSET(4) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// Core Clock Divide Ratio (DIVCORE)
        DIVCORE OFFSET(16) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// System Clock Source (SCS)
        SCS OFFSET(24) NUMBITS(4) [
            SystemOscillator = 1,
            SlowIRC = 2,
            FastIRC = 3,
            SystemPLL = 6
        ]
    ],
    RCCR [
        /// Slow Clock Divide Ratio (DIVSLOW)
        DIVSLOW OFFSET(0) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7
        ],

        /// Bus Clock Divide Ratio (DIVBUS)
        DIVBUS OFFSET(4) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// Core Clock Divide Ratio (DIVCORE)
        DIVCORE OFFSET(16) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// System Clock Source (SCS)
        SCS OFFSET(24) NUMBITS(4) [
            SystemOscillator = 1, // SOSC_CLK
            SlowIRC = 2,          // SIRC_CLK
            FastIRC = 3,          // FIRC_CLK
            SystemPLL = 6         // SPLL_CLK
        ]
    ],
    VCCR [
        /// Slow Clock Divide Ratio (DIVSLOW)
        DIVSLOW OFFSET(0) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7
        ],

        /// Bus Clock Divide Ratio (DIVBUS)
        DIVBUS OFFSET(4) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// Core Clock Divide Ratio (DIVCORE)
        DIVCORE OFFSET(16) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// System Clock Source (SCS)
        SCS OFFSET(24) NUMBITS(4) [
            SlowIRC = 2  // SIRC_CLK (Slow IRC)
        ]
    ],
    CLKOUTCNFG[
        CLKOUTSEL OFFSET(24) NUMBITS(4) []
    ],
    HCCR [
        /// Slow Clock Divide Ratio (DIVSLOW)
        DIVSLOW OFFSET(0) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7
        ],

        /// Bus Clock Divide Ratio (DIVBUS)
        DIVBUS OFFSET(4) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// Core Clock Divide Ratio (DIVCORE)
        DIVCORE OFFSET(16) NUMBITS(4) [
            Div1 = 0,
            Div2 = 1,
            Div3 = 2,
            Div4 = 3,
            Div5 = 4,
            Div6 = 5,
            Div7 = 6,
            Div8 = 7,
            Div9 = 8,
            Div10 = 9,
            Div11 = 10,
            Div12 = 11,
            Div13 = 12,
            Div14 = 13,
            Div15 = 14,
            Div16 = 15
        ],

        /// System Clock Source (SCS)
        SCS OFFSET(24) NUMBITS(4) [
            SystemOSC = 1,  // System OSC (SOSC_CLK)
            SlowIRC = 2,     // Slow IRC (SIRC_CLK)
            FastIRC = 3,     // Fast IRC (FIRC_CLK)
            SystemPLL = 6    // System PLL (SPLL_CLK)
        ]
    ],
    SOSCCSR [
        /// System OSC Enable
        SOSCEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// System OSC Clock Monitor
        SOSCCM OFFSET(16) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// System OSC Clock Monitor Reset Enable
        SOSCCMRE OFFSET(17) NUMBITS(1) [
            InterruptOnError = 0,
            ResetOnError = 1
        ],

        /// Lock Register - Prevents writes to this register
        LK OFFSET(23) NUMBITS(1) [
            Writable = 0,
            Locked = 1
        ],

        /// System OSC Valid
        SOSCVLD OFFSET(24) NUMBITS(1) [
            Invalid = 0,
            Valid = 1
        ],

        /// System OSC Selected as the System Clock Source
        SOSCSEL OFFSET(25) NUMBITS(1) [
            NotSelected = 0,
            Selected = 1
        ],

        /// System OSC Clock Error
        SOSCERR OFFSET(26) NUMBITS(1) [
            NoError = 0,
            ErrorDetected = 1
        ]
    ],
    SOSCDIV [
        /// System OSC Clock Divide 1
        SOSCDIV1 OFFSET(0) NUMBITS(3) [
            Disabled = 0,
            Div1 = 1,
            Div2 = 2,
            Div4 = 3,
            Div8 = 4,
            Div16 = 5,
            Div32 = 6,
            Div64 = 7
        ],

        /// System OSC Clock Divide 2
        SOSCDIV2 OFFSET(8) NUMBITS(3) [
            Disabled = 0,
            Div1 = 1,
            Div2 = 2,
            Div4 = 3,
            Div8 = 4,
            Div16 = 5,
            Div32 = 6,
            Div64 = 7
        ]
    ],
    SOSCCFG [
        /// External Reference Select
        EREFS OFFSET(2) NUMBITS(1) [
            ExternalReferenceClock = 0,
            InternalCrystalOscillator = 1
        ],

        /// High Gain Oscillator Select
        HGO OFFSET(3) NUMBITS(1) [
            LowGainOperation = 0,
            HighGainOperation = 1
        ],

        /// System OSC Range Select
        RANGE OFFSET(4) NUMBITS(2) [
            LowFrequency = 1,
            MediumFrequency = 2,
            HighFrequency = 3
        ]
    ],
    SIRCDIV[
        SIRCDIV2 OFFSET(8) NUMBITS(3) []
    ],
    SIRCCSR [
        /// Slow IRC Enable
        SIRCEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Slow IRC Stop Enable (in supported Stop modes)
        SIRCSTEN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Slow IRC Low Power Enable (in VLP modes)
        SIRCLPEN OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Lock Register - Prevents modification
        LK OFFSET(23) NUMBITS(1) [
            Writable = 0,
            Locked = 1
        ],

        /// Slow IRC Valid (output clock is valid)
        SIRCVLD OFFSET(24) NUMBITS(1) [
            Invalid = 0,
            Valid = 1
        ],

        /// Slow IRC Selected (system clock source)
        SIRCSEL OFFSET(25) NUMBITS(1) [
            NotSelected = 0,
            Selected = 1
        ]
    ],
    SIRCCFG [
        /// Frequency Range Select
        RANGE OFFSET(0) NUMBITS(1) [
            LowRange = 0,  // 2 MHz
            HighRange = 1  // 8 MHz
        ]
    ],
    FIRCCSR [
        /// Fast IRC Enable
        FIRCEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,  // Fast IRC is disabled
            Enabled = 1    // Fast IRC is enabled
        ],

        /// Fast IRC Regulator Enable
        FIRCREGOFF OFFSET(3) NUMBITS(1) [
            Enabled = 0,   // Fast IRC Regulator is enabled
            Disabled = 1   // Fast IRC Regulator is disabled
        ],

        /// Lock Register
        LK OFFSET(23) NUMBITS(1) [
            Writable = 0,  // Control Status Register can be written
            Locked = 1     // Control Status Register cannot be written
        ],

        /// Fast IRC Valid Status
        FIRCVLD OFFSET(24) NUMBITS(1) [
            Invalid = 0,   // Fast IRC is not enabled or clock is not valid
            Valid = 1      // Fast IRC is enabled and output clock is valid
        ],

        /// Fast IRC Selected Status
        FIRCSEL OFFSET(25) NUMBITS(1) [
            NotSelected = 0,  // Fast IRC is not the system clock source
            Selected = 1      // Fast IRC is the system clock source
        ],

        /// Fast IRC Clock Error
        FIRCERR OFFSET(26) NUMBITS(1) [
            NoError = 0,   // No error detected with the Fast IRC trimming
            Error = 1      // Error detected with the Fast IRC trimming
        ]
    ],
    FIRCDIV [
        /// Fast IRC Clock Divide 1
        FIRCDIV1 OFFSET(0) NUMBITS(3) [
            Disabled = 0,  // Output disabled
            Div1 = 1,      // Divide by 1
            Div2 = 2,      // Divide by 2
            Div4 = 3,      // Divide by 4
            Div8 = 4,      // Divide by 8
            Div16 = 5,     // Divide by 16
            Div32 = 6,     // Divide by 32
            Div64 = 7      // Divide by 64
        ],

        /// Fast IRC Clock Divide 2
        FIRCDIV2 OFFSET(8) NUMBITS(3) [
            Disabled = 0,  // Output disabled
            Div1 = 1,      // Divide by 1
            Div2 = 2,      // Divide by 2
            Div4 = 3,      // Divide by 4
            Div8 = 4,      // Divide by 8
            Div16 = 5,     // Divide by 16
            Div32 = 6,     // Divide by 32
            Div64 = 7      // Divide by 64
        ]
    ],
    FIRCCFG [
        /// Frequency Range Selection for Fast IRC
        RANGE OFFSET(0) NUMBITS(2) [
            MHz48 = 0,  // Fast IRC trimmed to 48 MHz
            MHz52 = 1,  // Fast IRC trimmed to 52 MHz
            MHz56 = 2,  // Fast IRC trimmed to 56 MHz
            MHz60 = 3   // Fast IRC trimmed to 60 MHz
        ]
    ],
    SPLLCSR [
        /// System PLL Enable
        SPLLEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,  // System PLL is disabled
            Enabled = 1    // System PLL is enabled
        ],

        /// System PLL Clock Monitor
        SPLLCM OFFSET(16) NUMBITS(1) [
            Disabled = 0,  // Clock Monitor is disabled
            Enabled = 1    // Clock Monitor is enabled
        ],

        /// System PLL Clock Monitor Reset Enable
        SPLLCMRE OFFSET(17) NUMBITS(1) [
            Interrupt = 0, // Generates interrupt on error detection
            Reset = 1      // Generates reset on error detection
        ],

        /// Lock Register (prevents further modification)
        LK OFFSET(23) NUMBITS(1) [
            Writable = 0,  // Control Status Register can be written
            Locked = 1     // Control Status Register cannot be written
        ],

        /// System PLL Valid
        SPLLVLD OFFSET(24) NUMBITS(1) [
            Invalid = 0,   // System PLL is not enabled or clock is not valid
            Valid = 1      // System PLL is enabled and output clock is valid
        ],

        /// System PLL Selected
        SPLLSEL OFFSET(25) NUMBITS(1) [
            NotSelected = 0, // System PLL is not the system clock source
            Selected = 1     // System PLL is the system clock source
        ],

        /// System PLL Clock Error
        SPLLERR OFFSET(26) NUMBITS(1) [
            NoError = 0,    // No error detected or clock monitor disabled
            Error = 1       // Clock monitor enabled and detected an error
        ]
    ],
    SPLLDIV [
        SPLLDIV1 OFFSET(0) NUMBITS(3) [
            Disabled = 0b000,
            Div1 = 0b001,
            Div2 = 0b010,
            Div4 = 0b011,
            Div8 = 0b100,
            Div16 = 0b101,
            Div32 = 0b110,
            Div64 = 0b111
        ],

        SPLLDIV2 OFFSET(8) NUMBITS(3) [
            Disabled = 0b000,
            Div1 = 0b001,
            Div2 = 0b010,
            Div4 = 0b011,
            Div8 = 0b100,
            Div16 = 0b101,
            Div32 = 0b110,
            Div64 = 0b111
        ]
    ],
    SPLLCFG [
        PREDIV OFFSET(8) NUMBITS(3) [
            Div1 = 0b000,
            Div2 = 0b001,
            Div3 = 0b010,
            Div4 = 0b011,
            Div5 = 0b100,
            Div6 = 0b101,
            Div7 = 0b110,
            Div8 = 0b111
        ],

        MULT OFFSET(16) NUMBITS(5) [
            Mul16 = 0b00000,
            Mul17 = 0b00001,
            Mul18 = 0b00010,
            Mul19 = 0b00011,
            Mul20 = 0b00100,
            Mul21 = 0b00101,
            Mul22 = 0b00110,
            Mul23 = 0b00111,
            Mul24 = 0b01000,
            Mul25 = 0b01001,
            Mul26 = 0b01010,
            Mul27 = 0b01011,
            Mul28 = 0b01100,
            Mul29 = 0b01101,
            Mul30 = 0b01110,
            Mul31 = 0b01111
        ]
    ]
];

pub const SCG_BASE: StaticRef<ScgRegisters> =
    unsafe { StaticRef::new(0x40064000 as *const ScgRegisters) };

#[repr(C)]
pub struct PccRegisters {
    pub ftfc: ReadWrite<u32, PCC_FTFC::Register>,
    pub dmamux: ReadWrite<u32, PCC_DMAMUX::Register>,
    pub can0: ReadWrite<u32, PCC_CAN0::Register>,
    pub lpit: ReadWrite<u32, PCC_LPIT::Register>,
    pub lptmr0: ReadWrite<u32, PCC_LPTMR0::Register>,
    pub porta: ReadWrite<u32, PCC_PORTA::Register>,
    pub portb: ReadWrite<u32, PCC_PORTB::Register>,
    pub portc: ReadWrite<u32, PCC_PORTC::Register>,
    pub portd: ReadWrite<u32, PCC_PORTD::Register>,
    pub porte: ReadWrite<u32, PCC_PORTE::Register>,
    pub lpuart0: ReadWrite<u32, PCC_LPUART0::Register>,
    pub ftm0: ReadWrite<u32, PCC_FTM0::Register>,
}

register_bitfields![u32,
    PCC_FTFC [
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ],
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    PCC_DMAMUX [
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ],
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    PCC_CAN0 [
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ],
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    PCC_LPIT [
        /// Peripheral Clock Source Select
        PCS OFFSET(24) NUMBITS(3) [
            _000 = 0,  // Clock is off.
            _001 = 1,  // Clock option 1
            _010 = 2,  // Clock option 2
            _011 = 3,  // Clock option 3
            _100 = 4,  // Clock option 4
            _101 = 5,  // Clock option 5
            _110 = 6,  // Clock option 6
            _111 = 7   // Clock option 7
        ],
        /// Clock Gate Control
        CGC OFFSET(30) NUMBITS(1) [
            _0 = 0,  // Clock disabled
            _1 = 1   // Clock enabled. The current clock selection and divider options are locked.
        ],
        /// Peripheral Present
        PR OFFSET(31) NUMBITS(1) [
            _0 = 0,  // Peripheral is not present.
            _1 = 1   // Peripheral is present.
        ]
    ],
    PCC_LPTMR0 [
        /// Peripheral Clock Divider Select
        PCD OFFSET(0) NUMBITS(3) [
            _000 = 0,  // Divide by 1
            _001 = 1,  // Divide by 2
            _010 = 2,  // Divide by 3
            _011 = 3,  // Divide by 4
            _100 = 4,  // Divide by 5
            _101 = 5,  // Divide by 6
            _110 = 6,  // Divide by 7
            _111 = 7   // Divide by 8
        ],
        /// Peripheral Clock Divider Fraction
        FRAC OFFSET(3) NUMBITS(1) [
            _0 = 0,  // Fractional value is 0
            _1 = 1   // Fractional value is 1
        ],
        /// Peripheral Clock Source Select
        PCS OFFSET(24) NUMBITS(3) [
            _000 = 0,  // Clock is off
            _001 = 1,  // Clock option 1
            _010 = 2,  // Clock option 2
            _011 = 3,  // Clock option 3
            _100 = 4,  // Clock option 4
            _101 = 5,  // Clock option 5
            _110 = 6,  // Clock option 6
            _111 = 7   // Clock option 7
        ],
        /// Clock Gate Control
        CGC OFFSET(30) NUMBITS(1) [
            _0 = 0,  // Clock disabled
            _1 = 1   // Clock enabled. The current clock selection and divider options are locked.
        ],
        /// Peripheral Present
        PR OFFSET(31) NUMBITS(1) [
            _0 = 0,  // Peripheral is not present.
            _1 = 1   // Peripheral is present.
        ]
    ],
    PCC_PORTA[
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ],
    PCC_PORTB[
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ],
    PCC_PORTC[
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ],
    PCC_PORTD[
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ],
    PCC_PORTE[
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ],
    PCC_LPUART0[
        PCS OFFSET(24) NUMBITS(3) [],
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ],
    PCC_FTM0[
        PCS OFFSET(24) NUMBITS(3) [],
        CGC OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PR OFFSET(31) NUMBITS(1) [
            NotPresent = 0,
            Present = 1
        ]
    ]

];

pub const PCC_BASE: StaticRef<PccRegisters> =
    unsafe { StaticRef::new(0x40065000 as *const PccRegisters) };

/// Clock struct
pub struct Clock {
    scg_registers: StaticRef<ScgRegisters>,
    pcc_registers: StaticRef<PccRegisters>,
    client: OptionalCell<&'static dyn ClockClient>,
}

/// Clock interrupt handling trait
pub trait ClockClient {
    fn event(&self);
}

impl Clock {
    /// Constructor
    pub const fn new() -> Clock {
        Clock {
            scg_registers: SCG_BASE,
            pcc_registers: PCC_BASE,
            client: OptionalCell::empty(),
        }
    }

    pub fn low_stop(&self) {
        self.stop_slow_irc();
    }

    pub fn low_start(&self) {
        self.start_slow_irc();
    }

    pub fn low_started(&self) -> bool {
        self.is_slow_irc_running()
    }

    pub fn high_stop(&self) {
        self.stop_fast_irc();
    }

    pub fn high_start(&self) {
        self.start_fast_irc();
    }

    pub fn high_started(&self) -> bool {
        self.is_fast_irc_running()
    }

    /// Set clock event callback client
    pub fn set_client(&self, client: &'static dyn ClockClient) {
        self.client.set(client);
    }

    /// Enable clock for a specific peripheral in PCC
    pub fn enable_peripheral_clock(&self, peripheral: Peripheral) {
        match peripheral {
            Peripheral::LPUART0 => self.pcc_registers.lpuart0.modify(PCC_LPUART0::CGC::SET),
            Peripheral::FTM0 => self.pcc_registers.ftm0.modify(PCC_FTM0::CGC::SET),
            Peripheral::LPIT => self.pcc_registers.lpit.modify(PCC_LPIT::CGC::SET),
            Peripheral::LPTMR0 => self.pcc_registers.lptmr0.modify(PCC_LPTMR0::CGC::SET),
            Peripheral::PORTA => self.pcc_registers.porta.modify(PCC_PORTA::CGC::SET),
            Peripheral::PORTB => self.pcc_registers.portb.modify(PCC_PORTB::CGC::SET),
            Peripheral::PORTC => self.pcc_registers.portc.modify(PCC_PORTC::CGC::SET),
            Peripheral::PORTD => self.pcc_registers.portd.modify(PCC_PORTD::CGC::SET),
            Peripheral::PORTE => self.pcc_registers.porte.modify(PCC_PORTE::CGC::SET),
        }
    }

    /// Disable clock for a specific peripheral in PCC
    pub fn disable_peripheral_clock(&self, peripheral: Peripheral) {
        match peripheral {
            Peripheral::LPUART0 => self.pcc_registers.lpuart0.modify(PCC_LPUART0::CGC::CLEAR),
            Peripheral::FTM0 => self.pcc_registers.ftm0.modify(PCC_FTM0::CGC::CLEAR),
            Peripheral::LPIT => self.pcc_registers.lpit.modify(PCC_LPIT::CGC::CLEAR),
            Peripheral::LPTMR0 => self.pcc_registers.lptmr0.modify(PCC_LPTMR0::CGC::CLEAR),
            Peripheral::PORTA => self.pcc_registers.porta.modify(PCC_PORTA::CGC::CLEAR),
            Peripheral::PORTB => self.pcc_registers.portb.modify(PCC_PORTB::CGC::CLEAR),
            Peripheral::PORTC => self.pcc_registers.portc.modify(PCC_PORTC::CGC::CLEAR),
            Peripheral::PORTD => self.pcc_registers.portd.modify(PCC_PORTD::CGC::CLEAR),
            Peripheral::PORTE => self.pcc_registers.porte.modify(PCC_PORTE::CGC::CLEAR),
        }
    }

    /// Select the system clock source
    pub fn set_system_clock(&self, clock_source: SystemClockSource) {
        self.scg_registers
            .rccr
            .modify(RCCR::SCS.val(clock_source as u32));
    }

    /// Get the current system clock source
    pub fn get_system_clock(&self) -> SystemClockSource {
        match self.scg_registers.csr.read(CSR::SCS) {
            1 => SystemClockSource::SystemOscillator,
            2 => SystemClockSource::SlowIRC,
            3 => SystemClockSource::FastIRC,
            6 => SystemClockSource::SystemPLL,
            _ => SystemClockSource::SlowIRC,
        }
    }

    /// Start system oscillator (SOSC)
    pub fn start_system_oscillator(&self) {
        self.scg_registers.sosccsr.modify(SOSCCSR::SOSCEN::SET);
    }

    /// Check if system oscillator is running
    pub fn is_system_oscillator_running(&self) -> bool {
        self.scg_registers.sosccsr.read(SOSCCSR::SOSCVLD) == 1
    }

    /// Stop system oscillator (SOSC)
    pub fn stop_system_oscillator(&self) {
        self.scg_registers.sosccsr.modify(SOSCCSR::SOSCEN::CLEAR);
    }

    /// Start slow IRC (SIRC)
    pub fn start_slow_irc(&self) {
        self.scg_registers.sirccsr.modify(SIRCCSR::SIRCEN::SET);
    }

    /// Check if slow IRC is running
    pub fn is_slow_irc_running(&self) -> bool {
        self.scg_registers.sirccsr.read(SIRCCSR::SIRCVLD) == 1
    }

    /// Stop slow IRC (SIRC)
    pub fn stop_slow_irc(&self) {
        self.scg_registers.sirccsr.modify(SIRCCSR::SIRCEN::CLEAR);
    }

    /// Start fast IRC (FIRC)
    pub fn start_fast_irc(&self) {
        self.scg_registers.firccsr.modify(FIRCCSR::FIRCEN::SET);
    }

    /// Check if fast IRC is running
    pub fn is_fast_irc_running(&self) -> bool {
        self.scg_registers.firccsr.read(FIRCCSR::FIRCVLD) == 1
    }

    /// Stop fast IRC (FIRC)
    pub fn stop_fast_irc(&self) {
        self.scg_registers.firccsr.modify(FIRCCSR::FIRCEN::CLEAR);
    }

    /// Start system PLL (SPLL)
    pub fn start_system_pll(&self) {
        self.scg_registers.spllcsr.modify(SPLLCSR::SPLLEN::SET);
    }

    /// Check if system PLL is running
    pub fn is_system_pll_running(&self) -> bool {
        self.scg_registers.spllcsr.read(SPLLCSR::SPLLVLD) == 1
    }

    /// Stop system PLL (SPLL)
    pub fn stop_system_pll(&self) {
        self.scg_registers.spllcsr.modify(SPLLCSR::SPLLEN::CLEAR);
    }
}

/// Enum for system clock source
pub enum SystemClockSource {
    SystemOscillator = 1, // SOSC_CLK
    SlowIRC = 2,          // SIRC_CLK
    FastIRC = 3,          // FIRC_CLK
    SystemPLL = 6,        // SPLL_CLK
}

/// Enum for peripherals controlled by PCC
pub enum Peripheral {
    LPUART0,
    FTM0,
    LPIT,
    LPTMR0,
    PORTA,
    PORTB,
    PORTC,
    PORTD,
    PORTE,
}
