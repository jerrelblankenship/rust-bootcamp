# Exercise 1 Hints - Level 2: Specific Guidance 🟡

## Concrete Steps for Argument Parsing

**Ready for more specific help?** Here's how to tackle the common issues.

### 🔧 Dependencies Fix

Your `Cargo.toml` needs this dependency:
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

The `derive` feature is crucial - it enables the `#[derive(Parser)]` macro.

### 🏗️ Commands Enum Structure

The missing `Commands` enum should look like this:
```rust
#[derive(Subcommand)]
enum Commands {
    Process {
        /// Input file to process
        input: PathBuf,
        
        /// Output file (optional)
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    
    Validate {
        /// Files to validate
        files: Vec<PathBuf>,
        
        /// Strict validation mode
        #[clap(short, long)]
        strict: bool,
    },
}
```

### 🎯 CLI Structure Fix

The derive attributes need actual values:
```rust
#[derive(Parser)]
#[clap(
    name = "file-processor",
    version = "1.0.0", 
    about = "A file processing tool"
)]
pub struct Cli {
    // ... fields
}
```

### 🔄 Error Handling Pattern

Replace panicky parsing with graceful handling:
```rust
// Bad (current)
let cli = Cli::parse();  // Panics on bad input

// Better approach  
fn main() {
    let cli = Cli::parse();  // clap handles errors gracefully
    
    match execute_command(&cli) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
```

### 🎨 Command Execution Pattern

```rust
fn execute_command(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    
    match &cli.command {
        Commands::Process { input, output } => {
            println!("Processing {} -> {:?}", input.display(), output);
            // TODO: Actual processing logic
            Ok(())
        },
        Commands::Validate { files, strict } => {
            println!("Validating {} files (strict: {})", files.len(), strict);
            // TODO: Actual validation logic
            Ok(())
        },
    }
}
```

### 🧪 Testing Your Progress

Try these commands once it compiles:
```bash
cargo run -- --help                    # Should show help
cargo run -- process input.txt         # Should work
cargo run -- validate file1.txt file2.txt --strict  # Should work
cargo run -- --verbose process input.txt  # Should show verbose output
```

### 🤔 C# Mental Model

Think of this structure:
```csharp
// C# equivalent concept
public class Options 
{
    [Option('v', "verbose")]
    public bool Verbose { get; set; }
    
    [Option('o', "output")]  
    public string Output { get; set; }
}

// Rust equivalent
struct Cli {
    #[clap(short, long)]
    verbose: bool,
    
    #[clap(short, long)]
    output: Option<String>,
}
```

### ⏱️ Still Stuck?

If you're still having trouble after **30+ minutes**, check Level 3 hints for more complete examples.

### 🎯 What Should Work Now

- Basic argument parsing
- Help generation (`--help`)
- Subcommand dispatch
- Error handling (graceful failures)

The foundation should be solid before adding complexity!