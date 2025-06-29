// Memory Information Module - WORKING SOLUTION
//
// This demonstrates:
// - Cross-platform memory monitoring
// - Zero-copy string parsing
// - Memory layout optimization
// - Safe error handling

use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    cached_bytes: u64,
    buffer_bytes: u64,
}

impl MemoryInfo {
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
    
    #[cfg(target_os = "linux")]
    fn collect_linux() -> Result<Self, MemoryError> {
        let content = fs::read_to_string("/proc/meminfo")?;
        let parser = MemoryInfoParser::new(&content);
        parser.parse_all()
    }
    
    #[cfg(target_os = "windows")]
    fn collect_windows() -> Result<Self, MemoryError> {
        // Simplified Windows implementation
        // In a real implementation, this would use Windows API
        use std::process::Command;
        
        let output = Command::new("wmic")
            .args(&["OS", "get", "TotalVisibleMemorySize,FreePhysicalMemory", "/format:csv"])
            .output()
            .map_err(|_| MemoryError::IoError(io::Error::new(
                io::ErrorKind::Other, 
                "Failed to run wmic command"
            )))?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse CSV output (simplified)
        let mut total_kb = 0;
        let mut free_kb = 0;
        
        for line in output_str.lines().skip(1) { // Skip header
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 3 && !fields[1].is_empty() {
                if let Ok(free) = fields[1].parse::<u64>() {
                    free_kb = free;
                }
                if let Ok(total) = fields[2].parse::<u64>() {
                    total_kb = total;
                }
                break;
            }
        }
        
        Ok(MemoryInfo {
            total_bytes: total_kb * 1024,
            available_bytes: free_kb * 1024,
            cached_bytes: 0,
            buffer_bytes: 0,
        })
    }
    
    #[cfg(target_os = "macos")]
    fn collect_macos() -> Result<Self, MemoryError> {
        // Simplified macOS implementation
        use std::process::Command;
        
        let output = Command::new("vm_stat")
            .output()
            .map_err(|_| MemoryError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "Failed to run vm_stat command"
            )))?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse vm_stat output
        let mut page_size = 4096u64; // Default page size
        let mut free_pages = 0u64;
        let mut total_pages = 0u64;
        
        for line in output_str.lines() {
            if line.contains("page size of") {
                if let Some(size_str) = line.split_whitespace().nth(7) {
                    page_size = size_str.parse().unwrap_or(4096);
                }
            } else if line.starts_with("Pages free:") {
                if let Some(free_str) = line.split_whitespace().nth(2) {
                    free_pages = free_str.trim_end_matches('.').parse().unwrap_or(0);
                }
            }
        }
        
        // Get total memory from system_profiler (simplified)
        let total_output = Command::new("sysctl")
            .args(&["-n", "hw.memsize"])
            .output()
            .map_err(|_| MemoryError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get total memory"
            )))?;
        
        let total_bytes = String::from_utf8_lossy(&total_output.stdout)
            .trim()
            .parse()
            .unwrap_or(0);
        
        Ok(MemoryInfo {
            total_bytes,
            available_bytes: free_pages * page_size,
            cached_bytes: 0,
            buffer_bytes: 0,
        })
    }
    
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.available_bytes)
    }
    
    pub fn usage_percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes() as f64 / self.total_bytes as f64) * 100.0
        }
    }
    
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }
    
    pub fn available_bytes(&self) -> u64 {
        self.available_bytes
    }
    
    pub fn cached_bytes(&self) -> u64 {
        self.cached_bytes
    }
    
    pub fn buffer_bytes(&self) -> u64 {
        self.buffer_bytes
    }
    
    pub fn format_summary(&self) -> String {
        format!("Memory: {:.1}% used ({} / {})",
                self.usage_percentage(),
                crate::utils::format_bytes(self.used_bytes()),
                crate::utils::format_bytes(self.total_bytes))
    }
    
    pub fn update(&mut self) -> Result<(), MemoryError> {
        let new_info = Self::collect()?;
        *self = new_info;
        Ok(())
    }
}

#[derive(Debug)]
pub enum MemoryError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    InvalidFormat,
    UnsupportedPlatform,
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::IoError(e) => write!(f, "IO error reading memory info: {}", e),
            MemoryError::ParseError(e) => write!(f, "Failed to parse memory value: {}", e),
            MemoryError::InvalidFormat => write!(f, "Invalid memory data format"),
            MemoryError::UnsupportedPlatform => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for MemoryError {}

impl From<io::Error> for MemoryError {
    fn from(error: io::Error) -> Self {
        MemoryError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for MemoryError {
    fn from(error: std::num::ParseIntError) -> Self {
        MemoryError::ParseError(error)
    }
}

// Zero-copy memory info parser for performance
struct MemoryInfoParser<'a> {
    content: &'a str,
}

impl<'a> MemoryInfoParser<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }
    
    fn parse_value(&self, field_name: &str) -> Option<u64> {
        for line in self.content.lines() {
            if line.starts_with(field_name) {
                // Find the numeric value efficiently
                if let Some(value_start) = line.find(char::is_numeric) {
                    let value_part = &line[value_start..];
                    if let Some(value_end) = value_part.find(|c: char| !c.is_numeric()) {
                        if let Ok(value) = value_part[..value_end].parse::<u64>() {
                            return Some(value * 1024); // Convert kB to bytes
                        }
                    }
                }
            }
        }
        None
    }
    
    fn parse_all(&self) -> Result<MemoryInfo, MemoryError> {
        let total_bytes = self.parse_value("MemTotal:")
            .ok_or(MemoryError::InvalidFormat)?;
        
        let available_bytes = self.parse_value("MemAvailable:")
            .or_else(|| self.parse_value("MemFree:"))
            .unwrap_or(0);
        
        let cached_bytes = self.parse_value("Cached:")
            .unwrap_or(0);
        
        let buffer_bytes = self.parse_value("Buffers:")
            .unwrap_or(0);
        
        Ok(MemoryInfo {
            total_bytes,
            available_bytes,
            cached_bytes,
            buffer_bytes,
        })
    }
}

// Memory layout optimization for high-frequency updates
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct RawMemoryStats {
    total_kb: u64,      // 8 bytes
    available_kb: u64,  // 8 bytes  
    cached_kb: u64,     // 8 bytes
    buffer_kb: u64,     // 8 bytes
    // Total: 32 bytes, optimally packed
}

impl RawMemoryStats {
    unsafe fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        
        let ptr = data.as_ptr() as *const Self;
        if (ptr as usize) % std::mem::align_of::<Self>() != 0 {
            return None;
        }
        
        Some(ptr.read())
    }
    
    fn to_memory_info(self) -> MemoryInfo {
        MemoryInfo {
            total_bytes: self.total_kb * 1024,
            available_bytes: self.available_kb * 1024,
            cached_bytes: self.cached_kb * 1024,
            buffer_bytes: self.buffer_kb * 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_info_collection() {
        let result = MemoryInfo::collect();
        
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            if let Ok(memory_info) = result {
                assert!(memory_info.total_bytes() > 0, "Total memory should be greater than 0");
                assert!(memory_info.available_bytes() <= memory_info.total_bytes(), 
                       "Available memory should not exceed total memory");
                assert!(memory_info.usage_percentage() >= 0.0 && memory_info.usage_percentage() <= 100.0);
            }
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            assert!(result.is_err(), "Should fail on unsupported platforms");
        }
    }
    
    #[test]
    fn test_memory_calculations() {
        let memory_info = MemoryInfo {
            total_bytes: 16 * 1024 * 1024 * 1024, // 16 GB
            available_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            cached_bytes: 1024 * 1024 * 1024, // 1 GB
            buffer_bytes: 512 * 1024 * 1024, // 512 MB
        };
        
        assert_eq!(memory_info.used_bytes(), 8 * 1024 * 1024 * 1024);
        assert!((memory_info.usage_percentage() - 50.0).abs() < 0.01);
    }
    
    #[test]
    fn test_memory_parser() {
        let test_content = r#"MemTotal:       16384000 kB
MemFree:         2048000 kB
MemAvailable:    8192000 kB
Buffers:          512000 kB
Cached:          4096000 kB"#;
        
        let parser = MemoryInfoParser::new(test_content);
        
        assert_eq!(parser.parse_value("MemTotal:"), Some(16384000 * 1024));
        assert_eq!(parser.parse_value("MemAvailable:"), Some(8192000 * 1024));
        assert_eq!(parser.parse_value("NonExistent:"), None);
        
        let result = parser.parse_all();
        assert!(result.is_ok());
        
        let memory_info = result.unwrap();
        assert_eq!(memory_info.total_bytes, 16384000 * 1024);
        assert_eq!(memory_info.available_bytes, 8192000 * 1024);
    }
    
    #[test]
    fn test_raw_memory_stats_layout() {
        use std::mem;
        
        assert_eq!(mem::size_of::<RawMemoryStats>(), 8 * 4); // 4 u64 fields
        assert_eq!(mem::align_of::<RawMemoryStats>(), 8);    // u64 alignment
        
        let data = [0u8; 32]; // 4 * 8 bytes
        unsafe {
            let stats = RawMemoryStats::from_bytes(&data);
            assert!(stats.is_some());
        }
        
        let small_data = [0u8; 16];
        unsafe {
            let stats = RawMemoryStats::from_bytes(&small_data);
            assert!(stats.is_none());
        }
    }
}
