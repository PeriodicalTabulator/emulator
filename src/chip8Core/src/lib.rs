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


    fn push(&mut self, val:u16){
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16{
        self.sp -= 1;
        self.stack[self.sp as usize]
        //note that we dont handle empty stack but think of it as if ROM with game would have empty stack
        // it would be bug inside of game and not in emulator so its acceptable to get panic

        //feel free to implement
    }


}

