const FONTSET_SIZE: usize = 80;

const FONT_SET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;


const START_ADDR: u16 = 0x200;

pub const SCREEN_WIDTH: usize =  64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Emu {
    // program counter
    pc: u16,
    // RAM
    ram: [u8; RAM_SIZE],
    // SCREEN : size is 64x32
    screen : [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    // Registers starting from V0 to VF
    v_reg: [u8; NUM_REGS],
    // 16 bit register called I register used for indexing into RAM for reads and writes
    i_reg: u16,
    // stack pointers
    sp: u16,
    // stack
    stack : [u16; STACK_SIZE],
    // Delay Timer , that counts every cycle and performs actions once it hits 0.
    dt: u8,
    // Sound Timer , counts every clock cycle and upon hitting 0 emits a noise.
    st: u8,
    // Keys
    keys: [bool; NUM_KEYS],
}

impl Emu {
    pub fn new() -> Self {
       let mut new_emu = Self {
           pc: START_ADDR,
           ram : [0; RAM_SIZE],
           screen: [false; SCREEN_WIDTH* SCREEN_HEIGHT],
           v_reg: [0; NUM_REGS],
           i_reg: 0,
           sp: 0,
           stack: [0; STACK_SIZE],
           keys: [false; NUM_KEYS],
           dt: 0,
           st: 0
       };

       new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONT_SET);

    new_emu
    }

    fn push(&mut self, val: u16) {
       self.stack[self.sp as usize]  = val;
       self.sp +=1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }


}
