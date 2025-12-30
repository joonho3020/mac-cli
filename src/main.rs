//! # mac-cli
//!
//! A command-line tool for controlling macOS system features including brightness,
//! volume, Apple Music, Bluetooth, and weather information.
//!
//! ## Features
//!
//! - **Brightness**: Get and set screen brightness (10-100%)
//! - **Volume**: Control system volume (0-100%)
//! - **Apple Music**: Play/pause, skip tracks, manage playlists
//! - **Bluetooth**: List paired and connected devices
//! - **Weather**: Get current weather for any location

mod brightness;
mod bluetooth;
mod music;
mod volume;
mod weather;

use brightness::BrightnessController;
use bluetooth::BluetoothController;
use clap::{Parser, Subcommand};
use music::MusicController;
use volume::VolumeController;
use weather::WeatherController;

/// macOS system control utility - control brightness, volume, music, Bluetooth, and weather
#[derive(Parser, Debug)]
#[command(name = "mac")]
#[command(about = "Control macOS system features and get weather info", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Control screen brightness (10-100%)
    Brightness {
        /// Brightness percentage to set (10-100). If not provided, shows current brightness
        percentage: Option<f32>,
    },

    /// Control system volume (0-100%)
    Volume {
        /// Volume percentage to set (0-100). If not provided, shows current volume
        percentage: Option<f32>,
    },

    /// Control Apple Music
    #[command(subcommand)]
    Music(MusicCommands),

    /// List Bluetooth devices
    Bluetooth,

    /// Get current weather
    Weather {
        /// Location (city, country). If not provided, auto-detects location
        location: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum MusicCommands {
    /// Play current track
    Play,
    /// Pause playback
    Pause,
    /// Next track
    Next,
    /// Previous track
    Previous,
    /// Show current track
    Current,
    /// List or play playlists
    Playlists {
        /// Playlist name to play directly
        name: Option<String>,

        /// Just list playlists without playing
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Brightness { percentage } => handle_brightness(percentage),
        Commands::Volume { percentage } => handle_volume(percentage),
        Commands::Music(music_cmd) => handle_music(music_cmd),
        Commands::Bluetooth => handle_bluetooth(),
        Commands::Weather { location } => handle_weather(location),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_brightness(percentage: Option<f32>) -> Result<(), String> {
    let controller = BrightnessController::new()?;

    match percentage {
        Some(pct) => {
            if pct == 0.0 {
                return Err("Brightness cannot be 0".to_string());
            }
            if pct < 10.0 || pct > 100.0 {
                return Err("Brightness must be between 10 and 100".to_string());
            }
            controller.set(pct / 100.0)?;
            println!("Brightness set to {:.0}%", pct);
        }
        None => {
            let brightness = controller.get()?;
            println!("{:.0}%", brightness * 100.0);
        }
    }

    Ok(())
}

fn handle_volume(percentage: Option<f32>) -> Result<(), String> {
    let controller = VolumeController::new()?;

    match percentage {
        Some(pct) => {
            if pct < 0.0 || pct > 100.0 {
                return Err("Volume must be between 0 and 100".to_string());
            }
            controller.set(pct / 100.0)?;
            println!("Volume set to {:.0}%", pct);
        }
        None => {
            let volume = controller.get()?;
            println!("{:.0}%", volume * 100.0);
        }
    }

    Ok(())
}

fn handle_music(cmd: MusicCommands) -> Result<(), String> {
    match cmd {
        MusicCommands::Play => {
            MusicController::play()?;
            println!("Playing");
        }
        MusicCommands::Pause => {
            MusicController::pause()?;
            println!("Paused");
        }
        MusicCommands::Next => {
            MusicController::next()?;
            println!("Next track");
        }
        MusicCommands::Previous => {
            MusicController::previous()?;
            println!("Previous track");
        }
        MusicCommands::Current => {
            let info = MusicController::current()?;
            println!("{}", info);
        }
        MusicCommands::Playlists { name, list } => {
            if list {
                // Just list playlists
                let playlists = MusicController::list_playlists()?;
                if playlists.is_empty() {
                    println!("No playlists found");
                } else {
                    println!("Playlists:");
                    for playlist in playlists {
                        println!("  - {}", playlist);
                    }
                }
            } else {
                match name {
                    Some(playlist_name) => {
                        // Play specific playlist
                        MusicController::play_playlist(&playlist_name)?;
                        println!("Playing playlist: {}", playlist_name);

                        // Show current track after a brief moment
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        match MusicController::current() {
                            Ok(info) => println!("Now playing: {}", info),
                            Err(_) => {} // Ignore error if track info not available
                        }
                    }
                    None => {
                        // Interactive mode with fzf
                        match MusicController::play_playlist_interactive() {
                            Ok(selected) => {
                                println!("Playing playlist: {}", selected);
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                match MusicController::current() {
                                    Ok(info) => println!("Now playing: {}", info),
                                    Err(_) => {}
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_bluetooth() -> Result<(), String> {
    let devices = BluetoothController::list_devices_simple()?;

    if devices.is_empty() {
        println!("No Bluetooth devices found");
    } else {
        println!("Bluetooth Devices:");
        for device in devices {
            println!("  - {}", device);
        }
    }

    Ok(())
}

fn handle_weather(location: Option<String>) -> Result<(), String> {
    let location_ref = location.as_deref();
    let weather = WeatherController::get_weather(location_ref)?;
    println!("{}", weather);

    Ok(())
}
