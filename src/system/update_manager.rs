use anyhow::Result;

/// Manages application updates via git
#[allow(dead_code)]
pub struct UpdateManager {
    /// Whether an update is available
    pub update_available: bool,
    /// Path to the project directory
    project_path: String,
}

impl UpdateManager {
    pub fn new(project_path: String) -> Self {
        Self {
            update_available: false,
            project_path,
        }
    }
    
    /// Check if updates are available by comparing local and remote git commits
    pub fn check_for_updates(&mut self) -> Result<bool> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            log::info!("Checking for updates...");
            
            let cmd = format!(
                "cd {} && git fetch origin main 2>/dev/null && git rev-parse HEAD && git rev-parse origin/main",
                self.project_path
            );
            
            let output = Command::new("sh")
                .args(&["-c", &cmd])
                .output()?;
            
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = result.lines().collect();
                
                if lines.len() == 2 {
                    let local = lines[0].trim();
                    let remote = lines[1].trim();
                    
                    if local != remote {
                        log::info!("Update available: {} -> {}", &local[..7], &remote[..7]);
                        self.update_available = true;
                        return Ok(true);
                    } else {
                        log::info!("App is up to date: {}", &local[..7]);
                        self.update_available = false;
                        return Ok(false);
                    }
                }
            }
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Update checking only available on Linux");
        }
        
        Ok(false)
    }
    
    /// Pull updates and restart the systemd service
    pub fn pull_and_restart_service(&self, _service_name: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            log::info!("Pulling updates and restarting service...");
            
            let cmd = format!(
                "cd {} && git pull origin main && sudo systemctl restart {}",
                self.project_path, _service_name
            );
            
            Command::new("sh")
                .args(&["-c", &cmd])
                .spawn()?;
            
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Service restart only available on Linux");
            Ok(())
        }
    }
}
