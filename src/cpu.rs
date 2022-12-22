pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const RAM_OFFSET: usize = 0x400;

const START_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 0x1000;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_regs: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
    offset: i32,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_regs: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
            offset: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_regs = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn get_ram(&self) -> &[u8] {
        &self.ram
    }

    pub fn get_offset(&self) -> i32 {
        self.offset
    }

    pub fn get_beep(&self) -> bool {
        self.st > 0
    }

    pub fn set_offset(&mut self, offset: i32) {
        if offset < 0 {
            self.offset = RAM_SIZE as i32 / RAM_OFFSET as i32 - 1;
        } else if offset * RAM_OFFSET as i32 >= RAM_SIZE as i32 {
            self.offset = 0;
        } else {
            self.offset = offset;
        }
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    pub fn tick_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // println!("BEEP!");
                // return true;
            }
            self.st -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = op & 0x000F;
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // println!("{:?}", self.ram);
        match (d1, d2, d3, d4) {
            // NOP
            (0, 0, 0, 0) => return,
            // CLS
            (0, 0, 0xE, 0) => self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            // RET
            (0, 0, 0xE, 0xE) => self.pc = self.pop(),
            // JMP NNN
            (1, _, _, _) => self.pc = op & 0xFFF,
            // CALL NNN
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = op & 0xFFF;
            }
            // SKIP VX == NN
            (3, _, _, _) => {
                if self.v_regs[d2 as usize] == (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            }
            // SKIP VX != NN
            (4, _, _, _) => {
                if self.v_regs[d2 as usize] != (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            }
            // SKIP VX == VY
            (5, _, _, 0) => {
                if self.v_regs[d2 as usize] == self.v_regs[d3 as usize] {
                    self.pc += 2;
                }
            }
            // SET VX = NN
            (6, _, _, _) => self.v_regs[d2 as usize] = (op & 0xFF) as u8,
            // ADD VX = NN
            (7, _, _, _) => {
                self.v_regs[d2 as usize] = self.v_regs[d2 as usize].wrapping_add((op & 0xFF) as u8)
            }
            // SET VX = VY
            (8, _, _, 0) => self.v_regs[d2 as usize] = self.v_regs[d3 as usize],
            // OR VX = VX | VY
            (8, _, _, 1) => self.v_regs[d2 as usize] |= self.v_regs[d3 as usize],
            // AND VX = VX & VY
            (8, _, _, 2) => self.v_regs[d2 as usize] &= self.v_regs[d3 as usize],
            // XOR VX = VX ^ VY
            (8, _, _, 3) => self.v_regs[d2 as usize] ^= self.v_regs[d3 as usize],
            // ADD VX = VX + VY
            (8, _, _, 4) => {
                let (val, overflow) =
                    self.v_regs[d2 as usize].overflowing_add(self.v_regs[d3 as usize]);
                self.v_regs[d2 as usize] = val;
                self.v_regs[0xF] = overflow as u8;
            }
            // SUB VX = VX - VY
            (8, _, _, 5) => {
                let (val, overflow) =
                    self.v_regs[d2 as usize].overflowing_sub(self.v_regs[d3 as usize]);
                self.v_regs[d2 as usize] = val;
                self.v_regs[0xF] = overflow as u8;
            }
            // SHR VX = VX >> 1
            (8, _, _, 6) => {
                self.v_regs[0xF] = self.v_regs[d2 as usize] & 0x1;
                self.v_regs[d2 as usize] >>= 1;
            }
            // SUBN VX = VY - VX
            (8, _, _, 7) => {
                let (val, overflow) =
                    self.v_regs[d3 as usize].overflowing_sub(self.v_regs[d2 as usize]);
                self.v_regs[d2 as usize] = val;
                self.v_regs[0xF] = if overflow { 0 } else { 1 };
            }
            // SHL VX = VX << 1
            (8, _, _, 0xE) => {
                self.v_regs[0xF] = self.v_regs[d2 as usize] >> 7;
                self.v_regs[d2 as usize] <<= 1;
            }
            // SKIP VX != VY
            (9, _, _, 0) => {
                if self.v_regs[d2 as usize] != self.v_regs[d3 as usize] {
                    self.pc += 2;
                }
            }
            // SET I = NNN
            (0xA, _, _, _) => self.i_reg = op & 0x0FFF,
            // JMP V0 + NNN
            (0xB, _, _, _) => self.pc = (op & 0x0FFF) + self.v_regs[0] as u16,
            // RND VX = rand() & NN
            (0xC, _, _, _) => self.v_regs[d2 as usize] = rand::random::<u8>() & (op & 0x00FF) as u8,
            // DRW VX, VY, N
            (0xD, _, _, _) => {
                let x_coord = self.v_regs[d2 as usize] as u16;
                let y_coord = self.v_regs[d3 as usize] as u16;

                let num_rows = d4;

                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0x80 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + y * SCREEN_WIDTH;
                            flipped |= self.screen[idx];
                            self.screen[idx] = !self.screen[idx];
                        }
                    }
                }
                self.v_regs[0xF] = flipped as u8;
            }
            // KEY VX = key()
            (0xE, _, 9, 0xE) => {
                if self.keys[self.v_regs[d2 as usize] as usize] {
                    self.pc += 2;
                }
            }
            // KEY VX = !key()
            (0xE, _, 0xA, 1) => {
                if !self.keys[self.v_regs[d2 as usize] as usize] {
                    self.pc += 2;
                }
            }
            // SET VX = DT
            (0xF, _, 0, 7) => self.v_regs[d2 as usize] = self.dt,

            // SET VX = K
            (0xF, _, 0, 0xA) => {
                let x = d2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_regs[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2;
                }
            }
            // SET DT = VX
            (0xF, _, 1, 5) => self.dt = self.v_regs[d2 as usize],
            // SET ST = VX
            (0xF, _, 1, 8) => self.st = self.v_regs[d2 as usize],
            // ADD I = VX
            (0xF, _, 1, 0xE) => self.i_reg += self.v_regs[d2 as usize] as u16,
            // SET I = FONT[VX]
            (0xF, _, 2, 9) => self.i_reg = (self.v_regs[d2 as usize] as u16) * 5,
            // BCD VX
            (0xF, _, 3, 3) => {
                let val = self.v_regs[d2 as usize];
                self.ram[self.i_reg as usize] = val / 100;
                self.ram[self.i_reg as usize + 1] = (val / 10) % 10;
                self.ram[self.i_reg as usize + 2] = (val % 100) % 10;
            }
            // SET [I] = VX
            (0xF, _, 5, 5) => {
                for i in 0..=d2 {
                    self.ram[self.i_reg as usize + i as usize] = self.v_regs[i as usize];
                }
            }
            // SET VX = [I]
            (0xF, _, 6, 5) => {
                for i in 0..=d2 {
                    self.v_regs[i as usize] = self.ram[self.i_reg as usize + i as usize];
                }
            }

            (_, _, _, _) => unimplemented!(
                "Unimplemented opcode: {:X} {} {}{}{}{} ",
                op,
                op,
                d1,
                d2,
                d3,
                d4
            ),
            // _ => return
        }
    }
}
