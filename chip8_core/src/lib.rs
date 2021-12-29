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
       Self {
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
       } 
    }
}
