/// Centralized UI styling constants for consistent look across the app
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
