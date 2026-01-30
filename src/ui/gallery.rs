/// Gallery view for browsing, deleting, and exporting sorted images
use crate::PixelSorterApp;
use eframe::egui;
use std::path::PathBuf;
use std::fs;

// Gallery state that needs to be stored in PixelSorterApp
#[derive(Debug, Clone)]
pub struct GalleryState {
    pub images: Vec<GalleryImage>,
    pub selected_indices: Vec<usize>,
    pub scroll_offset: f32,
    pub needs_refresh: bool,
}

#[derive(Debug, Clone)]
pub struct GalleryImage {
    pub path: PathBuf,
    pub filename: String,
    pub session_name: String,
    pub texture: Option<egui::TextureHandle>,
    pub thumbnail_loaded: bool,
}

impl GalleryState {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            selected_indices: Vec::new(),
            scroll_offset: 0.0,
            needs_refresh: true,
        }
    }

    pub fn refresh_images(&mut self) {
        self.images.clear();
        let sorted_dir = PathBuf::from("sorted_images");
        
        if !sorted_dir.exists() {
            return;
        }

        // Iterate through session folders
        if let Ok(entries) = fs::read_dir(&sorted_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let session_name = entry.file_name().to_string_lossy().to_string();
                    
                    // Iterate through images in session folder
                    if let Ok(image_entries) = fs::read_dir(entry.path()) {
                        for image_entry in image_entries.flatten() {
                            let path = image_entry.path();
                            if path.extension().and_then(|s| s.to_str()) == Some("png") {
                                let filename = image_entry.file_name().to_string_lossy().to_string();
                                self.images.push(GalleryImage {
                                    path: path.clone(),
                                    filename,
                                    session_name: session_name.clone(),
                                    texture: None,
                                    thumbnail_loaded: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Sort images by path (newest sessions first, then by iteration number)
        self.images.sort_by(|a, b| b.path.cmp(&a.path));
        self.needs_refresh = false;
    }

    pub fn toggle_selection(&mut self, index: usize) {
        if let Some(pos) = self.selected_indices.iter().position(|&i| i == index) {
            self.selected_indices.remove(pos);
        } else {
            self.selected_indices.push(index);
        }
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected_indices.contains(&index)
    }

    pub fn select_all(&mut self) {
        self.selected_indices = (0..self.images.len()).collect();
    }

    pub fn clear_selection(&mut self) {
        self.selected_indices.clear();
    }

    pub fn delete_selected(&mut self) -> Result<usize, String> {
        let mut deleted_count = 0;
        
        // Sort indices in reverse to avoid index shifting issues
        let mut indices_to_delete = self.selected_indices.clone();
        indices_to_delete.sort_by(|a, b| b.cmp(a));
        
        for index in indices_to_delete {
            if index < self.images.len() {
                let image = &self.images[index];
                if let Err(e) = fs::remove_file(&image.path) {
                    log::error!("Failed to delete {}: {}", image.path.display(), e);
                } else {
                    deleted_count += 1;
                }
            }
        }
        
        // Clean up empty session directories
        let sorted_dir = PathBuf::from("sorted_images");
        if let Ok(entries) = fs::read_dir(&sorted_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Ok(mut dir_entries) = fs::read_dir(entry.path()) {
                        if dir_entries.next().is_none() {
                            // Directory is empty, delete it
                            let _ = fs::remove_dir(entry.path());
                        }
                    }
                }
            }
        }
        
        self.selected_indices.clear();
        self.needs_refresh = true;
        
        Ok(deleted_count)
    }

    pub fn export_selected_to_usb(&self) -> Result<usize, String> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("mount")
                .output()
                .map_err(|e| format!("Failed to check mounts: {}", e))?;
            
            let mount_output = String::from_utf8_lossy(&output.stdout);
            
            // Find USB mount point
            for line in mount_output.lines() {
                if line.contains("/media/") && 
                   (line.contains("exfat") || line.contains("vfat") || line.contains("ntfs")) {
                    if let Some(on_idx) = line.find(" on ") {
                        if let Some(type_idx) = line.find(" type ") {
                            let mount_point = &line[on_idx + 4..type_idx];
                            let usb_path = PathBuf::from(mount_point);
                            
                            // Test if writable
                            let test_file = usb_path.join(".pixelsort_test");
                            if fs::write(&test_file, "test").is_ok() {
                                let _ = fs::remove_file(&test_file);
                                
                                // Create export directory
                                let dest_dir = usb_path.join("pixelsort_export");
                                fs::create_dir_all(&dest_dir)
                                    .map_err(|e| format!("Failed to create export dir: {}", e))?;
                                
                                // Copy selected images
                                let mut copied_count = 0;
                                for index in &self.selected_indices {
                                    if *index < self.images.len() {
                                        let image = &self.images[*index];
                                        let dest_path = dest_dir.join(&image.filename);
                                        if fs::copy(&image.path, &dest_path).is_ok() {
                                            copied_count += 1;
                                        }
                                    }
                                }
                                
                                return Ok(copied_count);
                            }
                        }
                    }
                }
            }
            
            Err("No writable USB drive found".to_string())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Err("USB export only available on Linux".to_string())
        }
    }
}

impl PixelSorterApp {
    pub fn render_gallery_layout(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        // Refresh images if needed
        if self.gallery_state.needs_refresh {
            self.gallery_state.refresh_images();
        }

        // Background
        egui::Area::new("gallery_background")
            .fixed_pos(egui::pos2(0.0, 0.0))
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                ui.painter().rect_filled(
                    screen_rect,
                    0.0,
                    egui::Color32::from_rgb(20, 20, 20),
                );
            });

        // Top bar with title and close button
        self.render_gallery_top_bar(ctx, screen_rect);

        // Bottom action bar
        self.render_gallery_action_bar(ctx, screen_rect);

        // Main gallery grid
        self.render_gallery_grid(ctx, screen_rect);
    }

    fn render_gallery_top_bar(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        let bar_height = 100.0;
        
        egui::Area::new("gallery_top_bar")
            .fixed_pos(egui::pos2(0.0, 0.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let bar_rect = egui::Rect::from_min_size(
                    egui::pos2(0.0, 0.0),
                    egui::vec2(screen_rect.width(), bar_height),
                );
                
                ui.painter().rect_filled(
                    bar_rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(30, 30, 30, 230),
                );
                
                // Title text
                let title = "Image Gallery";
                let font_id = egui::FontId::proportional(36.0);
                let galley = ui.painter().layout_no_wrap(
                    title.to_string(),
                    font_id,
                    egui::Color32::WHITE,
                );
                let title_pos = egui::pos2(40.0, (bar_height - galley.size().y) / 2.0);
                ui.painter().galley(title_pos, galley);
                
                // Image count
                let count_text = format!("{} images", self.gallery_state.images.len());
                let count_font = egui::FontId::proportional(20.0);
                let count_galley = ui.painter().layout_no_wrap(
                    count_text,
                    count_font,
                    egui::Color32::from_rgb(180, 180, 180),
                );
                let count_pos = egui::pos2(40.0 + galley.size().x + 30.0, (bar_height - count_galley.size().y) / 2.0);
                ui.painter().galley(count_pos, count_galley);
                
                // Close button (X) in top right
                let close_btn_size = 70.0;
                let close_btn_pos = egui::pos2(
                    screen_rect.width() - close_btn_size - 20.0,
                    (bar_height - close_btn_size) / 2.0,
                );
                let close_btn_rect = egui::Rect::from_min_size(close_btn_pos, egui::vec2(close_btn_size, close_btn_size));
                
                let (_, close_response) = ui.allocate_at_rect(close_btn_rect, egui::Sense::click());
                
                let close_color = if close_response.is_pointer_button_down_on() {
                    egui::Color32::from_rgb(200, 50, 50)
                } else if close_response.hovered() {
                    egui::Color32::from_rgb(180, 70, 70)
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 50)
                };
                
                ui.painter().circle_filled(close_btn_rect.center(), close_btn_size / 2.0, close_color);
                
                // Draw X
                let x_size = close_btn_size * 0.4;
                let center = close_btn_rect.center();
                ui.painter().line_segment(
                    [
                        egui::pos2(center.x - x_size / 2.0, center.y - x_size / 2.0),
                        egui::pos2(center.x + x_size / 2.0, center.y + x_size / 2.0),
                    ],
                    egui::Stroke::new(4.0, egui::Color32::WHITE),
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(center.x + x_size / 2.0, center.y - x_size / 2.0),
                        egui::pos2(center.x - x_size / 2.0, center.y + x_size / 2.0),
                    ],
                    egui::Stroke::new(4.0, egui::Color32::WHITE),
                );
                
                if close_response.clicked() {
                    self.current_phase = crate::ui::Phase::Edit;
                }
            });
    }

    fn render_gallery_action_bar(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        let bar_height = 120.0;
        let bar_y = screen_rect.height() - bar_height;
        
        egui::Area::new("gallery_action_bar")
            .fixed_pos(egui::pos2(0.0, bar_y))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let bar_rect = egui::Rect::from_min_size(
                    egui::pos2(0.0, bar_y),
                    egui::vec2(screen_rect.width(), bar_height),
                );
                
                ui.painter().rect_filled(
                    bar_rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(30, 30, 30, 230),
                );
                
                let button_radius = 45.0;
                let button_spacing = 30.0;
                let selected_count = self.gallery_state.selected_indices.len();
                
                // Center the buttons
                let total_buttons = 5;
                let total_width = (button_radius * 2.0 * total_buttons as f32) + (button_spacing * (total_buttons as f32 - 1.0));
                let start_x = (screen_rect.width() - total_width) / 2.0;
                let button_y = bar_y + (bar_height - button_radius * 2.0) / 2.0;
                
                let mut current_x = start_x;
                
                // Select All button
                let select_all_center = egui::pos2(current_x + button_radius, button_y + button_radius);
                let (_, select_all_response) = ui.allocate_at_rect(
                    egui::Rect::from_center_size(select_all_center, egui::vec2(button_radius * 2.0, button_radius * 2.0)),
                    egui::Sense::click()
                );
                
                self.render_action_button(
                    ui,
                    select_all_center,
                    button_radius,
                    "☑",
                    &select_all_response,
                    egui::Color32::from_rgb(70, 120, 180),
                );
                
                if select_all_response.clicked() {
                    self.gallery_state.select_all();
                }
                
                current_x += button_radius * 2.0 + button_spacing;
                
                // Clear Selection button
                let clear_center = egui::pos2(current_x + button_radius, button_y + button_radius);
                let (_, clear_response) = ui.allocate_at_rect(
                    egui::Rect::from_center_size(clear_center, egui::vec2(button_radius * 2.0, button_radius * 2.0)),
                    egui::Sense::click()
                );
                
                let clear_enabled = selected_count > 0;
                self.render_action_button(
                    ui,
                    clear_center,
                    button_radius,
                    "☐",
                    &clear_response,
                    if clear_enabled {
                        egui::Color32::from_rgb(100, 100, 100)
                    } else {
                        egui::Color32::from_rgb(50, 50, 50)
                    },
                );
                
                if clear_response.clicked() && clear_enabled {
                    self.gallery_state.clear_selection();
                }
                
                current_x += button_radius * 2.0 + button_spacing;
                
                // Export button
                let export_center = egui::pos2(current_x + button_radius, button_y + button_radius);
                let (_, export_response) = ui.allocate_at_rect(
                    egui::Rect::from_center_size(export_center, egui::vec2(button_radius * 2.0, button_radius * 2.0)),
                    egui::Sense::click()
                );
                
                let export_enabled = selected_count > 0;
                self.render_action_button(
                    ui,
                    export_center,
                    button_radius,
                    "💾",
                    &export_response,
                    if export_enabled {
                        egui::Color32::from_rgb(70, 150, 70)
                    } else {
                        egui::Color32::from_rgb(50, 50, 50)
                    },
                );
                
                if export_response.clicked() && export_enabled {
                    match self.gallery_state.export_selected_to_usb() {
                        Ok(count) => {
                            self.export_message = Some(format!("✓ Exported {} images to USB", count));
                            self.export_message_time = Some(std::time::Instant::now());
                        }
                        Err(e) => {
                            self.export_message = Some(format!("✗ Export failed: {}", e));
                            self.export_message_time = Some(std::time::Instant::now());
                        }
                    }
                }
                
                current_x += button_radius * 2.0 + button_spacing;
                
                // Delete button
                let delete_center = egui::pos2(current_x + button_radius, button_y + button_radius);
                let (_, delete_response) = ui.allocate_at_rect(
                    egui::Rect::from_center_size(delete_center, egui::vec2(button_radius * 2.0, button_radius * 2.0)),
                    egui::Sense::click()
                );
                
                let delete_enabled = selected_count > 0;
                self.render_action_button(
                    ui,
                    delete_center,
                    button_radius,
                    "🗑",
                    &delete_response,
                    if delete_enabled {
                        egui::Color32::from_rgb(180, 50, 50)
                    } else {
                        egui::Color32::from_rgb(50, 50, 50)
                    },
                );
                
                if delete_response.clicked() && delete_enabled {
                    match self.gallery_state.delete_selected() {
                        Ok(count) => {
                            self.export_message = Some(format!("✓ Deleted {} images", count));
                            self.export_message_time = Some(std::time::Instant::now());
                        }
                        Err(e) => {
                            self.export_message = Some(format!("✗ Delete failed: {}", e));
                            self.export_message_time = Some(std::time::Instant::now());
                        }
                    }
                }
                
                current_x += button_radius * 2.0 + button_spacing;
                
                // Refresh button
                let refresh_center = egui::pos2(current_x + button_radius, button_y + button_radius);
                let (_, refresh_response) = ui.allocate_at_rect(
                    egui::Rect::from_center_size(refresh_center, egui::vec2(button_radius * 2.0, button_radius * 2.0)),
                    egui::Sense::click()
                );
                
                self.render_action_button(
                    ui,
                    refresh_center,
                    button_radius,
                    "🔄",
                    &refresh_response,
                    egui::Color32::from_rgb(100, 100, 100),
                );
                
                if refresh_response.clicked() {
                    self.gallery_state.needs_refresh = true;
                }
                
                // Show selection count
                if selected_count > 0 {
                    let count_text = format!("{} selected", selected_count);
                    let font = egui::FontId::proportional(22.0);
                    let galley = ui.painter().layout_no_wrap(
                        count_text,
                        font,
                        egui::Color32::from_rgb(200, 200, 200),
                    );
                    let count_pos = egui::pos2(
                        (screen_rect.width() - galley.size().x) / 2.0,
                        bar_y + bar_height - 25.0,
                    );
                    ui.painter().galley(count_pos, galley);
                }
            });
    }

    fn render_action_button(
        &self,
        ui: &mut egui::Ui,
        center: egui::Pos2,
        radius: f32,
        emoji: &str,
        response: &egui::Response,
        base_color: egui::Color32,
    ) {
        let color = if response.is_pointer_button_down_on() {
            egui::Color32::from_rgb(
                (base_color.r() as f32 * 1.2).min(255.0) as u8,
                (base_color.g() as f32 * 1.2).min(255.0) as u8,
                (base_color.b() as f32 * 1.2).min(255.0) as u8,
            )
        } else if response.hovered() {
            egui::Color32::from_rgb(
                (base_color.r() as f32 * 1.1).min(255.0) as u8,
                (base_color.g() as f32 * 1.1).min(255.0) as u8,
                (base_color.b() as f32 * 1.1).min(255.0) as u8,
            )
        } else {
            base_color
        };
        
        let scale = if response.is_pointer_button_down_on() { 0.95 } else { 1.0 };
        let scaled_radius = radius * scale;
        
        ui.painter().circle_filled(center, scaled_radius, color);
        ui.painter().circle_stroke(
            center,
            scaled_radius,
            egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100)),
        );
        
        // Draw emoji
        let font = egui::FontId::proportional(radius * 0.8);
        let galley = ui.painter().layout_no_wrap(emoji.to_string(), font, egui::Color32::WHITE);
        let text_pos = center - galley.size() / 2.0;
        ui.painter().galley(text_pos, galley);
    }

    fn render_gallery_grid(&mut self, ctx: &egui::Context, screen_rect: egui::Rect) {
        let top_bar_height = 100.0;
        let bottom_bar_height = 120.0;
        let grid_y = top_bar_height;
        let grid_height = screen_rect.height() - top_bar_height - bottom_bar_height;
        
        let padding = 30.0;
        let thumbnail_size = 280.0;
        let thumbnail_spacing = 40.0;
        
        let available_width = screen_rect.width() - padding * 2.0;
        let columns = ((available_width + thumbnail_spacing) / (thumbnail_size + thumbnail_spacing)).floor() as usize;
        let columns = columns.max(1);
        
        egui::Area::new("gallery_grid")
            .fixed_pos(egui::pos2(padding, grid_y + padding))
            .order(egui::Order::Middle)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(grid_height - padding * 2.0)
                    .show(ui, |ui| {
                        // Show message if no images
                        if self.gallery_state.images.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(100.0);
                                ui.label(
                                    egui::RichText::new("No images found")
                                        .size(32.0)
                                        .color(egui::Color32::from_rgb(150, 150, 150))
                                );
                                ui.add_space(20.0);
                                ui.label(
                                    egui::RichText::new("Create some pixel-sorted images to see them here")
                                        .size(20.0)
                                        .color(egui::Color32::from_rgb(120, 120, 120))
                                );
                            });
                            return;
                        }
                        
                        // Render grid
                        let mut current_x = 0.0;
                        let mut current_y = 0.0;
                        
                        for (index, image) in self.gallery_state.images.iter().enumerate() {
                            let pos = egui::pos2(current_x, current_y);
                            let rect = egui::Rect::from_min_size(pos, egui::vec2(thumbnail_size, thumbnail_size + 50.0));
                            
                            let response = ui.allocate_rect(rect, egui::Sense::click());
                            
                            // Draw thumbnail background
                            let bg_color = if self.gallery_state.is_selected(index) {
                                egui::Color32::from_rgb(70, 120, 180)
                            } else if response.hovered() {
                                egui::Color32::from_rgb(60, 60, 60)
                            } else {
                                egui::Color32::from_rgb(40, 40, 40)
                            };
                            
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(pos, egui::vec2(thumbnail_size, thumbnail_size)),
                                8.0,
                                bg_color,
                            );
                            
                            // Try to load and display thumbnail
                            if let Ok(img) = image::open(&image.path) {
                                // Resize to thumbnail
                                let thumb = img.thumbnail(thumbnail_size as u32, thumbnail_size as u32);
                                let rgba = thumb.to_rgba8();
                                let size = [rgba.width() as usize, rgba.height() as usize];
                                let pixels = rgba.as_flat_samples();
                                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                                
                                let texture_id = format!("gallery_thumb_{}", index);
                                let texture = ctx.load_texture(&texture_id, color_image, Default::default());
                                
                                // Center the thumbnail in the square
                                let thumb_w = thumb.width() as f32;
                                let thumb_h = thumb.height() as f32;
                                let offset_x = (thumbnail_size - thumb_w) / 2.0;
                                let offset_y = (thumbnail_size - thumb_h) / 2.0;
                                
                                let thumb_rect = egui::Rect::from_min_size(
                                    egui::pos2(pos.x + offset_x, pos.y + offset_y),
                                    egui::vec2(thumb_w, thumb_h),
                                );
                                
                                ui.painter().image(
                                    texture.id(),
                                    thumb_rect,
                                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                    egui::Color32::WHITE,
                                );
                            }
                            
                            // Draw selection indicator
                            if self.gallery_state.is_selected(index) {
                                let check_size = 40.0;
                                let check_pos = egui::pos2(
                                    pos.x + thumbnail_size - check_size - 10.0,
                                    pos.y + 10.0,
                                );
                                ui.painter().circle_filled(
                                    egui::pos2(check_pos.x + check_size / 2.0, check_pos.y + check_size / 2.0),
                                    check_size / 2.0,
                                    egui::Color32::from_rgb(70, 180, 70),
                                );
                                
                                // Draw checkmark
                                let font = egui::FontId::proportional(check_size * 0.7);
                                let galley = ui.painter().layout_no_wrap("✓".to_string(), font, egui::Color32::WHITE);
                                let check_text_pos = egui::pos2(
                                    check_pos.x + (check_size - galley.size().x) / 2.0,
                                    check_pos.y + (check_size - galley.size().y) / 2.0,
                                );
                                ui.painter().galley(check_text_pos, galley);
                            }
                            
                            // Draw filename below thumbnail
                            let filename_y = pos.y + thumbnail_size + 10.0;
                            let font = egui::FontId::proportional(14.0);
                            let text = if image.filename.len() > 30 {
                                format!("{}...", &image.filename[..27])
                            } else {
                                image.filename.clone()
                            };
                            let galley = ui.painter().layout_no_wrap(
                                text,
                                font,
                                egui::Color32::from_rgb(200, 200, 200),
                            );
                            let text_x = pos.x + (thumbnail_size - galley.size().x) / 2.0;
                            ui.painter().galley(egui::pos2(text_x, filename_y), galley);
                            
                            // Handle click
                            if response.clicked() {
                                self.gallery_state.toggle_selection(index);
                            }
                            
                            // Move to next position
                            current_x += thumbnail_size + thumbnail_spacing;
                            if (index + 1) % columns == 0 {
                                current_x = 0.0;
                                current_y += thumbnail_size + thumbnail_spacing + 50.0; // +50 for filename
                            }
                        }
                    });
            });
    }
}
