# CLI Patterns and Best Practices

Essential patterns for building professional-grade CLI tools that users love.

## 🎯 Core CLI Design Principles

### 1. **Be Helpful by Default**
- Show helpful error messages with suggestions
- Provide comprehensive `--help` for every command
- Include examples in help text
- Use progress indicators for long operations

### 2. **Follow Unix Philosophy**
- Do one thing well
- Work with other tools (pipes, redirects)
- Be quiet unless there's something important to say
- Exit with meaningful status codes

### 3. **Respect User Environment**
- Detect terminal capabilities (colors, Unicode)
- Honor environment variables (`NO_COLOR`, `PAGER`)
- Use platform-appropriate config directories
- Handle signals gracefully (Ctrl+C, SIGPIPE)

## 🏗️ Common CLI Architecture Patterns

### Command Hierarchy Pattern

For complex tools with multiple operations:

```rust
#[derive(Parser)]
struct Cli {
    #[clap(short, long, global = true)]
    verbose: bool,
    
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// File operations
    File {
        #[clap(subcommand)]
        action: FileCommands,
    },
    /// Database operations  
    Database {
        #[clap(subcommand)]
        action: DatabaseCommands,
    },
}

#[derive(Subcommand)]
enum FileCommands {
    Process { input: PathBuf },
    Validate { files: Vec<PathBuf> },
}
```

**Use when:**
- Tool has multiple distinct areas of functionality
- Commands have different argument patterns
- You want git/cargo-style command structure

### Flag-Based Pattern

For simpler tools with variations of one main operation:

```rust
#[derive(Parser)]
struct Cli {
    /// Input files
    files: Vec<PathBuf>,
    
    /// Output format
    #[clap(short, long, value_enum, default_value = "json")]
    format: OutputFormat,
    
    /// Validation mode
    #[clap(short, long)]
    strict: bool,
    
    /// Recurse into directories
    #[clap(short, long)]
    recursive: bool,
}
```

**Use when:**
- Tool does one main thing with variations
- Arguments are mostly independent flags
- Simple, focused functionality

### Plugin/Extension Pattern

For extensible tools:

```rust
#[derive(Subcommand)]
enum Commands {
    /// Built-in commands
    Build,
    Test,
    
    /// External subcommands (plugin-*) 
    #[clap(external_subcommand)]
    External(Vec<String>),
}

fn handle_external_command(args: Vec<String>) -> Result<()> {
    let command_name = format!("my-tool-{}", args[0]);
    let status = Command::new(command_name)
        .args(&args[1..])
        .status()?;
    std::process::exit(status.code().unwrap_or(1));
}
```

**Use when:**
- Want to allow third-party extensions
- Core tool should remain focused
- Different teams may contribute commands

## 🎨 User Experience Patterns

### Progressive Disclosure

Start simple, add complexity as needed:

```rust
// Basic usage
my-tool process file.txt

// With options
my-tool process file.txt --output results/ --format json

// Advanced features
my-tool process file.txt --output results/ --format json --parallel 4 --filter "*.rs"
```

**Implementation:**
- Make common use cases simple
- Use sensible defaults
- Group related options
- Hide advanced features behind flags

### Confirmations for Destructive Operations

```rust
#[derive(Parser)]
struct DeleteCommand {
    files: Vec<PathBuf>,
    
    /// Skip confirmation prompt
    #[clap(short, long)]
    yes: bool,
    
    /// Show what would be deleted without doing it
    #[clap(long)]
    dry_run: bool,
}

fn delete_files(cmd: DeleteCommand) -> Result<()> {
    if cmd.dry_run {
        for file in &cmd.files {
            println!("Would delete: {}", file.display());
        }
        return Ok(());
    }
    
    if !cmd.yes {
        let prompt = format!("Delete {} files?", cmd.files.len());
        if !Confirm::new().with_prompt(prompt).interact()? {
            println!("Cancelled");
            return Ok(());
        }
    }
    
    // Perform deletion
    for file in &cmd.files {
        fs::remove_file(file)?;
        println!("Deleted: {}", file.display());
    }
    
    Ok(())
}
```

### Smart Defaults with Override

```rust
fn get_config_path() -> PathBuf {
    // 1. Command line argument
    if let Some(path) = std::env::args().find(|arg| arg.starts_with("--config=")) {
        return PathBuf::from(path.strip_prefix("--config=").unwrap());
    }
    
    // 2. Environment variable
    if let Ok(path) = std::env::var("MY_TOOL_CONFIG") {
        return PathBuf::from(path);
    }
    
    // 3. Standard locations
    if let Some(proj_dirs) = ProjectDirs::from("", "", "my-tool") {
        let config_path = proj_dirs.config_dir().join("config.toml");
        if config_path.exists() {
            return config_path;
        }
    }
    
    // 4. Current directory
    let local_config = PathBuf::from("my-tool.toml");
    if local_config.exists() {
        return local_config;
    }
    
    // 5. Built-in default
    proj_dirs.unwrap().config_dir().join("config.toml")
}
```

## 🔧 Implementation Patterns

### Error Aggregation Pattern

For operations that can have multiple failures:

```rust
#[derive(Debug)]
struct ValidationResults {
    successes: Vec<PathBuf>,
    errors: Vec<ValidationError>,
}

fn validate_files(files: Vec<PathBuf>) -> ValidationResults {
    let mut results = ValidationResults {
        successes: Vec::new(),
        errors: Vec::new(),
    };
    
    for file in files {
        match validate_file(&file) {
            Ok(()) => results.successes.push(file),
            Err(e) => results.errors.push(ValidationError {
                file: file.clone(),
                error: e,
            }),
        }
    }
    
    results
}

fn report_results(results: ValidationResults) {
    println!("✅ Validated {} files successfully", results.successes.len());
    
    if !results.errors.is_empty() {
        println!("❌ {} files had errors:", results.errors.len());
        for error in &results.errors {
            println!("  {}: {}", error.file.display(), error.error);
        }
    }
}
```

### Streaming Processing Pattern

For large datasets:

```rust
use std::io::{BufRead, BufReader};

fn process_large_file<R: BufRead>(reader: R) -> Result<ProcessingStats> {
    let mut stats = ProcessingStats::default();
    let pb = ProgressBar::new_spinner();
    
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        
        // Update progress every 1000 lines
        if line_num % 1000 == 0 {
            pb.set_message(format!("Processed {} lines", line_num));
        }
        
        // Process line
        match process_line(&line) {
            Ok(result) => stats.record_success(result),
            Err(e) => stats.record_error(line_num, e),
        }
        
        // Handle SIGPIPE gracefully
        if let Err(e) = io::stdout().flush() {
            if e.kind() == io::ErrorKind::BrokenPipe {
                return Ok(stats);
            }
        }
    }
    
    pb.finish_with_message("Processing complete");
    Ok(stats)
}
```

### Configuration Validation Pattern

```rust
impl Config {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validate port range
        if self.server.port == 0 || self.server.port > 65535 {
            errors.push(format!("Invalid port: {}. Must be 1-65535", self.server.port));
        }
        
        // Validate required fields
        if self.database.url.is_empty() {
            errors.push("Database URL cannot be empty".to_string());
        }
        
        // Validate file paths exist
        if !Path::new(&self.data_dir).exists() {
            errors.push(format!("Data directory does not exist: {}", self.data_dir));
        }
        
        // Validate enum values
        match self.log_level.as_str() {
            "debug" | "info" | "warn" | "error" => {},
            _ => errors.push(format!("Invalid log level: {}. Must be debug, info, warn, or error", self.log_level)),
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn load_and_validate_config() -> Result<Config> {
    let config = Config::load()?;
    
    if let Err(validation_errors) = config.validate() {
        eprintln!("❌ Configuration validation failed:");
        for error in &validation_errors {
            eprintln!("  • {}", error);
        }
        std::process::exit(1);
    }
    
    Ok(config)
}
```

## 🚀 Performance Patterns

### Parallel Processing

```rust
use rayon::prelude::*;

fn process_files_parallel(files: Vec<PathBuf>) -> Result<Vec<ProcessResult>> {
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar());
    
    let results: Result<Vec<_>, _> = files
        .par_iter()
        .map(|file| {
            let result = process_file(file);
            pb.inc(1);
            result
        })
        .collect();
    
    pb.finish();
    results
}
```

### Lazy Loading

```rust
struct LazyConfig {
    config: OnceCell<Config>,
}

impl LazyConfig {
    fn get(&self) -> &Config {
        self.config.get_or_init(|| {
            Config::load().unwrap_or_default()
        })
    }
}

static CONFIG: LazyConfig = LazyConfig {
    config: OnceCell::new(),
};
```

## 📋 CLI Usability Checklist

### Essential Features
- [ ] Comprehensive `--help` with examples
- [ ] `--version` flag with semantic versioning
- [ ] Proper exit codes (0=success, non-zero=error)
- [ ] Meaningful error messages with suggestions
- [ ] Support for `--dry-run` on destructive operations

### User Experience
- [ ] Progress indication for operations >2 seconds
- [ ] Colors that respect terminal capabilities
- [ ] Quiet mode (`-q`) for scripting
- [ ] Verbose mode (`-v`) for debugging
- [ ] Confirmation prompts for dangerous operations

### Integration
- [ ] Reads from stdin when appropriate
- [ ] Outputs to stdout/stderr correctly
- [ ] Works in Unix pipelines
- [ ] Handles SIGPIPE gracefully
- [ ] Supports configuration files and environment variables

### Quality
- [ ] Cross-platform compatibility
- [ ] Proper Unicode handling
- [ ] Memory efficient for large inputs
- [ ] Fast startup time (<100ms for simple operations)
- [ ] Shell completion scripts

## 🎓 Learning Resources

### Inspiration from Great CLI Tools
- **ripgrep** - Fast, user-friendly grep alternative
- **fd** - Intuitive find replacement
- **bat** - Cat with syntax highlighting
- **exa** - Modern ls with great UX
- **delta** - Beautiful diff viewer
- **hyperfine** - Command-line benchmarking

### Key Takeaways
1. **Start with user experience** - What should the tool feel like to use?
2. **Fail fast with helpful messages** - Don't make users guess what went wrong
3. **Provide escape hatches** - Always offer ways to override defaults
4. **Test in real environments** - Different terminals, platforms, and use cases
5. **Iterate based on feedback** - CLI tools improve through real-world usage

Remember: The best CLI tools are invisible when they work and helpful when they don't!