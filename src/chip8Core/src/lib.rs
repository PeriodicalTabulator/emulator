use rand::random;

pub const WINDOWS_WIDTH:usize = 64;
pub const WINDOWS_HEIGHT:usize = 32;

const START_ADDR:u16 = 0x200;

const RAM_SIZE:usize = 4096;
const NUM_REG: usize = 16;
const STACK_SIZE: usize = 16;
const KEY_SIZE: usize = 16;

//ive decided to create my own font for this emulator even if ROM comes with his own 
//it definitly makes faster compile by taking and matching font characters rather then getting it from ram to memory and compile it that way
/*it contains 8 bites in row and 5 collumns
so it will looks like this:
00100000
01100000
00100000
00100000
01110000 */

const FONTSET_SIZE:usize = 80;
const FONTSET:[u8; FONTSET_SIZE]= [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];



pub struct Emu{
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; WINDOWS_WIDTH * WINDOWS_HEIGHT],
    v_reg: [u8; NUM_REG],    //V0 to VF
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [u16; KEY_SIZE],
    dt: u8, //delay timer
    st: u8, //sound timer
}

impl Emu{
    pub fn new() -> Self {
       let mut new_emu = Self{
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; WINDOWS_WIDTH * WINDOWS_HEIGHT],
            v_reg: [0; NUM_REG],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [0; KEY_SIZE],
            dt: 0,
            st: 0, 
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
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

    pub fn reset(&mut self) {
    self.pc = START_ADDR;
    self.ram = [0; RAM_SIZE];
    self.screen = [false; WINDOWS_WIDTH * WINDOWS_HEIGHT];
    self.v_reg = [0; NUM_REG];
    self.i_reg = 0;
    self.sp = 0;
    self.stack = [0; STACK_SIZE];
    self.keys = [0; KEY_SIZE];
    self.dt = 0;
    self.st = 0;
    self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();

        self.execute(op);
    }

    fn  fetch (&mut self) -> u16{
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc +1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }
    pub fn tick_timer(&mut self){
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                        //sound
            }
            self.st -= 1;
        }
    }
    fn execute(&mut self, op:u16){
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x00F;
    
        match (digit1, digit2, digit3, digit4){
            //NOP
            (0,0,0,0) => return,
            //Clear screen
            (0,0,0xE,0) => {
                self.screen = [false; WINDOWS_WIDTH * WINDOWS_HEIGHT];
            },
            //Retrn from subroute
            (0,0,0xE,0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },
            //1NNN jump
            (1,_,_,_) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },
            //2NNN jump 
            (2,_,_,_) =>{
                let nnn = 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            },
            //3NNN skip if VX == NN
            (3,_,_,_) =>{
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc +=2;
                }
            },
            //4XNN skip if VX != NN
            (4,_,_,_) =>{
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc +=2;
                }
            },
            //5XY0 skip next if VX == VY
            (5,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y]{
                    self.pc += 2;
                }
            },
            //6XNN VX = NN
            (6,_,_,_) =>{
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },
            //7XNN VX += NN
            (7,_,_,_) =>{
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },
            //8XY0 VX = VY 
            (8,_,_,0) =>{
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            },
            (8,_,_,1) =>{
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },
            (8,_,_,2) =>{
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            },
            (8,_,_,3)=> {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            },
            (8,_,_,4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let(new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry {1} else {0};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            (8,_,_,5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] =new_vx;
                self.v_reg[0xF] = new_vf
            },
            (8,_,_,6) => {
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },
            (8,_,_,7) => {
                let x = digit2 as usize; 
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            (8,_,_,0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1 ;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },
            (9,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc +=2;
                }
            },
            (0xA,_,_,_) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },
            (0xB,_,_,_) =>{
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            },
            (0xC,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            }
            (_,_,_,_) => unimplemented!("Not in opcode: {}", op),
        }
    }

}

