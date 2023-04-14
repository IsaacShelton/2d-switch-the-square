use macroquad::prelude::*;

use crate::gamedata::GameData;
use crate::textures::Textures;

pub fn update(_textures: &Textures, gamedata: &mut GameData) {
    if is_mouse_button_down(MouseButton::Left) {
        if !gamedata.mouse_was_down {
            gamedata.is_left = !gamedata.is_left;
            gamedata.mouse_was_down = true;
        }
    } else {
        gamedata.mouse_was_down = false;
    }
}
