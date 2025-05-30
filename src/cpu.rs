use crate::{memory::Memory, screen::Screen};

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
            idx: 0
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
            0x0 => {
                match nnn {
                    0x0E0 => self.clear_screen(screen),
                    0x0EE => self.ret(),
                    _ => {}
                }
            },
            0x1 => {
                self.jump(nnn);
            },
            0x6 => {
                self.set(x, nn);
            }
            0x7 => {
                self.add_imm(x, nn);
            }
            0xA => {
                self.set_index(nnn);
            }
            0xD => {
                self.display(x, y, n, memory, screen);
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

    fn set(&mut self, reg_idx: u8, value: u8) {
        self.regs[reg_idx as usize] = value;
    }

    fn add_imm(&mut self, reg_idx: u8, imm: u8) {
        self.regs[reg_idx as usize] += imm;
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
            if y + row > 32 {
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

        if pixel_off_flag {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0;
        }
        screen.update();
    }
}