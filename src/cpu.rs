use crate::{memory::Memory, screen::Screen};
use rand::Rng;

pub struct Cpu {
    pc: u16,
    i: u16,
    stack: CallStack,
    delay_tim: u8,
    sound_tim: u8,
    regs: [u8; 16],
}

struct CallStack {
    stack: [u16; 10],
    idx: u8,
}

impl CallStack {
    fn new() -> Self {
        Self {
            stack: [0; 10],
            idx: 0,
        }
    }

    fn push(&mut self, addr: u16) {
        self.stack[self.idx as usize] = addr;
        self.idx += 1;
    }

    fn pop(&mut self) -> u16 {
        assert!(self.idx != 0);
        self.idx -= 1;
        self.stack[self.idx as usize]
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            i: 0,
            stack: CallStack::new(),
            delay_tim: 0,
            sound_tim: 0,
            regs: [0; 16],
        }
    }

    // returns false when reach end of program
    pub fn tick(&mut self, memory: &mut Memory, screen: &mut Screen) -> bool {
        let instr: u16 = self.fetch(memory);
        self.execute(instr, memory, screen);

        self.pc < 4096
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_tim > 0 {
            self.delay_tim -= 1;
        }
        if self.sound_tim > 0 {
            self.sound_tim -= 1;
        }
    }

    fn fetch(&mut self, memory: &Memory) -> u16 {
        let instr = ((memory.load(self.pc) as u16) << 8) | (memory.load(self.pc + 1) as u16);
        self.pc += 2;
        instr
    }

    fn execute(&mut self, instr: u16, memory: &mut Memory, screen: &mut Screen) {
        let first_nibble = ((instr & 0xF000) >> 12) as u8;
        let x = ((instr & 0x0F00) >> 8) as u8;
        let y = ((instr & 0x00F0) >> 4) as u8;
        let n = (instr & 0x000F) as u8;
        let nn = (instr & 0x00FF) as u8;
        let nnn = (instr & 0x0FFF) as u16;

        match first_nibble {
            0x0 => match nnn {
                0x0E0 => self.clear_screen(screen),
                0x0EE => self.ret(),
                _ => {}
            },
            0x1 => self.jump(nnn),
            0x2 => self.call(nnn),
            0x3 => self.skip_e_imm(x, nn),
            0x4 => self.skip_ne_imm(x, nn),
            0x5 => match n {
                0x0 => self.skip_e(x, y),
                _ => {}
            },
            0x6 => self.set_imm(x, nn),
            0x7 => self.add_imm(x, nn),
            0x8 => match n {
                0 => self.set(x, y),
                1 => self.or(x, y),
                2 => self.and(x, y),
                3 => self.xor(x, y),
                4 => self.add(x, y),
                5 => self.sub_a(x, y),
                6 => self.shift_right(x),
                7 => self.sub_b(x, y),
                0xE => self.shift_left(x),
                _ => {}
            },
            0x9 => match n {
                0x0 => self.skip_ne(x, y),
                _ => {}
            },
            0xA => self.set_index(nnn),
            0xB => self.jump_offset(nnn),
            0xC => self.random(x, nn),
            0xD => self.display(x, y, n, memory, screen),
            0xE => match nn {
                0x9E => self.skip_if_key_state(x, true, screen),
                0xA1 => self.skip_if_key_state(x, false, screen),
                _ => {}
            },
            0xF => match nn {
                0x07 => self.get_delay_tim(x),
                0x0A => self.get_key(x, screen),
                0x15 => self.set_delay_tim(x),
                0x18 => self.set_sound_tim(x),
                0x1E => self.add_to_index(x),
                0x29 => self.font_character(x),
                0x33 => self.bcd_conversion(x, memory),
                0x55 => self.store_memory(x, memory),
                0x65 => self.load_memory(x, memory),
                _ => {}
            }
            _ => {}
        }
    }

    fn clear_screen(&self, screen: &mut Screen) {
        screen.clear();
    }

    fn ret(&mut self) {
        self.pc = self.stack.pop();
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn set_imm(&mut self, reg_idx: u8, value: u8) {
        self.regs[reg_idx as usize] = value;
    }

    fn set(&mut self, x: u8, y: u8) {
        self.regs[x as usize] = self.regs[y as usize];
    }

    fn add_imm(&mut self, reg_idx: u8, imm: u8) {
        self.regs[reg_idx as usize] = u8::wrapping_add(self.regs[reg_idx as usize], imm);
    }

    fn set_index(&mut self, addr: u16) {
        self.i = addr;
    }

    fn display(&mut self, x: u8, y: u8, n: u8, memory: &Memory, screen: &mut Screen) {
        // Draw an n pixel tall sprite from memory location that the I register is holding
        // at horizontal coordinate in x register and vertical coordinate in y register
        // If any pixels were turned off, set VF flag to 1. Otherwise, it is set to 0.

        let x = self.regs[x as usize] % 64;
        let y = self.regs[y as usize] % 32;

        let mut pixel_off_flag = false;

        for row in 0..n {
            if y + row > 31 {
                break;
            }

            let byte = memory.load(self.i + row as u16);
            for col in 0u8..8 {
                if x + col > 63 {
                    break;
                }

                if byte & (1 << (7 - col)) != 0 {
                    pixel_off_flag = screen.toggle_pixel(x + col, y + row) || pixel_off_flag;
                }
            }
        }

        self.regs[0xF] = match pixel_off_flag {
            true => 1,
            false => 0,
        };

        screen.update();
    }

    fn call(&mut self, fn_addr: u16) {
        self.stack.push(self.pc);
        self.pc = fn_addr;
    }

    fn skip_e_imm(&mut self, x: u8, nn: u8) {
        if self.regs[x as usize] == nn {
            self.pc += 2;
        }
    }

    fn skip_ne_imm(&mut self, x: u8, nn: u8) {
        if self.regs[x as usize] != nn {
            self.pc += 2;
        }
    }

    fn skip_e(&mut self, x: u8, y: u8) {
        if self.regs[x as usize] == self.regs[y as usize] {
            self.pc += 2;
        }
    }

    fn skip_ne(&mut self, x: u8, y: u8) {
        if self.regs[x as usize] != self.regs[y as usize] {
            self.pc += 2;
        }
    }

    fn or(&mut self, x: u8, y: u8) {
        self.regs[x as usize] |= self.regs[y as usize];
    }

    fn and(&mut self, x: u8, y: u8) {
        self.regs[x as usize] &= self.regs[y as usize];
    }

    fn xor(&mut self, x: u8, y: u8) {
        self.regs[x as usize] ^= self.regs[y as usize];
    }

    fn add(&mut self, x: u8, y: u8) {
        let res: u16 = self.regs[x as usize] as u16 + self.regs[y as usize] as u16;
        self.regs[0xF] = match res > 255 {
            true => 1,
            false => 0,
        };

        self.regs[x as usize] = res as u8;
    }

    fn sub_a(&mut self, x: u8, y: u8) {
        let reg_idx = x;
        let (x, y) = (self.regs[x as usize], self.regs[y as usize]);
        self.regs[0xF] = match x > y {
            true => 1,
            false => 0,
        };

        self.regs[reg_idx as usize] = u8::wrapping_sub(x, y);
    }

    fn sub_b(&mut self, x: u8, y: u8) {
        let reg_idx = x;
        let (x, y) = (self.regs[x as usize], self.regs[y as usize]);
        self.regs[0xF] = match y > x {
            true => 1,
            false => 0,
        };

        self.regs[reg_idx as usize] = u8::wrapping_sub(y, x);
    }

    fn shift_left(&mut self, x: u8) {
        self.regs[0xF] = match (self.regs[x as usize] & 0x80) == 0x80 {
            true => 1,
            false => 0,
        };

        self.regs[x as usize] <<= 1;
    }

    fn shift_right(&mut self, x: u8) {
        self.regs[0xF] = match (self.regs[x as usize] & 0x1) == 0x1 {
            true => 1,
            false => 0,
        };

        self.regs[x as usize] >>= 1;
    }

    fn jump_offset(&mut self, nnn: u16) {
        self.jump(nnn + self.regs[0] as u16);
    }

    fn random(&mut self, x: u8, nn: u8) {
        let mut rng = rand::rng();

        self.regs[x as usize] = rng.random::<u8>() & nn;
    }

    fn skip_if_key_state(&mut self, x: u8, if_state: bool, screen: &mut Screen) {
        if screen.get_keypad().is_pressed_idx(self.regs[x as usize]) == if_state {
            self.pc += 2;
        }
    }

    fn get_delay_tim(&mut self, x: u8) {
        self.regs[x as usize] = self.delay_tim;
    }

    fn set_delay_tim(&mut self, x: u8) {
        self.delay_tim = self.regs[x as usize];
    }

    fn set_sound_tim(&mut self, x: u8) {
        self.sound_tim = self.regs[x as usize];
    }

    fn add_to_index(&mut self, x: u8) {
        self.i += self.regs[x as usize] as u16;
        self.regs[0xF] = match self.i > 0xFFF {
            true => 1,
            false => 0,
        };
    }

    fn get_key(&mut self, x: u8, screen: &mut Screen) {
        for idx in 0..16 {
            if screen.get_keypad().is_pressed_idx(idx) {
                self.regs[x as usize] = idx;
                return;
            }
        }

        self.pc -= 2;
    }

    fn font_character(&mut self, x: u8) {
        let ch = self.regs[x as usize] & 0xF;
        self.i = 0x50 + ch as u16 * 5;
    }

    fn bcd_conversion(&mut self, x: u8, memory: &mut Memory) {
        let x = self.regs[x as usize];
        let hundreds = x / 100;
        let tens = (x % 100) / 10;
        let ones = x % 10;
        memory.store(self.i, hundreds);
        memory.store(self.i + 1, tens);
        memory.store(self.i + 2, ones);
    }

    fn store_memory(&mut self, x: u8, memory: &mut Memory) {
        for reg in 0..=x {
            memory.store(self.i + reg as u16, self.regs[reg as usize]);
        }
    }

    fn load_memory(&mut self, x: u8, memory: &mut Memory) {
        for reg in 0..=x {
            self.regs[reg as usize] = memory.load(self.i + reg as u16);
        }
    }
}
