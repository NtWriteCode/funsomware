fn main() {
    // Only compile resources for Windows targets
    #[cfg(target_os = "windows")]
    {
        // Use winres crate to compile the .rc file and embed the icon
        let mut res = winres::WindowsResource::new();
        res.set_icon("rsrc/icon.ico");
        
        // Compile the resource
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
            eprintln!("The executable will be built without an icon.");
        }
    }
}
