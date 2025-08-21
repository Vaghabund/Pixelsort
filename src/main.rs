use nannou::prelude::*;
use nannou::image::{self, DynamicImage};
use std::fs;

// Module importieren
mod image_ops;
mod model;
mod ui;
mod midi;

// Verwendete Typen und Funktionen importieren
use model::{Model, SortMode};
use image_ops::{set_sort_mode, sort_and_update_texture, vertical_sort_and_update_texture};

fn main() {
    nannou::app(model).update(update).view(view).event(event).run();
}

// BLACK_VALUE und WHITE_VALUE entfernt, da nicht genutzt

fn model(app: &App) -> Model {
    let image_counter = 1;
    let assets = app.assets_path().unwrap();
    let img_path = fs::read_dir(&assets)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            if let Some(ext) = path.extension() {
                matches!(
                    ext.to_str().unwrap_or("").to_lowercase().as_str(),
                    "png" | "jpg" | "jpeg" | "bmp" | "gif"
                )
            } else {
                false
            }
        })
        .expect("Kein Bild im assets-Ordner gefunden!");
    let img = image::open(&img_path).expect("Bild konnte nicht geladen werden").to_rgba8();
    let (width, height) = img.dimensions();

    app.new_window()
        .size(width, height)
        .view(view)
        .build()
        .unwrap();

    let img_horizontal = img.clone();
    let img_vertical = img.clone();
    let texture = wgpu::Texture::from_image(app, &DynamicImage::ImageRgba8(img_horizontal.clone()));

    Model {
        texture,
        img_original: img.clone(),
        img_horizontal,
        img_vertical,
        brightness_value: 60,
        brightness_value_vertical: 60,
        width,
        height,
        needs_resort: true,
        vertical_mode: false,
        last_vertical_mode: false,
        image_counter,
        sort_mode: SortMode::Brightness,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.needs_resort {
        set_sort_mode(model.sort_mode); // Modus an image_ops weitergeben
        if !model.vertical_mode {
            sort_and_update_texture(app, model);
        } else {
            vertical_sort_and_update_texture(app, model);
        }
        model.needs_resort = false;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.texture(&model.texture);
    ui::draw_info(&draw, model); // UI-Modul für Textanzeige nutzen
    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: nannou::Event) {
    if let nannou::Event::WindowEvent { simple, .. } = event {
        if let Some(window_event) = simple {
            match window_event {
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Up) => {
                    model.increase_brightness();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Down) => {
                    model.decrease_brightness();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Return) => {
                    let saved_file = model.save_current_iteration();
                    println!("Gespeichert: {}", saved_file);
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::N) => {
                    model.switch_direction();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Delete) => {
                    // In den letzten Modus zurückschalten
                    let tmp = model.vertical_mode;
                    model.vertical_mode = model.last_vertical_mode;
                    model.last_vertical_mode = tmp;
                    model.needs_resort = true;
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::M) => {
                    model.switch_sort_mode();
                }
                _ => {}
            }
        }
    }
}
