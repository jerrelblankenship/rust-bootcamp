# Exercise 8 - Level 1 Hint (Gentle Guidance)

## Building Git-Style Subcommands

Fantastic progress! This final exercise teaches you to build complex CLI tools with subcommands like `git`, `cargo`, or `docker`.

### Understanding the Problem

Your tool currently has a flat command structure, but professional CLI tools use hierarchical subcommands:

```bash
# Flat (simple)
my-tool --input file.txt --format json

# Hierarchical (professional)
my-tool file convert --input file.txt --format json
my-tool config set editor vim
my-tool database migrate --version 1.2.3
```

### Key Concepts to Consider

**1. Command Hierarchies:**
- Top-level commands (file, config, database)
- Sub-commands (convert, set, migrate)
- Command-specific arguments and flags

**2. Shared vs Specific Options:**
- Global flags (--verbose, --config)
- Command-specific flags (--format for convert)
- Help for each level

**3. User Experience:**
- Intuitive command grouping
- Consistent naming patterns
- Helpful error messages
- Command discovery

### Starting Points

**Basic subcommand structure:**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    File {
        #[clap(subcommand)]
        action: FileCommands,
    },
    Config {
        #[clap(subcommand)]
        action: ConfigCommands,
    },
}
```

**Nested subcommands:**
```rust
#[derive(Subcommand)]
enum FileCommands {
    Convert { input: String, format: String },
    Validate { path: String },
}
```

### C# Comparison

In C#, you might use:
```csharp
// Flat command structure
[Verb("convert", HelpText = "Convert files")]
class ConvertOptions { }

// Rust supports deeper nesting naturally
match cli.command {
    Commands::File { action } => match action {
        FileCommands::Convert { input, format } => { }
    }
}
```

### Questions to Guide You

1. How do you organize related commands into groups?
2. How do you handle global flags vs command-specific flags?
3. How do you provide help at each command level?
4. How do you validate arguments for specific subcommands?

Start by grouping your existing functionality into logical command categories!