use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct Screen {
    framebuf: [[bool; 32]; 64],
    context: sdl2::Sdl,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
}

impl Screen {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let context = sdl2::init().unwrap();
        let canvas = Self::start_window(&context, name, width, height);
        let event_pump = context.event_pump().unwrap();

        Self {
            framebuf: [[false; 32]; 64],
            context,
            canvas,
            event_pump,
        }
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        self.framebuf[x][y] ^= self.framebuf[x][y];
    }

    pub fn get_canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn get_event_pump(&mut self) -> &mut sdl2::EventPump {
        &mut self.event_pump
    }

    fn start_window(context: &sdl2::Sdl, name: &str, width: u32, height: u32) -> Canvas<Window> {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
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
        let _ = context.event_pump();

        canvas
    }
}
