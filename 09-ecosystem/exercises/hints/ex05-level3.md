# Exercise 05 - Level 3 Hints: Cross-Platform Chaos

## 🎯 Complete Cross-Platform Solution

```rust
use std::fs;
use std::path::PathBuf;
use std::process;
use dirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running cross-platform operations...");
    
    // Cross-platform paths
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("myapp");
    let config_path = config_dir.join("config.toml");
    
    let log_dir = dirs::cache_dir()
        .ok_or("Could not find cache directory")?
        .join("myapp");
    let log_path = log_dir.join("myapp.log");
    
    // Cross-platform directory listing
    let temp_dir = std::env::temp_dir();
    let entries = fs::read_dir(&temp_dir)?;
    
    println!("Temp directory contents:");
    for entry in entries {
        let entry = entry?;
        println!("  {:?}", entry.file_name());
    }
    
    // Cross-platform file operations
    create_config_file(&config_path)?;
    setup_log_directory(&log_dir)?;
    
    // Cross-platform process ID
    let pid = process::id();
    println!("Process ID: {}", pid);
    
    Ok(())
}

fn create_config_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Create parent directories
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Create file
    fs::File::create(path)?;
    
    Ok(())
}

fn setup_log_directory(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Create directory with proper permissions
    fs::create_dir_all(path)?;
    
    // Set permissions (cross-platform)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
    
    Ok(())
}
```

## 🔧 Working Cargo.toml

```toml
[package]
name = "ex05-cross-platform"
version = "0.1.0"
edition = "2021"

[dependencies]
dirs = "5.0"
```

## 🎯 What Changed

1. **Portable paths**: Using `dirs` crate and `PathBuf`
2. **Standard file operations**: `std::fs` instead of shell commands
3. **Cross-platform process info**: `std::process::id()`
4. **Conditional compilation**: `#[cfg(unix)]` for platform-specific code

## 🏆 Success Criteria

Your program should work on Windows, macOS, and Linux!