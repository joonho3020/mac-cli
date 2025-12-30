use std::process::Command;

pub struct VolumeController;

impl VolumeController {
    pub fn new() -> Result<Self, String> {
        Ok(VolumeController)
    }

    fn run_script(script: &str) -> Result<String, String> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "AppleScript error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn get(&self) -> Result<f32, String> {
        let script = "output volume of (get volume settings)";
        let result = Self::run_script(script)?;

        let volume = result
            .parse::<f32>()
            .map_err(|_| "Failed to parse volume".to_string())?;

        // AppleScript returns 0-100, convert to 0.0-1.0
        Ok(volume / 100.0)
    }

    pub fn set(&self, volume: f32) -> Result<(), String> {
        if !(0.0..=1.0).contains(&volume) {
            return Err("Volume must be between 0.0 and 1.0".to_string());
        }

        // Convert to 0-100 for AppleScript
        let volume_pct = (volume * 100.0) as i32;
        let script = format!("set volume output volume {}", volume_pct);
        Self::run_script(&script)?;

        Ok(())
    }
}
