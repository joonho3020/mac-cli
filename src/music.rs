//! Apple Music control for macOS using AppleScript.
//!
//! This module provides an interface to control Apple Music playback,
//! including play/pause, track navigation, and playlist management.

use std::process::Command;

/// Controller for Apple Music on macOS.
///
/// Uses AppleScript to control Apple Music playback and playlist management.
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

    /// Plays the current track in Apple Music.
    pub fn play() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to play")?;
        Ok(())
    }

    /// Pauses the current playback in Apple Music.
    pub fn pause() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to pause")?;
        Ok(())
    }

    /// Skips to the next track in Apple Music.
    pub fn next() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to next track")?;
        Ok(())
    }

    /// Goes to the previous track in Apple Music.
    pub fn previous() -> Result<(), String> {
        Self::run_script("tell application \"Music\" to previous track")?;
        Ok(())
    }

    /// Gets information about the currently playing track.
    ///
    /// # Returns
    ///
    /// Returns a string in the format "Track Name - Artist Name" if playing,
    /// or "Not playing" if nothing is currently playing.
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

    /// Lists all available playlists in Apple Music.
    ///
    /// # Returns
    ///
    /// Returns a vector of playlist names.
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

    /// Plays a specific playlist by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the playlist to play.
    pub fn play_playlist(name: &str) -> Result<(), String> {
        let script = format!(r#"tell application "Music" to play playlist named "{}""#, name);
        Self::run_script(&script)?;
        Ok(())
    }

    /// Displays an interactive playlist picker using fzf and plays the selected playlist.
    ///
    /// # Returns
    ///
    /// Returns the name of the selected playlist.
    ///
    /// # Errors
    ///
    /// Returns an error if fzf is not installed or if no playlist is selected.
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
