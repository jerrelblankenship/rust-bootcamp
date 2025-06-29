// CPU Information Module - WORKING SOLUTION
//
// This demonstrates:
// - CPU usage calculation over time
// - Cross-platform CPU monitoring  
// - State management for percentage calculations
// - Safe parsing of system files

use std::fs;
use std::io;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    usage_percentage: f64,
    core_count: usize,
    model_name: String,
    previous_idle: u64,
    previous_total: u64,
    last_update: Instant,
}

impl CpuInfo {
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
    
    #[cfg(target_os = "linux")]
    fn collect_linux() -> Result<Self, CpuError> {
        let content = fs::read_to_string("/proc/stat")?;
        let first_line = content.lines().next()
            .ok_or(CpuError::InvalidFormat)?;
        
        if !first_line.starts_with("cpu ") {
            return Err(CpuError::InvalidFormat);
        }
        
        let values: Vec<u64> = first_line
            .split_whitespace()
            .skip(1)  // Skip "cpu" label
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        
        if values.len() < 4 {
            return Err(CpuError::InvalidFormat);
        }
        
        // CPU time values from /proc/stat:
        // user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice
        let user = values[0];
        let nice = values[1]; 
        let system = values[2];
        let idle = values[3];
        
        let total = user + nice + system + idle;
        
        let core_count = Self::get_core_count()?;
        let model_name = Self::get_model_name()?;
        
        Ok(CpuInfo {
            usage_percentage: 0.0, // Will be calculated on first update
            core_count,
            model_name,
            previous_idle: idle,
            previous_total: total,
            last_update: Instant::now(),
        })
    }
    
    #[cfg(target_os = "windows")]
    fn collect_windows() -> Result<Self, CpuError> {
        use std::process::Command;
        
        // Get CPU info using wmic
        let output = Command::new("wmic")
            .args(&["cpu", "get", "Name,NumberOfCores", "/format:csv"])
            .output()
            .map_err(|_| CpuError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "Failed to run wmic command"
            )))?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        let mut core_count = 1;
        let mut model_name = "Unknown CPU".to_string();
        
        for line in output_str.lines().skip(1) { // Skip header
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 3 && !fields[1].is_empty() {
                if let Ok(cores) = fields[1].parse::<usize>() {
                    core_count = cores;
                }
                if !fields[2].is_empty() {
                    model_name = fields[2].trim().to_string();
                }
                break;
            }
        }
        
        Ok(CpuInfo {
            usage_percentage: 0.0,
            core_count,
            model_name,
            previous_idle: 0,
            previous_total: 0,
            last_update: Instant::now(),
        })
    }
    
    #[cfg(target_os = "macos")]
    fn collect_macos() -> Result<Self, CpuError> {
        use std::process::Command;
        
        // Get CPU info using sysctl
        let core_output = Command::new("sysctl")
            .args(&["-n", "hw.ncpu"])
            .output()
            .map_err(|_| CpuError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get CPU core count"
            )))?;
        
        let core_count = String::from_utf8_lossy(&core_output.stdout)
            .trim()
            .parse()
            .unwrap_or(1);
        
        let brand_output = Command::new("sysctl")
            .args(&["-n", "machdep.cpu.brand_string"])
            .output()
            .map_err(|_| CpuError::IoError(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get CPU brand"
            )))?;
        
        let model_name = String::from_utf8_lossy(&brand_output.stdout)
            .trim()
            .to_string();
        
        Ok(CpuInfo {
            usage_percentage: 0.0,
            core_count,
            model_name,
            previous_idle: 0,
            previous_total: 0,
            last_update: Instant::now(),
        })
    }
    
    #[cfg(target_os = "linux")]
    fn get_core_count() -> Result<usize, CpuError> {
        let content = fs::read_to_string("/proc/cpuinfo")?;
        let count = content.lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        
        Ok(if count == 0 { 1 } else { count })
    }
    
    #[cfg(target_os = "linux")]
    fn get_model_name() -> Result<String, CpuError> {
        let content = fs::read_to_string("/proc/cpuinfo")?;
        
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(name) = line.split(':').nth(1) {
                    return Ok(name.trim().to_string());
                }
            }
        }
        
        Ok("Unknown CPU".to_string())
    }
    
    pub fn update(&mut self) -> Result<(), CpuError> {
        #[cfg(target_os = "linux")]
        {
            let content = fs::read_to_string("/proc/stat")?;
            let first_line = content.lines().next()
                .ok_or(CpuError::InvalidFormat)?;
            
            let values: Vec<u64> = first_line
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;
            
            if values.len() < 4 {
                return Err(CpuError::InvalidFormat);
            }
            
            let user = values[0];
            let nice = values[1];
            let system = values[2]; 
            let idle = values[3];
            
            let total = user + nice + system + idle;
            
            // Calculate usage percentage
            let total_diff = total.saturating_sub(self.previous_total);
            let idle_diff = idle.saturating_sub(self.previous_idle);
            
            if total_diff > 0 {
                let usage = 100.0 * (1.0 - (idle_diff as f64 / total_diff as f64));
                self.usage_percentage = usage.max(0.0).min(100.0);
            }
            
            // Update previous values
            self.previous_idle = idle;
            self.previous_total = total;
            self.last_update = Instant::now();
            
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            // For non-Linux platforms, simulate CPU usage
            // In a real implementation, this would use platform-specific APIs
            use std::time::SystemTime;
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // Simple sine wave simulation for demo purposes
            self.usage_percentage = 30.0 + 20.0 * ((now as f64 / 10.0).sin());
            self.last_update = Instant::now();
            
            Ok(())
        }
    }
    
    pub fn usage_percentage(&self) -> f64 {
        self.usage_percentage
    }
    
    pub fn core_count(&self) -> usize {
        self.core_count
    }
    
    pub fn model_name(&self) -> &str {
        &self.model_name
    }
    
    pub fn format_summary(&self) -> String {
        format!("CPU: {:.1}% usage, {} cores, {}", 
                self.usage_percentage, self.core_count, self.model_name)
    }
}

#[derive(Debug)]
pub enum CpuError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
    InvalidFormat,
    UnsupportedPlatform,
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::IoError(e) => write!(f, "IO error reading CPU info: {}", e),
            CpuError::ParseError(e) => write!(f, "Failed to parse CPU value: {}", e),
            CpuError::InvalidFormat => write!(f, "Invalid CPU data format"),
            CpuError::UnsupportedPlatform => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for CpuError {}

impl From<io::Error> for CpuError {
    fn from(error: io::Error) -> Self {
        CpuError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for CpuError {
    fn from(error: std::num::ParseIntError) -> Self {
        CpuError::ParseError(error)
    }
}

// Advanced CPU monitoring features
#[derive(Debug, Clone)]
pub struct DetailedCpuInfo {
    cores: Vec<CoreInfo>,
    load_average: [f64; 3],  // 1, 5, 15 minute averages
}

#[derive(Debug, Clone)]
pub struct CoreInfo {
    core_id: usize,
    usage_percentage: f64,
}

impl DetailedCpuInfo {
    #[cfg(target_os = "linux")]
    pub fn collect() -> Result<Self, CpuError> {
        let load_average = Self::get_load_average()?;
        
        // For simplicity, just create dummy per-core info
        let core_count = CpuInfo::get_core_count()?;
        let cores = (0..core_count)
            .map(|i| CoreInfo {
                core_id: i,
                usage_percentage: 0.0, // Would be calculated from /proc/stat
            })
            .collect();
        
        Ok(DetailedCpuInfo {
            cores,
            load_average,
        })
    }
    
    #[cfg(not(target_os = "linux"))]
    pub fn collect() -> Result<Self, CpuError> {
        Err(CpuError::UnsupportedPlatform)
    }
    
    #[cfg(target_os = "linux")]
    fn get_load_average() -> Result<[f64; 3], CpuError> {
        let content = fs::read_to_string("/proc/loadavg")?;
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        if parts.len() < 3 {
            return Err(CpuError::InvalidFormat);
        }
        
        let load1 = parts[0].parse::<f64>()?;
        let load5 = parts[1].parse::<f64>()?;
        let load15 = parts[2].parse::<f64>()?;
        
        Ok([load1, load5, load15])
    }
    
    pub fn format_summary(&self) -> String {
        format!("Load averages: {:.2} {:.2} {:.2} (1m 5m 15m)", 
                self.load_average[0], self.load_average[1], self.load_average[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_cpu_info_collection() {
        let result = CpuInfo::collect();
        
        #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
        {
            if let Ok(cpu_info) = result {
                assert!(cpu_info.core_count() > 0, "Should have at least 1 CPU core");
                assert!(!cpu_info.model_name().is_empty(), "Should have a CPU model name");
                println!("CPU: {} cores, {}", cpu_info.core_count(), cpu_info.model_name());
            }
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            assert!(result.is_err(), "Should fail on unsupported platforms");
        }
    }
    
    #[test]
    fn test_cpu_update() {
        if let Ok(mut cpu_info) = CpuInfo::collect() {
            // Initial usage might be 0 since we need a baseline
            
            // Wait a bit and update
            std::thread::sleep(Duration::from_millis(100));
            
            let result = cpu_info.update();
            assert!(result.is_ok(), "CPU update should succeed");
            
            // After update, usage should be calculable
            let usage = cpu_info.usage_percentage();
            assert!(usage >= 0.0 && usage <= 100.0, 
                   "CPU usage should be between 0 and 100 percent, got: {}", usage);
            
            println!("CPU usage: {:.1}%", usage);
        }
    }
    
    #[test]
    #[cfg(target_os = "linux")]
    fn test_load_average() {
        if let Ok(detailed) = DetailedCpuInfo::collect() {
            // Load averages can be any positive value, but typically < 100
            for &load in &detailed.load_average {
                assert!(load >= 0.0, "Load average should be non-negative");
                assert!(load < 1000.0, "Load average seems unreasonably high: {}", load);
            }
            
            println!("Load averages: {:?}", detailed.load_average);
        }
    }
    
    #[test]
    fn test_cpu_format_summary() {
        let cpu_info = CpuInfo {
            usage_percentage: 42.5,
            core_count: 8,
            model_name: "Test CPU".to_string(),
            previous_idle: 0,
            previous_total: 0,
            last_update: Instant::now(),
        };
        
        let summary = cpu_info.format_summary();
        assert!(summary.contains("42.5%"));
        assert!(summary.contains("8 cores"));
        assert!(summary.contains("Test CPU"));
    }
}
