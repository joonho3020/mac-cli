use std::process::Command;

pub struct MusicController;

impl MusicController {
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

    pub fn play() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to play")?;
        Ok(())
    }

    pub fn pause() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to pause")?;
        Ok(())
    }

    pub fn next() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to next track")?;
        Ok(())
    }

    pub fn previous() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to previous track")?;
        Ok(())
    }

    pub fn current() -> Result<String, String> {
        let script = r#"
            tell application "Music"
                if player state is playing then
                    set trackName to name of current track
                    set artistName to artist of current track
                    return trackName & " - " & artistName
                else
                    return "Not playing"
                end if
            end tell
        "#;

        Self::run_script(script)
    }

    pub fn is_playing() -> Result<bool, String> {
        let script = r#"tell application "Music" to return player state as string"#;
        let state = Self::run_script(script)?;
        Ok(state == "playing")
    }
}
