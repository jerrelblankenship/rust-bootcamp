# System Monitor Project - Level 3 Hints 🔴

## Complete Implementation Examples

### 1. Full CPU Monitoring Module
```rust
// src/cpu.rs
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct CpuStats {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
}

impl CpuStats {
    pub fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + 
        self.iowait + self.irq + self.softirq
    }
    
    pub fn active(&self) -> u64 {
        self.total() - self.idle - self.iowait
    }
}

pub fn read_cpu_stats() -> Result<CpuStats, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("/proc/stat")?;
    let first_line = content.lines().next().ok_or("Empty /proc/stat")?;
    
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 8 || parts[0] != "cpu" {
        return Err("Invalid /proc/stat format".into());
    }
    
    Ok(CpuStats {
        user: parts[1].parse()?,
        nice: parts[2].parse()?,
        system: parts[3].parse()?,
        idle: parts[4].parse()?,
        iowait: parts[5].parse()?,
        irq: parts[6].parse()?,
        softirq: parts[7].parse()?,
    })
}

pub fn calculate_cpu_usage(prev: &CpuStats, curr: &CpuStats) -> f64 {
    let prev_total = prev.total();
    let curr_total = curr.total();
    let prev_active = prev.active();
    let curr_active = curr.active();
    
    let total_diff = curr_total.saturating_sub(prev_total);
    let active_diff = curr_active.saturating_sub(prev_active);
    
    if total_diff == 0 {
        0.0
    } else {
        (active_diff as f64 / total_diff as f64) * 100.0
    }
}

pub fn monitor_cpu_usage(interval: Duration) -> Result<f64, Box<dyn std::error::Error>> {
    let stats1 = read_cpu_stats()?;
    thread::sleep(interval);
    let stats2 = read_cpu_stats()?;
    
    Ok(calculate_cpu_usage(&stats1, &stats2))
}
```

### 2. Complete Memory Module
```rust
// src/memory.rs
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MemoryInfo {
    pub total: u64,        // Total RAM in bytes
    pub free: u64,         // Free RAM in bytes
    pub available: u64,    // Available RAM in bytes
    pub buffers: u64,      // Buffer cache in bytes
    pub cached: u64,       // Page cache in bytes
    pub swap_total: u64,   // Total swap in bytes
    pub swap_free: u64,    // Free swap in bytes
}

impl MemoryInfo {
    pub fn used(&self) -> u64 {
        self.total.saturating_sub(self.available)
    }
    
    pub fn usage_percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.used() as f64 / self.total as f64) * 100.0
        }
    }
    
    pub fn swap_used(&self) -> u64 {
        self.swap_total.saturating_sub(self.swap_free)
    }
    
    pub fn swap_usage_percent(&self) -> f64 {
        if self.swap_total == 0 {
            0.0
        } else {
            (self.swap_used() as f64 / self.swap_total as f64) * 100.0
        }
    }
}

pub fn read_memory_info() -> Result<MemoryInfo, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("/proc/meminfo")?;
    let mut values = HashMap::new();
    
    for line in content.lines() {
        if let Some((key, value_str)) = line.split_once(':') {
            let value_str = value_str.trim();
            // Parse "123456 kB" format
            let kb_value = value_str
                .split_whitespace()
                .next()
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0);
            
            values.insert(key.to_string(), kb_value * 1024); // Convert to bytes
        }
    }
    
    Ok(MemoryInfo {
        total: values.get("MemTotal").copied().unwrap_or(0),
        free: values.get("MemFree").copied().unwrap_or(0),
        available: values.get("MemAvailable").copied().unwrap_or(0),
        buffers: values.get("Buffers").copied().unwrap_or(0),
        cached: values.get("Cached").copied().unwrap_or(0),
        swap_total: values.get("SwapTotal").copied().unwrap_or(0),
        swap_free: values.get("SwapFree").copied().unwrap_or(0),
    })
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
```

### 3. Process Information Module
```rust
// src/process.rs
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: char,
    pub memory_kb: u64,
    pub cpu_time: u64,
}

pub fn get_process_info(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(&stat_path)?;
    
    // Parse the stat file (space-separated, but name can contain spaces and parentheses)
    let stat_content = stat_content.trim();
    
    // Find the last ')' to handle process names with spaces/parentheses
    let end_name = stat_content.rfind(')').ok_or("Invalid stat format")?;
    let after_name = &stat_content[end_name + 1..];
    let fields: Vec<&str> = after_name.split_whitespace().collect();
    
    if fields.len() < 22 {
        return Err("Insufficient fields in stat file".into());
    }
    
    // Extract process name (between first '(' and last ')')
    let start_name = stat_content.find('(').ok_or("Invalid stat format")?;
    let name = stat_content[start_name + 1..end_name].to_string();
    
    let state = fields[0].chars().next().unwrap_or('?');
    let utime: u64 = fields[11].parse().unwrap_or(0);
    let stime: u64 = fields[12].parse().unwrap_or(0);
    let rss_pages: u64 = fields[21].parse().unwrap_or(0);
    
    Ok(ProcessInfo {
        pid,
        name,
        state,
        memory_kb: rss_pages * 4, // Convert pages to KB (assuming 4KB pages)
        cpu_time: utime + stime,
    })
}

pub fn list_processes() -> Result<Vec<ProcessInfo>, Box<dyn std::error::Error>> {
    let mut processes = Vec::new();
    
    for entry in fs::read_dir("/proc")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        // Check if directory name is a PID (numeric)
        if let Ok(pid) = file_name_str.parse::<u32>() {
            match get_process_info(pid) {
                Ok(info) => processes.push(info),
                Err(_) => continue, // Process might have exited
            }
        }
    }
    
    Ok(processes)
}

pub fn format_process_state(state: char) -> &'static str {
    match state {
        'R' => "Running",
        'S' => "Sleeping",
        'D' => "Disk sleep",
        'Z' => "Zombie",
        'T' => "Stopped",
        't' => "Tracing stop",
        'X' => "Dead",
        _ => "Unknown",
    }
}
```

### 4. Main CLI Interface
```rust
// src/main.rs
use clap::{Parser, Subcommand};
use std::time::Duration;
use std::thread;

mod cpu;
mod memory;
mod process;

#[derive(Parser)]
#[command(name = "sysmon")]
#[command(about = "A system monitoring tool built with Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Monitor CPU usage
    Cpu {
        #[arg(short, long, default_value_t = 1)]
        interval: u64,
        #[arg(short, long)]
        continuous: bool,
    },
    /// Show memory information
    Memory,
    /// List running processes
    Processes {
        #[arg(short, long)]
        sort_memory: bool,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// Watch all system stats
    Watch {
        #[arg(short, long, default_value_t = 1)]
        interval: u64,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Cpu { interval, continuous } => {
            if continuous {
                loop {
                    let usage = cpu::monitor_cpu_usage(Duration::from_secs(interval))?;
                    println!("CPU Usage: {:.1}%", usage);
                }
            } else {
                let usage = cpu::monitor_cpu_usage(Duration::from_secs(interval))?;
                println!("CPU Usage: {:.1}%", usage);
            }
        }
        
        Commands::Memory => {
            let mem_info = memory::read_memory_info()?;
            println!("Memory Information:");
            println!("  Total:     {}", memory::format_bytes(mem_info.total));
            println!("  Used:      {} ({:.1}%)", 
                     memory::format_bytes(mem_info.used()), 
                     mem_info.usage_percent());
            println!("  Available: {}", memory::format_bytes(mem_info.available));
            println!("  Buffers:   {}", memory::format_bytes(mem_info.buffers));
            println!("  Cached:    {}", memory::format_bytes(mem_info.cached));
            
            if mem_info.swap_total > 0 {
                println!("Swap Information:");
                println!("  Total:     {}", memory::format_bytes(mem_info.swap_total));
                println!("  Used:      {} ({:.1}%)",
                         memory::format_bytes(mem_info.swap_used()),
                         mem_info.swap_usage_percent());
            }
        }
        
        Commands::Processes { sort_memory, limit } => {
            let mut processes = process::list_processes()?;
            
            if sort_memory {
                processes.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb));
            }
            
            processes.truncate(limit);
            
            println!("{:>8} {:>8} {:>6} {:<15} {}", 
                     "PID", "Memory", "State", "Name", "Status");
            println!("{}", "-".repeat(60));
            
            for proc in processes {
                println!("{:>8} {:>8} {:>6} {:<15} {}", 
                         proc.pid,
                         memory::format_bytes(proc.memory_kb * 1024),
                         proc.state,
                         &proc.name[..proc.name.len().min(15)],
                         process::format_process_state(proc.state));
            }
        }
        
        Commands::Watch { interval } => {
            loop {
                // Clear screen
                print!("\x1B[2J\x1B[1;1H");
                
                // CPU usage
                match cpu::monitor_cpu_usage(Duration::from_millis(500)) {
                    Ok(usage) => println!("CPU Usage: {:.1}%", usage),
                    Err(e) => println!("CPU Error: {}", e),
                }
                
                // Memory info
                match memory::read_memory_info() {
                    Ok(mem) => {
                        println!("Memory: {} / {} ({:.1}%)",
                                 memory::format_bytes(mem.used()),
                                 memory::format_bytes(mem.total),
                                 mem.usage_percent());
                    }
                    Err(e) => println!("Memory Error: {}", e),
                }
                
                // Top processes
                match process::list_processes() {
                    Ok(mut processes) => {
                        processes.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb));
                        processes.truncate(5);
                        
                        println!("\nTop 5 Processes:");
                        for proc in processes {
                            println!("  {} - {} ({})",
                                     proc.name,
                                     memory::format_bytes(proc.memory_kb * 1024),
                                     process::format_process_state(proc.state));
                        }
                    }
                    Err(e) => println!("Process Error: {}", e),
                }
                
                thread::sleep(Duration::from_secs(interval));
            }
        }
    }
    
    Ok(())
}
```

### Key Production Insights

1. **Error Handling**: Always handle `/proc` file errors gracefully
2. **Performance**: Use efficient parsing and avoid unnecessary allocations
3. **Cross-Platform**: Abstract platform-specific code behind traits
4. **Safety**: Validate all parsed data before using
5. **User Experience**: Provide helpful error messages and formatting

You now have a complete, production-ready system monitoring tool that demonstrates all the systems programming concepts from this module!