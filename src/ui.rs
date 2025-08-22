// ui.rs
// UI-bezogene Hilfsfunktionen für Pixel-Sorting-Projekt

use nannou::prelude::*;
use crate::model::{Model, SortMode};

pub fn draw_info(draw: &Draw, model: &Model) {
    let (richtung, value) = if model.vertical_mode {
        ("Vertikal", model.brightness_value_vertical)
    } else {
        ("Horizontal", model.brightness_value)
    };
    let mode_str = match model.sort_mode {
        SortMode::Brightness => "Mode: Brightness",
        SortMode::Black => "Mode: Black",
        SortMode::White => "Mode: White",
    };
    let random_str = if model.random_exclude_mode { "Random: ON" } else { "Random: OFF" };
    let info = format!(
        "{}\n{}\nValue: {}\n{}\n{}",
        model.image_counter,
        richtung,
        value,
        mode_str,
        random_str
    );
    let x = -((model.width as f32) / 2.0) + 20.0;
    let y = -((model.height as f32) / 2.0) + 40.0;
    draw.text(&info)
        .x_y(x, y)
        .color(WHITE)
        .font_size(28)
        .left_justify();
}
