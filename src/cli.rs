use crate::{Result, monitor, monitor::Monitor, vcp};
use clap::{Parser, Subcommand};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "monitorconfig")]
#[command(author, version, about = "Native Windows CLI tool for managing monitor settings", long_about = None)]
pub struct Cli {
    /// Suppress all output (useful for background tasks and Task Scheduler)
    #[arg(short, long, global = true)]
    pub silent: bool,

    /// Wait this many seconds before executing the command (useful after resume from sleep).
    /// This delay happens inside the process, after the monitor handle is not yet acquired,
    /// so it is safer than a Task Scheduler action delay which races against monitor DDC/CI init.
    #[arg(short, long, global = true, value_name = "SECONDS", default_value = "0")]
    pub wait: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available monitors
    List {
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Get brightness level of a monitor
    GetBrightness {
        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Set brightness level of a monitor
    SetBrightness {
        /// Brightness value (0-100)
        value: u32,

        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,
    },

    /// Get contrast level of a monitor
    GetContrast {
        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Set contrast level of a monitor
    SetContrast {
        /// Contrast value (0-100)
        value: u32,

        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,
    },

    /// Get VCP feature value
    GetVcp {
        /// VCP code (e.g., 0x10 for brightness)
        #[arg(value_parser = parse_hex)]
        code: u8,

        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Set VCP feature value
    SetVcp {
        /// VCP code (e.g., 0x10 for brightness)
        #[arg(value_parser = parse_hex)]
        code: u8,

        /// Value to set
        value: u32,

        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,

        /// Number of times to retry if the command fails (e.g. monitor not ready after resume)
        #[arg(long, value_name = "COUNT", default_value = "0")]
        retry: u32,

        /// Seconds to wait between retry attempts
        #[arg(long, value_name = "SECONDS", default_value = "5")]
        retry_delay: u64,
    },

    /// List all VCP codes
    ListVcp {
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Scan monitor for all supported VCP codes
    ScanVcp {
        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Get monitor capabilities string
    GetCapabilities {
        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,
    },

    /// Save current monitor settings
    SaveSettings {
        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,
    },

    /// Reset monitor to factory defaults
    ResetDefaults {
        /// Device name or friendly name (e.g., \\.\DISPLAY1 or "Dell U2723DE") or use --primary
        #[arg(short, long)]
        device: Option<String>,

        /// Use primary monitor
        #[arg(short, long)]
        primary: bool,

        /// Only reset color settings
        #[arg(short, long)]
        color_only: bool,
    },
}

fn parse_hex(s: &str) -> std::result::Result<u8, String> {
    if let Some(stripped) = s.strip_prefix("0x") {
        u8::from_str_radix(stripped, 16).map_err(|e| e.to_string())
    } else {
        s.parse::<u8>().map_err(|e| e.to_string())
    }
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let silent = cli.silent;

    // Global startup delay — runs before any monitor handle is acquired, so the
    // DDC/CI subsystem has time to fully initialize after resume from sleep/hibernate.
    if cli.wait > 0 {
        thread::sleep(Duration::from_secs(cli.wait));
    }

    match cli.command {
        Commands::List { json } => list_monitors(json, silent),
        Commands::GetBrightness {
            device,
            primary,
            json,
        } => get_brightness(device, primary, json, silent),
        Commands::SetBrightness {
            value,
            device,
            primary,
        } => set_brightness(value, device, primary, silent),
        Commands::GetContrast {
            device,
            primary,
            json,
        } => get_contrast(device, primary, json, silent),
        Commands::SetContrast {
            value,
            device,
            primary,
        } => set_contrast(value, device, primary, silent),
        Commands::GetVcp {
            code,
            device,
            primary,
            json,
        } => get_vcp(code, device, primary, json, silent),
        Commands::SetVcp {
            code,
            value,
            device,
            primary,
            retry,
            retry_delay,
        } => set_vcp(code, value, device, primary, retry, retry_delay, silent),
        Commands::ListVcp { json } => list_vcp(json, silent),
        Commands::ScanVcp {
            device,
            primary,
            json,
        } => scan_vcp(device, primary, json, silent),
        Commands::GetCapabilities { device, primary } => get_capabilities(device, primary, silent),
        Commands::SaveSettings { device, primary } => save_settings(device, primary, silent),
        Commands::ResetDefaults {
            device,
            primary,
            color_only,
        } => reset_defaults(device, primary, color_only, silent),
    }
}

fn get_monitor(device: Option<String>, primary: bool) -> Result<monitor::PhysicalMonitor> {
    if primary {
        monitor::get_primary_monitor()
    } else if let Some(device_name) = device {
        monitor::find_monitor(&device_name)
    } else {
        monitor::get_primary_monitor()
    }
}

fn list_monitors(json: bool, silent: bool) -> Result<()> {
    let monitors = monitor::enumerate_monitors()?;

    if !silent {
        if json {
            let info: Vec<_> = monitors.iter().map(|m| m.info()).collect();
            println!("{}", serde_json::to_string_pretty(&info)?);
        } else {
            println!(
                "{:<20} {:<30} {}",
                "Device Name", "Friendly Name", "Primary"
            );
            println!("{}", "-".repeat(70));
            for mon in &monitors {
                let info = mon.info();
                println!(
                    "{:<20} {:<30} {}",
                    info.device_name,
                    info.friendly_name,
                    if info.is_primary { "Yes" } else { "" }
                );
            }
        }
    }

    Ok(())
}

fn get_brightness(device: Option<String>, primary: bool, json: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let brightness = mon.get_brightness()?;

    if !silent {
        if json {
            println!("{}", serde_json::to_string_pretty(&brightness)?);
        } else {
            println!(
                "Current brightness: {} (min: {}, max: {})",
                brightness.current, brightness.minimum, brightness.maximum
            );
        }
    }

    Ok(())
}

fn set_brightness(value: u32, device: Option<String>, primary: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    mon.set_brightness(value)?;
    if !silent {
        println!("Brightness set to {}", value);
    }
    Ok(())
}

fn get_contrast(device: Option<String>, primary: bool, json: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let contrast = mon.get_contrast()?;

    if !silent {
        if json {
            println!("{}", serde_json::to_string_pretty(&contrast)?);
        } else {
            println!(
                "Current contrast: {} (min: {}, max: {})",
                contrast.current, contrast.minimum, contrast.maximum
            );
        }
    }

    Ok(())
}

fn set_contrast(value: u32, device: Option<String>, primary: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    mon.set_contrast(value)?;
    if !silent {
        println!("Contrast set to {}", value);
    }
    Ok(())
}

fn get_vcp(
    code: u8,
    device: Option<String>,
    primary: bool,
    json: bool,
    silent: bool,
) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let vcp_mon = vcp::VcpMonitor::new(mon.handle());
    let response = vcp_mon.get_vcp_feature(code)?;

    if !silent {
        if json {
            println!("{}", serde_json::to_string_pretty(&response)?);
        } else {
            let info = vcp::get_vcp_code_info(code);
            if let Some(info) = info {
                println!("VCP Code: 0x{:02X} - {}", code, info.name);
                println!("Description: {}", info.description);
            } else {
                println!("VCP Code: 0x{:02X}", code);
            }
            println!(
                "Current value: {} (max: {})",
                response.current_value, response.maximum_value
            );
            println!("Type: {:?}", response.code_type);
        }
    }

    Ok(())
}

fn set_vcp(
    code: u8,
    value: u32,
    device: Option<String>,
    primary: bool,
    retry: u32,
    retry_delay: u64,
    silent: bool,
) -> Result<()> {
    let max_attempts = retry + 1;
    let mut last_err = None;

    for attempt in 1..=max_attempts {
        // Re-enumerate on every attempt — the monitor may not have been detected
        // on earlier attempts (e.g. display driver still initializing after resume).
        match try_set_vcp(code, value, device.as_deref(), primary) {
            Ok(()) => {
                if !silent {
                    println!("VCP code 0x{:02X} set to {}", code, value);
                }
                return Ok(());
            }
            Err(e) => {
                last_err = Some(e);
                if attempt < max_attempts {
                    if !silent {
                        eprintln!(
                            "Attempt {}/{} failed, retrying in {}s...",
                            attempt, max_attempts, retry_delay
                        );
                    }
                    // Wait between retries so the monitor DDC/CI controller has time
                    // to finish its post-power-on initialization sequence before the
                    // next I2C transaction is attempted.
                    thread::sleep(Duration::from_secs(retry_delay));
                }
            }
        }
    }

    Err(last_err.unwrap())
}

/// Single attempt at setting a VCP feature. Acquiring and releasing the physical
/// monitor handle is scoped tightly here so that a failed attempt does not leave
/// any handles open when the retry delay begins.
fn try_set_vcp(code: u8, value: u32, device: Option<&str>, primary: bool) -> Result<()> {
    let mon = get_monitor(device.map(|s| s.to_string()), primary)?;
    let vcp_mon = vcp::VcpMonitor::new(mon.handle());
    vcp_mon.set_vcp_feature(code, value)?;
    // `vcp_mon` drops here (no-op), then `mon` drops → DestroyPhysicalMonitor is
    // called cleanly before we return, ensuring no handle is held across the retry gap.
    Ok(())
}

fn list_vcp(json: bool, silent: bool) -> Result<()> {
    if !silent {
        if json {
            println!("{}", serde_json::to_string_pretty(&vcp::KNOWN_VCP_CODES)?);
        } else {
            println!("{:<6} {:<30} {}", "Code", "Name", "Description");
            println!("{}", "-".repeat(80));
            for info in vcp::KNOWN_VCP_CODES {
                println!(
                    "0x{:02X}   {:<30} {}",
                    info.code, info.name, info.description
                );
            }
        }
    }
    Ok(())
}

fn scan_vcp(device: Option<String>, primary: bool, json: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let vcp_mon = vcp::VcpMonitor::new(mon.handle());

    if !json && !silent {
        eprintln!("Scanning monitor for supported VCP codes...");
    }

    let features = vcp_mon.scan_vcp_features();

    if !silent {
        if json {
            println!("{}", serde_json::to_string_pretty(&features)?);
        } else {
            eprintln!("Found {} supported VCP codes\n", features.len());
            println!(
                "{:<6} {:<35} {:<12} {:<8} {}",
                "Code", "Name", "CurrentValue", "MaxValue", "Description"
            );
            println!("{}", "-".repeat(120));

            for response in features {
                let info = vcp::get_vcp_code_info(response.vcp_code);
                let name = info.map(|i| i.name).unwrap_or("Unknown");
                let description = info.map(|i| i.description).unwrap_or("");

                println!(
                    "0x{:02X}   {:<35} {:<12} {:<8} {}",
                    response.vcp_code,
                    name,
                    response.current_value,
                    response.maximum_value,
                    description
                );
            }
        }
    }

    Ok(())
}

fn get_capabilities(device: Option<String>, primary: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let vcp_mon = vcp::VcpMonitor::new(mon.handle());
    let caps = vcp_mon.get_capabilities()?;
    if !silent {
        println!("{}", caps);
    }
    Ok(())
}

fn save_settings(device: Option<String>, primary: bool, silent: bool) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let vcp_mon = vcp::VcpMonitor::new(mon.handle());
    vcp_mon.save_settings()?;
    if !silent {
        println!("Monitor settings saved");
    }
    Ok(())
}

fn reset_defaults(
    device: Option<String>,
    primary: bool,
    color_only: bool,
    silent: bool,
) -> Result<()> {
    let mon = get_monitor(device, primary)?;
    let vcp_mon = vcp::VcpMonitor::new(mon.handle());

    if color_only {
        vcp_mon.restore_factory_color_defaults()?;
        if !silent {
            println!("Monitor color settings reset to factory defaults");
        }
    } else {
        vcp_mon.restore_factory_defaults()?;
        if !silent {
            println!("Monitor reset to factory defaults");
        }
    }

    Ok(())
}
