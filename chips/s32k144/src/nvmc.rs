use core::cell::Cell;
use core::ops::{Index, IndexMut};
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::flash;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::cells::TakeCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

const FTFC_BASE: StaticRef<FtfcRegisters> =
    unsafe { StaticRef::new(0x40020000 as *const FtfcRegisters) };

#[repr(C)]
pub struct FtfcRegisters {
    pub fstat: ReadWrite<u8, FSTAT::Register>,
    pub fcnfg: ReadWrite<u8, FCNFG::Register>,
    pub fsec: ReadOnly<u8, FSEC::Register>,
    pub fopt: ReadOnly<u8, FOPT::Register>,
    pub fccob3: ReadWrite<u8, FCCOB3::Register>,
    pub fccob2: ReadWrite<u8, FCCOB2::Register>,
    pub fccob1: ReadWrite<u8, FCCOB1::Register>,
    pub fccob0: ReadWrite<u8, FCCOB0::Register>,
    pub fccob7: ReadWrite<u8, FCCOB7::Register>,
    pub fccob6: ReadWrite<u8, FCCOB6::Register>,
    pub fccob5: ReadWrite<u8, FCCOB5::Register>,
    pub fccob4: ReadWrite<u8, FCCOB4::Register>,
    pub fccobb: ReadWrite<u8, FCCOBB::Register>,
    pub fccoba: ReadWrite<u8, FCCOBA::Register>,
    pub fccob9: ReadWrite<u8, FCCOB9::Register>,
    pub fccob8: ReadWrite<u8, FCCOB8::Register>,
    pub fprot3: ReadWrite<u8, FPROT3::Register>,
    pub fprot2: ReadWrite<u8, FPROT2::Register>,
    pub fprot1: ReadWrite<u8, FPROT1::Register>,
    pub fprot0: ReadWrite<u8, FPROT0::Register>,
    _reserved0: [u8; 2],
    pub feprot: ReadWrite<u8, FEPROT::Register>,
    pub fdprot: ReadWrite<u8, FDPROT::Register>,
}

register_bitfields![u8,
    FSTAT [
        MGSTAT0 OFFSET(0) NUMBITS(1) [
            NO_ERROR = 0,
            ERROR = 1
        ],
        FPVIOL OFFSET(4) NUMBITS(1) [
            NO_PROTECTION_VIOLATION = 0,
            PROTECTION_VIOLATION = 1
        ],
        ACCERR OFFSET(5) NUMBITS(1) [
            NO_ACCESS_ERROR = 0,
            ACCESS_ERROR = 1
        ],
        RDCOLERR OFFSET(6) NUMBITS(1) [
            NO_COLLISION_ERROR = 0,
            COLLISION_ERROR = 1
        ],
        CCIF OFFSET(7) NUMBITS(1) [
            BUSY = 0,
            READY = 1
        ]
    ],
    FCNFG[
        EEERDY OFFSET(0) NUMBITS(1) [],
        RAMRDY OFFSET(1) NUMBITS(1) [],
        ERSSUSP OFFSET(4) NUMBITS(1) [],
        ERSAREQ OFFSET(5) NUMBITS(1) [],
        RDCOLLIE OFFSET(6) NUMBITS(1) [],
        CCIE OFFSET(7) NUMBITS(1) []
    ],
    FSEC[
        SEC OFFSET(0) NUMBITS(2) [],
        FSLACC OFFSET(2) NUMBITS(2) [],
        MEEN OFFSET(4) NUMBITS(2) [],
        KEYEN OFFSET(6) NUMBITS(2) []
    ],
    FOPT[
        OPT OFFSET(0) NUMBITS(8) []
    ],
    FCCOB3[CCOB3 OFFSET(0) NUMBITS(8) []],
    FCCOB2[CCOB2 OFFSET(0) NUMBITS(8) []],
    FCCOB1[CCOB1 OFFSET(0) NUMBITS(8) []],
    FCCOB0[CCOB0 OFFSET(0) NUMBITS(8) []],
    FCCOB7[CCOB7 OFFSET(0) NUMBITS(8) []],
    FCCOB6[CCOB6 OFFSET(0) NUMBITS(8) []],
    FCCOB5[CCOB5 OFFSET(0) NUMBITS(8) []],
    FCCOB4[CCOB4 OFFSET(0) NUMBITS(8) []],
    FCCOBB[CCOBB OFFSET(0) NUMBITS(8) []],
    FCCOBA[CCOBA OFFSET(0) NUMBITS(8) []],
    FCCOB9[CCOB9 OFFSET(0) NUMBITS(8) []],
    FCCOB8[CCOB8 OFFSET(0) NUMBITS(8) []],
    FPROT3[PROT OFFSET(0) NUMBITS(8) []],
    FPROT2[PROT OFFSET(0) NUMBITS(8) []],
    FPROT1[PROT OFFSET(0) NUMBITS(8) []],
    FPROT0[PROT OFFSET(0) NUMBITS(8) []],
    FEPROT[EPROT OFFSET(0) NUMBITS(8)],
    FDPROT[DPROT OFFSET(0) NUMBITS(8)],
    FCSESTAT[
        BSY OFFSET(0) NUMBITS(1) [],
        SB OFFSET(1) NUMBITS(1) [],
        BIN OFFSET(2) NUMBITS(1) [],
        BFN OFFSET(3) NUMBITS(1) [],
        BOK OFFSET(4) NUMBITS(1) [],
        RIN OFFSET(5) NUMBITS(1) [],
        EDB OFFSET(6) NUMBITS(1) [],
        IDB OFFSET(7) NUMBITS(1) [],
    ],
    FERSTAT[
        DFDIF OFFSET(1) NUMBITS(1)
    ],
    FERCNFG[
        DFDIE OFFSET(1) NUMBITS(1),
        FDFD OFFSET(5) NUMBITS(1)
    ]
];

const PAGE_SIZE: usize = 4096;

pub struct FtfcPage(pub [u8; PAGE_SIZE]);

impl Default for FtfcPage {
    fn default() -> Self {
        Self([0; PAGE_SIZE])
    }
}

impl FtfcPage {
    //fn len(&self) -> usize {
    //    self.0.len()
    //}
}

impl Index<usize> for FtfcPage {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        &self.0[idx]
    }
}

impl IndexMut<usize> for FtfcPage {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.0[idx]
    }
}

impl AsMut<[u8]> for FtfcPage {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FlashState {
    Ready,
    Read,
    Write,
    Erase,
}

pub struct Ftfc {
    registers: StaticRef<FtfcRegisters>,
    client: OptionalCell<&'static dyn flash::Client<Ftfc>>,
    buffer: TakeCell<'static, FtfcPage>,
    state: Cell<FlashState>,
    deferred_call: DeferredCall,
}

impl Ftfc {
    pub fn new() -> Self {
        Self {
            registers: FTFC_BASE,
            client: OptionalCell::empty(),
            buffer: TakeCell::empty(),
            state: Cell::new(FlashState::Ready),
            deferred_call: DeferredCall::new(),
        }
    }

    pub fn clear_fstat_errors(&self) {
        self.registers
            .fstat
            .modify(FSTAT::FPVIOL::SET + FSTAT::ACCERR::SET + FSTAT::RDCOLERR::SET);
    }

    pub fn is_ready(&self) -> bool {
        self.registers.fstat.is_set(FSTAT::CCIF)
    }

    pub fn configure_writeable(&self) {
        self.registers.fcnfg.modify(FCNFG::CCIE::SET);
    }

    pub fn configure_eraseable(&self) {
        self.registers.fcnfg.modify(FCNFG::ERSAREQ::SET);
    }

    pub fn erase_uicr(&self) {
        self.clear_fstat_errors();
        self.write_command(0x49, 0x10001000);
        self.registers.fstat.modify(FSTAT::CCIF::SET);
        self.wait_for_command_complete();
    }

    fn erase_page_helper(&self, page_number: usize) {
        self.clear_fstat_errors();
        self.write_command(0x09, (page_number * PAGE_SIZE) as u32);
        self.registers.fstat.modify(FSTAT::CCIF::SET);
        self.wait_for_command_complete();
    }

    pub fn read_range(
        &self,
        page_number: usize,
        buffer: &'static mut FtfcPage,
    ) -> Result<(), (ErrorCode, &'static mut FtfcPage)> {
        let mut addr = (page_number * PAGE_SIZE) as *const u8;
        unsafe {
            for i in 0..PAGE_SIZE {
                buffer.0[i] = *addr;
                addr = addr.offset(1);
            }
        }
        self.buffer.replace(buffer);
        self.state.set(FlashState::Read);
        self.deferred_call.set();
        Ok(())
    }

    pub fn write_page(
        &self,
        page_number: usize,
        buffer: &'static mut FtfcPage,
    ) -> Result<(), (ErrorCode, &'static mut FtfcPage)> {
        // 복사본을 만들어서 map_err에서 사용
        let buffer_ref = self
            .buffer
            .replace(buffer)
            .unwrap_or_else(|| panic!("Buffer should be available"));

        self.erase_page(page_number).map_err(|e| {
            let buf = self.buffer.take().unwrap(); // 에러 발생 시 원래 버퍼 반환
            (e, buf)
        })?;

        let mut addr = (page_number * PAGE_SIZE) as *mut u8;
        unsafe {
            for i in 0..PAGE_SIZE {
                addr.write_volatile(buffer_ref.0[i]);
                addr = addr.offset(1);
            }
        }

        self.buffer.replace(buffer_ref); // 다시 저장
        self.state.set(FlashState::Write);
        self.deferred_call.set();

        Ok(())
    }

    pub fn erase_page(&self, page_number: usize) -> Result<(), ErrorCode> {
        self.erase_page_helper(page_number);
        self.state.set(FlashState::Erase);
        self.deferred_call.set();
        Ok(())
    }

    fn write_command(&self, command: u8, address: u32) {
        self.registers.fccob0.set(command);
        self.registers.fccob1.set(((address >> 16) & 0xFF) as u8);
        self.registers.fccob2.set(((address >> 8) & 0xFF) as u8);
        self.registers.fccob3.set((address & 0xFF) as u8);
    }

    fn wait_for_command_complete(&self) {
        while !self.is_ready() {}
    }

    pub fn handle_interrupt(&self) {
        let state = self.state.get();
        self.state.set(FlashState::Ready);
        match state {
            FlashState::Read => {
                self.client.map(|client| {
                    self.buffer.take().map(|buffer| {
                        client.read_complete(buffer, Ok(()));
                    });
                });
            }
            FlashState::Write => {
                self.client.map(|client| {
                    self.buffer.take().map(|buffer| {
                        client.write_complete(buffer, Ok(()));
                    });
                });
            }
            FlashState::Erase => {
                self.client.map(|client| {
                    client.erase_complete(Ok(()));
                });
            }
            _ => {}
        }
    }
}

impl<C: flash::Client<Self>> flash::HasClient<'static, C> for Ftfc {
    fn set_client(&self, client: &'static C) {
        self.client.set(client);
    }
}

impl flash::Flash for Ftfc {
    type Page = FtfcPage;

    fn read_page(
        &self,
        page_number: usize,
        buf: &'static mut Self::Page,
    ) -> Result<(), (ErrorCode, &'static mut Self::Page)> {
        self.read_range(page_number, buf)
    }

    fn write_page(
        &self,
        page_number: usize,
        buf: &'static mut Self::Page,
    ) -> Result<(), (ErrorCode, &'static mut Self::Page)> {
        self.write_page(page_number, buf)
    }

    fn erase_page(&self, page_number: usize) -> Result<(), ErrorCode> {
        self.erase_page(page_number)
    }
}

impl DeferredCallClient for Ftfc {
    fn handle_deferred_call(&self) {
        self.handle_interrupt();
    }

    fn register(&'static self) {
        self.deferred_call.register(self);
    }
}
