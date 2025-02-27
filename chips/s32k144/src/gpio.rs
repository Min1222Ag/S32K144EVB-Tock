// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! GPIO abstraction for S32K144EVB

use kernel::hil::gpio::{Client, Input, Interrupt, InterruptEdge};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;

#[repr(C)]
pub struct GpioRegisters {
    pub pdor: ReadWrite<u32, PDOR::Register>, // Port Data Output Register
    pub psor: ReadWrite<u32, PSOR::Register>, // Port Set Output Register
    pub pcor: ReadWrite<u32, PCOR::Register>, // Port Clear Output Register
    pub ptor: ReadWrite<u32, PTOR::Register>, // Port Toggle Output Register
    pub pdir: ReadOnly<u32, PDIR::Register>,  // Port Data Input Register
    pub pddr: ReadWrite<u32, PDDR::Register>, // Port Data Direction Register
    pub pidr: ReadWrite<u32, PIDR::Register>, // Port Input Disable Register
}

register_bitfields![u32,
    PDOR[PDO OFFSET(0) NUMBITS(32) []],
    PSOR[PTSO OFFSET(0) NUMBITS(32) []],
    PCOR[PTCO OFFSET(0) NUMBITS(32) []],
    PTOR[PTTO OFFSET(0) NUMBITS(32) []],
    PDIR[PDI OFFSET(0) NUMBITS(32) []],
    PDDR[PDD OFFSET(0) NUMBITS(32) []],
    PIDR[PID OFFSET(0) NUMBITS(32) []]
];

/// S32K144 보드용 GPIO 포트 정의
#[derive(Copy, Clone)]
pub enum Pin {
    PTA0,
    PTA1,
    PTA2,
    PTA3,
    PTA4,
    PTA5,
    PTA6,
    PTA7,
    PTA8,
    PTA9,
    PTA10,
    PTA11,
    PTA12,
    PTA13,
    PTA14,
    PTA15,
    PTB0,
    PTB1,
    PTB2,
    PTB3,
    PTB4,
    PTB5,
    PTB6,
    PTB7,
    PTC0,
    PTC1,
    PTC2,
    PTC3,
    PTC4,
    PTC5,
    PTC6,
    PTC7,
    PTD0,
    PTD1,
    PTD2,
    PTD3,
    PTD4,
    PTD5,
    PTD6,
    PTD7,
    PTE0,
    PTE1,
    PTE2,
    PTE3,
    PTE4,
    PTE5,
    PTE6,
    PTE7,
}

/// GPIO 베이스 주소 (레퍼런스 매뉴얼 기반)
const GPIOA_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x400FF000 as *const GpioRegisters) };
const GPIOB_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x400FF040 as *const GpioRegisters) };
const GPIOC_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x400FF080 as *const GpioRegisters) };
const GPIOD_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x400FF0C0 as *const GpioRegisters) };
const GPIOE_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x400FF100 as *const GpioRegisters) };

/// S32K144 보드에서 사용할 GPIO 핀
pub struct GPIOPin<'a> {
    pin: Pin,
    registers: StaticRef<GpioRegisters>,
    client: OptionalCell<&'a dyn kernel::hil::gpio::Client>,
}

impl<'a> GPIOPin<'a> {
    pub const fn new(pin: Pin, base: StaticRef<GpioRegisters>) -> GPIOPin<'a> {
        GPIOPin {
            pin,
            registers: base,
            client: OptionalCell::empty(),
        }
    }

    pub fn enable_output(&self) {
        self.registers
            .pddr
            .modify(PDDR::PDD.val(self.registers.pddr.get() | (1 << self.pin as u32)));
    }

    pub fn enable_input(&self) {
        self.registers
            .pddr
            .modify(PDDR::PDD.val(self.registers.pddr.get() & !(1 << self.pin as u32)));
    }

    pub fn set_high(&self) {
        self.registers
            .psor
            .write(PSOR::PTSO.val(1 << self.pin as u32));
    }

    pub fn set_low(&self) {
        self.registers
            .pcor
            .write(PCOR::PTCO.val(1 << self.pin as u32));
    }

    pub fn toggle(&self) {
        self.registers
            .ptor
            .write(PTOR::PTTO.val(1 << self.pin as u32));
    }

    pub fn read(&self) -> bool {
        (self.registers.pdir.read(PDIR::PDI) & (1 << self.pin as u32)) != 0
    }
}

impl kernel::hil::gpio::Input for GPIOPin<'_> {
    fn read(&self) -> bool {
        (self.registers.pdir.read(PDIR::PDI) & (1 << self.pin as u32)) != 0
    }
}

impl<'a> kernel::hil::gpio::Output for GPIOPin<'a> {
    fn set(&self) {
        self.set_high();
    }

    fn clear(&self) {
        self.set_low();
    }

    fn toggle(&self) -> bool {
        self.registers
            .ptor
            .write(PTOR::PTTO.val(1 << self.pin as u32));
        self.read() // return : current status
    }
}

impl<'a> kernel::hil::gpio::Interrupt<'a> for GPIOPin<'a> {
    fn set_client(&self, client: &'a dyn kernel::hil::gpio::Client) {
        self.client.set(client);
    }

    fn enable_interrupts(&self, mode: InterruptEdge) {
        // 실제 인터럽트 활성화를 위한 레지스터 설정
        match mode {
            InterruptEdge::RisingEdge => {
                // Rising Edge 인터럽트 설정 (예시 코드)
                self.registers
                    .pddr
                    .modify(PDDR::PDD.val(1 << self.pin as u32));
            }
            InterruptEdge::FallingEdge => {
                // Falling Edge 인터럽트 설정 (예시 코드)
                self.registers
                    .pddr
                    .modify(PDDR::PDD.val(0 << self.pin as u32));
            }
            InterruptEdge::EitherEdge => {
                // Both Edge 인터럽트 설정 (예시 코드)
                self.registers
                    .pddr
                    .modify(PDDR::PDD.val(1 << self.pin as u32 | 0 << self.pin as u32));
            }
        }
    }

    fn disable_interrupts(&self) {
        // 인터럽트 비활성화: 특정 레지스터 값을 변경
        self.registers.pddr.modify(PDDR::PDD.val(0));
    }

    fn is_pending(&self) -> bool {
        (self.registers.pdir.read(PDIR::PDI) & (1 << self.pin as u32)) != 0
    }
}

impl<'a> kernel::hil::gpio::Configure for GPIOPin<'a> {
    fn configuration(&self) -> kernel::hil::gpio::Configuration {
        if self.read() {
            kernel::hil::gpio::Configuration::Input
        } else {
            kernel::hil::gpio::Configuration::Output
        }
    }

    fn make_input(&self) -> kernel::hil::gpio::Configuration {
        self.enable_input();
        kernel::hil::gpio::Configuration::Input
    }

    fn make_output(&self) -> kernel::hil::gpio::Configuration {
        self.enable_output();
        kernel::hil::gpio::Configuration::Output
    }

    fn disable_input(&self) -> kernel::hil::gpio::Configuration {
        self.registers
            .pddr
            .modify(PDDR::PDD.val(self.registers.pddr.get() & !(1 << self.pin as u32)));
        kernel::hil::gpio::Configuration::LowPower
    }

    fn disable_output(&self) -> kernel::hil::gpio::Configuration {
        self.registers
            .pddr
            .modify(PDDR::PDD.val(self.registers.pddr.get() & !(1 << self.pin as u32)));
        kernel::hil::gpio::Configuration::LowPower
    }

    fn deactivate_to_low_power(&self) {
        //TODOs
    }

    fn set_floating_state(&self, _mode: kernel::hil::gpio::FloatingState) {
        // S32K144는 기본적으로 풀다운/풀업 설정이 별도 핀 컨트롤에 있음. 필요하면 구현
    }

    fn floating_state(&self) -> kernel::hil::gpio::FloatingState {
        kernel::hil::gpio::FloatingState::PullNone
    }
}

impl<'a> kernel::hil::led::Led for GPIOPin<'a> {
    fn on(&self) {
        self.set_high();
    }

    fn off(&self) {
        self.set_low();
    }

    fn toggle(&self) {
        self.toggle();
    }

    /// LED 핀을 초기화 (출력으로 설정)
    fn init(&self) {
        self.enable_output();
    }

    /// 현재 LED 상태 읽기 (켜짐: `true`, 꺼짐: `false`)
    fn read(&self) -> bool {
        self.read()
    }
}

/// GPIO 포트 집합
pub struct Port<'a, const N: usize> {
    pub pins: [GPIOPin<'a>; N],
}

impl<'a, const N: usize> Port<'a, N> {
    pub fn new(pins: [GPIOPin<'a>; N]) -> Self {
        Port { pins }
    }
}

/// S32K144 GPIO 포트 초기화 함수
pub fn s32k144_gpio_create<'a>() -> Port<'a, 48> {
    Port::new([
        // GPIOA
        GPIOPin::new(Pin::PTA0, GPIOA_BASE),
        GPIOPin::new(Pin::PTA1, GPIOA_BASE),
        GPIOPin::new(Pin::PTA2, GPIOA_BASE),
        GPIOPin::new(Pin::PTA3, GPIOA_BASE),
        GPIOPin::new(Pin::PTA4, GPIOA_BASE),
        GPIOPin::new(Pin::PTA5, GPIOA_BASE),
        GPIOPin::new(Pin::PTA6, GPIOA_BASE),
        GPIOPin::new(Pin::PTA7, GPIOA_BASE),
        GPIOPin::new(Pin::PTA8, GPIOA_BASE),
        GPIOPin::new(Pin::PTA9, GPIOA_BASE),
        GPIOPin::new(Pin::PTA10, GPIOA_BASE),
        GPIOPin::new(Pin::PTA11, GPIOA_BASE),
        GPIOPin::new(Pin::PTA12, GPIOA_BASE),
        GPIOPin::new(Pin::PTA13, GPIOA_BASE),
        GPIOPin::new(Pin::PTA14, GPIOA_BASE),
        GPIOPin::new(Pin::PTA15, GPIOA_BASE),
        // GPIOB
        GPIOPin::new(Pin::PTB0, GPIOB_BASE),
        GPIOPin::new(Pin::PTB1, GPIOB_BASE),
        GPIOPin::new(Pin::PTB2, GPIOB_BASE),
        GPIOPin::new(Pin::PTB3, GPIOB_BASE),
        GPIOPin::new(Pin::PTB4, GPIOB_BASE),
        GPIOPin::new(Pin::PTB5, GPIOB_BASE),
        GPIOPin::new(Pin::PTB6, GPIOB_BASE),
        GPIOPin::new(Pin::PTB7, GPIOB_BASE),
        // GPIOC
        GPIOPin::new(Pin::PTC0, GPIOC_BASE),
        GPIOPin::new(Pin::PTC1, GPIOC_BASE),
        GPIOPin::new(Pin::PTC2, GPIOC_BASE),
        GPIOPin::new(Pin::PTC3, GPIOC_BASE),
        GPIOPin::new(Pin::PTC4, GPIOC_BASE),
        GPIOPin::new(Pin::PTC5, GPIOC_BASE),
        GPIOPin::new(Pin::PTC6, GPIOC_BASE),
        GPIOPin::new(Pin::PTC7, GPIOC_BASE),
        // GPIOD
        GPIOPin::new(Pin::PTD0, GPIOD_BASE),
        GPIOPin::new(Pin::PTD1, GPIOD_BASE),
        GPIOPin::new(Pin::PTD2, GPIOD_BASE),
        GPIOPin::new(Pin::PTD3, GPIOD_BASE),
        GPIOPin::new(Pin::PTD4, GPIOD_BASE),
        GPIOPin::new(Pin::PTD5, GPIOD_BASE),
        GPIOPin::new(Pin::PTD6, GPIOD_BASE),
        GPIOPin::new(Pin::PTD7, GPIOD_BASE),
        // GPIOE
        GPIOPin::new(Pin::PTE0, GPIOE_BASE),
        GPIOPin::new(Pin::PTE1, GPIOE_BASE),
        GPIOPin::new(Pin::PTE2, GPIOE_BASE),
        GPIOPin::new(Pin::PTE3, GPIOE_BASE),
        GPIOPin::new(Pin::PTE4, GPIOE_BASE),
        GPIOPin::new(Pin::PTE5, GPIOE_BASE),
        GPIOPin::new(Pin::PTE6, GPIOE_BASE),
        GPIOPin::new(Pin::PTE7, GPIOE_BASE),
    ])
}
