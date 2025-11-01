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
            
            log::info!("Checking for updates in: {}", self.project_path);
            
            let cmd = format!(
                "cd {} && git fetch origin main 2>&1 && git rev-parse HEAD && git rev-parse origin/main",
                self.project_path
            );
            
            let output = Command::new("sh")
                .args(&["-c", &cmd])
                .output()?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                log::error!("Git command failed. stdout: {}, stderr: {}", stdout, stderr);
                return Err(anyhow::anyhow!("Git command failed: {}", stderr));
            }
            
            let result = String::from_utf8_lossy(&output.stdout);
            log::info!("Git output: {}", result);
            let lines: Vec<&str> = result.lines().collect();
            
            if lines.len() >= 2 {
                let local = lines[lines.len() - 2].trim();
                let remote = lines[lines.len() - 1].trim();
                
                log::info!("Local commit: {}, Remote commit: {}", local, remote);
                
                if local != remote {
                    log::info!("Update available: {} -> {}", &local[..7.min(local.len())], &remote[..7.min(remote.len())]);
                    self.update_available = true;
                    return Ok(true);
                } else {
                    log::info!("App is up to date: {}", &local[..7.min(local.len())]);
                    self.update_available = false;
                    return Ok(false);
                }
            } else {
                log::error!("Unexpected git output format. Got {} lines", lines.len());
                return Err(anyhow::anyhow!("Unexpected git output format"));
            }
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Update checking only available on Linux");
            Ok(false)
        }
    }
    
    /// Pull updates and restart the systemd service
    pub fn pull_and_restart_service(&self, service_name: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            log::info!("Starting update process for service: {}", service_name);
            
            // Spawn the background update script that will:
            // 1. Wait for app to exit
            // 2. Pull updates
            // 3. Rebuild the binary
            // 4. Restart the service
            let update_script = format!("{}/deployment/update_and_rebuild.sh", self.project_path);
            
            log::info!("Spawning background update script: {}", update_script);
            Command::new("sh")
                .args(&[&update_script, &self.project_path, service_name])
                .spawn()?;
            
            log::info!("Background update spawned - exiting app to allow rebuild");
            
            // Exit immediately to allow the background script to rebuild
            std::process::exit(0);
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("Service restart only available on Linux");
            Ok(())
        }
    }
}
