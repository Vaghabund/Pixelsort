// model.rs
// Enthält die zentrale Model-Struktur und Enums für das Pixel-Sorting-Projekt

use image::{self, RgbaImage};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortMode {
    Brightness,
    Black,
    White,
}

pub struct Model {
    pub img_original: RgbaImage,
    pub img_horizontal: RgbaImage,
    pub img_vertical: RgbaImage,
    pub brightness_value: u8,
    pub brightness_value_vertical: u8,
    pub width: u32,
    pub height: u32,
    pub needs_resort: bool,
    pub vertical_mode: bool,
    pub last_vertical_mode: bool,
    pub sort_mode: SortMode,
    pub random_exclude_mode: bool,
    pub image_counter: usize,
}

impl Model {
    pub fn new() -> Self {
        // Locate assets directory: prefer ./assets, fall back to executable's parent/assets
        // Search for assets directory starting from CWD, then executable dir and its ancestors
        let mut img_path: Option<std::path::PathBuf> = None;

        let mut candidates = Vec::new();
        candidates.push(std::path::PathBuf::from("assets"));
        if let Ok(mut p) = std::env::current_exe() {
            if let Some(mut dir) = p.parent() {
                // Walk up a few levels looking for assets
                for _ in 0..6 {
                    candidates.push(dir.join("assets"));
                    if let Some(parent) = dir.parent() {
                        dir = parent;
                    } else {
                        break;
                    }
                }
            }
        }

        for assets_dir in candidates {
            if let Ok(entries) = fs::read_dir(&assets_dir) {
                img_path = entries
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
                    });

                if img_path.is_some() {
                    break;
                }
            }
        }

        let img_path = img_path.expect("No image found in any 'assets' directory (searched CWD and exe parent tree)");
        
        let original_img = image::open(&img_path).expect("Bild konnte nicht geladen werden").to_rgba8();
        
        // Downscale image to 480x320 resolution
        let img = image::imageops::resize(&original_img, 480, 320, image::imageops::FilterType::Lanczos3);
        let (width, height) = img.dimensions();

        Model {
            img_original: img.clone(),
            img_horizontal: img.clone(),
            img_vertical: img.clone(),
            brightness_value: 60,
            brightness_value_vertical: 60,
            width,
            height,
            needs_resort: true,
            vertical_mode: false,
            last_vertical_mode: false,
            sort_mode: SortMode::Brightness,
            random_exclude_mode: false,
            image_counter: 1,
        }
    }

    pub fn update(&mut self) {
        if self.needs_resort {
            if !self.vertical_mode {
                crate::image_ops::sort_and_update_pixels(self);
            } else {
                crate::image_ops::vertical_sort_and_update_pixels(self);
            }
            self.needs_resort = false;
        }
    }

    pub fn render(&self, frame: &mut [u8]) {
        let img = if self.vertical_mode {
            &self.img_vertical
        } else {
            &self.img_horizontal
        };

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % 480) as u32;
            let y = (i / 480) as u32;
            
            if x < self.width && y < self.height {
                let img_pixel = img.get_pixel(x, y);
                pixel.copy_from_slice(&[img_pixel[0], img_pixel[1], img_pixel[2], 255]);
            } else {
                pixel.copy_from_slice(&[0, 0, 0, 255]);
            }
        }
    }

    pub fn switch_sort_mode(&mut self) {
        // Modi durchschalten: Brightness -> Black -> White -> Brightness
        self.sort_mode = match self.sort_mode {
            SortMode::Brightness => SortMode::Black,
            SortMode::Black => SortMode::White,
            SortMode::White => SortMode::Brightness,
        };
        
        // Bei Moduswechsel auf letzte gespeicherte Version zurücksetzen
        if !self.vertical_mode {
            // Horizontal: Zurück zur letzten Basis für horizontales Sortieren
            self.img_horizontal = self.img_original.clone();
        } else {
            // Vertikal: Zurück zur letzten Basis für vertikales Sortieren  
            self.img_vertical = self.img_horizontal.clone();
        }
        self.needs_resort = true;
    }

    pub fn toggle_random_exclude(&mut self) {
        self.random_exclude_mode = !self.random_exclude_mode;
        self.needs_resort = true;
    }

    pub fn switch_direction(&mut self) {
        // Richtung umschalten (horizontal <-> vertikal)
        self.last_vertical_mode = self.vertical_mode;
        self.vertical_mode = !self.vertical_mode;
        
        // Bei Richtungswechsel auch auf entsprechende Basis zurücksetzen
        if !self.vertical_mode {
            // Wechsel zu horizontal: Basis ist img_original (letzte Enter-Speicherung)
            self.img_horizontal = self.img_original.clone();
        } else {
            // Wechsel zu vertikal: Basis ist img_horizontal (aktueller horizontaler Stand)
            self.img_vertical = self.img_horizontal.clone();
        }
        self.needs_resort = true;
    }

    pub fn save_current_iteration(&mut self) -> String {
        // Ordner 'output' anlegen, falls nicht vorhanden
        let output_dir = Path::new("output");
        if !output_dir.exists() {
            let _ = fs::create_dir(output_dir);
        }
        
        // Bild speichern und Bildnummer erhöhen
        let filename = output_dir.join(format!("Bild{}.png", self.image_counter));
        
        if !self.vertical_mode {
            let _ = self.img_horizontal.save(&filename);
            // Nach horizontalem Sortieren: Bild als neue Basis für beide Modi
            self.img_vertical = self.img_horizontal.clone();
            self.img_original = self.img_horizontal.clone();
        } else {
            let _ = self.img_vertical.save(&filename);
            // Nach vertikalem Sortieren: Bild als neue Basis für beide Modi
            self.img_horizontal = self.img_vertical.clone();
            self.img_original = self.img_vertical.clone();
        }
        
        self.image_counter += 1;
        self.needs_resort = true;
        
        format!("Bild{}.png", self.image_counter - 1)
    }

    pub fn increase_brightness(&mut self) {
        if !self.vertical_mode {
            if self.brightness_value <= 245 {
                self.brightness_value += 10;
            } else {
                self.brightness_value = 255;
            }
        } else {
            if self.brightness_value_vertical <= 245 {
                self.brightness_value_vertical += 10;
            } else {
                self.brightness_value_vertical = 255;
            }
        }
        self.needs_resort = true;
    }

    pub fn decrease_brightness(&mut self) {
        if !self.vertical_mode {
            if self.brightness_value >= 10 {
                self.brightness_value -= 10;
            } else {
                self.brightness_value = 0;
            }
        } else {
            if self.brightness_value_vertical >= 10 {
                self.brightness_value_vertical -= 10;
            } else {
                self.brightness_value_vertical = 0;
            }
        }
        self.needs_resort = true;
    }
}
