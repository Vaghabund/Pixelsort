use crate::PixelSorterApp;
use crate::ui::state::Phase;
use crate::ui::widgets::{vertical_slider, circular_button_styled};
use crate::pixel_sorter::SortingAlgorithm;
use eframe::egui;
use std::time::Instant;

impl PixelSorterApp {
    pub fn render_button_overlay(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context, screen_rect: egui::Rect) {
        // No background panel needed - buttons float directly
        match self.current_phase {
            Phase::Input => self.render_input_buttons_circular(ctx, screen_rect),
            Phase::Edit => self.render_edit_buttons_circular(ctx, screen_rect),
            Phase::Crop => self.render_crop_buttons_circular(ctx, screen_rect),
        }
    }

    // ============================================================================
    // PHASE 1: INPUT - Two circles in right bottom corner
    // ============================================================================
    fn render_input_buttons_circular(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        const LARGE_BUTTON_RADIUS: f32 = 120.0;  // Take Picture (even larger for primary action)
        const SMALL_BUTTON_RADIUS: f32 = 60.0;   // Upload Image
        const SPACING: f32 = 20.0;

        // Calculate positions - right bottom corner alignment
        let large_center = egui::pos2(
            screen_rect.max.x - LARGE_BUTTON_RADIUS - SPACING,
            screen_rect.max.y - LARGE_BUTTON_RADIUS - SPACING,
        );

        let small_center = egui::pos2(
            screen_rect.max.x - SMALL_BUTTON_RADIUS - SPACING,
            large_center.y - LARGE_BUTTON_RADIUS - SMALL_BUTTON_RADIUS - SPACING,
        );

        // Draw buttons using Area widgets
        egui::Area::new("take_picture_btn")
            .fixed_pos(large_center - egui::vec2(LARGE_BUTTON_RADIUS, LARGE_BUTTON_RADIUS))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if self.circular_button(ui, LARGE_BUTTON_RADIUS, "", "take_pic") {
                    self.capture_and_sort(ctx);
                }
            });

        egui::Area::new("upload_btn")
            .fixed_pos(small_center - egui::vec2(SMALL_BUTTON_RADIUS, SMALL_BUTTON_RADIUS))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if self.circular_button(ui, SMALL_BUTTON_RADIUS, "Upload", "upload_img") {
                    self.load_image(ctx);
                }
            });
    }

    // ============================================================================
    // PHASE 2: EDIT - Horizontal sliders on right, buttons on left in two rows
    // ============================================================================
    fn render_edit_buttons_circular(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        const BUTTON_RADIUS: f32 = 100.0;  // Even larger buttons for better touch targets
        const SLIDER_WIDTH: f32 = 80.0;    // Wider sliders with bigger handles
        const SLIDER_HEIGHT: f32 = 300.0;
        const SPACING: f32 = 20.0;

        // Right side: Horizontal sliders (side by side)
        self.render_vertical_sliders(ctx, screen_rect, SLIDER_WIDTH, SLIDER_HEIGHT, SPACING);

        // Left side: Buttons in two rows, aligned to left border
        // Row 1: Algorithm and Sort Mode buttons (top row) - 2 buttons
        let row1_y = screen_rect.max.y - BUTTON_RADIUS * 4.0 - SPACING * 3.0;

        // Algorithm button (left)
        egui::Area::new("algo_btn")
            .fixed_pos(egui::pos2(SPACING, row1_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if self.circular_button(ui, BUTTON_RADIUS, self.current_algorithm.name(), "algo") {
                    self.cycle_algorithm();
                    self.apply_pixel_sort(ctx);
                }
            });

        // Sort Mode button (right of Algorithm)
        egui::Area::new("mode_btn")
            .fixed_pos(egui::pos2(SPACING + BUTTON_RADIUS * 2.0 + SPACING, row1_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if self.circular_button(ui, BUTTON_RADIUS, self.sorting_params.sort_mode.name(), "mode") {
                    self.sorting_params.sort_mode = self.sorting_params.sort_mode.next();
                    self.apply_pixel_sort(ctx);
                }
            });

        // Row 2: Action buttons (bottom row) - Crop, Save, New - 3 buttons
        let row2_y = screen_rect.max.y - BUTTON_RADIUS * 2.0 - SPACING;

        // Crop button (left)
        egui::Area::new("crop_btn")
            .fixed_pos(egui::pos2(SPACING, row2_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if circular_button_styled(ui, BUTTON_RADIUS, "Crop",
                    egui::Color32::from_rgb(60, 60, 70)) {
                    self.current_phase = Phase::Crop;
                    self.crop_rect = None;
                }
            });

        // Save button (middle)
        egui::Area::new("save_btn")
            .fixed_pos(egui::pos2(SPACING + BUTTON_RADIUS * 2.0 + SPACING, row2_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if circular_button_styled(ui, BUTTON_RADIUS, "Save",
                    egui::Color32::from_rgb(60, 60, 70)) {
                    self.save_and_continue_iteration(ctx);
                }
            });

        // New Image button (right)
        egui::Area::new("new_btn")
            .fixed_pos(egui::pos2(SPACING + (BUTTON_RADIUS * 2.0 + SPACING) * 2.0, row2_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if circular_button_styled(ui, BUTTON_RADIUS, "New",
                    egui::Color32::from_rgb(60, 60, 70)) {
                    self.start_new_photo_session();
                }
            });

        // Optional: Export to USB button if USB present (bottom left corner)
        if self.usb_present() {
            let export_y = screen_rect.max.y - BUTTON_RADIUS - SPACING / 2.0;
            egui::Area::new("export_btn")
                .fixed_pos(egui::pos2(SPACING, export_y))
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    if circular_button_styled(ui, BUTTON_RADIUS * 0.7, "USB",
                        egui::Color32::from_rgba_unmultiplied(40, 80, 40, 180)) {
                        match self.copy_to_usb() {
                            Ok(()) => {
                                self.export_message = Some("✓ Exported to USB!".to_string());
                                self.export_message_time = Some(Instant::now());
                            }
                            Err(e) => {
                                self.export_message = Some(format!("✗ Export failed: {}", e));
                                self.export_message_time = Some(Instant::now());
                            }
                        }
                    }
                });
        }
    }

    // ============================================================================
    // PHASE 3: CROP - Vertical sliders on right, Cancel/Apply on left
    // ============================================================================
    fn render_crop_buttons_circular(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        const BUTTON_RADIUS: f32 = 100.0;  // Even larger buttons for better touch targets
        const SPACING: f32 = 20.0;

        // Left side: Two buttons stacked vertically
        let left_x = SPACING + BUTTON_RADIUS;
        let button_vertical_spacing = SPACING * 2.0;

        // Center buttons vertically
        let total_height = BUTTON_RADIUS * 4.0 + button_vertical_spacing;
        let start_y = (screen_rect.height() - total_height) / 2.0 + screen_rect.min.y;

        // Cancel button (top)
        egui::Area::new("cancel_crop_btn")
            .fixed_pos(egui::pos2(left_x, start_y + BUTTON_RADIUS) - egui::vec2(BUTTON_RADIUS, BUTTON_RADIUS))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if circular_button_styled(ui, BUTTON_RADIUS, "Cancel",
                    egui::Color32::from_rgba_unmultiplied(80, 40, 40, 180)) {
                    self.current_phase = Phase::Edit;
                    self.crop_rect = None;
                }
            });

        // Apply Crop button (bottom)
        egui::Area::new("apply_crop_btn")
            .fixed_pos(egui::pos2(
                left_x,
                start_y + BUTTON_RADIUS * 3.0 + button_vertical_spacing
            ) - egui::vec2(BUTTON_RADIUS, BUTTON_RADIUS))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                if circular_button_styled(ui, BUTTON_RADIUS, "Apply",
                    egui::Color32::from_rgba_unmultiplied(40, 80, 40, 180)) {
                    self.apply_crop_and_sort(ctx);
                }
            });
    }

    // ============================================================================
    // VERTICAL SLIDERS (for Edit and Crop phases) - Placed horizontally
    // ============================================================================
    fn render_vertical_sliders(&mut self, ctx: &egui::Context, screen_rect: egui::Rect,
                                slider_width: f32, _slider_height: f32, spacing: f32) {
        // Place sliders side by side on the right edge with more space between them
        let slider_spacing = spacing * 3.0;  // Triple the spacing between sliders

        // More padding at top and bottom to prevent handle cutoff
        let knob_radius = slider_width * 0.8; // Same calculation as in vertical_slider (updated to 0.8)
        let top_padding = spacing * 3.0 + knob_radius; // Extra space for top handle
        let bottom_padding = spacing * 5.0; // Extra space for label and bottom handle

        // Stretch sliders to fill screen height (with padding)
        let full_slider_height = screen_rect.height() - top_padding - bottom_padding;

        // Start from right edge, moving left
        let slider2_x = screen_rect.max.x - slider_width - spacing;
        let slider1_x = slider2_x - slider_width - slider_spacing;

        // Start from top with padding
        let start_y = screen_rect.min.y + top_padding;

        // Threshold slider (left one)
        let mut threshold = self.sorting_params.threshold;
        let threshold_changed = egui::Area::new("threshold_slider")
            .fixed_pos(egui::pos2(slider1_x, start_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    vertical_slider(ui, &mut threshold,
                        0.0..=125.0, slider_width, full_slider_height, "Threshold")
                }).inner
            }).inner;

        if threshold_changed {
            self.sorting_params.threshold = threshold;
            self.apply_pixel_sort(ctx);
        }

        // Hue slider (right one)
        let mut color_tint = self.sorting_params.color_tint;
        let hue_changed = egui::Area::new("hue_slider")
            .fixed_pos(egui::pos2(slider2_x, start_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    vertical_slider(ui, &mut color_tint,
                        0.0..=360.0, slider_width, full_slider_height, "Hue")
                }).inner
            }).inner;

        if hue_changed {
            if !self.tint_enabled && color_tint > 0.0 {
                self.tint_enabled = true;
            }
            self.sorting_params.color_tint = color_tint;
            self.apply_pixel_sort(ctx);
        }
    }

    // ============================================================================
    // HELPER METHODS
    // ============================================================================

    /// Basic circular button with default styling
    fn circular_button(&self, ui: &mut egui::Ui, radius: f32, text: &str, _id: &str) -> bool {
        // Glassmorphism: rgba(255, 255, 255, 0.15) = white with 15% opacity
        circular_button_styled(ui, radius, text, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38))
    }

    pub fn cycle_algorithm(&mut self) {
        let all = SortingAlgorithm::all();
        let idx = all.iter().position(|&a| a == self.current_algorithm).unwrap_or(0);
        let next_idx = (idx + 1) % all.len();
        self.current_algorithm = all[next_idx];
    }
}
