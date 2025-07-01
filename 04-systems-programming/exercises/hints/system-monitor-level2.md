# System Monitor Project - Level 2 Hints 🟡

## Specific Implementation Strategies

### 1. CPU Usage Calculation
```rust
// /proc/stat format: cpu user nice system idle iowait irq softirq steal guest
pub fn parse_cpu_stats(stat_line: &str) -> Result<CpuStats, ParseError> {
    let parts: Vec<&str> = stat_line.split_whitespace().collect();
    if parts.len() < 5 || parts[0] != "cpu" {
        return Err(ParseError::InvalidFormat);
    }
    
    Ok(CpuStats {
        user: parts[1].parse()?,
        nice: parts[2].parse()?,
        system: parts[3].parse()?,
        idle: parts[4].parse()?,
        // ... handle other fields
    })
}

// Calculate CPU usage percentage
pub fn calculate_cpu_usage(prev: &CpuStats, curr: &CpuStats) -> f64 {
    let prev_total = prev.user + prev.nice + prev.system + prev.idle;
    let curr_total = curr.user + curr.nice + curr.system + curr.idle;
    
    let total_diff = curr_total - prev_total;
    let idle_diff = curr.idle - prev.idle;
    
    if total_diff == 0 {
        0.0
    } else {
        100.0 * (1.0 - (idle_diff as f64 / total_diff as f64))
    }
}
```

### 2. Memory Information Parsing
```rust
// /proc/meminfo parsing
pub fn parse_memory_info() -> Result<MemoryInfo, std::io::Error> {
    let content = std::fs::read_to_string("/proc/meminfo")?;
    let mut memory_info = MemoryInfo::default();
    
    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim().split_whitespace().next().unwrap_or("0");
            let kb_value: u64 = value.parse().unwrap_or(0);
            
            match key {
                "MemTotal" => memory_info.total = kb_value * 1024,
                "MemFree" => memory_info.free = kb_value * 1024,
                "MemAvailable" => memory_info.available = kb_value * 1024,
                "Buffers" => memory_info.buffers = kb_value * 1024,
                "Cached" => memory_info.cached = kb_value * 1024,
                _ => {}
            }
        }
    }
    
    Ok(memory_info)
}
```

### 3. Process Information
```rust
pub fn get_process_info(pid: u32) -> Result<ProcessInfo, ProcessError> {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = std::fs::read_to_string(&stat_path)
        .map_err(|_| ProcessError::NotFound)?;
    
    // Parse /proc/PID/stat (space-separated fields)
    let fields: Vec<&str> = stat_content.split_whitespace().collect();
    if fields.len() < 24 {
        return Err(ProcessError::InvalidFormat);
    }
    
    Ok(ProcessInfo {
        pid,
        name: fields[1].trim_matches(|c| c == '(' || c == ')').to_string(),
        state: fields[2].chars().next().unwrap_or('?'),
        memory_kb: fields[23].parse().unwrap_or(0) * 4, // Pages to KB
        // ... parse other fields
    })
}
```

### 4. Error Handling Patterns
```rust
#[derive(Debug)]
pub enum SystemError {
    FileNotFound(String),
    PermissionDenied(String),
    ParseError(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for SystemError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                SystemError::FileNotFound(err.to_string())
            }
            std::io::ErrorKind::PermissionDenied => {
                SystemError::PermissionDenied(err.to_string())
            }
            _ => SystemError::Io(err),
        }
    }
}

// Graceful fallbacks
pub fn get_cpu_usage() -> f64 {
    match read_cpu_stats() {
        Ok(stats) => calculate_usage(&stats),
        Err(SystemError::FileNotFound(_)) => {
            eprintln!("Warning: /proc/stat not found, CPU usage unavailable");
            0.0
        }
        Err(SystemError::PermissionDenied(_)) => {
            eprintln!("Warning: Permission denied reading CPU stats");
            0.0
        }
        Err(e) => {
            eprintln!("Error reading CPU stats: {:?}", e);
            0.0
        }
    }
}
```

### 5. Cross-Platform Abstraction
```rust
// Platform-specific implementations
#[cfg(target_os = "linux")]
mod platform {
    pub use super::linux::*;
}

#[cfg(target_os = "macos")]
mod platform {
    pub use super::macos::*;
}

#[cfg(target_os = "windows")]
mod platform {
    pub use super::windows::*;
}

// Common interface
pub trait SystemMonitor {
    fn get_cpu_usage(&self) -> Result<f64, SystemError>;
    fn get_memory_info(&self) -> Result<MemoryInfo, SystemError>;
    fn get_process_list(&self) -> Result<Vec<ProcessInfo>, SystemError>;
}
```

### 6. CLI Interface Structure
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sysmon")]
#[command(about = "A system monitoring tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show CPU usage
    Cpu {
        /// Update interval in seconds
        #[arg(short, long, default_value_t = 1)]
        interval: u64,
    },
    /// Show memory information
    Memory,
    /// List processes
    Processes {
        /// Sort by memory usage
        #[arg(short, long)]
        sort_memory: bool,
    },
    /// Watch system stats continuously
    Watch {
        #[arg(short, long, default_value_t = 1)]
        interval: u64,
    },
}
```

Need complete working implementation? Check Level 3 hints!