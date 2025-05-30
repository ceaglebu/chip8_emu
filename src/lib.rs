pub mod screen;
pub mod cpu;
pub mod memory;
pub mod emu;

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
