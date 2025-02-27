use core::cell::Cell;
use kernel::hil::uart::Error;
use kernel::hil::uart::{Configure, Parameters, Receive, Transmit};
use kernel::hil::uart::{ReceiveClient, TransmitClient};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const LPUART0_BASE: StaticRef<LpuartRegisters> =
    unsafe { StaticRef::new(0x4006A000 as *const LpuartRegisters) };

#[repr(C)]
pub struct LpuartRegisters {
    verid: ReadOnly<u32, VERID::Register>,
    param: ReadOnly<u32, PARAM::Register>,
    global: ReadWrite<u32, GLOBAL::Register>,
    pincfg: ReadWrite<u32, PINCFG::Register>,
    baud: ReadWrite<u32, BAUD::Register>,
    stat: ReadWrite<u32, STAT::Register>,
    ctrl: ReadWrite<u32, CTRL::Register>,
    data: ReadWrite<u32, DATA::Register>,
    match_: ReadWrite<u32, MATCH::Register>,
    modir: ReadWrite<u32, MODIR::Register>,
    fifo: ReadWrite<u32, FIFO::Register>,
    water: ReadWrite<u32, WATER::Register>,
}

register_bitfields![u32,
    VERID [
        FEATURE OFFSET(0) NUMBITS(16) [
            Standard = 0b0000000000000001,
            StandardWithModemIrDA = 0b0000000000000011
        ],
        MINOR OFFSET(16) NUMBITS(8) [],
        MAJOR OFFSET(24) NUMBITS(8) []
    ],
    PARAM [
        TXFIFO OFFSET(0) NUMBITS(8) [],
        RXFIFO OFFSET(8) NUMBITS(8) []
    ],
    GLOBAL [
        RST OFFSET(1) NUMBITS(1) []
    ],
    PINCFG [
        TRGSEL OFFSET(0) NUMBITS(2) []
    ],
    BAUD [
        SBR OFFSET(0) NUMBITS(13) [],
        SBNS OFFSET(13) NUMBITS(1) [
            OneStopBit = 0,
            TwoStopBits = 1
        ],
        RXEDGIE OFFSET(14) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        LBKDIE OFFSET(15) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RESYNCDIS OFFSET(16) NUMBITS(1) [
            Enabled = 0,
            Disabled = 1
        ],
        BOTHEDGE OFFSET(17) NUMBITS(1) [
            RisingEdge = 0,
            BothEdges = 1
        ],
        MATCFG OFFSET(18) NUMBITS(2) [],
        RIDMAE OFFSET(20) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RDMAE OFFSET(21) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TDMAE OFFSET(23) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        OSR OFFSET(24) NUMBITS(5) [],
        M10 OFFSET(29) NUMBITS(1) [
            Bit7to9 = 0,
            Bit10 = 1
        ],
        MAEN2 OFFSET(30) NUMBITS(1) [
            Normal = 0,
            MatchMode = 1
        ],
        MAEN1 OFFSET(31) NUMBITS(1) [
            Normal = 0,
            MatchMode = 1
        ]
    ],
    STAT [
        MA2F OFFSET(14) NUMBITS(1) [
            NotEqual = 0,
            Equal = 1
        ],
        MA1F OFFSET(15) NUMBITS(1) [
            NotEqual = 0,
            Equal = 1
        ],
        PF OFFSET(16) NUMBITS(1) [
            NoError = 0,
            Error = 1
        ],
        FE OFFSET(17) NUMBITS(1) [
            NoError = 0,
            Error = 1
        ],
        NF OFFSET(18) NUMBITS(1) [
            NoNoise = 0,
            NoiseDetected = 1
        ],
        OR OFFSET(19) NUMBITS(1) [
            NoOverrun = 0,
            Overrun = 1
        ],
        IDLE OFFSET(20) NUMBITS(1) [
            NotDetected = 0,
            Detected = 1
        ],
        RDRF OFFSET(21) NUMBITS(1) [
            Empty = 0,
            Full = 1
        ],
        TC OFFSET(22) NUMBITS(1) [
            Transmitting = 0,
            Idle = 1
        ],
        TDRE OFFSET(23) NUMBITS(1) [
            Full = 0,
            Empty = 1
        ],
        RAF OFFSET(24) NUMBITS(1) [
            Idle = 0,
            Active = 1
        ],
        LBKDE OFFSET(25) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        BRK13 OFFSET(26) NUMBITS(1) [
            ShortBreak = 0,
            LongBreak = 1
        ],
        RWUID OFFSET(27) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RXINV OFFSET(28) NUMBITS(1) [
            Normal = 0,
            Inverted = 1
        ],
        MSBF OFFSET(29) NUMBITS(1) [
            LSBFirst = 0,
            MSBFirst = 1
        ],
        RXEDGIF OFFSET(30) NUMBITS(1) [
            NoEdge = 0,
            EdgeDetected = 1
        ],
        LBKDIF OFFSET(31) NUMBITS(1) [
            NoBreak = 0,
            BreakDetected = 1
        ]
    ],
    CTRL [
        PT OFFSET(0) NUMBITS(1) [
            Even = 0,
            Odd = 1
        ],
        PE OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        ILT OFFSET(2) NUMBITS(1) [
            StartBit = 0,
            StopBit = 1
        ],
        WAKE OFFSET(3) NUMBITS(1) [
            IdleLine = 0,
            AddressMark = 1
        ],
        M OFFSET(4) NUMBITS(1) [
            Data8Bit = 0,
            Data9Bit = 1
        ],
        RSRC OFFSET(5) NUMBITS(1) [
            InternalLoop = 0,
            SingleWire = 1
        ],
        DOZEEN OFFSET(6) NUMBITS(1) [
            Enabled = 0,
            Disabled = 1
        ],
        LOOPS OFFSET(7) NUMBITS(1) [
            Normal = 0,
            Loopback = 1
        ],
        IDLECFG OFFSET(8) NUMBITS(3) [
            Idle1 = 0,
            Idle2 = 1,
            Idle4 = 2,
            Idle8 = 3,
            Idle16 = 4,
            Idle32 = 5,
            Idle64 = 6,
            Idle128 = 7
        ],
        M7 OFFSET(11) NUMBITS(1) [
            Bit8To10 = 0,
            Bit7 = 1
        ],
        MA2IE OFFSET(14) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        MA1IE OFFSET(15) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        SBK OFFSET(16) NUMBITS(1) [
            Normal = 0,
            SendBreak = 1
        ],
        RWU OFFSET(17) NUMBITS(1) [
            Normal = 0,
            Standby = 1
        ],
        RE OFFSET(18) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TE OFFSET(19) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        ILIE OFFSET(20) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RIE OFFSET(21) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TCIE OFFSET(22) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TIE OFFSET(23) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        PEIE OFFSET(24) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        FEIE OFFSET(25) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        NEIE OFFSET(26) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        ORIE OFFSET(27) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TXINV OFFSET(28) NUMBITS(1) [
            Normal = 0,
            Inverted = 1
        ],
        TXDIR OFFSET(29) NUMBITS(1) [
            Input = 0,
            Output = 1
        ],
        R9T8 OFFSET(30) NUMBITS(1) [],
        R8T9 OFFSET(31) NUMBITS(1) []
    ],
    DATA [
        R0T0 OFFSET(0) NUMBITS(1) [],
        R1T1 OFFSET(1) NUMBITS(1) [],
        R2T2 OFFSET(2) NUMBITS(1) [],
        R3T3 OFFSET(3) NUMBITS(1) [],
        R4T4 OFFSET(4) NUMBITS(1) [],
        R5T5 OFFSET(5) NUMBITS(1) [],
        R6T6 OFFSET(6) NUMBITS(1) [],
        R7T7 OFFSET(7) NUMBITS(1) [],
        R8T8 OFFSET(8) NUMBITS(1) [],
        R9T9 OFFSET(9) NUMBITS(1) [],
        IDLINE OFFSET(11) NUMBITS(1) [
            IdleBeforeReceiving = 1,
            NotIdleBeforeReceiving = 0
        ],
        RXEMPT OFFSET(12) NUMBITS(1) [
            BufferEmpty = 1,
            BufferContainsData = 0
        ],
        FRETSC OFFSET(13) NUMBITS(1) [
            FrameErrorOrSpecial = 1,
            NormalCharacter = 0
        ],
        PARITYE OFFSET(14) NUMBITS(1) [
            ParityError = 1,
            NoParityError = 0
        ],
        NOISY OFFSET(15) NUMBITS(1) [
            NoisyData = 1,
            CleanData = 0
        ]
    ],
    MATCH [
        MA1 OFFSET(0) NUMBITS(10) [],
        MA2 OFFSET(16) NUMBITS(10) []
    ],
    MODIR [
        TXCTSE OFFSET(0) NUMBITS(1) [
            NoEffect = 0,
            ClearToSendEnabled = 1
        ],
        TXRTSE OFFSET(1) NUMBITS(1) [
            NoEffect = 0,
            RequestToSendEnabled = 1
        ],
        TXRTSPOL OFFSET(2) NUMBITS(1) [
            ActiveLow = 0,
            ActiveHigh = 1
        ],
        RXRTSE OFFSET(3) NUMBITS(1) [
            NoEffect = 0
        ],
        TXCTSC OFFSET(4) NUMBITS(1) [
            SampledAtCharacterStart = 0,
            SampledWhenIdle = 1
        ],
        TXCTSSRC OFFSET(5) NUMBITS(1) [
            CTSBPin = 0,
            InvertedReceiverMatch = 1
        ],
        RTSWATER OFFSET(8) NUMBITS(2) [],
        TNP OFFSET(16) NUMBITS(2) [
            OneOverOSR = 0b00,
            TwoOverOSR = 0b01,
            ThreeOverOSR = 0b10,
            FourOverOSR = 0b11
        ],
        IREN OFFSET(18) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    FIFO [
        RXFIFOSIZE OFFSET(0) NUMBITS(3) [
            Depth1 = 0b000,
            Depth4 = 0b001,
            Depth8 = 0b010,
            Depth16 = 0b011,
            Depth32 = 0b100,
            Depth64 = 0b101,
            Depth128 = 0b110,
            Depth256 = 0b111
        ],
        RXFE OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TXFIFOSIZE OFFSET(4) NUMBITS(3) [
            Depth1 = 0b000,
            Depth4 = 0b001,
            Depth8 = 0b010,
            Depth16 = 0b011,
            Depth32 = 0b100,
            Depth64 = 0b101,
            Depth128 = 0b110,
            Depth256 = 0b111
        ],
        TXFE OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RXUFE OFFSET(8) NUMBITS(1) [
            NoInterrupt = 0,
            Interrupt = 1
        ],
        TXOFE OFFSET(9) NUMBITS(1) [
            NoInterrupt = 0,
            Interrupt = 1
        ],
        RXIDEN OFFSET(10) NUMBITS(3) [
            Disabled = 0b000,
            Idle1Char = 0b001,
            Idle2Chars = 0b010,
            Idle4Chars = 0b011,
            Idle8Chars = 0b100,
            Idle16Chars = 0b101,
            Idle32Chars = 0b110,
            Idle64Chars = 0b111
        ],
        RXFLUSH OFFSET(14) NUMBITS(1) [
            NoFlush = 0,
            Flush = 1
        ],
        TXFLUSH OFFSET(15) NUMBITS(1) [
            NoFlush = 0,
            Flush = 1
        ],
        RXUF OFFSET(16) NUMBITS(1) [
            NoUnderflow = 0,
            UnderflowOccurred = 1
        ],
        TXOF OFFSET(17) NUMBITS(1) [
            NoOverflow = 0,
            OverflowOccurred = 1
        ],
        RXEMPT OFFSET(22) NUMBITS(1) [
            NotEmpty = 0,
            Empty = 1
        ],
        TXEMPT OFFSET(23) NUMBITS(1) [
            NotEmpty = 0,
            Empty = 1
        ]
    ],
    WATER [
        TXWATER OFFSET(0) NUMBITS(2) [],
        TXCOUNT OFFSET(8) NUMBITS(3) [],
        RXWATER OFFSET(16) NUMBITS(2) [],
        RXCOUNT OFFSET(24) NUMBITS(3) []
    ]
];
pub struct Lpuart0<'a> {
    registers: StaticRef<LpuartRegisters>,
    tx_client: OptionalCell<&'a dyn TransmitClient>,
    rx_client: OptionalCell<&'a dyn ReceiveClient>,
    tx_buffer: kernel::utilities::cells::TakeCell<'static, [u8]>,
    rx_buffer: kernel::utilities::cells::TakeCell<'static, [u8]>,
    tx_len: Cell<usize>,
    rx_len: Cell<usize>,
    //tx_remaining_bytes: Cell<usize>,
    //rx_remaining_bytes: Cell<usize>,
    tx_position: Cell<usize>,
    rx_position: Cell<usize>,
}

impl<'a> Lpuart0<'a> {
    pub const fn new(regs: StaticRef<LpuartRegisters>) -> Lpuart0<'a> {
        Lpuart0 {
            registers: regs,
            tx_client: OptionalCell::empty(),
            rx_client: OptionalCell::empty(),
            tx_buffer: kernel::utilities::cells::TakeCell::empty(),
            rx_buffer: kernel::utilities::cells::TakeCell::empty(),
            tx_len: Cell::new(0),
            rx_len: Cell::new(0),
            //tx_remaining_bytes: Cell::new(0),
            //rx_remaining_bytes: Cell::new(0),
            tx_position: Cell::new(0),
            rx_position: Cell::new(0),
        }
    }

    pub fn enable(&self) {
        self.registers.ctrl.modify(CTRL::TE::SET + CTRL::RE::SET);
    }

    pub fn disable(&self) {
        self.registers
            .ctrl
            .modify(CTRL::TE::CLEAR + CTRL::RE::CLEAR);
    }

    pub fn enable_rx_interrupts(&self) {
        self.registers.ctrl.modify(CTRL::RIE::SET);
    }

    pub fn enable_tx_interrupts(&self) {
        self.registers.ctrl.modify(CTRL::TIE::SET + CTRL::TCIE::SET);
    }

    pub fn disable_rx_interrupts(&self) {
        self.registers.ctrl.modify(CTRL::RIE::CLEAR);
    }

    pub fn disable_tx_interrupts(&self) {
        self.registers
            .ctrl
            .modify(CTRL::TIE::CLEAR + CTRL::TCIE::CLEAR);
    }

    pub fn handle_interrupt(&self) {
        if self.registers.stat.is_set(STAT::TDRE) {
            self.tx_buffer.take().map(|buf| {
                let pos = self.tx_position.get();
                if pos < self.tx_len.get() {
                    self.registers.data.set(buf[pos] as u32);
                    self.tx_position.set(pos + 1);
                    self.tx_buffer.replace(buf);
                } else {
                    self.disable_tx_interrupts();
                    self.tx_client.map(|client| {
                        self.tx_buffer.replace(buf);

                        self.tx_buffer.take().map(|buf| {
                            client.transmitted_buffer(buf, self.tx_len.get(), Ok(()));
                        });
                    });
                }
            });
        }

        if self.registers.stat.is_set(STAT::TDRE) {
            self.rx_buffer.take().map(|buf| {
                let pos = self.rx_position.get();
                if pos < self.rx_len.get() {
                    self.registers.data.set(buf[pos] as u32);
                    self.rx_position.set(pos + 1);
                    self.rx_buffer.replace(buf);
                } else {
                    self.disable_rx_interrupts();
                    self.rx_client.map(|client| {
                        self.rx_buffer.replace(buf);

                        self.rx_buffer.take().map(|buf| {
                            client.received_buffer(buf, self.rx_len.get(), Ok(()), Error::None);
                        });
                    });
                }
            });
        }
    }
}

impl<'a> Transmit<'a> for Lpuart0<'a> {
    fn set_transmit_client(&self, client: &'a dyn TransmitClient) {
        self.tx_client.set(client);
    }

    fn transmit_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if len == 0 {
            return Err((ErrorCode::SIZE, buffer));
        }
        if self.tx_buffer.is_some() {
            return Err((ErrorCode::BUSY, buffer));
        }
        self.tx_buffer.replace(buffer);
        self.tx_len.set(len);
        self.tx_position.set(0);
        self.enable_tx_interrupts();
        Ok(())
    }

    fn transmit_word(&self, word: u32) -> Result<(), ErrorCode> {
        if !self.registers.stat.is_set(STAT::TDRE) {
            return Err(ErrorCode::BUSY);
        }

        self.registers.data.set(word);
        Ok(())
    }

    fn transmit_abort(&self) -> Result<(), ErrorCode> {
        self.disable_tx_interrupts();
        Err(ErrorCode::FAIL)
    }
}

impl<'a> Receive<'a> for Lpuart0<'a> {
    fn set_receive_client(&self, client: &'a dyn ReceiveClient) {
        self.rx_client.set(client);
    }

    fn receive_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if len == 0 {
            return Err((ErrorCode::SIZE, buffer));
        }
        if self.rx_buffer.is_some() {
            return Err((ErrorCode::BUSY, buffer));
        }
        self.rx_buffer.replace(buffer);
        self.rx_len.set(len);
        self.rx_position.set(0);
        self.enable_rx_interrupts();
        Ok(())
    }

    fn receive_word(&self) -> Result<(), ErrorCode> {
        if !self.registers.stat.is_set(STAT::RDRF) {
            return Err(ErrorCode::FAIL);
        }

        let _received_data = (self.registers.data.get() & 0xFF) as u8;

        Ok(())
    }

    fn receive_abort(&self) -> Result<(), ErrorCode> {
        self.disable_rx_interrupts();
        Err(ErrorCode::FAIL)
    }
}

impl Configure for Lpuart0<'_> {
    fn configure(&self, params: Parameters) -> Result<(), ErrorCode> {
        self.registers.baud.modify(BAUD::SBR.val(params.baud_rate));
        Ok(())
    }
}
