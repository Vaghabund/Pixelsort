// image_ops.rs
// Bildverarbeitungsfunktionen für Pixel-Sorting-Projekt

use nannou::image;
use crate::model::SortMode;

// Globaler Modus für Sortierung (Workaround, da nannou keine Model-Referenz in Segmentfunktionen erlaubt)
static mut CURRENT_SORT_MODE: SortMode = SortMode::Brightness;

pub fn set_sort_mode(mode: SortMode) {
    unsafe {
        CURRENT_SORT_MODE = mode;
    }
}

pub fn brightness(px: &image::Rgba<u8>) -> u8 {
    let [r, g, b, _a] = px.0;
    ((r as u16 + g as u16 + b as u16) / 3) as u8
}

pub fn sort_row_segment(img: &mut image::RgbaImage, y: u32, x_start: u32, x_end: u32) {
    let mut segment: Vec<_> = (x_start..x_end)
        .map(|x| *img.get_pixel(x, y))
        .collect();
    let mode = unsafe { CURRENT_SORT_MODE };
    segment.sort_by_key(|px| match mode {
        SortMode::Brightness => brightness(px),
        SortMode::Black => px[0] as u8, // nach Rotanteil sortieren
        SortMode::White => 255 - px[0] as u8, // nach invertiertem Rotanteil sortieren
    });
    for (i, px) in segment.into_iter().enumerate() {
        img.put_pixel(x_start + i as u32, y, px);
    }
}

pub fn sort_column_segment(img: &mut image::RgbaImage, x: u32, y_start: u32, y_end: u32) {
    let mut segment: Vec<_> = (y_start..y_end)
        .map(|y| *img.get_pixel(x, y))
        .collect();
    let mode = unsafe { CURRENT_SORT_MODE };
    segment.sort_by_key(|px| match mode {
        SortMode::Brightness => brightness(px),
        SortMode::Black => px[0] as u8, // nach Rotanteil sortieren
        SortMode::White => 255 - px[0] as u8, // nach invertiertem Rotanteil sortieren
    });
    for (i, px) in segment.into_iter().enumerate() {
        img.put_pixel(x, y_start + i as u32, px);
    }
}

pub fn get_next_segment_row_bright(img: &image::RgbaImage, mut x: u32, y: u32, width: u32, brightness_value: u8) -> (u32, u32) {
    while x < width && brightness(img.get_pixel(x, y)) < brightness_value { x += 1; }
    let start = x;
    while x < width && brightness(img.get_pixel(x, y)) > brightness_value { x += 1; }
    (start, x)
}

pub fn get_next_segment_column_bright(img: &image::RgbaImage, x: u32, mut y: u32, height: u32, brightness_value: u8) -> (u32, u32) {
    while y < height && brightness(img.get_pixel(x, y)) < brightness_value { y += 1; }
    let start = y;
    while y < height && brightness(img.get_pixel(x, y)) > brightness_value { y += 1; }
    (start, y)
}

// Hauptfunktionen für horizontales und vertikales Sortieren
use nannou::image::DynamicImage;
use crate::model::Model;

pub fn sort_and_update_texture(app: &nannou::App, model: &mut Model) {
    let mut img = model.img_original.clone();
    let (width, height) = (model.width, model.height);
    // Spalten sortieren
    for x in 0..width {
        let mut y = 0;
        while y < height {
            let (start, end) = get_next_segment_column_bright(&img, x, y, height, model.brightness_value);
            if start >= end || start >= height {
                break;
            }
            sort_column_segment(&mut img, x, start, end);
            y = end + 1;
        }
    }
    // Zeilen sortieren
    for y in 0..height {
        let mut x = 0;
        while x < width {
            let (start, end) = get_next_segment_row_bright(&img, x, y, width, model.brightness_value);
            if start >= end || start >= width {
                break;
            }
            sort_row_segment(&mut img, y, start, end);
            x = end + 1;
        }
    }
    model.img_horizontal = img.clone();
    model.texture = nannou::wgpu::Texture::from_image(app, &DynamicImage::ImageRgba8(img));
}

pub fn vertical_sort_and_update_texture(app: &nannou::App, model: &mut Model) {
    let mut img = model.img_horizontal.clone();
    let (width, height) = (model.width, model.height);
    for x in 0..width {
        let mut y = 0;
        while y < height {
            let (start, end) = get_next_segment_column_bright(&img, x, y, height, model.brightness_value_vertical);
            if start >= end || start >= height {
                break;
            }
            sort_column_segment(&mut img, x, start, end);
            y = end + 1;
        }
    }
    model.img_vertical = img.clone();
    model.texture = nannou::wgpu::Texture::from_image(app, &DynamicImage::ImageRgba8(img));
}
