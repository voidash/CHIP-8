use rand::random;

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

    pub fn reset(&mut self) { 
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONT_SET);
    }

     // Define how CPU will process each instruction and move through the game
    // 1. fetch value from game
    // 2. Decode the instruction
    // 3. Execute, which will modify our CPU registers or RAM.
    // 4. Move the PC to the next instrction and repeat.

    pub fn tick(&mut self){
        // Fetch
        let op = self.fetch();
        // decoding 
        // executing
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        //fetch the instruction we are about to execute.
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;


        // equivalent to adding two bytes to create a 16 bit 
        // 2f + 4e = 2f4e
        // 2f is higher byte and 4e is lower byte
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timers(&mut self) {
        //every frame decrease the timer by 1;

        // delay timer 
        if self.dt > 0 {
            self.dt -= 1;
        }
        //sound timer
        if self.st > 0 {
            if self.st == 1 {
                // beep
            }
            self.st -= 1;
        }
    }

    fn execute(&mut self,op: u16) {
        // 2f and fe

        // digit 1 is 2 becuase 0010 1111 1111 1110 & 1111 0000 000 0000  = 0010 0000 0000 0000 
        // shift 0010 0000 0000 0000 by 12. which makes it 0010  

        let digit1 = (op & 0xF000)  >> 12;
        let digit2 = (op & 0x0F00)  >> 8;
        let digit3 = (op & 0x00F0)  >> 4;
        let digit4 = op & 0x000F;

        match(digit1, digit2, digit3, digit4) {
            // NOP : do nothing
            (0,0,0,0) => return,
            // CLS : clear screen
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            // 00EE Return from subroutine
            (0,0,0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },
            // 1NNN - Jump 
            // N means any digit
            (1,_,_,_) => {
                // extract those three digits as first digit is simple 0000001 or 1
                let nnn = op & 0xfff;
                self.pc = nnn;
            },
            // 2NNN - Call Subroutine
            (2,_,_,_) => {
                let nnn = op & 0xFFF;
                // push current address to stack to stack 
                self.push(self.pc);
                // update program counter to point to that three digit address.
                self.pc = nnn;
            },

            // 3XNN- skip if VX == NN
            (3,_,_,_) =>  {
                // represents our register and is a 2nd digit
                let x = digit2 as usize;
                // last two bytes
                let nn = (op & 0xff) as u8; 

                if self.v_reg[x] == nn {
                    //skip 1 and go directly to 2
                    self.pc += 2;
                }
            },
            //skip if VX != NN
            (4, _, _ ,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xff) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },

            // skip if VX == VY
            (5, _, _ , 0) =>  {
                    let x = digit2 as usize;
                    let y = digit3 as usize;

                    if self.v_reg[x] == self.v_reg[y] {
                        self.pc += 2;
                    }
            },
            // 6XNN : VX = NN
            // set VX to the number given. ( 8 bit number)
            (6,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xff) as u8;
                self.v_reg[x] = nn;
            },

            // 7NNN: VX += NN
            (7, _, _ ,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xff) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },

            // 8NN0 : VX = VY
            (8,_,_,0) => {
                let X = digit2 as usize;
                let Y = digit3 as usize;

                self.v_reg[X] = self.v_reg[Y];
            }
            // 8XY1 : Bitwose OR: VX | VY
            (8,_,_,1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },
            // 8XY2: Bitwise AND
            (8,_,_,2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            },
            // 8XY3: Bitwise XOR
            (8,_,_,3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            },
            // 8XY4 - VX += VY
            (8,_,_,4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                // flag register
                // set to 1 if there is a carry , set to 0 if there isn't
                // VF is carry flag
                let new_vf = if carry { 1 } else {0};
                self.v_reg[x] = new_vx;
                self.v_reg[0xf] = new_vf;
            },
            // VX -= VY
            (8,_,_,5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx , borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                //set flag to 0 if there is a borrow
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xf] = new_vf;
            },
            // single right shift VX >> 
            (8,_,_,6) => {
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xf] = lsb;
            },
            // Vx = Vy - Vx
            (8,_,_,7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx , borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                //set flag to 0 if there is a borrow
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xf] = new_vf;

            },
            // left shift by 1 , whatever is removed should be stored in flag.
            (8,_,_,0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<=1;
                self.v_reg[0xf] = msb;
            },
            // 9xy0 vx!=vy 
            (9,_,_,0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    //increment program counter
                    self.pc += 2;
                }
            },
            // I reg is used for pointing to RAM. now set whatever is present in 3 hex digits
            (0xA, _ , _, _) => {
                let nnn = op & 0xfff;
                self.i_reg = nnn;
            },
            // set program counter to v_reg + nnn 
            (0xB, _, _ ,_) => {
                let nnn = op & 0xfff;
                self.pc = (self.v_reg[0] as u16) + nnn;
            }

            // CXNN: set VX to rand && NN
            (0xC, _, _ , _)  => {
                let x = digit2 as usize;
                let nn = (op & 0xff) as u8;
                let rng : u8 = random();
                self.v_reg[x] = rng & nn;
            }

            // DNNN: draw sprite .2nd and 3rd digit specify X and Y. Last specifies how tall the pixel is [1 - 16]
            (0xD, _, _ ,_ ) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                let num_rows = digit4;
                
                let mut flipped = false;
                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b10000000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;
                            flipped |= self.screen[idx];
                            self.screen[idx] = true;

                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                }else {
                    self.v_reg[0xF] = 0;
                }


            },

            // Ex9e: Skip if key is pressed
            // 16 different keys 0 to 0xF
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize; 
                let vx = self.v_reg[x];

                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            } ,

            // EXA1
            // skip if key is not pressed
            (0xE, _ , 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            },
            //FX07 
            // set VX = DT (delay timer)
            (0xf, _ , 0 , 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            },

            // WAIT for key press
            // Blocking
            (0xf, _ , 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                
                if !pressed {
                    self.pc -= 2;
                }
            },
            //set Delay timer to VX
            (0xf, _ , 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            },
            // fx18: st = vx
            (0xf, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            },
            // fx1e :   i+= vx
            (0xf, _ ,1 , 0xE) =>  {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },
            //set I to font address
            (0xf, _ , 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg =  c * 5;
            },

            // save our number in BCD format. with hundreds, tens and ones.
            (0xf , _ , 3, 3) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },
            //store v0 to vx in RAM specified by i_register
            (0xf, _ , 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;

                for idx in 0..=x {
                    self.ram[i+idx] = self.v_reg[idx];
                }
            },
            //load i into v0 to vx
            (0xf, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i+idx];
                }
            }

            (_ , _, _, _) => unimplemented!("unimplemented opcode: {}", op),

        }
    
    }

    // for displaying
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    //load game code into our ram
    pub fn load(&mut self , data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();

        self.ram[start..end].copy_from_slice(data);
    }

}
