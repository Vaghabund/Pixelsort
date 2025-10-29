use anyhow::Result;

/// Handles system-level operations like shutdown and reboot
pub struct SystemControl;

impl SystemControl {
    /// Initiate system shutdown
    pub fn shutdown() -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            log::info!("Initiating system shutdown...");
            
            Command::new("sudo")
                .args(&["shutdown", "-h", "now"])
                .spawn()
                .map_err(|e| anyhow::anyhow!("Failed to shutdown: {}", e))?;
            
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Shutdown only available on Linux");
            Err(anyhow::anyhow!("Shutdown not supported on this platform"))
        }
    }
    
    /// Initiate system reboot
    pub fn reboot() -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            log::info!("Initiating system reboot...");
            
            Command::new("sudo")
                .args(&["reboot"])
                .spawn()
                .map_err(|e| anyhow::anyhow!("Failed to reboot: {}", e))?;
            
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Reboot only available on Linux");
            Err(anyhow::anyhow!("Reboot not supported on this platform"))
        }
    }
    
    /// Restart a systemd service
    pub fn restart_service(service_name: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            log::info!("Restarting service: {}", service_name);
            
            Command::new("sudo")
                .args(&["systemctl", "restart", service_name])
                .spawn()
                .map_err(|e| anyhow::anyhow!("Failed to restart service: {}", e))?;
            
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Service restart only available on Linux");
            Err(anyhow::anyhow!("Service restart not supported on this platform"))
        }
    }
}
