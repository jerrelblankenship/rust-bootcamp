// Exercise 05: Cross-Platform Chaos - Fix platform-specific issues!
//
// This exercise has code that only works on some platforms.
// It uses platform-specific crates or approaches that break portability.
//
// Your mission: Make this code work on Windows, macOS, and Linux!

// Expected: Cross-platform file operations, path handling, and process management
// Currently: Platform-specific code that breaks on different systems

use std::process::Command;
use std::path::Path;

// Problem: Platform-specific path separators and commands
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running cross-platform operations...");
    
    // BAD: Hard-coded Unix path separators
    let config_path = "/home/user/.config/myapp/config.toml";
    let log_path = "/var/log/myapp.log";
    
    // BAD: Unix-specific commands
    let output = Command::new("ls")
        .arg("-la")
        .arg("/tmp")
        .output()?;
    
    println!("Directory listing: {}", String::from_utf8_lossy(&output.stdout));
    
    // BAD: Platform-specific file operations
    create_config_file(config_path)?;
    setup_log_directory(log_path)?;
    
    // BAD: Unix-specific process management
    let _pid = unsafe { libc::getpid() };
    
    Ok(())
}

fn create_config_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // BAD: Assumes Unix-style paths
    let parent = Path::new(path).parent().unwrap();
    
    // BAD: Unix-specific directory creation
    Command::new("mkdir")
        .arg("-p")
        .arg(parent)
        .output()?;
    
    // BAD: Unix-specific file creation
    Command::new("touch")
        .arg(path)
        .output()?;
    
    Ok(())
}

fn setup_log_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // BAD: Hard-coded Unix path
    let log_dir = "/var/log";
    
    // BAD: Unix-specific permissions
    Command::new("chmod")
        .arg("755")
        .arg(log_dir)
        .output()?;
    
    Ok(())
}

// Cargo.toml for this exercise (platform-specific dependencies):
/*
[package]
name = "ex05-cross-platform"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"  # Only works on Unix-like systems

# Better approach would use cross-platform crates:
# - dirs for standard directories
# - which for finding executables
# - sysinfo for system information
# - std::fs for file operations
# - std::env for environment variables

[target.'cfg(unix)'.dependencies]
# Unix-specific dependencies

[target.'cfg(windows)'.dependencies]
# Windows-specific dependencies
*/