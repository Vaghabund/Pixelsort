use eframe::egui;

/// Fit image inside rect while maintaining aspect ratio (contain mode)
pub fn fit_image_in_rect(image_size: egui::Vec2, container_size: egui::Vec2) -> egui::Vec2 {
    let scale = (container_size.x / image_size.x).min(container_size.y / image_size.y);
    image_size * scale
}

/// Cover mode: scale to fill entire container (crops overflow)
pub fn cover_image_in_rect(image_size: egui::Vec2, container_size: egui::Vec2) -> egui::Vec2 {
    let scale = (container_size.x / image_size.x).max(container_size.y / image_size.y);
    image_size * scale
}

/// Center a rect within another rect
pub fn center_rect_in_rect(content_size: egui::Vec2, container: egui::Rect) -> egui::Rect {
    let offset = (container.size() - content_size) * 0.5;
    egui::Rect::from_min_size(container.min + offset, content_size)
}
