// Process Information Module - PARTIALLY WORKING
//
// This module works but has specific bugs to fix:
// 1. Error handling for directory iteration
// 2. Missing platform implementations
// 3. Incomplete process counting logic
//
// FIX ONE BUG AT A TIME!

use std::fs;
use std::io;

// Working struct definition
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub total_processes: u32,
    pub running_processes: u32,
    pub sleeping_processes: u32,
    pub zombie_processes: u32,
}

impl ProcessInfo {
    // Working constructor with platform detection
    pub fn collect() -> Result<Self, ProcessError> {
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
            Err(ProcessError::UnsupportedPlatform)
        }
    }
    
    // PARTIALLY WORKING: Linux implementation with error handling issues
    #[cfg(target_os = "linux")]
    fn collect_linux() -> Result<Self, ProcessError> {
        let mut total = 0;
        let mut running = 0;
        let mut sleeping = 0;
        let mut zombie = 0;
        
        // BUG FIX NEEDED: No error handling for directory access
        let proc_entries = fs::read_dir("/proc")?;  // What if /proc doesn't exist or no permissions?
        
        for entry in proc_entries {
            let entry = entry?;
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();
            
            // Check if it's a process directory (numeric name)
            if name_str.chars().all(|c| c.is_ascii_digit()) {
                total += 1;
                
                // Try to read process status
                let stat_path = format!("/proc/{}/stat", name_str);
                
                // BUG FIX NEEDED: File reading might fail - needs better error handling
                if let Ok(stat_content) = fs::read_to_string(&stat_path) {
                    if let Some(state) = Self::parse_process_state(&stat_content) {
                        match state {
                            'R' => running += 1,
                            'S' | 'D' | 'I' => sleeping += 1,
                            'Z' => zombie += 1,
                            _ => {} // Other states
                        }
                    }
                }
                // BUG: If file reading fails, we silently ignore it
                // Should we count it as unknown state?
            }
        }
        
        Ok(ProcessInfo {
            total_processes: total,
            running_processes: running,
            sleeping_processes: sleeping,
            zombie_processes: zombie,
        })
    }
    
    // Working helper function for parsing process state
    fn parse_process_state(stat_content: &str) -> Option<char> {
        // Process stat format: pid (comm) state ppid pgrp session tty_nr tpgid flags ...
        let parts: Vec<&str> = stat_content.split_whitespace().collect();
        if parts.len() >= 3 {
            // State is the 3rd field (index 2)
            parts[2].chars().next()
        } else {
            None
        }
    }
    
    // Working Windows implementation (mock for now)
    #[cfg(target_os = "windows")]
    fn collect_windows() -> Result<Self, ProcessError> {
        // TODO: Real Windows implementation would use Windows API
        // For now, return mock data
        Ok(ProcessInfo {
            total_processes: 150,
            running_processes: 10,
            sleeping_processes: 135,
            zombie_processes: 0,
        })
    }
    
    // Working macOS implementation (mock for now)
    #[cfg(target_os = "macos")]
    fn collect_macos() -> Result<Self, ProcessError> {
        // TODO: Real macOS implementation would use system calls
        // For now, return mock data
        Ok(ProcessInfo {
            total_processes: 200,
            running_processes: 15,
            sleeping_processes: 180,
            zombie_processes: 1,
        })
    }
    
    // Working formatter
    pub fn format_summary(&self) -> String {
        format!(
            "Processes: {} total ({} running, {} sleeping, {} zombie)",
            self.total_processes,
            self.running_processes,
            self.sleeping_processes,
            self.zombie_processes
        )
    }
    
    // Working update method
    pub fn update(&mut self) -> Result<(), ProcessError> {
        let new_info = Self::collect()?;
        *self = new_info;
        Ok(())
    }
}

// Working error type
#[derive(Debug)]
pub enum ProcessError {
    IoError(io::Error),
    PermissionDenied,
    InvalidFormat,
    UnsupportedPlatform,
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::IoError(e) => write!(f, "IO error reading process info: {}", e),
            ProcessError::PermissionDenied => write!(f, "Permission denied accessing process info"),
            ProcessError::InvalidFormat => write!(f, "Invalid process info format"),
            ProcessError::UnsupportedPlatform => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for ProcessError {}

// Working From implementation
impl From<io::Error> for ProcessError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::PermissionDenied => ProcessError::PermissionDenied,
            _ => ProcessError::IoError(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_info_collection() {
        let result = ProcessInfo::collect();
        
        // Should work on supported platforms
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            assert!(result.is_ok(), "Process collection should work on supported platforms");
            
            let process_info = result.unwrap();
            assert!(process_info.total_processes > 0, "Total processes should be greater than 0");
            
            // Total should be sum of all categories
            let sum = process_info.running_processes + 
                     process_info.sleeping_processes + 
                     process_info.zombie_processes;
            assert!(sum <= process_info.total_processes, 
                   "Sum of categorized processes should not exceed total");
        }
    }
    
    #[test]
    fn test_process_state_parsing() {
        let test_stat = "1234 (test) R 1 1 1 0 -1 4194304 123 456 0 0 10 5";
        let state = ProcessInfo::parse_process_state(test_stat);
        assert_eq!(state, Some('R'));
        
        let invalid_stat = "invalid";
        let state = ProcessInfo::parse_process_state(invalid_stat);
        assert_eq!(state, None);
    }
    
    #[test]
    fn test_error_handling() {
        // Test that we can handle permission errors gracefully
        let error = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let process_error = ProcessError::from(error);
        
        match process_error {
            ProcessError::PermissionDenied => {}, // Expected
            _ => panic!("Should convert to PermissionDenied"),
        }
    }
}

// BUGS TO FIX:
// 1. Add better error handling for /proc directory access
// 2. Handle individual process stat file reading failures gracefully
// 3. Consider what to do with processes we can't read (count as unknown?)
// 4. Add validation that process counts make sense
//
// LEARNING OBJECTIVES:
// - Handle file system permission errors gracefully
// - Iterate through directories safely with error handling
// - Parse system process information from /proc filesystem
// - Handle partial failures in data collection
//
// SUCCESS CRITERIA:
// - No panics when /proc is inaccessible
// - Graceful handling of individual process read failures
// - Process counts are consistent and reasonable
// - Tests pass on all supported platforms