# Project Hints - Level 2: Specific Implementation 🟡

## Concrete Steps for the Developer Tools CLI

**Need more specific guidance?** Here's how to tackle the common implementation challenges.

### 🔧 Essential Dependencies to Add

Uncomment these in your `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.0", features = ["derive", "env"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.7"
colored = "2.0"
indicatif = "0.17"
anyhow = "1.0"  # For easy error handling
walkdir = "2.3"  # For file operations
```

### 🏗️ Module Structure Template

Create these files with basic stubs:

**src/commands/git.rs**:
```rust
use clap::Subcommand;

#[derive(Subcommand)]
pub enum GitCommands {
    Status,
    Branches,
    Stats,
}

pub fn execute(command: GitCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        GitCommands::Status => {
            println!("Git status - TODO: implement");
            Ok(())
        },
        GitCommands::Branches => {
            println!("Git branches - TODO: implement");
            Ok(())
        },
        GitCommands::Stats => {
            println!("Git stats - TODO: implement");
            Ok(())
        },
    }
}
```

**src/config.rs**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub editor: EditorConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditorConfig {
    pub command: String,
}

#[derive(Debug, Deserialize, Serialize)]  
pub struct ServerConfig {
    pub default_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig {
                command: "vim".to_string(),
            },
            server: ServerConfig {
                default_port: 8080,
            },
        }
    }
}
```

### 🎯 Start with File Commands

Focus on getting this working first:

```rust
// In src/commands/file.rs
#[derive(clap::ValueEnum, Clone)]
pub enum ProcessMode {
    Copy,
    Transform, 
    Analyze,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ValidationRules {
    Basic,
    Strict,
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Csv,
}

fn process_files(
    input: Vec<PathBuf>,
    output: Option<PathBuf>, 
    mode: ProcessMode,
    force: bool
) -> Result<(), Box<dyn std::error::Error>> {
    use indicatif::{ProgressBar, ProgressStyle};
    
    let pb = ProgressBar::new(input.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap());
    
    for (i, file) in input.iter().enumerate() {
        pb.set_message(format!("Processing {}", file.display()));
        
        // TODO: Actual file processing based on mode
        std::thread::sleep(std::time::Duration::from_millis(100)); // Simulate work
        
        pb.inc(1);
    }
    
    pb.finish_with_message("Processing complete!");
    Ok(())
}
```

### 🎨 Error Handling Pattern

```rust
// In src/error.rs  
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    
    #[error("Invalid configuration: {message}")]
    ConfigError { message: String },
    
    #[error("Command failed: {message}")]
    CommandError { message: String },
}

pub type CliResult<T> = Result<T, CliError>;
```

### 🔧 Main Function Pattern

```rust
fn main() {
    // Set up colored output
    if std::env::var("NO_COLOR").is_err() {
        colored::control::set_override(true);
    }
    
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging based on verbose flag
    if cli.verbose {
        println!("{} Verbose mode enabled", "Info:".blue().bold());
    }
    
    // Load configuration if specified
    let _config = if let Some(config_path) = &cli.config {
        println!("{} Loading config from: {}", "Info:".blue().bold(), config_path.display());
        // TODO: Load config from file
        Config::default()
    } else {
        Config::default()
    };
    
    // Dispatch to command
    match cli.command {
        Commands::File { action } => crate::commands::file::execute(action),
        Commands::Git { action } => crate::commands::git::execute(action),
        Commands::Server { action } => crate::commands::server::execute(action),
        Commands::Config { action } => crate::commands::config::execute(action),
    }
}
```

### 🧪 Testing Your Progress

Test each piece as you implement it:

```bash
# Test basic structure
cargo build

# Test help
cargo run -- --help
cargo run -- file --help

# Test one command
cargo run -- file process test.txt

# Test with options
cargo run -- --verbose file process test.txt --mode copy
```

### 🎯 Implementation Priority

1. **Get it compiling**: Focus on stubs that compile first
2. **File commands**: Start with the most straightforward operations
3. **Error handling**: Apply patterns from ex02 throughout
4. **Progress and colors**: Add UX polish from ex05/ex06
5. **Configuration**: Add config support from ex03
6. **Remaining commands**: Implement git, server, config commands

### ⏱️ Still Struggling?

If you're still having trouble after **45+ minutes**, check Level 3 hints for more complete implementations.

### 🎯 What Should Work Now

- Basic compilation and argument parsing
- File commands with progress bars
- Proper error handling with colored output
- Configuration loading (even if basic)
- Help generation for all commands

The foundation should be solid and one command should work end-to-end!