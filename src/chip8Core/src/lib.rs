pub const WINDOWS_WIDTH:usize = 64;
pub const WINDOWS_HEIGHT:usize = 32

const START_ADDR:u16 = 0x200

const RAM_SIZE:usize = 4096;
const NUM_REG: usize = 16;
const STACK_SIZE: usize = 16;
const KEY_SIZE: usize = 16;
pub struct Emu{
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; WINDOWS_WIDTH * WINDOWS_HEIGHT],
    v_reg: [u8; NUM_REG],    //V0 to VF
    i_reg: u16,
    sp: u16,
    stack: [u16, STACK_SIZE],
    keys: [u16, KEY_SIZE],
    dt: u8, //delay timer
    st: u8, //sound timer
}

impl Emu{
    pub fn new() -> Self {
        Self{
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; WINDOWS_WIDTH * WINDOWS_HEIGHT],
            v_reg: [0; NUM_REG],
            i_reg: 0,
            sp: 0,
            stack: [0, STACK_SIZE],
            keys: [0, KEY_SIZE],
            dt: 0,
            st: 0, 
        } 
    }
}

