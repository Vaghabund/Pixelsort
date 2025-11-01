/// Centralized UI styling constants for consistent look across the app
/// 
/// EDIT THIS FILE TO CHANGE: Colors, sizes, visual appearance
use eframe::egui;

// ============================================================================
// 🎨 QUICK EDIT: COLOR PALETTE
// Change these values to customize the app's appearance
// ============================================================================

// Button colors (RGBA: Red, Green, Blue, Alpha 0-255)
const BUTTON_NORMAL_ALPHA: u8 = 38;   // Transparency when idle
const BUTTON_HOVER_ALPHA: u8 = 50;    // Transparency when hovering
const BUTTON_ACTIVE_ALPHA: u8 = 64;   // Transparency when pressed
const BUTTON_BORDER_ALPHA: u8 = 30;   // Border transparency
const BUTTON_SHADOW_ALPHA_VAL: u8 = 60; // Shadow darkness

// Slider colors (RGB: Red, Green, Blue)
const SLIDER_RAIL_R: u8 = 60;         // Rail background red
const SLIDER_RAIL_G: u8 = 60;         // Rail background green
const SLIDER_RAIL_B: u8 = 70;         // Rail background blue

const SLIDER_FILL_R: u8 = 100;        // Fill bar red
const SLIDER_FILL_G: u8 = 150;        // Fill bar green
const SLIDER_FILL_B: u8 = 255;        // Fill bar blue
const SLIDER_FILL_ALPHA: u8 = 120;    // Fill bar transparency

const SLIDER_KNOB_RATIO_VAL: f32 = 0.25; // Knob size relative to slider width (0.25 = 25%)

// ============================================================================
// 🔢 QUICK EDIT: SIZE CONSTANTS
// Change these values to adjust button/slider sizes
// ============================================================================

const BUTTON_LARGE_RADIUS: f32 = 120.0;   // Primary action buttons (Take Picture)
const BUTTON_NORMAL_RADIUS: f32 = 100.0;  // Standard buttons (Edit phase)
const BUTTON_SMALL_RADIUS: f32 = 60.0;    // Secondary buttons (Upload)
const BUTTON_SPACING: f32 = 20.0;         // Space between buttons

const SLIDER_WIDTH: f32 = 200.0;          // Width of slider tracks

// ============================================================================
// Internal functions (uses constants above)
// ============================================================================

/// Button colors (semi-transparent glassmorphism)
pub fn button_fill_normal() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, BUTTON_NORMAL_ALPHA) 
}

pub fn button_fill_hover() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, BUTTON_HOVER_ALPHA) 
}

pub fn button_fill_active() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, BUTTON_ACTIVE_ALPHA) 
}

pub fn button_border() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(255, 255, 255, BUTTON_BORDER_ALPHA) 
}

pub const BUTTON_SHADOW_ALPHA: u8 = BUTTON_SHADOW_ALPHA_VAL;

/// Slider colors
pub fn slider_rail_fill() -> egui::Color32 { 
    egui::Color32::from_rgb(SLIDER_RAIL_R, SLIDER_RAIL_G, SLIDER_RAIL_B)
}

pub fn slider_fill() -> egui::Color32 { 
    egui::Color32::from_rgba_unmultiplied(SLIDER_FILL_R, SLIDER_FILL_G, SLIDER_FILL_B, SLIDER_FILL_ALPHA) 
}

pub const SLIDER_KNOB_RATIO: f32 = SLIDER_KNOB_RATIO_VAL;

// ============================================================================
// Button/Slider size configuration structs
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
            large_radius: BUTTON_LARGE_RADIUS,
            normal_radius: BUTTON_NORMAL_RADIUS,
            small_radius: BUTTON_SMALL_RADIUS,
            spacing: BUTTON_SPACING,
        }
    }
}

/// Slider dimensions
pub struct SliderSizes {
    pub width: f32,
}

impl SliderSizes {
    pub fn standard() -> Self {
        Self {
            width: SLIDER_WIDTH,
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
