use super::cpu::Cpu;
use super::memory::Memory;
use super::screen::Screen;
use std::time;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Emu {
    cpu: Cpu,
    memory: Memory,
    screen: Screen,
}

impl Emu {
    pub fn new() -> Self {
        let memory = Memory::new();
        let screen = Screen::new("chip8 emu", 18, 18);

        Self {
            memory,
            screen,
            cpu: Cpu::new(),
        }
    }

    pub fn run(&mut self, game_file: &str) {
        self.store_font();
        self.store_game(game_file);

        let mut open: bool = true;
        let mut running: bool = true;

        let mut last_tick_time = time::Instant::now();
        let mut last_timer_time = time::Instant::now();

        while open {
            if running && last_tick_time.elapsed().as_micros() > (1000000.0 / 700.0) as u128 {
                running = self.cpu.tick(&mut self.memory, &mut self.screen);
                last_tick_time = time::Instant::now();
            }

            if running && last_timer_time.elapsed().as_micros() > (1000000.0 / 60.0) as u128 {
                self.cpu.decrement_timers();
                last_timer_time = time::Instant::now();
            }

            open = self.screen.handle_input();
        }
    }

    fn store_font(&mut self) {
        let font: [u8; 5 * 16] = [
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

        for (i, addr) in (0x050u16..=0x09F).enumerate() {
            self.memory.store(addr, font[i]);
        }
    }

    fn store_game(&mut self, game_file: &str) {
        let path = Path::new(game_file);
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };

        let mut buf = [0; 1];
        let mut pc: u16 = 0x200;
        loop {
            let size = match file.read(&mut buf) {
                Err(_) => panic!("couldn't read from file"),
                Ok(size) => size,
            };

            if size == 0 {
                break;
            }

            self.memory.store(pc, buf[0]);
            pc += 1;
        }
    }
}
