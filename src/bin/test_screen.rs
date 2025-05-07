use chip8_emu::screen::Screen;

fn main() {
    let mut screen: Screen = Default::default();
    screen.toggle_pixel(0, 0);
    screen.start_window("test", 800, 600);
}
