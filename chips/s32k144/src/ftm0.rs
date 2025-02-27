use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

use crate::pinmux::Pinmux;

pub const FTM0_BASE: StaticRef<FtmRegisters> =
    unsafe { StaticRef::new(0x40038000 as *const FtmRegisters) };

#[repr(C)]
pub struct FtmRegisters {
    sc: ReadWrite<u32, SC::Register>,
    cnt: ReadWrite<u32, CNT::Register>,
    mod_: ReadWrite<u32, MOD::Register>,
    csc: [ReadWrite<u32, CSC::Register>; 8],
    cv: [ReadWrite<u32, CV::Register>; 8],
    cntin: ReadWrite<u32, CNTIN::Register>,
    status: ReadWrite<u32, STATUS::Register>,
    mode: ReadWrite<u32, MODE::Register>,
    sync: ReadWrite<u32, SYNC::Register>,
    outinit: ReadWrite<u32, OUTINIT::Register>,
    outmask: ReadWrite<u32, OUTMASK::Register>,
    combine: ReadWrite<u32, COMBINE::Register>,
    deadtime: ReadWrite<u32, DEADTIME::Register>,
    exttrig: ReadWrite<u32, EXTTRIG::Register>,
    pol: ReadWrite<u32, POL::Register>,
    fms: ReadWrite<u32, FMS::Register>,
    filter: ReadWrite<u32, FILTER::Register>,
    fltctrl: ReadWrite<u32, FLTCTRL::Register>,
    qdctrl: ReadWrite<u32, QDCTRL::Register>,
    conf: ReadWrite<u32, CONF::Register>,
    fltpol: ReadWrite<u32, FLTPOL::Register>,
    synconf: ReadWrite<u32, SYNCONF::Register>,
    invctrl: ReadWrite<u32, INVCTRL::Register>,
    swoctrl: ReadWrite<u32, SWOCTRL::Register>,
    pwmload: ReadWrite<u32, PWMLOAD::Register>,
    hcr: ReadWrite<u32, HCR::Register>,
    pairdeadtime: [ReadWrite<u32, PAIRDEADTIME::Register>; 4],
}

register_bitfields![u32,
    SC[
        PS OFFSET(0) NUMBITS(3) [
            DivideBy1 = 0,
            DivideBy2 = 1,
            DivideBy4 = 2,
            DivideBy8 = 3,
            DivideBy16 = 4,
            DivideBy32 = 5,
            DivideBy64 = 6,
            DivideBy128 = 7
        ],
        CLKS OFFSET(3) NUMBITS(2) [
            NoClock = 0,
            SystemClock = 1,
            FixedClock = 2,
            ExternalClock = 3
        ],
        CPWMS OFFSET(5) NUMBITS(1) [
            UpCounting = 0,
            UpDownCounting = 1
        ],
        TOIE OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TOF OFFSET(7) NUMBITS(1) [
            NoOverflow = 0,
            Overflow = 1
        ],
        PWMEN0 OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN1 OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN2 OFFSET(10) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN3 OFFSET(11) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN4 OFFSET(12) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN5 OFFSET(13) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN6 OFFSET(14) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PWMEN7 OFFSET(15) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FLTPS OFFSET(16) NUMBITS(4) [
            DivideBy1 = 0,
            DivideBy2 = 1,
            DivideBy4 = 2,
            DivideBy8 = 3,
            DivideBy16 = 4,
            DivideBy32 = 5,
            DivideBy64 = 6,
            DivideBy128 = 7,
            DivideBy256 = 8
        ]
    ],
    CNT[
        COUNT OFFSET(0) NUMBITS(16) []
    ],
    MOD[
        MOD OFFSET(0) NUMBITS(16) []
    ],
    CSC[
        DMA OFFSET(0) NUMBITS(1) [ Disabled = 0, Enabled = 1 ],
        ICRST OFFSET(1) NUMBITS(1) [ NoReset = 0, Reset = 1 ],
        ELSA OFFSET(2) NUMBITS(1) [ Low = 0, High = 1 ],
        ELSB OFFSET(3) NUMBITS(1) [ Low = 0, High = 1 ],
        MSA OFFSET(4) NUMBITS(1) [ Disabled = 0, Enabled = 1 ],
        MSB OFFSET(5) NUMBITS(1) [ Disabled = 0, Enabled = 1 ],
        CHIE OFFSET(6) NUMBITS(1) [ Disabled = 0, Enabled = 1 ],
        CHF OFFSET(7) NUMBITS(1) [ NoFlag = 0, Set = 1 ],
        TRIGMODE OFFSET(8) NUMBITS(1) [ Normal = 0, Trigger = 1 ],
        CHIS OFFSET(9) NUMBITS(1) [ Low = 0, High = 1 ],
        CHOV OFFSET(10) NUMBITS(1) [ Low = 0, High = 1 ]
    ],
    CV[
        VAL OFFSET(0) NUMBITS(16) []
    ],
    CNTIN[
        INIT OFFSET(0) NUMBITS(16) []
    ],
    STATUS[
        CH0F OFFSET(0) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH1F OFFSET(1) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH2F OFFSET(2) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH3F OFFSET(3) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH4F OFFSET(4) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH5F OFFSET(5) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH6F OFFSET(6) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ],
        CH7F OFFSET(7) NUMBITS(1) [ NoEvent = 0, EventOccurred = 1 ]
    ],
    MODE[
        FTMEN OFFSET(0) NUMBITS(1) [ TPMCompatible = 0, FTMEnhanced = 1 ],
        WPDIS OFFSET(2) NUMBITS(1) [ Enabled = 0, Disabled = 1 ],
        PWMSYNC OFFSET(3) NUMBITS(1) [ NoRestrictions = 0, Restricted = 1 ],
        CAPTEST OFFSET(4) NUMBITS(1) [ Disabled = 0, Enabled = 1 ],
        FAULTM OFFSET(5) NUMBITS(2) [
            Disabled = 0b00,
            EvenChannelsManual = 0b01,
            AllChannelsManual = 0b10,
            AllChannelsAuto = 0b11
        ],
        FAULTIE OFFSET(7) NUMBITS(1) [ Disabled = 0, Enabled = 1 ]
    ],
    SYNC[
        CNTMIN OFFSET(0) NUMBITS(1) [],
        CNTMAX OFFSET(1) NUMBITS(1) [],
        REINIT OFFSET(2) NUMBITS(1) [],
        SYNCHOM OFFSET(3) NUMBITS(1) [],
        TRIG0 OFFSET(4) NUMBITS(1) [],
        TRIG1 OFFSET(5) NUMBITS(1) [],
        TRIG2 OFFSET(6) NUMBITS(1) [],
        SWSYNC OFFSET(7) NUMBITS(1) [],
    ],
    OUTINIT[
        CH0OI OFFSET(0) NUMBITS(1) [],
        CH1OI OFFSET(1) NUMBITS(1) [],
        CH2OI OFFSET(2) NUMBITS(1) [],
        CH3OI OFFSET(3) NUMBITS(1) [],
        CH4OI OFFSET(4) NUMBITS(1) [],
        CH5OI OFFSET(5) NUMBITS(1) [],
        CH6OI OFFSET(6) NUMBITS(1) [],
        CH7OI OFFSET(7) NUMBITS(1) []
    ],
    OUTMASK[
        CH0OM OFFSET(0) NUMBITS(1) [],
        CH1OM OFFSET(1) NUMBITS(1) [],
        CH2OM OFFSET(2) NUMBITS(1) [],
        CH3OM OFFSET(3) NUMBITS(1) [],
        CH4OM OFFSET(4) NUMBITS(1) [],
        CH5OM OFFSET(5) NUMBITS(1) [],
        CH6OM OFFSET(6) NUMBITS(1) [],
        CH7OM OFFSET(7) NUMBITS(1) [],
    ],
    COMBINE[
        COMBINE0 OFFSET(0) NUMBITS(1) [],
        COMP0 OFFSET(1) NUMBITS(1) [
            Same = 0,
            Complement = 1
        ],
        DECAPEN0 OFFSET(2) NUMBITS(1) [],
        DECAP0 OFFSET(3) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],
        DTEN0 OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        SYNCEN0 OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULTEN0 OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        MCOMBINE0 OFFSET(7) NUMBITS(1) [],

        COMBINE1 OFFSET(8) NUMBITS(1) [],
        COMP1 OFFSET(9) NUMBITS(1) [
            Same = 0,
            Complement = 1
        ],
        DECAPEN1 OFFSET(10) NUMBITS(1) [],
        DECAP1 OFFSET(11) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],
        DTEN1 OFFSET(12) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        SYNCEN1 OFFSET(13) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULTEN1 OFFSET(14) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        MCOMBINE1 OFFSET(15) NUMBITS(1) [],

        COMBINE2 OFFSET(16) NUMBITS(1) [],
        COMP2 OFFSET(17) NUMBITS(1) [
            Same = 0,
            Complement = 1
        ],
        DECAPEN2 OFFSET(18) NUMBITS(1) [],
        DECAP2 OFFSET(19) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],
        DTEN2 OFFSET(20) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        SYNCEN2 OFFSET(21) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULTEN2 OFFSET(22) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        MCOMBINE2 OFFSET(23) NUMBITS(1) [],

        COMBINE3 OFFSET(24) NUMBITS(1) [],
        COMP3 OFFSET(25) NUMBITS(1) [
            Same = 0,
            Complement = 1
        ],
        DECAPEN3 OFFSET(26) NUMBITS(1) [],
        DECAP3 OFFSET(27) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],
        DTEN3 OFFSET(28) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        SYNCEN3 OFFSET(29) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULTEN3 OFFSET(30) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        MCOMBINE3 OFFSET(31) NUMBITS(1) []
    ],
    DEADTIME[
        DTVAL OFFSET(0) NUMBITS(6) [],
        DTPS OFFSET(6) NUMBITS(2) [],
        DTVALEX OFFSET(16) NUMBITS(4) []
    ],
    EXTTRIG[
        CH2TRIG OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CH3TRIG OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CH4TRIG OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CH5TRIG OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CH0TRIG OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CH1TRIG OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        INITTRIGEN OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TRIGF OFFSET(7) NUMBITS(1) [
            NoTrigger = 0,
            TriggerGenerated = 1
        ],
        CH6TRIG OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        CH7TRIG OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    POL[
        POL0 OFFSET(0) NUMBITS(1) [],
        POL1 OFFSET(1) NUMBITS(1) [],
        POL2 OFFSET(2) NUMBITS(1) [],
        POL3 OFFSET(3) NUMBITS(1) [],
        POL4 OFFSET(4) NUMBITS(1) [],
        POL5 OFFSET(5) NUMBITS(1) [],
        POL6 OFFSET(6) NUMBITS(1) [],
        POL7 OFFSET(7) NUMBITS(1) [],
    ],
    FMS[
        FAULTF0 OFFSET(0) NUMBITS(1) [],
        FAULTF1 OFFSET(1) NUMBITS(1) [],
        FAULTF2 OFFSET(2) NUMBITS(1) [],
        FAULTF3 OFFSET(3) NUMBITS(1) [],
        FAULTIN OFFSET(5) NUMBITS(1) [],
        WPEN OFFSET(6) NUMBITS(1) [],
        FAULTF OFFSET(7) NUMBITS(1) []
    ],
    FILTER[
        CH0FVAL OFFSET(0) NUMBITS(4) [],
        CH1FVAL OFFSET(4) NUMBITS(4) [],
        CH2FVAL OFFSET(8) NUMBITS(4) [],
        CH3FVAL OFFSET(12) NUMBITS(4) [],
    ],
    FLTCTRL[
        FAULT0EN  OFFSET(0)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULT1EN  OFFSET(1)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULT2EN  OFFSET(2)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FAULT3EN  OFFSET(3)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FFLTR0EN  OFFSET(4)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FFLTR1EN  OFFSET(5)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FFLTR2EN  OFFSET(6)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FFLTR3EN  OFFSET(7)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FFVAL  OFFSET(8)  NUMBITS(4) [],
        FSTATE  OFFSET(15)  NUMBITS(1) [
            SafeValues = 0,
            TriState = 1
        ]
    ],
    QDCTRL[
        QUADEN OFFSET(0)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TOFDIR OFFSET(1)  NUMBITS(1) [
            BottomCounting = 0,
            TopCounting = 1
        ],
        QUADIR OFFSET(2)  NUMBITS(1) [
            Decrement = 0,
            Increment = 1
        ],
        QUADMODE OFFSET(3)  NUMBITS(1) [
            PhaseAB = 0,
            CountDirection = 1
        ],
        PHBPOL OFFSET(4)  NUMBITS(1) [
            Normal = 0,
            Inverted = 1
        ],
        PHAPOL OFFSET(5)  NUMBITS(1) [
            Normal = 0,
            Inverted = 1
        ],
        PHBFLTREN OFFSET(6)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PHAFLTREN OFFSET(7)  NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    CONF[
        LDFQ OFFSET(0) NUMBITS(5) [],
        BDMMODE OFFSET(6) NUMBITS(2) [],
        GTBEEN OFFSET(9) NUMBITS(1) [],
        GTBEOUT OFFSET(10) NUMBITS(1) [],
        ITRIGR OFFSET(11) NUMBITS(1) []
    ],
    FLTPOL[
        FLT0POL OFFSET(0) NUMBITS(1) [],
        FLT1POL OFFSET(1) NUMBITS(1) [],
        FLT2POL OFFSET(2) NUMBITS(1) [],
        FLT3POL OFFSET(3) NUMBITS(1) []
    ],
    SYNCONF[
        HWTRIGMODE  OFFSET(0)  NUMBITS(1) [
            ClearOnTrigger = 0,
            NoClearOnTrigger = 1
        ],
        CNTINC      OFFSET(2)  NUMBITS(1) [
            UpdateOnClock = 0,
            UpdateOnSync = 1
        ],
        INVC        OFFSET(4)  NUMBITS(1) [
            UpdateOnClock = 0,
            UpdateOnSync = 1
        ],
        SWOC        OFFSET(5)  NUMBITS(1) [
            UpdateOnClock = 0,
            UpdateOnSync = 1
        ],
        SYNCMODE    OFFSET(7)  NUMBITS(1) [
            LegacySync = 0,
            EnhancedSync = 1
        ],
        SWRSTCNT    OFFSET(8)  NUMBITS(1) [
            NoSoftwareSync = 0,
            SoftwareSync = 1
        ],
        SWWRBUF     OFFSET(9)  NUMBITS(1) [
            NoSoftwareSync = 0,
            SoftwareSync = 1
        ],
        SWOM        OFFSET(10) NUMBITS(1) [
            NoSoftwareSync = 0,
            SoftwareSync = 1
        ],
        SWINVC      OFFSET(11) NUMBITS(1) [
            NoSoftwareSync = 0,
            SoftwareSync = 1
        ],
        SWSOC       OFFSET(12) NUMBITS(1) [
            NoSoftwareSync = 0,
            SoftwareSync = 1
        ],
        HWRSTCNT    OFFSET(16) NUMBITS(1) [
            NoHardwareSync = 0,
            HardwareSync = 1
        ],
        HWWRBUF     OFFSET(17) NUMBITS(1) [
            NoHardwareSync = 0,
            HardwareSync = 1
        ],
        HWOM        OFFSET(18) NUMBITS(1) [
            NoHardwareSync = 0,
            HardwareSync = 1
        ],
        HWINVC      OFFSET(19) NUMBITS(1) [
            NoHardwareSync = 0,
            HardwareSync = 1
        ],
        HWSOC       OFFSET(20) NUMBITS(1) [
            NoHardwareSync = 0,
            HardwareSync = 1
        ]
    ],
    INVCTRL[
        INV0EN OFFSET(0) NUMBITS(1) [],
        INV1EN OFFSET(1) NUMBITS(1) [],
        INV2EN OFFSET(2) NUMBITS(1) [],
        INV3EN OFFSET(3) NUMBITS(1) [],
    ],
    SWOCTRL[
        CH0OC OFFSET(0) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH1OC OFFSET(1) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH2OC OFFSET(2) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH3OC OFFSET(3) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH4OC OFFSET(4) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH5OC OFFSET(5) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH6OC OFFSET(6) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH7OC OFFSET(7) NUMBITS(1) [
            NotAffected = 0,
            Affected = 1
        ],
        CH0OCV OFFSET(8) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH1OCV OFFSET(9) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH2OCV OFFSET(10) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH3OCV OFFSET(11) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH4OCV OFFSET(12) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH5OCV OFFSET(13) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH6OCV OFFSET(14) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ],
        CH7OCV OFFSET(15) NUMBITS(1) [
            Force0 = 0,
            Force1 = 1
        ]
    ],
    PWMLOAD[
        CH0SEL  OFFSET(0)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH1SEL  OFFSET(1)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH2SEL  OFFSET(2)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH3SEL  OFFSET(3)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH4SEL  OFFSET(4)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH5SEL  OFFSET(5)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH6SEL  OFFSET(6)  NUMBITS(1)  [Excluded = 0, Included = 1],
        CH7SEL  OFFSET(7)  NUMBITS(1)  [Excluded = 0, Included = 1],
        HCSEL   OFFSET(8)  NUMBITS(1)  [Disabled = 0, Enabled = 1],
        LDOK    OFFSET(9)  NUMBITS(1)  [Disabled = 0, Enabled = 1],
        GLEN    OFFSET(10) NUMBITS(1)  [Disabled = 0, Enabled = 1],
        GLDOK   OFFSET(11) NUMBITS(1)  [NoAction = 0, SetLDOK = 1]
    ],
    HCR[
        HCVAL OFFSET(0) NUMBITS(16) []
    ],
    PAIRDEADTIME[
        DTVAL OFFSET(0) NUMBITS(6) [],
        DTPS OFFSET(6) NUMBITS(2) [],
        DTVALEX OFFSET(16) NUMBITS(4) []
    ]
];

pub struct Pwm {
    registers: StaticRef<FtmRegisters>,
}

impl Pwm {
    pub const fn new() -> Pwm {
        Pwm {
            registers: FTM0_BASE,
        }
    }
    // ftm channel setting
    fn start_pwm(
        &self,
        pin: Pinmux,
        frequency_hz: usize,
        duty_cycle: usize,
    ) -> Result<(), ErrorCode> {
        let (port, channel) = pin.into();

        if channel >= 8 {
            return Err(ErrorCode::INVAL);
        }

        if frequency_hz == 0 {
            return self.stop_pwm(pin);
        }

        let prescaler = 0;
        let counter_top = (16000000 / frequency_hz) >> prescaler;
        let dc_out = counter_top * duty_cycle / 100;

        // 해당 핀을 PWM (FTM) 기능으로 설정
        let port_reg = crate::pinmux::PORT_BASES[port];
        port_reg.pcr[channel].modify(crate::pinmux::PCR::MUX::ALT2_FTM);

        // FTM 설정
        self.registers
            .sc
            .modify(SC::CLKS::SystemClock + SC::PS::DivideBy1);
        self.registers.mod_.write(MOD::MOD.val(counter_top as u32));

        // 해당 채널을 PWM 모드로 설정
        self.registers.csc[channel].modify(CSC::MSB::Enabled + CSC::ELSB::High);
        self.registers.cv[channel].write(CV::VAL.val(dc_out as u32));

        // PWMLOAD를 사용하여 설정 적용
        self.registers.pwmload.modify(PWMLOAD::LDOK::Enabled);

        // 동기화 설정 적용
        self.registers.sync.modify(SYNC::SWSYNC::SET);

        Ok(())
    }

    fn stop_pwm(&self, pin: Pinmux) -> Result<(), ErrorCode> {
        let (_, channel) = pin.into();

        if channel >= 8 {
            return Err(ErrorCode::INVAL);
        }

        self.registers.csc[channel].modify(CSC::MSB::Disabled);
        Ok(())
    }

    pub fn handle_interrupt(&self) {
        let status = self.registers.status.get();

        // 채널별 인터럽트 플래그 확인 및 처리
        for channel in 0..8 {
            if (status & (1 << channel)) != 0 {
                // 해당 채널 인터럽트 발생 -> 처리 후 플래그 클리어
                self.registers.status.modify(STATUS::CH0F::NoEvent);
            }
        }
    }
}

impl kernel::hil::pwm::Pwm for Pwm {
    type Pin = Pinmux;

    fn start(&self, pin: &Self::Pin, frequency: usize, duty_cycle: usize) -> Result<(), ErrorCode> {
        self.start_pwm(*pin, frequency, duty_cycle)
    }

    fn stop(&self, pin: &Self::Pin) -> Result<(), ErrorCode> {
        self.stop_pwm(*pin)
    }

    fn get_maximum_frequency_hz(&self) -> usize {
        5333333
    }

    fn get_maximum_duty_cycle(&self) -> usize {
        100
    }
}
