// Memory Information Module - PARTIALLY WORKING
//
// This module works but has specific bugs to fix:
// 1. Array bounds error in parsing logic
// 2. Missing From trait implementation for error conversion
// 3. Incomplete field calculations
//
// FIX ONE BUG AT A TIME!

use std::fs;
use std::io;

// Working struct definition
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub cached_bytes: u64,
    pub buffer_bytes: u64,
}

impl MemoryInfo {
    // Working constructor with platform detection
    pub fn collect() -> Result<Self, MemoryError> {
        #[cfg(target_os = "linux")]
        {
            Self::collect_linux()
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::collect_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::collect_macos()
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(MemoryError::UnsupportedPlatform)
        }
    }
    
    // PARTIALLY WORKING: Linux implementation with parsing bug
    #[cfg(target_os = "linux")]
    fn collect_linux() -> Result<Self, MemoryError> {
        let content = fs::read_to_string("/proc/meminfo")?;
        
        let mut total_bytes = 0;
        let mut available_bytes = 0;
        let mut cached_bytes = 0;
        let mut buffer_bytes = 0;
        
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                // BUG FIX NEEDED: This can panic with array bounds error
                let parts: Vec<&str> = line.split_whitespace().collect();
                total_bytes = parts[1].parse::<u64>()? * 1024;  // BUG: What if parts.len() < 2?
            } else if line.starts_with("MemAvailable:") {
                // BUG FIX NEEDED: Same bounds issue here
                let parts: Vec<&str> = line.split_whitespace().collect(); 
                available_bytes = parts[1].parse::<u64>()? * 1024;  // BUG: Unsafe array access
            } else if line.starts_with("Cached:") {
                // TODO: Fix this parsing too
                let parts: Vec<&str> = line.split_whitespace().collect();
                cached_bytes = parts[1].parse::<u64>()? * 1024;
            } else if line.starts_with("Buffers:") {
                // TODO: Fix this parsing too  
                let parts: Vec<&str> = line.split_whitespace().collect();
                buffer_bytes = parts[1].parse::<u64>()? * 1024;
            }
        }
        
        // BUG FIX NEEDED: used_bytes calculation is missing
        let used_bytes = 0;  // TODO: Calculate properly
        
        Ok(MemoryInfo {
            total_bytes,
            available_bytes,
            used_bytes,  // BUG: This should be total - available
            cached_bytes,
            buffer_bytes,
        })
    }
    
    // Working Windows implementation (mock for now)
    #[cfg(target_os = "windows")]
    fn collect_windows() -> Result<Self, MemoryError> {
        // TODO: Real Windows implementation would use Windows API
        // For now, return mock data
        Ok(MemoryInfo {
            total_bytes: 16 * 1024 * 1024 * 1024, // 16 GB
            available_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            used_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            cached_bytes: 1024 * 1024 * 1024, // 1 GB
            buffer_bytes: 512 * 1024 * 1024, // 512 MB
        })
    }
    
    // Working macOS implementation (mock for now)
    #[cfg(target_os = "macos")]
    fn collect_macos() -> Result<Self, MemoryError> {
        // TODO: Real macOS implementation would use mach system calls
        // For now, return mock data
        Ok(MemoryInfo {
            total_bytes: 32 * 1024 * 1024 * 1024, // 32 GB
            available_bytes: 16 * 1024 * 1024 * 1024, // 16 GB
            used_bytes: 16 * 1024 * 1024 * 1024, // 16 GB
            cached_bytes: 2 * 1024 * 1024 * 1024, // 2 GB
            buffer_bytes: 1024 * 1024 * 1024, // 1 GB
        })
    }
    
    // Working calculated properties
    pub fn usage_percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
        }
    }
    
    // Working formatter
    pub fn format_summary(&self) -> String {
        use crate::utils::{format_bytes, format_percentage};
        
        format!(
            "Used: {} / {} ({})",
            format_bytes(self.used_bytes),
            format_bytes(self.total_bytes),
            format_percentage(self.usage_percentage())
        )
    }
    
    // Working update method
    pub fn update(&mut self) -> Result<(), MemoryError> {
        let new_info = Self::collect()?;
        *self = new_info;
        Ok(())
    }
}

// Working error type but missing From implementation
#[derive(Debug)]
pub enum MemoryError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    UnsupportedPlatform,
    InvalidFormat,
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::IoError(e) => write!(f, "IO error reading memory info: {}", e),
            MemoryError::ParseError(e) => write!(f, "Failed to parse memory value: {}", e),
            MemoryError::UnsupportedPlatform => write!(f, "Platform not supported"),
            MemoryError::InvalidFormat => write!(f, "Invalid memory info format"),
        }
    }
}

impl std::error::Error for MemoryError {}

// Working From implementation for IO errors
impl From<io::Error> for MemoryError {
    fn from(error: io::Error) -> Self {
        MemoryError::IoError(error)
    }
}

// BUG FIX NEEDED: Missing From implementation for ParseIntError
// TODO: Add this implementation:
// impl From<std::num::ParseIntError> for MemoryError {
//     fn from(error: std::num::ParseIntError) -> Self {
//         MemoryError::ParseError(error)
//     }
// }

// Working utility function for safe parsing
fn parse_memory_line(content: &str, field_name: &str) -> Option<u64> {
    for line in content.lines() {
        if line.starts_with(field_name) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<u64>() {
                    return Some(value * 1024); // Convert KB to bytes
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_info_collection() {
        let result = MemoryInfo::collect();
        
        // Should work on supported platforms
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            assert!(result.is_ok(), "Memory collection should work on supported platforms");
            
            let memory_info = result.unwrap();
            assert!(memory_info.total_bytes > 0, "Total memory should be greater than 0");
            assert!(memory_info.available_bytes <= memory_info.total_bytes, 
                   "Available memory should not exceed total memory");
        }
        
        // Should fail on unsupported platforms
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            assert!(result.is_err(), "Memory collection should fail on unsupported platforms");
        }
    }
    
    #[test]
    fn test_memory_calculations() {
        let memory_info = MemoryInfo {
            total_bytes: 16 * 1024 * 1024 * 1024, // 16 GB
            available_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            used_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            cached_bytes: 1024 * 1024 * 1024, // 1 GB
            buffer_bytes: 512 * 1024 * 1024, // 512 MB
        };
        
        assert!((memory_info.usage_percentage() - 50.0).abs() < 0.01);
    }
    
    #[test]
    fn test_parse_memory_line() {
        let test_content = r#"MemTotal:       16384000 kB
MemFree:         2048000 kB
MemAvailable:    8192000 kB"#;
        
        assert_eq!(parse_memory_line(test_content, "MemTotal:"), Some(16384000 * 1024));
        assert_eq!(parse_memory_line(test_content, "MemAvailable:"), Some(8192000 * 1024));
        assert_eq!(parse_memory_line(test_content, "NonExistent:"), None);
    }
}

// BUGS TO FIX:
// 1. Add bounds checking before accessing parts[1] in parsing logic
// 2. Implement From<std::num::ParseIntError> for MemoryError
// 3. Fix used_bytes calculation (should be total - available)
// 4. Use the safe parse_memory_line function instead of unsafe parsing
//
// LEARNING OBJECTIVES:
// - Handle array bounds safely in parsing code
// - Implement proper error conversion traits
// - Read system information from /proc filesystem (Linux)
// - Handle cross-platform compatibility
// - Practice safe parsing patterns
//
// SUCCESS CRITERIA:
// - All parsing is safe from array bounds errors
// - Error conversion works properly with ? operator
// - Memory calculations are correct
// - Tests pass on all supported platforms