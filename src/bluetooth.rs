use std::process::Command;

pub struct BluetoothController;

impl BluetoothController {
    pub fn list_devices() -> Result<String, String> {
        // Use system_profiler to get Bluetooth device info
        let output = Command::new("system_profiler")
            .arg("SPBluetoothDataType")
            .arg("-json")
            .output()
            .map_err(|e| format!("Failed to execute system_profiler: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "system_profiler error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn list_devices_simple() -> Result<Vec<String>, String> {
        // Simple approach: parse the output to get device names
        let output = Command::new("system_profiler")
            .arg("SPBluetoothDataType")
            .output()
            .map_err(|e| format!("Failed to execute system_profiler: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "system_profiler error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        // Parse connected devices
        for line in output_str.lines() {
            let trimmed = line.trim();
            // Look for device entries (they're typically indented and followed by a colon)
            if trimmed.ends_with(':') && !trimmed.starts_with("Bluetooth") && trimmed.len() > 1 {
                // Remove the trailing colon
                let device_name = trimmed.trim_end_matches(':').to_string();
                // Filter out common section headers and status indicators
                if !device_name.contains("Devices")
                    && !device_name.contains("Services")
                    && !device_name.contains("Controller")
                    && device_name != "Connected"
                    && device_name != "Not Connected"
                    && device_name != "Paired"
                    && device_name != "Not Paired"
                {
                    devices.push(device_name);
                }
            }
        }

        Ok(devices)
    }
}
