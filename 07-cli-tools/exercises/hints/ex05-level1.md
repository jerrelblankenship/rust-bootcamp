# Exercise 5 - Level 1 Hint (Gentle Guidance)

## Adding Progress Bars to Your CLI Tool

Great progress! Now you're focusing on user experience - one of the most important aspects of CLI design.

### Understanding the Problem

Your tool currently processes files silently with just dots (`.....`) which is poor user experience. Users want to know:
- How much progress has been made
- How long it will take to complete
- Whether the program is still working

### Key Concepts to Consider

**1. When to Show Progress:**
- Long-running operations (>2 seconds)
- File processing with known size
- Operations where user waits

**2. What NOT to Show:**
- Progress bars in pipes (`program | other_tool`)
- Progress when output is redirected (`program > file`)
- Progress in non-interactive terminals

**3. Progress Components:**
- Visual bar: `[████████░░░░░░░░]`
- Percentage: `45%`
- Current/Total: `450MB/1GB`
- Rate: `15.3 MB/s`
- ETA: `2m 30s`

### Starting Points

**Check if you should show progress:**
```rust
use std::io::IsTerminal;

// Only show progress in interactive terminals
if std::io::stderr().is_terminal() {
    // Show progress bar
}
```

**Basic progress calculation:**
```rust
let progress_percent = (bytes_processed * 100) / total_bytes;
```

### C# Comparison

In C#, you might use:
```csharp
// Check if console output is redirected
if (!Console.IsOutputRedirected) {
    // Show progress
    Console.Write($"\rProgress: {percent}%");
}
```

In Rust, we use `IsTerminal` trait for similar detection.

### Questions to Guide You

1. How do you detect if the terminal is interactive?
2. What's the difference between stdout and stderr for progress?
3. How do you calculate percentage from bytes processed?
4. How do you update the same line instead of printing new lines?

### Progress Bar Libraries

Rust has excellent progress bar libraries:
- `indicatif` - Professional progress bars
- Manual implementation using ANSI codes

Try starting with a simple percentage display first, then enhance it with a visual bar!

Remember: Progress bars should enhance the user experience, not slow down the actual operation.