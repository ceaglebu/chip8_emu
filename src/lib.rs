pub mod cpu;
pub mod emu;
pub mod keypad;
pub mod memory;
pub mod screen;

#[cfg(test)]
mod tests {
    use super::*;

    use screen::Screen;

    #[test]
    fn screen() {
        // let mut screen: Screen = Default::default();

        // screen.toggle_pixel(0, 0);
    }
}
