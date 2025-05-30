use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::keypad::{Keypad, ButtonCode};

pub struct Screen {
    framebuf: [[bool; 32]; 64],
    context: sdl2::Sdl,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    keypad: Keypad,
    pwidth: u32,
    pheight: u32,
}

impl Screen {
    pub fn new(name: &str, pwidth: u32, pheight: u32) -> Self {
        let context = sdl2::init().unwrap();
        let canvas = Self::start_window(&context, name, pwidth * 64, pheight * 32);
        let event_pump = context.event_pump().unwrap();

        Self {
            framebuf: [[false; 32]; 64],
            context,
            canvas,
            event_pump,
            keypad: Keypad::new(),
            pwidth,
            pheight,
        }
    }

    // returns true if pixel was turned off
    pub fn toggle_pixel(&mut self, x: u8, y: u8) -> bool {
        let (x, y) = (x as usize, y as usize);
        self.framebuf[x][y] = !self.framebuf[x][y];

        self.framebuf[x][y]
    }

    pub fn get_canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn get_event_pump(&mut self) -> &mut sdl2::EventPump {
        &mut self.event_pump
    }

    pub fn get_keypad(&mut self) -> &mut Keypad {
        &mut self.keypad
    }

    pub fn handle_input(&mut self) -> bool {
        let mut open = true;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    open = false;
                    break;
                },
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(button_code) = keycode_to_button_code(keycode) {
                        self.keypad.set_pressed(button_code, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(button_code) = keycode_to_button_code(keycode) {
                        self.keypad.set_pressed(button_code, false);
                    }
                }
                _ => {}
            }
        }
        open
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        for (x, col) in self.framebuf.iter().enumerate() {
            for (y, pixel) in col.iter().enumerate() {
                if *pixel {
                    self.canvas
                        .fill_rect(Rect::new(
                            x as i32 * self.pwidth as i32,
                            y as i32 * self.pheight as i32,
                            self.pwidth,
                            self.pheight,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }

    pub fn get_events(&mut self) -> impl Iterator {
        self.event_pump.poll_iter()
    }

    pub fn clear(&mut self) {
        self.framebuf = [[false; 32]; 64];
    }

    fn start_window(context: &sdl2::Sdl, name: &str, width: u32, height: u32) -> Canvas<Window> {
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window(name, width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().software().build().unwrap();
        canvas
    }
}

fn keycode_to_button_code(keycode: Keycode) -> Option<ButtonCode> {
    match keycode {
        Keycode::NUM_1 => Some(ButtonCode::B1),
        Keycode::NUM_2 => Some(ButtonCode::B2),
        Keycode::NUM_3 => Some(ButtonCode::B3),
        Keycode::NUM_4 => Some(ButtonCode::BC),
        Keycode::Q => Some(ButtonCode::B4),
        Keycode::W => Some(ButtonCode::B5),
        Keycode::E => Some(ButtonCode::B6),
        Keycode::R => Some(ButtonCode::BD),
        Keycode::A => Some(ButtonCode::B7),
        Keycode::S => Some(ButtonCode::B8),
        Keycode::D => Some(ButtonCode::B9),
        Keycode::F => Some(ButtonCode::BE),
        Keycode::Z => Some(ButtonCode::BA),
        Keycode::X => Some(ButtonCode::B0),
        Keycode::C => Some(ButtonCode::BB),
        Keycode::V => Some(ButtonCode::BF),
        _ => None
    }
}