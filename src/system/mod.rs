// System domain - system-level operations and management

pub mod control;
pub mod update_manager;

// Re-export commonly used types
pub use control::SystemControl;
pub use update_manager::UpdateManager;
