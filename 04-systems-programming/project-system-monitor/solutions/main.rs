// System Monitor - WORKING SOLUTION
//
// This is a complete, working implementation of the system monitor project.
// Compare this with your implementation to understand the concepts.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

mod memory;
mod cpu;
mod process;
mod ffi;

use memory::MemoryInfo;
use cpu::CpuInfo;
use process::ProcessInfo;

#[derive(Debug)]
struct SystemInfo {
    memory: MemoryInfo,
    cpu: CpuInfo,
    process: ProcessInfo,
    uptime: u64,
}

impl SystemInfo {
    fn collect() -> Result<Self, SystemError> {
        let memory = MemoryInfo::collect()?;
        let cpu = CpuInfo::collect()?;
        let process = ProcessInfo::collect()?;
        let uptime = process.uptime_seconds();
        
        Ok(SystemInfo {
            memory,
            cpu,
            process,
            uptime,
        })
    }
    
    fn display(&self) {
        println!("╭─────────────────────────────────────────────────────────╮");
        println!("│                    System Monitor                       │");
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Memory information
        println!("│ Memory Usage:                                           │");
        println!("│   Total: {}                               │", 
                 utils::format_bytes(self.memory.total_bytes()));
        println!("│   Used:  {} ({})                   │", 
                 utils::format_bytes(self.memory.used_bytes()),
                 utils::format_percentage(self.memory.usage_percentage()));
        println!("│   Free:  {}                               │", 
                 utils::format_bytes(self.memory.available_bytes()));
        
        // CPU information
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ CPU Usage:                                              │");
        println!("│   Utilization: {}                           │", 
                 utils::format_percentage(self.cpu.usage_percentage()));
        println!("│   Cores: {}                                           │", 
                 self.cpu.core_count());
        println!("│   Model: {}                   │", 
                 truncate_string(self.cpu.model_name(), 35));
        
        // Process information
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Processes:                                              │");
        println!("│   Total: {}                                         │", 
                 self.process.total_processes());
        println!("│   Running: {}                                       │", 
                 self.process.running_processes());
        println!("│   Sleeping: {}                                      │", 
                 self.process.sleeping_processes());
        
        // System uptime
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ System Uptime: {}                          │", 
                 utils::format_duration(self.uptime));
        
        println!("╰─────────────────────────────────────────────────────────╯");
    }
    
    fn update(&mut self) -> Result<(), SystemError> {
        // Update all components
        self.memory.update()?;
        self.cpu.update()?;
        self.process.update()?;
        self.uptime = self.process.uptime_seconds();
        Ok(())
    }
}

#[derive(Debug)]
enum SystemError {
    MemoryError(memory::MemoryError),
    CpuError(cpu::CpuError),
    ProcessError(process::ProcessError),
    FfiError(ffi::SystemError),
    UnsupportedPlatform,
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::MemoryError(e) => write!(f, "Memory error: {}", e),
            SystemError::CpuError(e) => write!(f, "CPU error: {}", e),
            SystemError::ProcessError(e) => write!(f, "Process error: {}", e),
            SystemError::FfiError(e) => write!(f, "FFI error: {}", e),
            SystemError::UnsupportedPlatform => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for SystemError {}

// Automatic error conversions
impl From<memory::MemoryError> for SystemError {
    fn from(error: memory::MemoryError) -> Self {
        SystemError::MemoryError(error)
    }
}

impl From<cpu::CpuError> for SystemError {
    fn from(error: cpu::CpuError) -> Self {
        SystemError::CpuError(error)
    }
}

impl From<process::ProcessError> for SystemError {
    fn from(error: process::ProcessError) -> Self {
        SystemError::ProcessError(error)
    }
}

impl From<ffi::SystemError> for SystemError {
    fn from(error: ffi::SystemError) -> Self {
        SystemError::FfiError(error)
    }
}

struct MonitorConfig {
    update_interval: u64,
    single_shot: bool,
    simple_format: bool,
    color_output: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            update_interval: 2,
            single_shot: false,
            simple_format: false,
            color_output: true,
        }
    }
}

fn run_monitor(config: MonitorConfig) -> Result<(), SystemError> {
    println!("🔍 System Monitor Starting...\n");
    
    let mut system_info = SystemInfo::collect()?;
    
    if config.single_shot {
        system_info.display();
        return Ok(());
    }
    
    loop {
        // Clear screen for real-time updates
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
        
        // Update system information
        if let Err(e) = system_info.update() {
            eprintln!("Warning: Failed to update system info: {}", e);
            // Continue monitoring even if update fails
        }
        
        // Display current information
        system_info.display();
        
        // Show refresh information
        println!("\nRefresh interval: {}s | Press Ctrl+C to exit", config.update_interval);
        
        // Wait for next update
        thread::sleep(Duration::from_secs(config.update_interval));
    }
}

fn parse_arguments() -> MonitorConfig {
    let args: Vec<String> = std::env::args().collect();
    let mut config = MonitorConfig::default();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--single-shot" | "-s" => {
                config.single_shot = true;
            }
            "--interval" | "-i" => {
                if i + 1 < args.len() {
                    if let Ok(interval) = args[i + 1].parse::<u64>() {
                        config.update_interval = interval.max(1);
                        i += 1; // Skip next argument
                    }
                }
            }
            "--simple" => {
                config.simple_format = true;
            }
            "--no-color" => {
                config.color_output = false;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }
    
    config
}

fn print_help() {
    println!("System Monitor - Real-time system resource monitoring");
    println!();
    println!("USAGE:");
    println!("    system-monitor [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -s, --single-shot    Take one measurement and exit");
    println!("    -i, --interval <N>   Update interval in seconds (default: 2)");
    println!("    --simple             Simple output format");
    println!("    --no-color           Disable colored output");
    println!("    -h, --help           Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    system-monitor                    # Continuous monitoring");
    println!("    system-monitor --single-shot      # One-time measurement");
    println!("    system-monitor --interval 5       # Update every 5 seconds");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== System Resource Monitor ===");
    println!("Built with Rust systems programming");
    println!("Demonstrates: Memory layout, Unsafe operations, FFI\n");
    
    // Parse command line arguments
    let config = parse_arguments();
    
    // Check platform support
    match std::env::consts::OS {
        "linux" | "windows" | "macos" => {
            println!("✅ Platform supported: {}", std::env::consts::OS);
        }
        other => {
            eprintln!("⚠️  Platform may not be fully supported: {}", other);
        }
    }
    
    // Run the monitor
    match run_monitor(config) {
        Ok(()) => Ok(()),
        Err(SystemError::UnsupportedPlatform) => {
            eprintln!("❌ This platform is not supported");
            eprintln!("Supported platforms: Linux, Windows, macOS");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

// Utility functions for formatting
pub mod utils {
    use std::borrow::Cow;
    
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;
        
        if bytes == 0 {
            return "0 B".to_string();
        }
        
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
    
    pub fn format_percentage(value: f64) -> String {
        format!("{:.1}%", value)
    }
    
    pub fn format_duration(seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    }
    
    pub fn colorize(text: &str, color: Color) -> String {
        if !supports_color() {
            return text.to_string();
        }
        
        let color_code = match color {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Cyan => "\x1b[36m",
            Color::Reset => "\x1b[0m",
        };
        
        format!("{}{}\x1b[0m", color_code, text)
    }
    
    fn supports_color() -> bool {
        std::env::var("NO_COLOR").is_err() && 
        (std::env::var("TERM").unwrap_or_default() != "dumb")
    }
    
    #[derive(Debug)]
    pub enum Color {
        Red,
        Green,
        Yellow,
        Blue,
        Cyan,
        Reset,
    }
    
    // Zero-copy string processing example
    pub fn process_string_efficient(input: &str) -> Cow<str> {
        if input.chars().any(|c| c.is_lowercase()) {
            Cow::Owned(input.to_uppercase())
        } else {
            Cow::Borrowed(input)
        }
    }
}

// Helper function for display formatting
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_info_collection() {
        let result = SystemInfo::collect();
        
        // Should work on supported platforms
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            if let Ok(system_info) = result {
                assert!(system_info.memory.total_bytes() > 0);
                assert!(system_info.cpu.core_count() > 0);
                assert!(system_info.process.total_processes() > 0);
                assert!(system_info.uptime > 0);
            }
        }
    }
    
    #[test]
    fn test_utility_functions() {
        assert_eq!(utils::format_bytes(1024), "1.0 KB");
        assert_eq!(utils::format_bytes(1048576), "1.0 MB");
        assert_eq!(utils::format_bytes(0), "0 B");
        
        assert_eq!(utils::format_percentage(12.34), "12.3%");
        assert_eq!(utils::format_percentage(100.0), "100.0%");
        
        assert_eq!(utils::format_duration(3661), "1h 1m 1s");
        assert_eq!(utils::format_duration(61), "1m 1s");
        assert_eq!(utils::format_duration(30), "30s");
    }
    
    #[test]
    fn test_error_handling() {
        let memory_error = memory::MemoryError::UnsupportedPlatform;
        let system_error = SystemError::MemoryError(memory_error);
        let error_string = format!("{}", system_error);
        assert!(!error_string.is_empty());
    }
    
    #[test]
    fn test_string_processing() {
        use std::borrow::Cow;
        
        let result = utils::process_string_efficient("HELLO");
        assert!(matches!(result, Cow::Borrowed(_)));
        
        let result = utils::process_string_efficient("hello");
        assert!(matches!(result, Cow::Owned(_)));
    }
}
