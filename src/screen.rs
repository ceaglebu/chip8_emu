use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct Screen {
    framebuf: [[bool; 32]; 64],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            framebuf: [[false; 32]; 64],
        }
    }
}

impl Screen {
    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        self.framebuf[x][y] ^= self.framebuf[x][y];
    }

    pub fn start_window(&mut self, name: &str, width: u32, height: u32) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video();
        let window = video_subsystem
            .unwrap()
            .window(name, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.clear();
        let _ = sdl_context.event_pump();

        loop {
            canvas.present();
        }
    }
}
