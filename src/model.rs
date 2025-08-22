// model.rs
// Enthält die zentrale Model-Struktur und Enums für das Pixel-Sorting-Projekt

use nannou::image;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortMode {
    Brightness,
    Black,
    White,
}

#[derive(Clone)]
pub struct Model {
    pub image_counter: usize,
    pub texture: nannou::wgpu::Texture,
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
    pub random_exclude_mode: bool, // Neuer Toggle für Random-Exclude
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
}
