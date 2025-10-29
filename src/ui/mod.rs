use std::sync::Arc;
use std::time::Instant;
use eframe::egui;
use tokio::sync::RwLock;

use crate::system::UpdateManager;
use crate::processing::{PixelSorter, SortingAlgorithm, SortingParameters};
use crate::hardware::CameraController;

// Module declarations
mod state;
mod helpers;
mod widgets;
mod screens;
mod indicators;
mod menus;
mod viewport;
mod buttons;

// Re-export public types
pub use state::{Phase, DragState, HandlePosition};

// ============================================================================
// MAIN APP STRUCT
// ============================================================================

pub struct PixelSorterApp {
    // Phase management
    pub current_phase: Phase,
    
    // Image data
    pub original_image: Option<image::RgbImage>,
    pub processed_image: Option<image::RgbImage>,
    pub camera_texture: Option<egui::TextureHandle>,
    pub processed_texture: Option<egui::TextureHandle>,
    
    // Processing
    pub pixel_sorter: Arc<PixelSorter>,
    pub current_algorithm: SortingAlgorithm,
    pub sorting_params: SortingParameters,
    pub is_processing: bool,
    
    // Camera
    pub camera_controller: Option<Arc<RwLock<CameraController>>>,
    pub last_camera_update: Option<Instant>,
    pub preview_mode: bool,
    
    // Crop state
    pub crop_rect: Option<egui::Rect>, // In image coordinates
    pub drag_state: DragState,
    
    // Session management
    pub iteration_counter: u32,
    pub current_session_folder: Option<String>,
    
    // Export status
    pub export_message: Option<String>,
    pub export_message_time: Option<Instant>,
    
    // Splash screen
    pub show_splash: bool,
    pub splash_start_time: Option<Instant>,
    pub splash_logo: Option<egui::TextureHandle>,
    
    // Exit mechanism for kiosk mode
    pub exit_tap_count: u32,
    pub exit_tap_last_time: Option<Instant>,
    
    // Sleep mode (power saving after idle)
    pub is_sleeping: bool,
    pub is_waking: bool,
    pub wake_start_time: Option<Instant>,
    pub last_interaction_time: Instant,
    pub sleep_logo: Option<egui::TextureHandle>,
    
    // Update checking
    pub update_manager: UpdateManager,
    pub update_check_time: Option<Instant>,
    pub startup_check_done: bool,
    
    // Shutdown menu
    pub show_shutdown_menu: bool,
    
    // Developer menu
    pub show_developer_menu: bool,
    
    // Other
    pub tint_enabled: bool,
}

// ============================================================================
// INITIALIZATION
// ============================================================================

impl PixelSorterApp {
    pub fn new(
        pixel_sorter: Arc<PixelSorter>,
        camera_controller: Option<Arc<RwLock<CameraController>>>,
    ) -> Self {
        // Start camera streaming if available
        if let Some(ref camera) = camera_controller {
            if let Ok(mut camera_lock) = camera.try_write() {
                let _ = camera_lock.start_streaming();
            }
        }

        Self {
            current_phase: Phase::Input,
            original_image: None,
            processed_image: None,
            camera_texture: None,
            processed_texture: None,
            pixel_sorter,
            current_algorithm: SortingAlgorithm::Horizontal,
            sorting_params: SortingParameters::default(),
            is_processing: false,
            camera_controller,
            last_camera_update: None,
            preview_mode: true,
            crop_rect: None,
            drag_state: DragState::None,
            iteration_counter: 0,
            current_session_folder: None,
            export_message: None,
            export_message_time: None,
            show_splash: true,
            splash_start_time: Some(Instant::now()),
            splash_logo: None,
            exit_tap_count: 0,
            exit_tap_last_time: None,
            is_sleeping: false,
            is_waking: false,
            wake_start_time: None,
            last_interaction_time: Instant::now(),
            sleep_logo: None,
            update_manager: UpdateManager::new("/home/pixelsort/Pixelsort".to_string()),
            update_check_time: None,
            startup_check_done: false,
            show_shutdown_menu: false,
            show_developer_menu: false,
            tint_enabled: false,
        }
    }

    fn usb_present(&self) -> bool {
        let usb_paths = ["/media/pixelsort", "/media/pi", "/media/usb", "/media", "/mnt/usb", "/mnt"];
        for base_path in &usb_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let usb_path = entry.path();
                    
                    // Skip if not a directory or if it's the pi user home
                    if !usb_path.is_dir() || usb_path.to_string_lossy().contains("/home/") {
                        continue;
                    }
                    
                    // Check if we can write to this path (indicates writable USB)
                    let test_file = usb_path.join(".pixelsort_usb_check");
                    if std::fs::write(&test_file, "test").is_ok() {
                        let _ = std::fs::remove_file(&test_file);
                        return true;
                    }
                }
            }
        }
        false
    }
}

// ============================================================================
// MAIN UPDATE LOOP
// ============================================================================

impl eframe::App for PixelSorterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if UPS requested shutdown
        if crate::hardware::is_shutdown_requested() {
            log::warn!("UPS shutdown requested - closing application");
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        
        // Hide cursor in kiosk mode - force it every frame
        ctx.set_cursor_icon(egui::CursorIcon::None);
        ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::None);
        
        // ESC key to open developer menu (for debugging with keyboard)
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_developer_menu = !self.show_developer_menu;
        }
        
        // Hidden developer menu trigger (bottom-right corner, tap 5 times within 3 seconds)
        let screen_size = ctx.screen_rect().size();
        egui::Area::new("dev_menu_trigger")
            .fixed_pos(egui::pos2(screen_size.x - 50.0, screen_size.y - 50.0))
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let trigger_size = egui::vec2(50.0, 50.0);
                let (_rect, response) = ui.allocate_exact_size(trigger_size, egui::Sense::click());
                
                if response.clicked() {
                    let now = Instant::now();
                    
                    if let Some(last_time) = self.exit_tap_last_time {
                        if now.duration_since(last_time).as_secs() > 3 {
                            self.exit_tap_count = 0;
                        }
                    }
                    
                    self.exit_tap_count += 1;
                    self.exit_tap_last_time = Some(now);
                    
                    if self.exit_tap_count >= 5 {
                        self.show_developer_menu = true;
                        self.exit_tap_count = 0;
                    }
                }
            });
        
        // Show splash screen for 2 seconds
        if self.show_splash {
            if let Some(start_time) = self.splash_start_time {
                let elapsed = start_time.elapsed().as_secs_f32();
                if elapsed > 2.0 {
                    self.show_splash = false;
                } else {
                    self.render_splash_screen(ctx, elapsed);
                    ctx.request_repaint();
                    return;
                }
            }
        }
        
        // Background update check
        if !self.startup_check_done {
            if self.update_check_time.is_none() {
                self.update_check_time = Some(Instant::now());
            } else if let Some(start_time) = self.update_check_time {
                if start_time.elapsed().as_secs() >= 30 {
                    let _ = self.update_manager.check_for_updates();
                    self.startup_check_done = true;
                }
            }
        }
        
        // Check for user interaction
        let has_interaction = ctx.input(|i| {
            i.pointer.any_pressed() || 
            i.pointer.any_down() || 
            i.pointer.is_moving() ||
            !i.events.is_empty() ||
            i.key_pressed(egui::Key::Space) ||
            i.key_pressed(egui::Key::Escape) ||
            i.key_pressed(egui::Key::Enter)
        });
        
        if has_interaction {
            if self.is_sleeping {
                self.is_sleeping = false;
                self.is_waking = true;
                self.wake_start_time = Some(Instant::now());
                log::info!("Waking from sleep mode...");
            }
            self.last_interaction_time = Instant::now();
        }
        
        // Wake-up animation
        if self.is_waking {
            if let Some(wake_start) = self.wake_start_time {
                if wake_start.elapsed().as_secs() >= 1 {
                    self.is_waking = false;
                    self.wake_start_time = None;
                    log::info!("Wake-up complete");
                }
            }
        }
        
        // Sleep mode check (5 minutes)
        let idle_duration = self.last_interaction_time.elapsed().as_secs();
        if !self.is_sleeping && idle_duration >= 300 {
            self.is_sleeping = true;
        }
        
        // If sleeping, show sleep screen
        if self.is_sleeping {
            self.render_sleep_screen(ctx);
            ctx.request_repaint();
            return;
        }
        
        // If waking, show wake-up screen
        if self.is_waking {
            self.render_waking_screen(ctx);
            ctx.request_repaint();
            return;
        }
        
        // Update camera preview at 30 FPS if in Input phase
        if self.current_phase == Phase::Input && !self.is_processing {
            self.update_camera_preview(ctx);
            ctx.request_repaint();
        }

        // Render UI
        self.render_ui(ctx);
    }
}

impl PixelSorterApp {
    fn update_camera_preview(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let should_update = match self.last_camera_update {
            None => true,
            Some(last) => now.duration_since(last) >= std::time::Duration::from_millis(33),
        };

        if should_update {
            if let Some(camera) = self.camera_controller.clone() {
                if let Ok(mut camera_lock) = camera.try_write() {
                    if let Ok(preview_image) = camera_lock.get_fast_preview_image() {
                        self.update_camera_texture(ctx, &preview_image);
                        self.last_camera_update = Some(now);
                    }
                }
            }
        }
    }

    fn render_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let full_rect = ui.max_rect();
                
                self.render_viewport(ui, full_rect, ctx);
                self.render_button_overlay(ui, ctx, full_rect);
                self.render_battery_indicator(ctx, full_rect);
                self.render_shutdown_button(ctx, full_rect);
                self.render_developer_menu(ctx, full_rect);
                self.render_export_message(ctx, full_rect);
            });
    }
}
