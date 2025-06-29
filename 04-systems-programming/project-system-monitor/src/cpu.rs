// CPU Information Module - PARTIALLY WORKING
//
// This module works but has specific bugs to fix:
// 1. Division by zero error in CPU calculation
// 2. Missing error handling for file operations
// 3. Incomplete platform support
//
// FIX ONE BUG AT A TIME!

use std::fs;
use std::io;

// Working struct definition
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub usage_percentage: f64,
    pub core_count: usize,
    pub total_time: u64,
    pub idle_time: u64,
}

impl CpuInfo {
    // Working constructor with platform detection
    pub fn collect() -> Result<Self, CpuError> {
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
            Err(CpuError::UnsupportedPlatform)
        }
    }
    
    // PARTIALLY WORKING: Linux implementation with division by zero bug
    #[cfg(target_os = "linux")]
    fn collect_linux() -> Result<Self, CpuError> {
        // BUG FIX NEEDED: No error handling for missing file
        let content = fs::read_to_string("/proc/stat")?;
        
        // Parse first line which contains overall CPU stats
        let first_line = content.lines().next()
            .ok_or(CpuError::InvalidFormat)?;
        
        if !first_line.starts_with("cpu ") {
            return Err(CpuError::InvalidFormat);
        }
        
        // Parse CPU time values
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(CpuError::InvalidFormat);
        }
        
        // CPU time fields: user, nice, system, idle, iowait, irq, softirq, steal
        let user: u64 = parts[1].parse().map_err(|_| CpuError::ParseError)?;
        let nice: u64 = parts[2].parse().map_err(|_| CpuError::ParseError)?;
        let system: u64 = parts[3].parse().map_err(|_| CpuError::ParseError)?;
        let idle: u64 = parts[4].parse().map_err(|_| CpuError::ParseError)?;
        
        let total_time = user + nice + system + idle;
        let idle_time = idle;
        
        // BUG FIX NEEDED: This can cause division by zero panic!
        let usage_percentage = Self::calculate_cpu_usage(idle_time, total_time);  // BUG: What if total_time is 0?
        
        // Count CPU cores
        let core_count = Self::count_cpu_cores(&content);
        
        Ok(CpuInfo {
            usage_percentage,
            core_count,
            total_time,
            idle_time,
        })
    }
    
    // BUG FIX NEEDED: This function can panic on division by zero
    fn calculate_cpu_usage(idle: u64, total: u64) -> f64 {
        let used = total - idle;
        (used as f64 / total as f64) * 100.0  // BUG: What if total is 0?
    }
    
    // Working helper function
    fn count_cpu_cores(content: &str) -> usize {
        content.lines()
            .filter(|line| line.starts_with("cpu") && line.chars().nth(3).map_or(false, |c| c.is_ascii_digit()))
            .count()
    }
    
    // Working Windows implementation (mock for now)
    #[cfg(target_os = "windows")]
    fn collect_windows() -> Result<Self, CpuError> {
        // TODO: Real Windows implementation would use Windows API
        // For now, return mock data
        Ok(CpuInfo {
            usage_percentage: 25.0,
            core_count: 8,
            total_time: 1000000,
            idle_time: 750000,
        })
    }
    
    // Working macOS implementation (mock for now)
    #[cfg(target_os = "macos")]
    fn collect_macos() -> Result<Self, CpuError> {
        // TODO: Real macOS implementation would use system calls
        // For now, return mock data
        Ok(CpuInfo {
            usage_percentage: 15.0,
            core_count: 4,
            total_time: 2000000,
            idle_time: 1700000,
        })
    }
    
    // Working formatter
    pub fn format_summary(&self) -> String {
        use crate::utils::format_percentage;
        
        format!(
            "CPU: {} ({} cores)",
            format_percentage(self.usage_percentage),
            self.core_count
        )
    }
    
    // Working update method
    pub fn update(&mut self) -> Result<(), CpuError> {
        let new_info = Self::collect()?;
        *self = new_info;
        Ok(())
    }
}

// Working error type
#[derive(Debug)]
pub enum CpuError {
    IoError(io::Error),
    ParseError,
    InvalidFormat,
    UnsupportedPlatform,
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::IoError(e) => write!(f, "IO error reading CPU info: {}", e),
            CpuError::ParseError => write!(f, "Failed to parse CPU values"),
            CpuError::InvalidFormat => write!(f, "Invalid CPU info format"),
            CpuError::UnsupportedPlatform => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for CpuError {}

// Working From implementation
impl From<io::Error> for CpuError {
    fn from(error: io::Error) -> Self {
        CpuError::IoError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_info_collection() {
        let result = CpuInfo::collect();
        
        // Should work on supported platforms
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            assert!(result.is_ok(), "CPU collection should work on supported platforms");
            
            let cpu_info = result.unwrap();
            assert!(cpu_info.usage_percentage >= 0.0 && cpu_info.usage_percentage <= 100.0,
                   "CPU usage should be between 0 and 100 percent");
            assert!(cpu_info.core_count > 0, "Core count should be greater than 0");
        }
    }
    
    #[test]
    fn test_cpu_usage_calculation() {
        // Normal case
        let usage = CpuInfo::calculate_cpu_usage(750, 1000);
        assert!((usage - 25.0).abs() < 0.01);
        
        // Edge case: total is 0 (should not panic after fix)
        let usage = CpuInfo::calculate_cpu_usage(0, 0);
        assert!(usage >= 0.0); // Should not panic
    }
    
    #[test]
    fn test_core_counting() {
        let test_content = r#"cpu  100 200 300 400
cpu0 10 20 30 40
cpu1 15 25 35 45
cpu2 20 30 40 50
cpu3 25 35 45 55"#;
        
        let core_count = CpuInfo::count_cpu_cores(test_content);
        assert_eq!(core_count, 4);
    }
}

// BUGS TO FIX:
// 1. Add zero check in calculate_cpu_usage() to prevent division by zero
// 2. Handle file not found errors gracefully
// 3. Add proper error handling for edge cases
//
// LEARNING OBJECTIVES:
// - Handle division by zero safely in calculations
// - Read system information from /proc filesystem (Linux)
// - Parse numeric data safely with proper error handling
// - Handle cross-platform compatibility
//
// SUCCESS CRITERIA:
// - No division by zero panics
// - Graceful handling of missing /proc/stat file
// - CPU usage percentage is always between 0-100
// - Tests pass on all supported platforms