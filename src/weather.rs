use std::process::Command;

pub struct WeatherController;

impl WeatherController {
    pub fn get_weather(location: Option<&str>) -> Result<String, String> {
        // Use wttr.in service which provides weather info without API keys
        let url = if let Some(loc) = location {
            format!("https://wttr.in/{}?format=3", loc.replace(' ', "+"))
        } else {
            // Auto-detect location
            "https://wttr.in/?format=3".to_string()
        };

        // Use curl to fetch weather data
        let output = Command::new("curl")
            .arg("-s") // silent mode
            .arg(&url)
            .output()
            .map_err(|e| format!("Failed to execute curl: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to fetch weather data: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let weather = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if weather.is_empty() {
            return Err("No weather data received".to_string());
        }

        Ok(weather)
    }
}
