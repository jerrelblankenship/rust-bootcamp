# Exercise 6 - Level 1 Hint (Gentle Guidance)

## Adding Color Output to Your CLI Tool

Excellent work! Now you're enhancing the visual experience of your CLI tool with colors.

### Understanding the Problem

Your tool currently outputs plain text, but modern CLI tools use colors to:
- Highlight important information (errors in red, success in green)
- Improve readability and user experience
- Provide visual hierarchy in output

### Key Concepts to Consider

**1. When to Use Colors:**
- Error messages (red)
- Success messages (green)
- Warnings (yellow)
- Important information (bold/bright)
- Different data types (different colors)

**2. When NOT to Use Colors:**
- When output is piped to another program
- When output is redirected to a file
- When NO_COLOR environment variable is set
- In non-interactive terminals

**3. Terminal Capability Detection:**
- Some terminals don't support colors
- Some environments explicitly disable colors
- Respect user preferences

### Starting Points

**Basic color detection:**
```rust
use std::io::IsTerminal;

// Only use colors in interactive terminals
if std::io::stdout().is_terminal() {
    // Use colors
} else {
    // Plain text only
}
```

**Environment variable checks:**
```rust
// Respect NO_COLOR standard
if std::env::var("NO_COLOR").is_err() {
    // Colors are allowed
}
```

### C# Comparison

In C#, you might check:
```csharp
if (!Console.IsOutputRedirected && 
    Environment.GetEnvironmentVariable("NO_COLOR") == null) {
    Console.ForegroundColor = ConsoleColor.Red;
    Console.WriteLine("Error message");
    Console.ResetColor();
}
```

In Rust, we use ANSI color codes or the `colored` crate for similar functionality.

### Questions to Guide You

1. How do you detect if the terminal supports colors?
2. What's the difference between stdout and stderr for colored output?
3. How do you create ANSI color codes manually?
4. When should you avoid using colors entirely?

### Color Standards

- **Red**: Errors, failures, deletions
- **Green**: Success, additions, completion
- **Yellow**: Warnings, modifications
- **Blue**: Information, links
- **Cyan**: Highlights, special values
- **Bold**: Important text, headers

Try starting with basic red/green for errors/success, then expand to a full color scheme!