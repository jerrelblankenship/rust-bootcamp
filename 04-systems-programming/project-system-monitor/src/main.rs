// System Monitor - Partially Working with Specific Bugs
//
// PARTIALLY WORKING CODE - FIX SPECIFIC BUGS!
//
// This project has a working structure but specific bugs to fix:
// 1. Display formatting functions are incomplete
// 2. Error handling needs From trait implementations  
// 3. Update loop has timing issues
// 4. Memory/CPU modules have parsing bugs
//
// APPROACH: Fix one bug at a time, compile, test, repeat

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// These modules exist but have bugs to fix
mod memory;
mod cpu;
mod process;

use memory::MemoryInfo;
use cpu::CpuInfo;
use process::ProcessInfo;

// Working structure - just needs complete implementation
#[derive(Debug)]
struct SystemInfo {
    memory: MemoryInfo,
    cpu: CpuInfo,
    process: ProcessInfo,
    uptime: u64,
}

impl SystemInfo {
    // Working method - just calls the modules
    fn collect() -> Result<Self, SystemError> {
        let memory = MemoryInfo::collect()?;
        let cpu = CpuInfo::collect()?;
        let process = ProcessInfo::collect()?;
        let uptime = Self::get_uptime()?;
        
        Ok(Self {
            memory,
            cpu, 
            process,
            uptime,
        })
    }
    
    // BUG FIX NEEDED: This display method is incomplete
    fn display(&self) {
        println!("🖥️  System Monitor");
        println!("==================");
        
        // TODO: Fix these formatting functions - they're incomplete!
        // self.display_memory();
        // self.display_cpu();
        // self.display_processes();
        // self.display_uptime();
        
        println!("Press Ctrl+C to exit...");
    }
    
    // BUG FIX NEEDED: These display methods need implementation
    fn display_memory(&self) {
        // TODO: Format memory information nicely
        // HINT: Use the memory info fields and format_bytes utility
        todo!("Implement memory display")
    }
    
    fn display_cpu(&self) {
        // TODO: Format CPU information nicely  
        // HINT: Show percentage with 1 decimal place
        todo!("Implement CPU display")
    }
    
    fn display_processes(&self) {
        // TODO: Format process count
        // HINT: Show total number of running processes
        todo!("Implement process display")
    }
    
    fn display_uptime(&self) {
        // TODO: Format uptime in human-readable form
        // HINT: Convert seconds to hours, minutes, seconds
        todo!("Implement uptime display")
    }
    
    // Working method for updates
    fn update(&mut self) -> Result<(), SystemError> {
        let new_info = Self::collect()?;
        *self = new_info;
        Ok(())
    }
    
    // Working platform-specific uptime
    fn get_uptime() -> Result<u64, SystemError> {
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let content = fs::read_to_string("/proc/uptime")?;
            let uptime_str = content.split_whitespace().next()
                .ok_or(SystemError::ParseError("Invalid uptime format".to_string()))?;
            let uptime_f64: f64 = uptime_str.parse()
                .map_err(|e| SystemError::ParseError(format!("Uptime parse error: {}", e)))?;
            Ok(uptime_f64 as u64)
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            // Return mock data for non-Linux platforms
            Ok(3600) // 1 hour mock uptime
        }
    }
}

// Working error type - just needs From implementations
#[derive(Debug)]
enum SystemError {
    IoError(io::Error),
    ParseError(String),
    MemoryError(memory::MemoryError),
    CpuError(cpu::CpuError), 
    ProcessError(process::ProcessError),
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::IoError(e) => write!(f, "IO error: {}", e),
            SystemError::ParseError(e) => write!(f, "Parse error: {}", e),
            SystemError::MemoryError(e) => write!(f, "Memory error: {}", e),
            SystemError::CpuError(e) => write!(f, "CPU error: {}", e),
            SystemError::ProcessError(e) => write!(f, "Process error: {}", e),
        }
    }
}

impl std::error::Error for SystemError {}

// Working From implementations
impl From<io::Error> for SystemError {
    fn from(error: io::Error) -> Self {
        SystemError::IoError(error)
    }
}

// BUG FIX NEEDED: These From implementations are missing
// TODO: Add From implementations for memory::MemoryError, cpu::CpuError, process::ProcessError
// impl From<memory::MemoryError> for SystemError { ... }

// Working configuration structure  
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

// Working monitoring loop
fn run_monitor(config: MonitorConfig) -> Result<(), SystemError> {
    println!("🔍 System Monitor Starting...\n");
    
    let mut system_info = SystemInfo::collect()?;
    
    if config.single_shot {
        system_info.display();
        return Ok(());
    }
    
    loop {
        // BUG FIX NEEDED: Screen clearing doesn't work properly
        // TODO: Fix screen clearing - current implementation is incomplete
        clear_screen();
        
        system_info.update()?;
        system_info.display();
        
        thread::sleep(Duration::from_secs(config.update_interval));
    }
}

// BUG FIX NEEDED: Screen clearing is incomplete
fn clear_screen() {
    // TODO: Implement proper screen clearing
    // HINT: Use ANSI escape codes or print newlines
    print!("\x1B[2J\x1B[1;1H");  // ANSI clear screen
    io::stdout().flush().unwrap();
}

// Working argument parsing
fn parse_arguments() -> MonitorConfig {
    let args: Vec<String> = std::env::args().collect();
    let mut config = MonitorConfig::default();
    
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--single-shot" => config.single_shot = true,
            "--simple" => config.simple_format = true,
            "--no-color" => config.color_output = false,
            "--interval" => {
                // Simple implementation - could be improved
                config.update_interval = 5;
            }
            _ => {} // Ignore unknown arguments
        }
    }
    
    config
}

// Working main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== System Resource Monitor ===");
    println!("Built with Rust systems programming");
    println!("Demonstrates: Memory layout, Unsafe operations, FFI\n");
    
    let config = parse_arguments();
    
    if config.single_shot {
        let system_info = SystemInfo::collect()?;
        system_info.display();
    } else {
        run_monitor(config)?;
    }
    
    Ok(())
}

// Working utility functions
mod utils {
    // Working byte formatter
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.1} {}", size, UNITS[unit_index])
    }
    
    // Working percentage formatter
    pub fn format_percentage(value: f64) -> String {
        format!("{:.1}%", value)
    }
    
    // Working duration formatter
    pub fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    }
    
    // BUG FIX NEEDED: Color formatting is incomplete
    pub fn colorize(text: &str, color: Color) -> String {
        if std::env::var("NO_COLOR").is_ok() {
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
        
        // TODO: Fix color formatting - missing reset code
        format!("{}{}", color_code, text)  // BUG: Missing reset!
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_info_collection() {
        let result = SystemInfo::collect();
        // Should not panic, but might return error on restricted systems
        assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_utility_functions() {
        assert_eq!(utils::format_bytes(1024), "1.0 KB");
        assert_eq!(utils::format_bytes(1048576), "1.0 MB");
        assert_eq!(utils::format_percentage(50.123), "50.1%");
        assert_eq!(utils::format_duration(3661), "1h 1m 1s");
    }
    
    #[test]
    fn test_config_parsing() {
        let config = MonitorConfig::default();
        assert_eq!(config.update_interval, 2);
        assert!(!config.single_shot);
    }
}

// BUGS TO FIX:
// 1. Complete display_memory(), display_cpu(), display_processes(), display_uptime() methods
// 2. Add missing From trait implementations for error conversion
// 3. Fix colorize() function to include reset code
// 4. Fix modules: memory.rs, cpu.rs, process.rs (they have parsing bugs)
// 5. Uncomment the display method calls once formatting functions are implemented
//
// LEARNING OBJECTIVES:
// - Debug specific bugs in working code rather than building from scratch
// - Practice error handling and trait implementations
// - Implement display formatting for system information
// - Handle cross-platform compatibility issues
// - Build a complete, production-ready application
//
// SUCCESS CRITERIA:
// - All code compiles without errors
// - Application displays real system metrics
// - Updates work in real-time
// - Error handling is robust and user-friendly
// - Output is well-formatted and professional