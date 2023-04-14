use macroquad::prelude::*;

pub struct GameData {
    pub is_left: bool,
    pub mouse_was_down: bool,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            is_left: true,
            mouse_was_down: false,
        }
    }
}
