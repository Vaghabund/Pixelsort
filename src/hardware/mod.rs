// Hardware domain - interfaces to physical hardware components

pub mod camera;
pub mod camera_controller;
pub mod ups_monitor;

// Re-export commonly used types
pub use camera_controller::CameraController;
pub use ups_monitor::{UpsConfig, BatteryStatus, get_battery_status, is_shutdown_requested, start_monitoring};
