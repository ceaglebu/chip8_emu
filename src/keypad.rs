pub struct Keypad {
    pressed: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            pressed: [false; 16]
        }
    }

    pub fn set_pressed(&mut self, button_code: ButtonCode, pressed: bool) {
        self.pressed[button_code as usize] = pressed;
    }

    pub fn is_pressed_code(&self, button_code: ButtonCode) -> bool {
        self.pressed[button_code as usize]
    }

    pub fn is_pressed_idx(&self, idx: u8) -> bool {
        self.pressed[idx as usize]
    }
}
#[derive(Debug)]
pub enum ButtonCode {
    B1 = 0,
    B2 = 1,
    B3 = 2,
    BC = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    BD = 7,
    B7 = 8,
    B8 = 9,
    B9 = 10,
    BE = 11,
    BA = 12,
    B0 = 13,
    BB = 14,
    BF = 15,
}