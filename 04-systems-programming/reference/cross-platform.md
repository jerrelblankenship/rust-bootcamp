# Cross-Platform Systems Programming in Rust 🌐

## Platform Abstractions

### C# Cross-Platform Approach
```csharp
// C# handles platform differences automatically
if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows)) {
    // Windows-specific code
} else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux)) {
    // Linux-specific code  
} else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX)) {
    // macOS-specific code
}
```

### Rust Cross-Platform Patterns
```rust
// Conditional compilation at build time
#[cfg(target_os = "windows")]
mod platform {
    pub fn get_system_info() -> SystemInfo {
        // Windows WinAPI implementation
    }
}

#[cfg(target_os = "linux")]
mod platform {
    pub fn get_system_info() -> SystemInfo {
        // Linux /proc filesystem implementation
    }
}

#[cfg(target_os = "macos")]
mod platform {
    pub fn get_system_info() -> SystemInfo {
        // macOS sysctl implementation
    }
}

// Common interface
pub use platform::get_system_info;
```

## File System Differences

### Path Handling
```rust
use std::path::{Path, PathBuf};

// Wrong: Hardcoded path separators
let bad_path = "/proc/meminfo";  // Only works on Unix

// Right: Cross-platform path construction
let good_path = Path::new("proc").join("meminfo");

// Platform-specific paths
#[cfg(unix)]
const PROC_MEMINFO: &str = "/proc/meminfo";

#[cfg(windows)]
const PROC_MEMINFO: &str = ""; // Not available on Windows
```

### Directory Locations
```rust
use dirs;

// C# equivalent: Environment.GetFolderPath()
let config_dir = dirs::config_dir()
    .ok_or("Could not find config directory")?;

let app_config = config_dir.join("myapp").join("config.toml");
```

## System API Differences

### Memory Information
```rust
// Platform-specific implementations
#[cfg(target_os = "linux")]
pub fn get_memory_info() -> Result<MemoryInfo, SystemError> {
    // Parse /proc/meminfo
    let content = std::fs::read_to_string("/proc/meminfo")?;
    parse_proc_meminfo(&content)
}

#[cfg(target_os = "windows")]
pub fn get_memory_info() -> Result<MemoryInfo, SystemError> {
    // Use Windows API
    unsafe {
        let mut info = MEMORYSTATUSEX::default();
        info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        
        if GlobalMemoryStatusEx(&mut info) != 0 {
            Ok(MemoryInfo {
                total: info.ullTotalPhys,
                available: info.ullAvailPhys,
                // ...
            })
        } else {
            Err(SystemError::WindowsApi(GetLastError()))
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_memory_info() -> Result<MemoryInfo, SystemError> {
    // Use macOS sysctl
    use std::mem;
    use libc::{sysctl, CTL_HW, HW_MEMSIZE};
    
    let mut size = 0u64;
    let mut len = mem::size_of::<u64>();
    let mut mib = [CTL_HW, HW_MEMSIZE];
    
    unsafe {
        if sysctl(
            mib.as_mut_ptr(),
            2,
            &mut size as *mut _ as *mut _,
            &mut len,
            std::ptr::null_mut(),
            0
        ) == 0 {
            Ok(MemoryInfo {
                total: size,
                // Need additional syscalls for other info...
            })
        } else {
            Err(SystemError::Sysctl)
        }
    }
}
```

## Build Configuration

### Cargo.toml Platform Dependencies
```toml
[dependencies]
# Cross-platform dependencies
serde = "1.0"
tokio = "1.0"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["memoryapi", "sysinfoapi"] }

[target.'cfg(unix)'.dependencies]
libc = "0.2"

[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"
```

### Conditional Features
```toml
[features]
default = ["system-monitor"]
system-monitor = []
windows-service = []

[target.'cfg(windows)'.dependencies]
windows-service = { version = "0.4", optional = true }
```

## Testing Cross-Platform Code

### Platform-Specific Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(unix)]
    fn test_proc_parsing() {
        // Test /proc filesystem parsing
        let sample = "MemTotal: 16384000 kB\nMemFree: 8192000 kB\n";
        let info = parse_proc_meminfo(sample).unwrap();
        assert_eq!(info.total, 16384000 * 1024);
    }
    
    #[test]
    #[cfg(windows)]
    fn test_windows_api() {
        // Test Windows API calls
        let info = get_memory_info().unwrap();
        assert!(info.total > 0);
    }
    
    #[test]
    fn test_cross_platform_interface() {
        // Test that works on all platforms
        let info = get_memory_info().unwrap();
        assert!(info.total > 0);
        assert!(info.available <= info.total);
    }
}
```

## Deployment Considerations

### Binary Distribution
```bash
# Build for multiple targets
cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-unknown-linux-gnu
cargo build --target x86_64-apple-darwin

# Use cross for easier cross-compilation
cargo install cross
cross build --target x86_64-pc-windows-gnu
```

### Runtime Detection
```rust
pub fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        family: std::env::consts::FAMILY,
    }
}

// Runtime capability detection
pub fn supports_feature(feature: &str) -> bool {
    match feature {
        "proc_filesystem" => cfg!(unix),
        "windows_api" => cfg!(windows),
        "sysctl" => cfg!(target_os = "macos"),
        _ => false,
    }
}
```

## Best Practices

### 1. Abstract Early
```rust
// Define common interface first
pub trait SystemInfo {
    fn get_memory(&self) -> Result<MemoryInfo, Error>;
    fn get_cpu(&self) -> Result<CpuInfo, Error>;
}

// Platform-specific implementations
struct LinuxSystemInfo;
struct WindowsSystemInfo;
struct MacSystemInfo;

// Factory function
pub fn create_system_info() -> Box<dyn SystemInfo> {
    #[cfg(target_os = "linux")]
    return Box::new(LinuxSystemInfo);
    
    #[cfg(target_os = "windows")]
    return Box::new(WindowsSystemInfo);
    
    #[cfg(target_os = "macos")]
    return Box::new(MacSystemInfo);
}
```

### 2. Graceful Degradation
```rust
pub fn get_cpu_usage() -> Option<f64> {
    #[cfg(unix)]
    {
        match parse_proc_stat() {
            Ok(usage) => Some(usage),
            Err(_) => None,
        }
    }
    
    #[cfg(not(unix))]
    {
        // Feature not available on this platform
        None
    }
}
```

### 3. Error Handling
```rust
#[derive(Debug)]
pub enum SystemError {
    NotSupported(String),
    
    #[cfg(unix)]
    ProcFs(std::io::Error),
    
    #[cfg(windows)]
    WindowsApi(u32),
    
    #[cfg(target_os = "macos")]
    Sysctl(String),
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SystemError::NotSupported(feature) => {
                write!(f, "Feature '{}' not supported on this platform", feature)
            }
            #[cfg(unix)]
            SystemError::ProcFs(e) => write!(f, "Failed to read /proc: {}", e),
            #[cfg(windows)]
            SystemError::WindowsApi(code) => write!(f, "Windows API error: {}", code),
            #[cfg(target_os = "macos")]
            SystemError::Sysctl(msg) => write!(f, "sysctl error: {}", msg),
        }
    }
}
```

Cross-platform systems programming in Rust gives you the best of both worlds: platform-specific optimization when needed, and portable code by default.