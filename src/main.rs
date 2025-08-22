use nannou::prelude::*;
use nannou::image::{self, DynamicImage};
use std::fs;

// Module importieren
mod image_ops;
mod model;
mod ui;
mod midi;
mod random_sort;

// Verwendete Typen und Funktionen importieren
use model::{Model, SortMode};
use image_ops::{set_sort_mode, sort_and_update_texture, vertical_sort_and_update_texture};
use midi::MidiState;

// MIDI-Status als globale Option (Workaround, da nannou Model nicht an main gibt)
static mut MIDI_STATE: Option<MidiState> = None;

fn main() {
    // MIDI initialisieren
    let midi_state = MidiState::new();
    midi_state.start_listening();
    unsafe {
        MIDI_STATE = Some(midi_state);
    }
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
        random_exclude_mode: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // MIDI-Status abfragen und Model ggf. anpassen
    unsafe {
        let midi_ptr = std::ptr::addr_of!(MIDI_STATE);
        if let Some(midi) = (*midi_ptr).as_ref() {
            // Button-Trigger verarbeiten
            
            // Mode Switch Button
            let mut mode_switch = midi.mode_switch_trigger.lock().unwrap();
            if *mode_switch {
                model.switch_sort_mode(); // Nutzt die vorhandene Methode
                println!("MIDI: Mode gewechselt zu {:?}", model.sort_mode);
                *mode_switch = false; // Trigger zurücksetzen
            }
            
            // Direction Switch Button
            let mut direction_switch = midi.direction_switch_trigger.lock().unwrap();
            if *direction_switch {
                model.switch_direction(); // Nutzt die vorhandene Methode
                let direction_name = if model.vertical_mode { "Vertikal" } else { "Horizontal" };
                println!("MIDI: Direction gewechselt zu {}", direction_name);
                *direction_switch = false; // Trigger zurücksetzen
            }
            
            // Random Toggle Button
            let mut random_toggle = midi.random_toggle_trigger.lock().unwrap();
            if *random_toggle {
                model.toggle_random_exclude();
                println!("MIDI: Random Exclude Mode: {}", model.random_exclude_mode);
                *random_toggle = false; // Trigger zurücksetzen
            }
            
            // Save Button
            let mut save_trigger = midi.save_trigger.lock().unwrap();
            if *save_trigger {
                let saved_file = model.save_current_iteration();
                println!("MIDI SAVE: {}", saved_file);
                *save_trigger = false; // Trigger zurücksetzen
            }
            
            // Threshold-Regler (kontinuierlich)
            let midi_threshold = *midi.threshold.lock().unwrap();
            let current_threshold = if model.vertical_mode {
                model.brightness_value_vertical
            } else {
                model.brightness_value
            };
            
            if current_threshold != midi_threshold {
                if model.vertical_mode {
                    model.brightness_value_vertical = midi_threshold;
                } else {
                    model.brightness_value = midi_threshold;
                }
                model.needs_resort = true;
            }
        }
    }
    
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
