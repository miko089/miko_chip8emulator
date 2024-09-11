
pub struct Chip8 {
    memory: [u8; 4096], // 4kB of RAM
    vx:     [u8; 16], // 16 8-bit registers from V0 to VF
    dt:     u8, // 8-bit delay timer
    st:     u8, // 8-bit sound timer
    pc:     u16, // 16-bit program counter (index of current operation from RAM)
    stack:  [u16; 16], // 16 elements 16-bit stack
    sp:     u8, // 8-bit stack pointer
    i:      u16, // index of sprite
    gfx:    [u8; 64 * 32], // state of screen
    pub keyboard: [bool; 16], // true if pressed
}

impl Chip8 {
    // new creates a new Chip8 instance
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            memory:   [0; 4096],
            vx:       [0; 16],
            dt:       0,
            st:       0,
            pc:       0x200,
            stack:    [0; 16],
            sp:       0,
            i:        0,
            gfx:      [0; 64 * 32],
            keyboard: [false; 16]
        };
        chip8.load_fonts();
        chip8
    }

    // timer_tick decrements the delay timer and sound timer
    pub fn timer_tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn is_sound_playing(&self) -> bool {
        self.st > 0
    }

    // load_fonts loads the hexadecimal fonts into memory (digits from 0 to F)
    pub fn load_fonts(&mut self) {
        // each digit is 5 bytes long, every byte is a row on screen
        let d_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        let d_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
        let d_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
        let d_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
        let d_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
        let d_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
        let d_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
        let d_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
        let d_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
        let d_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
        let d_a: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
        let d_b: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
        let d_c: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
        let d_d: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
        let d_e: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
        let d_f: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];
        let digits =
            [d_0, d_1, d_2, d_3, d_4, d_5, d_6, d_7, d_8, d_9, d_a, d_b, d_c, d_d, d_e, d_f];
        for (i, digit) in digits.iter().enumerate() {
            for (j, row) in digit.iter().enumerate() {
                self.memory[i * 5 + j] = *row;
            }
        }
    }

    // load_rom loads the ROM into memory
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // check if rom is too big
        if rom.len() > 4096 - 0x200 {
            panic!("ROM is too big to fit in memory");
        }
        for (i, byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

    // get_screen returns the current state of the screen
    pub fn get_screen(&self) -> &[u8] {
        &self.gfx
    }

    // clear_screen clears the screen
    fn clear_screen(&mut self) {
        self.gfx = [0; 64 * 32];
    }

    // return from subroutine
    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    // jump to address
    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    // call a subroutine at address
    fn call(&mut self, address: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = address;
    }

    // skip next instruction if Vx == byte
    fn se(&mut self, byte1: u8, byte2: u8) {
        if byte1 == byte2 {
            self.pc += 2;
        }
    }

    // skip next instruction if Vx != byte
    fn sne(&mut self, byte1: u8, byte2: u8) {
        if byte1 != byte2 {
            self.pc += 2;
        }
    }


    // execute next instruction
    pub fn next_instruction(&mut self) {
        let instruction: u16 =
            ((self.memory[self.pc as usize] as u16) << 8) |
                self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        let opcode = instruction & 0xF000;
        match opcode {
            0x0000 => match instruction {
                0x00E0 => self.clear_screen(),
                0x00EE => self.ret(),
                _      => (),
            },
            0x1000 => {
                let addr= instruction & 0x0FFF;
                self.jump(addr);
            }
            0x2000 => {
                let address = instruction & 0x0FFF;
                self.call(address);
            },
            0x3000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let byte = (instruction & 0x00FF) as u8;
                self.se(self.vx[x], byte);
            },
            0x4000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let byte = (instruction & 0x00FF) as u8;
                self.sne(self.vx[x], byte);
            },
            0x5000 => {
                if instruction & 0x000F != 0 {
                    return;
                }
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let y = ((instruction & 0x00F0) >> 4) as usize;
                self.se(self.vx[x], self.vx[y]);
            },
            0x6000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let byte = (instruction & 0x00FF) as u8;
                self.vx[x] = byte;
            },
            0x7000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let byte = (instruction & 0x00FF) as u8;
                self.vx[x] = self.vx[x].wrapping_add(byte);
            },
            0x8000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let y = ((instruction & 0x00F0) >> 4) as usize;
                match instruction & 0x000F {
                    // set Vx = Vy
                    0x0000 => self.vx[x] = self.vx[y],

                    // set Vx = Vx | Vy
                    0x0001 => self.vx[x] |= self.vx[y],

                    // set Vx = Vx & Vy
                    0x0002 => self.vx[x] &= self.vx[y],

                    // set Vx = Vx ^ Vy
                    0x0003 => self.vx[x] ^= self.vx[y],

                    // set Vx = Vx + Vy, set VF = carry (if overflow, than VF = 1, else 0)
                    0x0004 => {
                        let (res, overflow) = self.vx[x].overflowing_add(self.vx[y]);
                        self.vx[0xf] = if overflow { 1 } else { 0 };
                        self.vx[x] = res;
                    },

                    // set Vx = Vx - Vy, set VF = NOT borrow (if Vx > Vy, then VF = 1, else 0)
                    0x0005 => {
                        let (res, overflow) = self.vx[x].overflowing_sub(self.vx[y]);
                        self.vx[0xf] = if overflow { 0 } else { 1 };
                        self.vx[x] = res;
                    },

                    // if least-significant bit of Vx is 1, then VF = 1, else 0. Then Vx = Vx >> 1
                    0x0006 => {
                        self.vx[0xf] = self.vx[x] & 0x1;
                        self.vx[x] >>= 1;
                    },

                    // if Vy > Vx, then VF = 1, else 0. Then Vx = Vy - Vx
                    0x0007 => {
                        let (res, overflow) = self.vx[y].overflowing_sub(self.vx[x]);
                        self.vx[0xf] = if overflow { 0 } else { 1 };
                        self.vx[x] = res;
                    },

                    // Vf is set to most significant bit of Vx, then Vx = Vx << 1
                    0x000E => {
                        self.vx[0xf] = self.vx[x] >> 7;
                        self.vx[x] <<= 1;
                    },
                    _      => (),
                }
            },
            0x9000 => {
                if instruction & 0x000F != 0 {
                    return;
                }
                // skip next instruction if Vx != Vy
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let y = ((instruction & 0x00F0) >> 4) as usize;
                self.sne(self.vx[x], self.vx[y]);
            },
            0xA000 => {
                // set I = nnn
                let addr = instruction & 0x0FFF;
                self.i = addr;
            },
            0xB000 => {
                // jump to location nnn + V0
                let addr = instruction & 0x0FFF;
                self.jump(addr + self.vx[0] as u16);
            },
            0xC000 => {
                // set Vx = random byte AND kk
                let x = ((instruction & 0x0F00) >> 8) as usize;
                let byte = (instruction & 0x00FF) as u8;
                self.vx[x] = rand::random::<u8>() & byte;
            },
            0xD000 => {
                // draw sprite at (Vx, Vy) with width 8 and height n
                let vx = self.vx[((instruction & 0x0F00) >> 8) as usize] as usize;
                let vy = self.vx[((instruction & 0x00F0) >> 4) as usize] as usize;
                let n = (instruction & 0x000F) as usize;

                self.vx[0xf] = 0;

                for i in 0..n {
                    let sprite = self.memory[self.i as usize + i];
                    for j in 0..8 {
                        let bit = (sprite >> (7 - j)) & 0x1;
                        let x = (vx + j) % 64;
                        let y = (vy + i) % 32;
                        let idx = x + y * 64;

                        if bit == 1 && self.gfx[idx] == 1 {
                            self.vx[0xf] = 1;
                        }

                        self.gfx[idx] ^= bit;
                    }
                }
            },
            0xE000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                match instruction & 0x00FF {
                    // skip next instruction if key with value Vx is pressed
                    0x009E => {
                        let key = self.vx[x];
                        if self.keyboard[key as usize] == true {
                            self.pc += 2;
                        }
                    },
                    // skip next instruction if key with value Vx is not pressed
                    0x00A1 => {
                        let key = self.vx[x];
                        if self.keyboard[key as usize] == false {
                            self.pc += 2;
                        }
                    },
                    _      => (),
                }
            },
            0xF000 => {
                let x = ((instruction & 0x0F00) >> 8) as usize;
                match instruction & 0x00FF {
                    // set Vx = delay timer value
                    0x0007 => self.vx[x] = self.dt,

                    // wait for key press, store key value in Vx
                    0x000A => {
                        let mut key_pressed = false;
                        for i in 0..16 {
                            if self.keyboard[i] {
                                self.vx[x] = i as u8;
                                key_pressed = true;
                                break;
                            }
                        }
                        if !key_pressed {
                            self.pc -= 2;
                        }
                    },

                    // set delay timer = Vx
                    0x0015 => self.dt = self.vx[x],

                    // set sound timer = Vx
                    0x0018 => self.st = self.vx[x],

                    // set I = I + Vx
                    0x001E => self.i += self.vx[x] as u16,

                    // set I = location of sprite for digit Vx
                    0x0029 => self.i = self.vx[x] as u16 * 5,

                    // store BCD representation of Vx in memory locations I, I+1, I+2
                    0x0033 => {
                        let vx = self.vx[x];
                        self.memory[self.i as usize] = vx / 100;
                        self.memory[self.i as usize + 1] = (vx / 10) % 10;
                        self.memory[self.i as usize + 2] = vx % 10;
                    },

                    // store registers V0 through Vx in memory starting at location I
                    0x0055 => {
                        for i in 0..=x {
                            self.memory[self.i as usize + i] = self.vx[i];
                        }
                        self.i = self.i + ((x + 1) as u16);
                    },

                    // read registers V0 through Vx from memory starting at location I
                    0x0065 => {
                        for i in 0..x + 1 {
                            self.vx[i] = self.memory[self.i as usize + i];
                        }
                        self.i = self.i + ((x + 1) as u16);
                    },
                    _      => (),
                }
            },
            _ => (),
        }
    }
}
