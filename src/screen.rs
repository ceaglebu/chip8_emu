use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct Screen {
    framebuf: [[bool; 32]; 64],
    context: sdl2::Sdl,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
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
