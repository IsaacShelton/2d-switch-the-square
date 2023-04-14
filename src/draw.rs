use crate::gamedata::GameData;
use crate::textures::Textures;
use macroquad::prelude::*;

pub fn draw(_textures: &Textures, gamedata: &GameData) {
    draw_text(
        if gamedata.is_left { "Red" } else { "Blue" },
        10.0,
        60.0,
        40.0,
        if gamedata.is_left { RED } else { BLUE },
    );

    let center = vec2(screen_width(), screen_height()) * 0.5;

    let size = 128.0;
    let red_size = if gamedata.is_left { size + 40.0 } else { size };
    let blue_size = if !gamedata.is_left { size + 40.0 } else { size };

    draw_rectangle(
        center.x - red_size / 2.0 - 256.0,
        center.y - red_size / 2.0,
        red_size,
        red_size,
        RED,
    );

    draw_rectangle(
        center.x - blue_size / 2.0 + 256.0,
        center.y - blue_size / 2.0,
        blue_size,
        blue_size,
        BLUE,
    );
}
