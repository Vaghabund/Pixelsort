use crate::PixelSorterApp;
use crate::system::SystemControl;
use eframe::egui;
use std::time::Instant;

const UI_PADDING: f32 = 20.0;

impl PixelSorterApp {
    pub fn render_shutdown_button(&mut self, ctx: &egui::Context, _screen_rect: egui::Rect) {
        egui::Area::new("shutdown_button")
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(UI_PADDING, UI_PADDING))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let button_size = 80.0; // Doubled from 40.0
                let button_pos = ui.cursor().min;
                let button_rect = egui::Rect::from_min_size(
                    button_pos,
                    egui::vec2(button_size, button_size),
                );

                let response = ui.allocate_rect(button_rect, egui::Sense::click());

                // Draw power icon (circle with vertical line at top)
                let center = button_rect.center();
                let radius = button_size * 0.35;

                // Glassmorphism background
                let bg_color = if response.is_pointer_button_down_on() {
                    egui::Color32::from_rgba_unmultiplied(255, 80, 80, 200) // Bright red on press
                } else if response.hovered() {
                    egui::Color32::from_rgba_unmultiplied(220, 50, 50, 180) // Red on hover
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38) // Glassmorphism rgba(255, 255, 255, 0.15)
                };

                // Apply scale transform on press
                let scale = if response.is_pointer_button_down_on() { 0.95 } else { 1.0 };
                let scaled_radius = button_size * 0.45 * scale;

                ui.painter().circle_filled(center, scaled_radius, bg_color);

                // Power symbol: partial circle (arc) + line
                let stroke = egui::Stroke::new(6.0, egui::Color32::WHITE); // Doubled from 3.0

                // Vertical line (power button line)
                let line_start = egui::pos2(center.x, center.y - radius * 0.8);
                let line_end = egui::pos2(center.x, center.y + radius * 0.3);
                ui.painter().line_segment([line_start, line_end], stroke);

                // Arc (incomplete circle)
                use std::f32::consts::PI;
                let num_points = 20;
                let start_angle = PI * 0.7; // Start at bottom-left
                let end_angle = PI * 2.3;   // End at bottom-right

                let mut points = Vec::new();
                for i in 0..=num_points {
                    let t = i as f32 / num_points as f32;
                    let angle = start_angle + (end_angle - start_angle) * t;
                    let x = center.x + radius * angle.cos();
                    let y = center.y + radius * angle.sin();
                    points.push(egui::pos2(x, y));
                }

                for i in 0..points.len() - 1 {
                    ui.painter().line_segment([points[i], points[i + 1]], stroke);
                }

                // Handle click
                if response.clicked() {
                    self.show_shutdown_menu = true;
                }
            });

        // Show shutdown confirmation menu
        if self.show_shutdown_menu {
            egui::Window::new("Power Options")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    let menu_width = 350.0; 
                    let button_height = 80.0; 
                    let cancel_height = 60.0; 

                    ui.set_min_width(menu_width);

                    ui.vertical_centered(|ui| {
                        // Shutdown button
                        if ui.add_sized([menu_width, button_height], egui::Button::new("🔌 Shutdown")).clicked() {
                            log::info!("Shutdown requested by user");
                            if let Err(e) = SystemControl::shutdown() {
                                log::error!("Failed to shutdown: {}", e);
                                self.export_message = Some(format!("✗ Shutdown failed: {}", e));
                                self.export_message_time = Some(Instant::now());
                            }
                            self.show_shutdown_menu = false;
                        }

                        ui.add_space(5.0);

                        // Reboot button
                        if ui.add_sized([menu_width, button_height], egui::Button::new("🔄 Reboot")).clicked() {
                            log::info!("Reboot requested by user");
                            if let Err(e) = SystemControl::reboot() {
                                log::error!("Failed to reboot: {}", e);
                                self.export_message = Some(format!("✗ Reboot failed: {}", e));
                                self.export_message_time = Some(Instant::now());
                            }
                            self.show_shutdown_menu = false;
                        }

                        ui.add_space(5.0);

                        // Exit app button
                        if ui.add_sized([menu_width, button_height], egui::Button::new("❌ Exit App")).clicked() {
                            log::info!("Exit app requested by user");
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        ui.add_space(10.0);

                        // Cancel button
                        if ui.add_sized([menu_width, cancel_height], egui::Button::new("Cancel")).clicked() {
                            self.show_shutdown_menu = false;
                        }
                    });
                });
        }
    }

    pub fn render_developer_menu(&mut self, ctx: &egui::Context, _screen_rect: egui::Rect) {
        if !self.show_developer_menu {
            return;
        }

        egui::Window::new("🛠 Developer Menu")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let menu_width = 933.0; // 700.0 * 1.33
                let button_width = 800.0; // 600.0 * 1.33
                let button_height = 107.0; // 80.0 * 1.33
                
                ui.set_min_width(menu_width);

                ui.heading(egui::RichText::new("Developer Tools").size(32.0)); // Added size double
                ui.add_space(20.0); // Doubled from 10.0

                ui.vertical_centered(|ui| {
                    // System info section
                    ui.group(|ui| {
                        ui.set_min_width(button_width);
                        ui.label(egui::RichText::new("System Info").strong().size(24.0)); // Added size double
                        ui.separator();

                        // Battery status
                        let battery = crate::hardware::get_battery_status();
                        if battery.is_available {
                            let battery_text = if battery.is_charging {
                                format!("🔋 Battery: {:.0}% ({:.1}V) ⚡ Charging",
                                       battery.percentage, battery.voltage)
                            } else {
                                format!("🔋 Battery: {:.0}% ({:.1}V)",
                                       battery.percentage, battery.voltage)
                            };
                            ui.label(egui::RichText::new(battery_text).size(20.0)); // Added size double
                        } else {
                            ui.label(egui::RichText::new("🔋 Battery: Not detected").size(20.0)); // Added size double
                        }

                        // Current phase
                        ui.label(egui::RichText::new(format!("📍 Phase: {:?}", self.current_phase)).size(20.0)); // Added size double

                        // Session info
                        if let Some(ref session) = self.current_session_folder {
                            ui.label(egui::RichText::new(format!("📁 Session: {}", session)).size(20.0)); // Added size double
                            ui.label(egui::RichText::new(format!("🔢 Iteration: {}", self.iteration_counter)).size(20.0)); // Added size double
                        }
                    });

                    ui.add_space(20.0); // Doubled from 10.0

                    // Actions section
                    ui.group(|ui| {
                        ui.set_min_width(button_width);
                        ui.label(egui::RichText::new("Actions").strong().size(24.0));
                        ui.separator();

                        // Update status
                        if self.update_manager.update_available {
                            ui.label(egui::RichText::new("🆕 Update Available!").color(egui::Color32::from_rgb(100, 220, 100)).size(22.0));
                            ui.add_space(5.0);

                            // Restart & Update button
                            if ui.add_sized([button_width, button_height], egui::Button::new(egui::RichText::new("🔄 Pull & Restart").size(24.0))).clicked() {
                                log::info!("Pull & Restart requested");
                                self.export_message = Some("🔄 Pulling updates and restarting...".to_string());
                                self.export_message_time = Some(Instant::now());
                                self.show_developer_menu = false;

                                // Pull updates and restart service using update_manager
                                let _ = self.update_manager.pull_and_restart_service("pixelsort-kiosk");
                            }
                        } else {
                            ui.label(egui::RichText::new("✅ App is up to date").color(egui::Color32::GRAY).size(20.0));
                            ui.add_space(5.0);

                            // Manual check button
                            if ui.add_sized([button_width, button_height], egui::Button::new(egui::RichText::new("🔄 Check Now").size(24.0))).clicked() {
                                log::info!("Manual update check requested");
                                match self.update_manager.check_for_updates() {
                                    Ok(update_found) => {
                                        if update_found {
                                            self.export_message = Some("🆕 Update found! Restart to apply.".to_string());
                                            log::info!("Update available!");
                                        } else {
                                            self.export_message = Some("✅ Already up to date".to_string());
                                            log::info!("No updates available");
                                        }
                                        self.update_check_time = Some(Instant::now());
                                    }
                                    Err(e) => {
                                        self.export_message = Some(format!("❌ Update check failed: {}", e));
                                        log::error!("Update check failed: {}", e);
                                    }
                                }
                                self.export_message_time = Some(Instant::now());
                                // Don't close menu - let user see the result
                            }
                        }

                        ui.add_space(10.0); // Doubled from 5.0

                        // Clear session
                        if ui.add_sized([button_width, button_height], egui::Button::new(egui::RichText::new("🗑 Clear Session").size(24.0))).clicked() { // Doubled from [300.0, 40.0] and added text size
                            self.iteration_counter = 0;
                            self.current_session_folder = None;
                            self.export_message = Some("✓ Session cleared".to_string());
                            self.export_message_time = Some(Instant::now());
                            log::info!("Session manually cleared");
                            self.show_developer_menu = false;
                        }

                        ui.add_space(10.0); // Doubled from 5.0

                        // Restart app
                        if ui.add_sized([button_width, button_height], egui::Button::new(egui::RichText::new("🔁 Restart App").size(24.0))).clicked() { // Doubled from [300.0, 40.0] and added text size
                            log::info!("App restart requested");
                            self.export_message = Some("🔁 Restarting...".to_string());
                            self.export_message_time = Some(Instant::now());
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            // Note: systemd will auto-restart if configured with Restart=on-failure
                        }

                        ui.add_space(10.0); // Doubled from 5.0

                        // Exit app
                        if ui.add_sized([button_width, button_height], egui::Button::new(egui::RichText::new("❌ Exit App").size(24.0))).clicked() { // Doubled from [300.0, 40.0] and added text size
                            log::info!("Exit app requested from dev menu");
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.add_space(20.0); // Doubled from 10.0

                    // Close button
                    if ui.add_sized([button_width, button_height], egui::Button::new(egui::RichText::new("Close Menu").size(24.0))).clicked() { // Doubled from [300.0, 40.0] and added text size
                        self.show_developer_menu = false;
                    }
                });

                ui.add_space(10.0); // Doubled from 5.0
                ui.label(
                    egui::RichText::new("Tip: Press ESC or 5-tap bottom-right corner to toggle this menu")
                        .size(18.0)
                        .color(egui::Color32::GRAY)
                );
            });
    }

    pub fn render_usb_export_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_usb_export_dialog {
            return;
        }

        egui::Window::new("💾 Export to USB")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let menu_width = 450.0;
                let button_height = 70.0;
                
                ui.set_min_width(menu_width);

                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    ui.label(egui::RichText::new("📁 Copy all sorted images to USB?").size(22.0));
                    
                    ui.add_space(15.0);
                    
                    // Checkbox for delete after copy
                    ui.horizontal(|ui| {
                        ui.add_space((menu_width - 320.0) / 2.0); // Center the checkbox
                        ui.checkbox(&mut self.usb_export_delete_after, 
                            egui::RichText::new("Delete images after copying").size(18.0));
                    });
                    
                    ui.add_space(20.0);
                    
                    // Copy button
                    if ui.add_sized([menu_width, button_height], 
                        egui::Button::new(egui::RichText::new("✓ Copy to USB").size(24.0))
                            .fill(egui::Color32::from_rgb(40, 120, 40)))
                        .clicked() 
                    {
                        log::info!("USB export started (delete_after: {})", self.usb_export_delete_after);
                        
                        match self.copy_to_usb() {
                            Ok(()) => {
                                self.export_message = Some("✓ Exported to USB!".to_string());
                                
                                // Delete files if checkbox was checked
                                if self.usb_export_delete_after {
                                    if let Err(e) = std::fs::remove_dir_all("sorted_images") {
                                        log::error!("Failed to delete images: {}", e);
                                        self.export_message = Some(format!("⚠ Exported but deletion failed: {}", e));
                                    } else {
                                        log::info!("Deleted local sorted_images folder after export");
                                        self.export_message = Some("✓ Exported and deleted local copies!".to_string());
                                        // Reset session since we deleted everything
                                        self.current_session_folder = None;
                                        self.iteration_counter = 0;
                                    }
                                }
                                
                                self.export_message_time = Some(Instant::now());
                            }
                            Err(e) => {
                                self.export_message = Some(format!("✗ Export failed: {}", e));
                                self.export_message_time = Some(Instant::now());
                            }
                        }
                        
                        self.show_usb_export_dialog = false;
                        self.usb_export_delete_after = false; // Reset for next time
                    }
                    
                    ui.add_space(10.0);
                    
                    // Cancel button
                    if ui.add_sized([menu_width, button_height], 
                        egui::Button::new(egui::RichText::new("Cancel").size(24.0)))
                        .clicked() 
                    {
                        self.show_usb_export_dialog = false;
                        self.usb_export_delete_after = false; // Reset
                    }
                });
            });
    }
}
