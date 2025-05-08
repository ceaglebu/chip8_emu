use chip8_emu::screen::Screen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut screen: Screen = Screen::new("test", 800, 600);
    screen.toggle_pixel(0, 0);

    let events = screen.get_event_pump();

    'mainloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
        screen.get_canvas().present();
    }
}
