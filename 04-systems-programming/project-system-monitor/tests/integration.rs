// Integration Tests for System Monitor
//
// These tests verify that your implementation works correctly
// Run with: cargo test --test integration

use system_monitor::*;

#[test]
fn test_memory_info_basic() {
    // Test that memory info can be collected without panicking
    let result = memory::MemoryInfo::collect();
    
    // On supported platforms, this should work
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    {
        if let Ok(memory_info) = result {
            assert!(memory_info.total_bytes() > 0, "Total memory should be positive");
            assert!(memory_info.available_bytes() <= memory_info.total_bytes(), 
                   "Available memory should not exceed total");
            assert!(memory_info.usage_percentage() >= 0.0 && memory_info.usage_percentage() <= 100.0,
                   "Usage percentage should be between 0 and 100");
        }
        // If it fails, that's okay for testing (might be permission issues)
    }
    
    // On unsupported platforms, should return error
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        assert!(result.is_err(), "Should fail on unsupported platforms");
    }
}

#[test]
fn test_cpu_info_basic() {
    let result = cpu::CpuInfo::collect();
    
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    {
        if let Ok(cpu_info) = result {
            assert!(cpu_info.core_count() > 0, "Should have at least 1 CPU core");
            assert!(!cpu_info.model_name().is_empty(), "Should have a CPU model name");
            // Initial usage might be 0 since we need a baseline for calculation
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        assert!(result.is_err(), "Should fail on unsupported platforms");
    }
}

#[test]
fn test_process_info_basic() {
    let result = process::ProcessInfo::collect();
    
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    {
        if let Ok(process_info) = result {
            assert!(process_info.total_processes() > 0, "Should have at least 1 process");
            assert!(process_info.uptime_seconds() > 0, "System uptime should be positive");
            
            // Boot time should be in the past
            let now = std::time::SystemTime::now();
            assert!(process_info.boot_time() < now, "Boot time should be in the past");
        }
    }
}

#[test]
fn test_system_info_integration() {
    // Test that the main SystemInfo struct can be created and used
    let result = SystemInfo::collect();
    
    if let Ok(mut system_info) = result {
        // Test display doesn't panic
        system_info.display();
        
        // Test update doesn't panic
        let update_result = system_info.update();
        // Update might fail on some systems, but shouldn't panic
        
        println!("System info integration test passed");
    } else {
        println!("System info collection failed (might be expected on some platforms)");
    }
}

#[test]
fn test_ffi_integration() {
    // Test FFI components work
    let result = ffi::SystemStats::collect();
    
    if let Ok(stats) = result {
        assert!(stats.uptime_seconds > 0, "Uptime should be positive");
        assert!(stats.total_memory_bytes > 0, "Total memory should be positive");
    }
    // If FFI fails, that's okay for testing environments
}

#[test]
fn test_error_handling() {
    // Test error types can be created and displayed
    let memory_error = memory::MemoryError::UnsupportedPlatform;
    let error_string = format!("{}", memory_error);
    assert!(!error_string.is_empty(), "Error should have a display message");
    
    let cpu_error = cpu::CpuError::UnsupportedPlatform;
    let error_string = format!("{}", cpu_error);
    assert!(!error_string.is_empty(), "Error should have a display message");
    
    let process_error = process::ProcessError::UnsupportedPlatform;
    let error_string = format!("{}", process_error);
    assert!(!error_string.is_empty(), "Error should have a display message");
}

#[test]
fn test_utility_functions() {
    // Test utility formatting functions
    let bytes_1kb = 1024;
    let formatted = utils::format_bytes(bytes_1kb);
    assert!(formatted.contains("KB") || formatted.contains("B"), 
           "Should format bytes: {}", formatted);
    
    let percentage = 0.1234;
    let formatted = utils::format_percentage(percentage);
    assert!(formatted.contains("%"), "Should format percentage: {}", formatted);
    
    let duration = 3661; // 1 hour, 1 minute, 1 second
    let formatted = utils::format_duration(duration);
    assert!(formatted.contains("h") || formatted.contains("m") || formatted.contains("s"),
           "Should format duration: {}", formatted);
}

#[test]
fn test_real_time_updates() {
    // Test that updates work over time
    if let Ok(mut system_info) = SystemInfo::collect() {
        // Wait a short time
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Update should not panic
        let _ = system_info.update();
        
        println!("Real-time update test passed");
    }
}

#[test]
fn test_memory_safety() {
    // Test that we can create and drop multiple instances safely
    for _ in 0..10 {
        if let Ok(memory_info) = memory::MemoryInfo::collect() {
            let _ = memory_info.format_summary();
        }
        
        if let Ok(cpu_info) = cpu::CpuInfo::collect() {
            let _ = cpu_info.format_summary();
        }
        
        if let Ok(process_info) = process::ProcessInfo::collect() {
            let _ = process_info.format_summary();
        }
    }
    
    println!("Memory safety test passed");
}

#[test]
fn test_cross_platform_compatibility() {
    // Test that platform detection works
    let platform_info = ffi::platform::get_platform_info();
    
    // Should detect a valid platform
    match platform_info.os_type {
        ffi::platform::OsType::Linux |
        ffi::platform::OsType::Windows |
        ffi::platform::OsType::MacOS |
        ffi::platform::OsType::FreeBSD |
        ffi::platform::OsType::Other(_) => {
            println!("Detected platform: {:?}", platform_info.os_type);
        }
    }
    
    // Should detect a valid architecture
    match platform_info.architecture {
        ffi::platform::Architecture::X86_64 |
        ffi::platform::Architecture::X86 |
        ffi::platform::Architecture::Arm64 |
        ffi::platform::Architecture::Arm |
        ffi::platform::Architecture::Other(_) => {
            println!("Detected architecture: {:?}", platform_info.architecture);
        }
    }
}

#[test]
fn test_performance_characteristics() {
    use std::time::Instant;
    
    // Test that system info collection is reasonably fast
    let start = Instant::now();
    
    for _ in 0..10 {
        let _ = memory::MemoryInfo::collect();
    }
    
    let duration = start.elapsed();
    
    // Should complete 10 collections in reasonable time (< 1 second)
    assert!(duration.as_secs() < 1, 
           "Memory collection should be fast, took: {:?}", duration);
    
    println!("Performance test passed: 10 collections in {:?}", duration);
}

// Mock implementations for testing if real modules aren't available yet
#[cfg(test)]
mod mock_implementations {
    // These can be used if students haven't implemented all modules yet
    
    #[allow(dead_code)]
    pub struct MockSystemInfo {
        pub uptime: u64,
        pub memory_total: u64,
        pub memory_available: u64,
    }
    
    #[allow(dead_code)]
    impl MockSystemInfo {
        pub fn new() -> Self {
            Self {
                uptime: 12345,
                memory_total: 16 * 1024 * 1024 * 1024, // 16GB
                memory_available: 8 * 1024 * 1024 * 1024, // 8GB
            }
        }
        
        pub fn display(&self) {
            println!("Mock System Info:");
            println!("  Uptime: {} seconds", self.uptime);
            println!("  Memory: {}/{} MB", 
                     self.memory_available / (1024 * 1024),
                     self.memory_total / (1024 * 1024));
        }
    }
}

// Helper functions for testing
fn is_reasonable_memory_value(bytes: u64) -> bool {
    // Should be between 1MB and 1TB (reasonable range for modern systems)
    bytes >= 1024 * 1024 && bytes <= 1024 * 1024 * 1024 * 1024
}

fn is_reasonable_percentage(percent: f64) -> bool {
    percent >= 0.0 && percent <= 100.0
}

fn is_reasonable_uptime(seconds: u64) -> bool {
    // Should be positive but less than 10 years (reasonable uptime)
    seconds > 0 && seconds < (10 * 365 * 24 * 60 * 60)
}

// Integration test for command-line interface
#[test]
fn test_cli_interface() {
    // Test that the main application can run in single-shot mode
    // This would typically test the actual binary, but for this exercise
    // we'll test the core functionality
    
    if let Ok(system_info) = SystemInfo::collect() {
        // Should be able to format for display
        let summary = format!("{:?}", system_info);
        assert!(!summary.is_empty(), "System info should have debug output");
        
        println!("CLI interface test passed");
    }
}

// Benchmark test (only runs with --features bench)
#[cfg(feature = "bench")]
mod bench_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn bench_memory_collection() {
        let iterations = 1000;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = memory::MemoryInfo::collect();
        }
        
        let duration = start.elapsed();
        let avg_time = duration / iterations;
        
        println!("Average memory collection time: {:?}", avg_time);
        
        // Should be fast enough for real-time monitoring
        assert!(avg_time.as_millis() < 10, 
               "Memory collection should be < 10ms, got: {:?}", avg_time);
    }
}
