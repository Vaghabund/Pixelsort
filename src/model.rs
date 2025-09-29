// model.rs
// Enthält die zentrale Model-Struktur und Enums für das Pixel-Sorting-Projekt

use image;
use std::fs;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortMode {
    Brightness,
    Black,
    White,
}

#[derive(Clone)]
pub struct Model {
    pub image_counter: usize,
    pub img_original: image::RgbaImage,
    pub img_horizontal: image::RgbaImage,
    pub img_vertical: image::RgbaImage,
    pub brightness_value: u8,
    pub brightness_value_vertical: u8,
    pub width: u32,
    pub height: u32,
    pub needs_resort: bool,
    pub vertical_mode: bool,
    pub last_vertical_mode: bool,
    pub sort_mode: SortMode,
    pub random_exclude_mode: bool,
}

impl Model {
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
        use std::fs;
        use std::path::Path;
        
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

    pub fn new(width: u32, height: u32) -> Self {
        // Find first image in assets directory
        let img_path = fs::read_dir("assets")
            .unwrap_or_else(|_| fs::read_dir(".").unwrap())
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
            .expect("No image found in assets or current directory!");

        let img = image::open(&img_path)
            .expect("Could not load image")
            .to_rgba8();

        // Resize to fit the window
        let img = image::imageops::resize(&img, width, height, image::imageops::FilterType::Lanczos3);
        let (img_width, img_height) = img.dimensions();

        let img_horizontal = img.clone();
        let img_vertical = img.clone();

        Model {
            image_counter: 1,
            img_original: img,
            img_horizontal,
            img_vertical,
            brightness_value: 60,
            brightness_value_vertical: 60,
            width: img_width,
            height: img_height,
            needs_resort: true,
            vertical_mode: false,
            last_vertical_mode: false,
            sort_mode: SortMode::Brightness,
            random_exclude_mode: false,
        }
    }

    pub fn update(&mut self) {
        if self.needs_resort {
            crate::image_ops::set_sort_mode(self.sort_mode);
            if !self.vertical_mode {
                crate::image_ops::sort_and_update_image(self);
            } else {
                crate::image_ops::vertical_sort_and_update_image(self);
            }
            self.needs_resort = false;
        }
    }

    pub fn render(&self, frame: &mut [u8]) {
        let img = if self.vertical_mode { &self.img_vertical } else { &self.img_horizontal };
        
        // Pi5 optimization: Use parallel chunked processing for NEON vectorization
        use rayon::prelude::*;
        
        let width = self.width as usize;
        let height = self.height as usize;
        
        // Process frame in parallel chunks optimized for Pi5's 4 cores
        frame.par_chunks_exact_mut(4).enumerate().for_each(|(i, pixel)| {
            let x = i % width;
            let y = i / width;
            
            if x < width && y < height {
                let img_pixel = img.get_pixel(x as u32, y as u32);
                // Optimized pixel copy - Pi5 NEON can vectorize this
                pixel[0] = img_pixel[0]; // R
                pixel[1] = img_pixel[1]; // G  
                pixel[2] = img_pixel[2]; // B
                pixel[3] = 255;          // A
            } else {
                // Fast clear for out-of-bounds pixels
                pixel.copy_from_slice(&[0, 0, 0, 255]);
            }
        });
    }
}
