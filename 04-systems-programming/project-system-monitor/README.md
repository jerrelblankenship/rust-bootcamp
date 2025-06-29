# System Monitor Project - IMPROVED VERSION

Build a real-time system resource monitor by fixing specific bugs in partially working code! This project demonstrates all systems programming concepts through focused debugging rather than building everything from scratch.

## 🎯 What You'll Build

A complete system monitoring application that displays:
- **Memory Usage** - Total, used, available memory with percentages
- **CPU Utilization** - Real-time CPU usage percentage  
- **Process Count** - Number of running processes
- **System Uptime** - How long the system has been running
- **Real-time Updates** - Refreshes every few seconds

## 🚀 Getting Started (Much Easier Now!)

### **Step 1: Try to Build (You'll see specific errors)**
```bash
cd project-system-monitor
cargo build
```

**What's Different**: Instead of "module not found" errors, you'll see specific bugs in working code that you can fix step by step.

### **Step 2: Fix ONE Error at a Time**
The codebase is mostly working with specific bugs to fix:
- Memory parsing has an off-by-one error
- CPU calculation has a division by zero bug
- Display formatting has missing fields
- Error handling needs proper From implementations

### **Step 3: Progressive Fixes**
Each file has 1-2 focused bugs rather than being completely broken.

## 📋 Step-by-Step Fix Guide (Much More Focused)

### **Phase 1: Memory Module Bug Fixes (15 minutes)**

#### Bug Fix 1.1: Memory Parsing Error
- **File**: `src/memory.rs`
- **Problem**: Off-by-one error in parsing `/proc/meminfo`
- **Fix**: Check array bounds before accessing `parts[1]`

#### Bug Fix 1.2: Missing Error Conversion
- **Problem**: `From<ParseIntError>` not implemented
- **Fix**: Add the missing trait implementation

### **Phase 2: CPU Module Bug Fixes (15 minutes)**

#### Bug Fix 2.1: Division by Zero
- **File**: `src/cpu.rs`
- **Problem**: CPU calculation crashes when total is zero
- **Fix**: Add zero check before division

#### Bug Fix 2.2: File Reading Error
- **Problem**: `/proc/stat` reading doesn't handle file not found
- **Fix**: Proper error handling for missing files

### **Phase 3: Display and Integration (15 minutes)**

#### Bug Fix 3.1: Display Formatting
- **File**: `src/main.rs`
- **Problem**: Display function missing field implementations
- **Fix**: Complete the formatting functions

#### Bug Fix 3.2: Update Loop
- **Problem**: Real-time updates don't work
- **Fix**: Fix the timer and screen clearing logic

## 🔧 What Makes This Better

### **Before (Too Hard)**
```rust
// COMPLETELY MISSING - create everything from scratch
mod memory;    // ERROR: Module not found
mod cpu;       // ERROR: Module not found  
mod process;   // ERROR: Module not found
```

### **After (Just Right)**
```rust
// PARTIALLY WORKING - fix specific bugs
mod memory;    // ✅ Exists but has parsing bug
mod cpu;       // ✅ Exists but has division by zero
mod process;   // ✅ Exists but has error handling issue
```

## 🐛 Example Bug Fixes You'll Make

### Bug Fix Example 1: Memory Parsing
```rust
// BROKEN: Can crash on malformed input
let parts: Vec<&str> = line.split_whitespace().collect();
let value = parts[1].parse::<u64>()?;  // BUG: What if parts.len() < 2?

// FIXED: Safe parsing with bounds check
let parts: Vec<&str> = line.split_whitespace().collect();
if parts.len() >= 2 {
    let value = parts[1].parse::<u64>()?;
    // Use value...
}
```

### Bug Fix Example 2: CPU Calculation
```rust
// BROKEN: Division by zero crash
fn calculate_cpu_usage(idle: u64, total: u64) -> f64 {
    let used = total - idle;
    (used as f64 / total as f64) * 100.0  // BUG: What if total is 0?
}

// FIXED: Safe division with zero check
fn calculate_cpu_usage(idle: u64, total: u64) -> f64 {
    if total == 0 {
        return 0.0;
    }
    let used = total - idle;
    (used as f64 / total as f64) * 100.0
}
```

## 🏆 Success Criteria

You've completed the project successfully when:

- ✅ **Compiles without errors** - All bugs are fixed
- ✅ **Displays real data** - Shows actual system metrics
- ✅ **Updates in real-time** - Refreshes every few seconds
- ✅ **Handles errors gracefully** - Doesn't crash on edge cases
- ✅ **Professional output** - Clean, readable display format

## 🧪 Testing Your Fixes

### **After Each Bug Fix**
```bash
cargo build   # Should show fewer errors each time
```

### **Integration Testing**
```bash
cargo run     # Should display system information
```

### **Edge Case Testing**
```bash
# Test error handling
chmod 000 /proc/meminfo  # Remove read permissions (Linux)
cargo run                # Should handle gracefully
chmod 644 /proc/meminfo  # Restore permissions
```

## 🆘 When You Get Stuck

### **Common Bug Types and Solutions**

**"Index out of bounds" errors:**
- Always check array length before accessing elements
- Use `.get(index)` instead of `[index]` for safe access

**"Division by zero" panics:**
- Check denominator before division
- Return sensible defaults for edge cases

**"From trait not implemented" errors:**
- Add missing `impl From<SourceError> for TargetError`
- Use `?` operator for error propagation

**"File not found" on non-Linux:**
- Use conditional compilation (`#[cfg(target_os = "linux")]`)
- Provide appropriate fallbacks for other platforms

## 🔗 Real-World Applications

These bug patterns teach skills used in:
- **Database Systems** (safe parsing, error handling)
- **Web Servers** (resource monitoring, graceful degradation)  
- **Game Engines** (performance monitoring, real-time updates)
- **System Administration Tools** (cross-platform compatibility)

## ➡️ What's Next?

After completing this project:

1. **Add features** - Network monitoring, disk usage
2. **Optimize performance** - Reduce allocations, cache calculations  
3. **Cross-platform support** - Windows and macOS implementations
4. **GUI version** - Use a Rust GUI framework
5. **Move to Module 05** - Apply systems skills to concurrency

---

**Ready for focused debugging?** Start by running `cargo build` and fixing the first error! 🔧