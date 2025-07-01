# System Monitor Project - Level 1 Hints 🟢

## Understanding the System Monitor Project

You're building a system monitoring tool that reads system information - CPU usage, memory stats, and process information. This combines all the systems programming concepts you've learned!

## Key Questions to Consider

1. **Where does system info come from?** `/proc` filesystem on Linux, system calls, or APIs
2. **How do we parse text data efficiently?** String processing without excessive allocations
3. **What about cross-platform support?** Platform-specific implementations with common interface

## Project Structure Overview

```
src/
├── main.rs          # CLI interface and main logic
├── cpu.rs           # CPU usage monitoring  
├── memory.rs        # Memory statistics
├── process.rs       # Process information
└── ffi.rs           # System call interfaces
```

## Core Concepts Applied

### From Module 1-3 (Foundations)
- **Error handling**: Graceful failure when files don't exist
- **String parsing**: Efficient text processing
- **Structs**: Organizing system data

### From Module 4 (Systems Programming)
- **Memory layout**: Efficient data structures
- **Unsafe code**: System call interfaces
- **FFI**: Calling system APIs

## Common Issues to Fix

1. **File reading errors**: `/proc` files might not exist
2. **Parsing failures**: System data format variations
3. **Permission issues**: Some system info requires root
4. **Platform differences**: Linux vs macOS vs Windows

## Development Strategy

1. **Start simple**: Get basic file reading working
2. **Add error handling**: Handle missing files gracefully
3. **Improve parsing**: Make text processing robust
4. **Add features**: CPU usage, memory stats, process list
5. **Polish interface**: Better CLI and output formatting

## C# Comparison

In C#, you might use:
```csharp
// System information
var process = Process.GetCurrentProcess();
var memoryUsage = process.WorkingSet64;

// Performance counters
var cpuCounter = new PerformanceCounter("Processor", "% Processor Time", "_Total");
var cpuUsage = cpuCounter.NextValue();
```

In Rust, you're building this from scratch using system files and APIs!

## Testing Strategy

- **Unit tests**: Test parsing functions with sample data
- **Integration tests**: Test against real system files
- **Error cases**: Test with missing or malformed files
- **Cross-platform**: Test on different systems

Need more specific guidance? Check Level 2 hints after trying for 15+ more minutes!