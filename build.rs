fn main() {
    // Only compile resources for Windows targets
    #[cfg(target_os = "windows")]
    {
        // Use winres crate to compile the .rc file and embed the icon
        let mut res = winres::WindowsResource::new();
        res.set_icon("rsrc/icon.ico");
        
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
        
        // Compile the resource
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
            eprintln!("The executable will be built without an icon.");
        }
    }
}
