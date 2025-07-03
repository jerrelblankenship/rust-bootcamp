# Exercise 4 - Level 1 Hint (Gentle Guidance)

## Making Your CLI Pipe-Friendly

You're working on a fundamental Unix philosophy: tools should work well together in pipelines!

### Understanding the Problem

Your current tool breaks several pipeline conventions:
1. It always requires a filename (can't read from stdin)
2. It prints status messages that pollute pipeline output
3. It doesn't handle signals properly
4. Exit codes don't follow conventions

### Key Concepts to Consider

**1. Reading from stdin vs files:**
- When no filename is provided, read from stdin
- This allows: `echo "data" | your_tool`

**2. Silent by default:**
- Only output the actual processed data to stdout
- Status messages go to stderr (if needed)
- Add a `--verbose` flag for detailed output

**3. Unix pipeline conventions:**
```bash
# Your tool should work in chains like:
cat data.txt | your_tool | sort | uniq > output.txt
```

### Starting Points

**Check if stdin is available:**
```rust
use std::io::{self, IsTerminal};

// If no file argument AND stdin is piped
if args.len() == 1 && !io::stdin().is_terminal() {
    // Read from stdin
}
```

**Separate output streams:**
```rust
// Data output (for pipelines)
println!("processed data");

// Status messages (not in pipeline)
eprintln!("Processing...");
```

### C# Comparison

In C#, you might check:
```csharp
if (Console.IsInputRedirected) {
    // Read from stdin
}
```

In Rust, we use the `IsTerminal` trait to detect if stdin is from a pipe.

### Questions to Guide You

1. How can you make the tool read from stdin when no file is specified?
2. Where should status messages go to avoid breaking pipelines?
3. What's the difference between stdout and stderr?
4. How do you detect if your output is going to a pipe vs terminal?

Try implementing just the stdin reading first, then move on to cleaning up the output messages!

Remember: A good Unix tool is silent when it succeeds and only speaks when there's a problem.