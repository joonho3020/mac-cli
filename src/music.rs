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

    pub fn list_playlists() -> Result<Vec<String>, String> {
        let script = r#"
            tell application "Music"
                set playlistNames to name of playlists
                return playlistNames
            end tell
        "#;

        let result = Self::run_script(script)?;

        // AppleScript returns comma-separated list
        let playlists: Vec<String> = result
            .split(", ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(playlists)
    }

    pub fn play_playlist(name: &str) -> Result<(), String> {
        let script = format!(r#"tell application "Music" to play playlist named "{}""#, name);
        Self::run_script(&script)?;
        Ok(())
    }

    pub fn play_playlist_interactive() -> Result<String, String> {
        use std::io::Write;

        let playlists = Self::list_playlists()?;

        if playlists.is_empty() {
            return Err("No playlists found".to_string());
        }

        // Use fzf for interactive selection
        let input = playlists.join("\n");

        let mut child = Command::new("fzf")
            .arg("--prompt=Select playlist: ")
            .arg("--height=40%")
            .arg("--reverse")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .map_err(|e| format!("Failed to start fzf (is it installed?): {}", e))?;

        // Write playlists to fzf stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .map_err(|e| format!("Failed to write to fzf: {}", e))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to read fzf output: {}", e))?;

        if !output.status.success() {
            return Err("No playlist selected".to_string());
        }

        let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if selected.is_empty() {
            return Err("No playlist selected".to_string());
        }

        Self::play_playlist(&selected)?;
        Ok(selected)
    }
}
