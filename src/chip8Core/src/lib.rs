pub const WINDOWS_WIDTH:usize = 64;
pub const WINDOWS_HEIGHT:usize = 32

const RAM_SIZE:usize = 4096;
const NUM_REG: usize = 16;
pub struct Emu{
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; WINDOWS_WIDTH * WINDOWS_HEIGHT],
    v_reg: [u8; NUM_REG],    //V0 to VF
    i_reg: u16,
}