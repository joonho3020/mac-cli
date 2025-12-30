mod brightness;
mod bluetooth;
mod music;
mod volume;

use brightness::BrightnessController;
use bluetooth::BluetoothController;
use clap::{Parser, Subcommand};
use music::MusicController;
use volume::VolumeController;

/// macOS system control utility - control brightness, volume, music, and Bluetooth
#[derive(Parser, Debug)]
#[command(name = "mac-control")]
#[command(about = "Control macOS system features", long_about = None)]
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
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Brightness { percentage } => handle_brightness(percentage),
        Commands::Volume { percentage } => handle_volume(percentage),
        Commands::Music(music_cmd) => handle_music(music_cmd),
        Commands::Bluetooth => handle_bluetooth(),
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
