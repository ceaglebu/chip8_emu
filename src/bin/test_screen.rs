use chip8_emu::screen::Screen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut screen: Screen = Screen::new("test", 18, 18);

    screen.toggle_pixel(0, 0);
    screen.toggle_pixel(2, 0);
    screen.toggle_pixel(0, 1);
    screen.toggle_pixel(0, 2);
    screen.toggle_pixel(0, 4);

    screen.toggle_pixel(63, 31);

    'mainloop: loop {
        for event in screen.get_event_pump().poll_iter() {
            // when abstracting to embedded, need to abstract event iterator and event types
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
        screen.update();
    }
}
