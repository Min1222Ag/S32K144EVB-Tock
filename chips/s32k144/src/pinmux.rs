use kernel::utilities::cells::VolatileCell;
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;

const NUM_PORTS: usize = 5;
const PIN_PER_PORT: usize = 32;

// 사용된 핀을 추적
static mut USED_PINS: [VolatileCell<u32>; NUM_PORTS] = [
    VolatileCell::new(0),
    VolatileCell::new(0),
    VolatileCell::new(0),
    VolatileCell::new(0),
    VolatileCell::new(0),
];

pub const PORT_BASES: [StaticRef<PortRegisters>; NUM_PORTS] = [
    unsafe { StaticRef::new(0x40049000 as *const PortRegisters) }, // PORTA
    unsafe { StaticRef::new(0x4004A000 as *const PortRegisters) }, // PORTB
    unsafe { StaticRef::new(0x4004B000 as *const PortRegisters) }, // PORTC
    unsafe { StaticRef::new(0x4004C000 as *const PortRegisters) }, // PORTD
    unsafe { StaticRef::new(0x4004D000 as *const PortRegisters) }, // PORTE
];

/// PORT 레지스터 구조체
#[repr(C)]
pub struct PortRegisters {
    pub pcr: [ReadWrite<u32, PCR::Register>; 32], // 각 핀의 설정 레지스터 (PCR)
}

register_bitfields![u32,
    pub PCR [
        MUX OFFSET(8) NUMBITS(3) [
            GPIO = 1,      // 일반 GPIO 모드
            ALT2_FTM = 2,  // FTM (PWM) 기능
            ALT3_UART = 3, // UART 기능
            ALT4_CAN = 4,  // CAN 기능
            ALT5_SPI = 5   // SPI 기능
        ]
    ]
];

/// **S32K144 핀 멀티플렉서 핸들러**
#[derive(Copy, Clone)]
pub struct Pinmux {
    port: usize, // 0: PORTA, 1: PORTB, 2: PORTC, 3: PORTD, 4: PORTE
    pin: usize,  // 해당 포트 내 핀 번호
}

impl Pinmux {
    /// **새로운 `Pinmux` 생성**
    ///
    /// * `port`: 0 ~ 4 (PORTA ~ PORTE)
    /// * `pin`: 0 ~ 31
    ///
    /// 이미 사용된 핀을 중복할 경우 패닉 발생.
    pub unsafe fn new(port: usize, pin: usize) -> Pinmux {
        if port >= NUM_PORTS || pin >= PIN_PER_PORT {
            panic!("Invalid port ({}) or pin ({})!", port, pin);
        }

        let used_pins = USED_PINS[port].get();
        if used_pins & (1 << pin) != 0 {
            panic!("Pin P{}{} is already in use!", port + 65, pin);
        } else {
            USED_PINS[port].set(used_pins | (1 << pin));
            Pinmux { port, pin }
        }
    }
}

impl From<Pinmux> for (usize, usize) {
    fn from(val: Pinmux) -> Self {
        (val.port, val.pin)
    }
}
