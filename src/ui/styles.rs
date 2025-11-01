/// Centralized UI styling constants for consistent look across the app
use eframe::egui;

// ============================================================================
// COLOR PALETTE - Glassmorphism theme
// ============================================================================

/// Button colors (semi-transparent glassmorphism)
pub fn button_fill_normal() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38) 
}

pub fn button_fill_hover() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50) 
}

pub fn button_fill_active() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 64) 
}

pub fn button_border() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30) 
}

pub const BUTTON_SHADOW_ALPHA: u8 = 60;

/// Slider colors
pub fn slider_rail_fill() -> egui::Color32 { 
    egui::Color32::from_rgb(60, 60, 70) // Match dark button color
}

pub fn slider_fill() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(100, 150, 255, 120) 
}

pub const SLIDER_KNOB_RATIO: f32 = 0.25;

// ============================================================================
// SIZE CONSTANTS
// ============================================================================

/// Button sizes for different phases
pub struct ButtonSizes {
    pub large_radius: f32,   // Primary action (Take Picture)
    pub normal_radius: f32,  // Standard buttons (Edit phase)
    pub small_radius: f32,   // Secondary actions (Upload)
    pub spacing: f32,        // Standard spacing between elements
}

impl ButtonSizes {
    pub fn standard() -> Self {
        Self {
            large_radius: 120.0,
            normal_radius: 100.0,
            small_radius: 60.0,
            spacing: 20.0,
        }
    }
}

/// Slider dimensions
pub struct SliderSizes {
    pub width: f32,
    pub spacing_between: f32,  // Space between multiple sliders
}

impl SliderSizes {
    pub fn standard() -> Self {
        Self {
            width: 200.0,
            spacing_between: 40.0,  // Double standard spacing
        }
    }
}

// ============================================================================
// MENU STYLING
// ============================================================================

/// Menu styling configuration
pub struct MenuStyle {
    pub width: f32,
    pub button_width: f32,
    pub button_height: f32,
    pub cancel_button_height: f32,
    pub spacing: f32,
    pub heading_size: f32,
    pub label_size: f32,
}

impl MenuStyle {
    /// Standard developer menu style (large, detailed)
    pub fn developer() -> Self {
        Self {
            width: 933.0,              // 700.0 * 1.33
            button_width: 800.0,       // 600.0 * 1.33
            button_height: 107.0,      // 80.0 * 1.33
            cancel_button_height: 80.0, // 60.0 * 1.33
            spacing: 20.0,
            heading_size: 32.0,
            label_size: 20.0,
        }
    }

    /// Power/shutdown menu style (matches developer menu sizing)
    pub fn power() -> Self {
        Self {
            width: 933.0,              // Match developer menu
            button_width: 800.0,       // Match developer menu
            button_height: 107.0,      // Match developer menu
            cancel_button_height: 80.0, // Match developer menu
            spacing: 20.0,
            heading_size: 32.0,
            label_size: 24.0,
        }
    }

    /// USB export dialog style (smaller, focused)
    pub fn usb_export() -> Self {
        Self {
            width: 450.0,
            button_width: 400.0,
            button_height: 70.0,
            cancel_button_height: 60.0,
            spacing: 15.0,
            heading_size: 28.0,
            label_size: 20.0,
        }
    }
}

/// Button text styling helper
pub fn button_text(text: &str, size: f32) -> egui::RichText {
    egui::RichText::new(text).size(size)
}
