use eframe::egui;

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
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 26),  // 0.1 opacity
            egui::Stroke::NONE,
        );

        // Calculate normalized position (inverted for vertical)
        let min = *range.start();
        let max = *range.end();
        let normalized = (*value - min) / (max - min);

        // Handle dragging
        if response.dragged() || response.clicked() {
            if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
                // Invert y-axis (top = max, bottom = min)
                let new_normalized = 1.0 - ((mouse_pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);
                *value = min + new_normalized * (max - min);
                changed = true;
                response.mark_changed();
            }
        }

        // Filled portion (from bottom up) - subtle blue fill
        let filled_height = rect.height() * normalized;
        if filled_height > 0.0 {
            let filled_rect = egui::Rect::from_min_max(
                egui::pos2(rail_rect.min.x, rail_rect.max.y - filled_height),
                rail_rect.max,
            );
            painter.rect(
                filled_rect,
                rail_rect.width() / 2.0,
                egui::Color32::from_rgba_unmultiplied(100, 150, 255, 120),
                egui::Stroke::NONE,
            );
        }

        // CSS: Knob/handle - larger size for better touch
        let knob_y = rect.bottom() - rect.height() * normalized;
        let knob_center = egui::pos2(rect.center().x, knob_y);
        let knob_radius = 30.0;  // Increased from 25 to 30 (60px diameter)

        // CSS: box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3)
        painter.circle(
            knob_center + egui::vec2(0.0, 2.0),
            knob_radius,
            egui::Color32::from_black_alpha(77),  // 0.3 opacity
            egui::Stroke::NONE,
        );

        // CSS: background: rgba(255, 255, 255, 0.9), border: 4px solid rgba(255, 255, 255, 0.3)
        painter.circle(
            knob_center,
            knob_radius,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 230),  // 0.9 opacity
            egui::Stroke::new(4.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 77)),  // 0.3 opacity
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
            // Active/pressed state: rgba(255, 255, 255, 0.25)
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 64)
        } else if response.hovered() {
            // Hover state: slightly brighter than base
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50)
        } else {
            // Normal state: rgba(255, 255, 255, 0.15)
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
            egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30)),
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
