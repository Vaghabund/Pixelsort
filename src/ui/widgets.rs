use eframe::egui;

/// Ratio of knob diameter relative to the slider `width` parameter.
///
/// The widget creates a rail by shrinking the rect horizontally by `width * 0.25` on
/// each side (rail_width = width * 0.5). The knob diameter is equal to the rail width,
/// so knob radius = (rail_width / 2) = width * 0.25. Expose the factor here so it's
/// easy to tune in one place.
pub const SLIDER_KNOB_RATIO: f32 = 0.25;

/// Helper: given the slider `width` parameter, return the knob radius used by the
/// vertical slider implementation so callers can reserve matching padding.
pub fn vertical_slider_knob_radius(slider_width: f32) -> f32 {
    slider_width * SLIDER_KNOB_RATIO
}

// --- UI style constants inspired by the provided CSS ---
// Colors use `from_rgba_unmultiplied(r,g,b,a)` where `a` is 0-255.
pub fn button_fill_normal() -> egui::Color32 { egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38) }
pub fn button_fill_hover() -> egui::Color32 { egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50) }
pub fn button_fill_active() -> egui::Color32 { egui::Color32::from_rgba_unmultiplied(255, 255, 255, 64) }
pub fn button_border() -> egui::Color32 { egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30) }
pub const BUTTON_SHADOW_ALPHA: u8 = 60; // used with from_black_alpha

pub fn slider_rail_fill() -> egui::Color32 { egui::Color32::from_rgba_unmultiplied(255, 255, 255, 26) }
pub fn slider_fill() -> egui::Color32 { egui::Color32::from_rgba_unmultiplied(100, 150, 255, 120) }


/// Vertical slider widget with custom touch-friendly styling
pub fn vertical_slider(
    ui: &mut egui::Ui,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    width: f32,
    height: f32,
    label: &str
) -> bool {
    let desired_size = egui::vec2(width, height);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let mut changed = false;

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // CSS: height: 60px, border-radius: 30px (fully rounded)
        // Background rail with glassmorphism: rgba(255, 255, 255, 0.1)
        let rail_rect = rect.shrink2(egui::vec2(width * 0.25, 0.0));
        painter.rect(
            rail_rect,
            rail_rect.width() / 2.0,  // Fully rounded
            slider_rail_fill(),
            egui::Stroke::NONE,
        );

    // Calculate normalized position (inverted for vertical)
    let min = *range.start();
    let max = *range.end();
    let normalized = (*value - min) / (max - min);

        // Compute knob geometry so both dragging and fill calculations use the same values
        let knob_radius = width * SLIDER_KNOB_RATIO;
        let knob_diameter = knob_radius * 2.0;
        let travel = rect.height() - knob_diameter; // how far the knob center can move

        // Handle dragging: map mouse position to normalized value while keeping knob inside rail
        if response.dragged() || response.clicked() {
            if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
                // Map mouse position into normalized value, accounting for knob_radius offset.
                // top = rect.top() + knob_radius => normalized = 1.0
                // bottom = rect.top() + knob_radius + travel => normalized = 0.0
                let pos_in_travel = (mouse_pos.y - (rect.top() + knob_radius)).clamp(0.0, travel);
                let new_normalized = 1.0 - (pos_in_travel / travel).clamp(0.0, 1.0);

                *value = min + new_normalized * (max - min);
                changed = true;
                response.mark_changed();
            }
        }

        // Compute knob center such that the knob sits fully inside the rail at both ends.
        let knob_y = rect.top() + knob_radius + travel * (1.0 - normalized);
        let knob_center = egui::pos2(rect.center().x, knob_y);

        // Filled portion (from bottom up) - subtle blue fill
        // Make the fill reach the top edge of the knob (so it visually meets the knob)
        let filled_top = (knob_center.y - knob_radius).max(rail_rect.min.y);
        let filled_rect = egui::Rect::from_min_max(
            egui::pos2(rail_rect.min.x, filled_top),
            rail_rect.max,
        );
        // Only draw if there's visible height
        if filled_rect.height() > 0.0 {
            painter.rect(
                filled_rect,
                // Keep corner radius equal to rail's radius for consistent rounding
                rail_rect.width() / 2.0,
                slider_fill(),
                egui::Stroke::NONE,
            );
        }

        // CSS: box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3)
        painter.circle(
            knob_center + egui::vec2(0.0, 2.0),
            knob_radius,
            egui::Color32::from_black_alpha(BUTTON_SHADOW_ALPHA),
            egui::Stroke::NONE,
        );

        // CSS: background: rgba(255, 255, 255, 0.9)
        // Border thickness scales with knob size (keeps visual proportions)
        let stroke_width = (knob_radius * 0.13).max(1.0);
        painter.circle(
            knob_center,
            knob_radius,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 230), // 0.9 opacity
            egui::Stroke::new(stroke_width, button_border()),
        );

        // Show value bubble when dragging (on top layer to avoid clipping)
        if response.dragged() {
            let text = format!("{:.0}", value);
            let font_id = egui::FontId::proportional(18.0);

            // Use a separate layer for the bubble to ensure it's on top
            let layer_id = egui::LayerId::new(egui::Order::Tooltip, ui.id().with("value_bubble"));
            let layer_painter = ui.ctx().layer_painter(layer_id);

            let galley = layer_painter.layout_no_wrap(text, font_id, egui::Color32::WHITE);

            let bubble_size = galley.size() + egui::vec2(20.0, 12.0);
            let bubble_pos = egui::pos2(rect.left() - bubble_size.x - 12.0, knob_y - bubble_size.y / 2.0);
            let bubble_rect = egui::Rect::from_min_size(bubble_pos, bubble_size);

            // Glassmorphism bubble
            layer_painter.rect(
                bubble_rect,
                6.0,
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 230),
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50)),
            );

            let text_pos = bubble_rect.center() - galley.size() / 2.0;
            layer_painter.galley(text_pos, galley);
        }

        // Label below slider - increased font size
        let label_font = egui::FontId::proportional(18.0);  // Increased from 14 to 18
        let label_galley = painter.layout_no_wrap(label.to_string(), label_font, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 204));  // 0.8 opacity
        let label_pos = egui::pos2(
            rect.center().x - label_galley.size().x / 2.0,
            rect.bottom() + 40.0,
        );

        // Label background for readability
        let label_bg_rect = egui::Rect::from_min_size(
            label_pos - egui::vec2(4.0, 2.0),
            label_galley.size() + egui::vec2(8.0, 4.0),
        );
        painter.rect(
            label_bg_rect,
            3.0,
            egui::Color32::from_black_alpha(180),
            egui::Stroke::NONE,
        );
        painter.galley(label_pos, label_galley);
    }

    changed
}

/// Render a circular button with custom fill color
pub fn circular_button_styled(ui: &mut egui::Ui, radius: f32, text: &str, base_fill: egui::Color32) -> bool {
    let size = egui::vec2(radius * 2.0, radius * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let center = rect.center();

        // Apply scale transform on press (CSS: transform: scale(0.95))
        let scale = if response.is_pointer_button_down_on() { 0.95 } else { 1.0 };
        let scaled_radius = radius * scale;

        // Determine colors based on interaction state (CSS glassmorphism)
        let fill_color = if response.is_pointer_button_down_on() {
            // Active/pressed state
            button_fill_active()
        } else if response.hovered() {
            // Hover state
            button_fill_hover()
        } else {
            // Normal state
            base_fill
        };

        // Draw shadow for depth (CSS: box-shadow effect)
        painter.circle(
            center + egui::vec2(2.0, 4.0),
            scaled_radius,
            egui::Color32::from_black_alpha(60),
            egui::Stroke::NONE,
        );

        // Draw main circle with subtle border
        painter.circle(
            center,
            scaled_radius,
            fill_color,
            egui::Stroke::new(1.0, button_border()),
        );

        // Draw text in center
        let font_id = egui::FontId::proportional(radius / 3.0); // Scale text with button
        let galley = painter.layout_no_wrap(text.to_string(), font_id, egui::Color32::WHITE);
        let text_pos = center - galley.size() / 2.0;
        painter.galley(text_pos, galley);

        // Change cursor on hover
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }

    response.clicked()
}
