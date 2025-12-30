//! Weather information retrieval using wttr.in service.
//!
//! This module provides an interface to fetch current weather information
//! for a given location or auto-detected location using the wttr.in API.

use std::process::Command;

/// Controller for fetching weather information.
///
/// Uses the wttr.in service to retrieve weather data without requiring API keys.
pub struct WeatherController;

impl WeatherController {
    /// Gets current weather information for a location.
    ///
    /// # Arguments
    ///
    /// * `location` - Optional location string (e.g., "San Francisco" or "London, UK").
    ///                If None, the location is auto-detected based on IP address.
    ///
    /// # Returns
    ///
    /// Returns a formatted weather string including location, conditions, and temperature in Celsius.
    pub fn get_weather(location: Option<&str>) -> Result<String, String> {
        // Use wttr.in service which provides weather info without API keys
        // The 'm' parameter ensures metric units (Celsius)
        let url = if let Some(loc) = location {
            format!("https://wttr.in/{}?format=3&m", loc.replace(' ', "+"))
        } else {
            // Auto-detect location
            "https://wttr.in/?format=3&m".to_string()
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
