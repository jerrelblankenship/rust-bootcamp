# C# to Rust CLI Development Guide

A comprehensive comparison of console application patterns between C# and Rust.

## 🎯 Overview

Coming from C# console development? This guide maps familiar C# patterns to their Rust equivalents, helping you leverage your existing knowledge while learning Rust CLI development.

## 📋 Command Line Parsing

### C# Approach (CommandLineParser)

```csharp
using CommandLine;

[Verb("process", HelpText = "Process files")]
public class ProcessOptions
{
    [Option('i', "input", Required = true, HelpText = "Input file")]
    public string InputFile { get; set; }
    
    [Option('o', "output", HelpText = "Output file")]
    public string OutputFile { get; set; }
    
    [Option('v', "verbose", HelpText = "Verbose output")]
    public bool Verbose { get; set; }
}

// Usage
static void Main(string[] args)
{
    Parser.Default.ParseArguments<ProcessOptions>(args)
        .WithParsed(opts => ProcessFiles(opts))
        .WithNotParsed(errs => HandleErrors(errs));
}
```

### Rust Equivalent (clap)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "file-processor", about = "Process files")]
struct Cli {
    #[clap(short, long, help = "Input file")]
    input: String,
    
    #[clap(short, long, help = "Output file")]
    output: Option<String>,
    
    #[clap(short, long, help = "Verbose output")]
    verbose: bool,
    
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process files
    Process {
        #[clap(long)]
        mode: String,
    },
}

// Usage
fn main() {
    let cli = Cli::parse();  // Automatic error handling
    
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    
    match cli.command {
        Commands::Process { mode } => process_files(&cli.input, cli.output.as_ref(), &mode),
    }
}
```

**Key Differences:**
- **C#**: Separate classes for each verb, manual error handling
- **Rust**: Enum-based subcommands, automatic error handling and help generation
- **C#**: Properties with attributes
- **Rust**: Fields with derive macros

## 🔧 Configuration Management

### C# Approach (appsettings.json + IConfiguration)

```csharp
// appsettings.json
{
  "Database": {
    "ConnectionString": "...",
    "Timeout": 30
  },
  "Logging": {
    "Level": "Information"
  }
}

// Configuration class
public class AppSettings
{
    public DatabaseSettings Database { get; set; }
    public LoggingSettings Logging { get; set; }
}

// Usage
var config = new ConfigurationBuilder()
    .AddJsonFile("appsettings.json")
    .AddEnvironmentVariables()
    .AddCommandLine(args)
    .Build();

var settings = config.Get<AppSettings>();
```

### Rust Equivalent (serde + toml)

```rust
// config.toml
[database]
connection_string = "..."
timeout = 30

[logging]
level = "info"

// Configuration struct
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    database: DatabaseConfig,
    logging: LoggingConfig,
}

#[derive(Deserialize, Serialize)]
struct DatabaseConfig {
    connection_string: String,
    timeout: u32,
}

// Usage
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

**Key Differences:**
- **C#**: JSON with hierarchical configuration builder
- **Rust**: TOML with serde serialization
- **C#**: Dependency injection integration
- **Rust**: Manual configuration loading and validation

## ❌ Error Handling

### C# Approach (Exceptions)

```csharp
public class ProcessingException : Exception
{
    public ProcessingException(string message) : base(message) { }
    public ProcessingException(string message, Exception inner) : base(message, inner) { }
}

try
{
    var content = File.ReadAllText(filename);
    var result = ProcessContent(content);
    File.WriteAllText(outputFile, result);
}
catch (FileNotFoundException ex)
{
    Console.Error.WriteLine($"File not found: {ex.FileName}");
    Environment.Exit(1);
}
catch (UnauthorizedAccessException ex)
{
    Console.Error.WriteLine($"Permission denied: {ex.Message}");
    Environment.Exit(1);
}
catch (ProcessingException ex)
{
    Console.Error.WriteLine($"Processing failed: {ex.Message}");
    Environment.Exit(1);
}
```

### Rust Equivalent (Result types)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ProcessingError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    
    #[error("Processing failed: {message}")]
    ProcessingFailed { message: String },
}

fn process_file(filename: &str, output_file: &str) -> Result<(), ProcessingError> {
    let content = std::fs::read_to_string(filename)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => ProcessingError::FileNotFound { 
                path: filename.to_string() 
            },
            std::io::ErrorKind::PermissionDenied => ProcessingError::PermissionDenied { 
                path: filename.to_string() 
            },
            _ => ProcessingError::ProcessingFailed { 
                message: e.to_string() 
            },
        })?;
    
    let result = process_content(&content)?;
    
    std::fs::write(output_file, result)
        .map_err(|e| ProcessingError::ProcessingFailed { 
            message: e.to_string() 
        })?;
    
    Ok(())
}

// Usage
fn main() {
    if let Err(e) = process_file("input.txt", "output.txt") {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

**Key Differences:**
- **C#**: Exception-based with try-catch blocks
- **Rust**: Result-based with explicit error handling
- **C#**: Stack unwinding and automatic propagation
- **Rust**: Explicit error propagation with `?` operator

## 🎨 Console Output and Colors

### C# Approach

```csharp
// Basic colors
Console.ForegroundColor = ConsoleColor.Red;
Console.WriteLine("Error message");
Console.ResetColor();

// Advanced (with libraries like Colorful.Console)
Console.WriteLineFormatted("This is {0:red} and {1:green}", 
    Colorful.Formatter.FormatValue("red"), 
    Colorful.Formatter.FormatValue("green"));

// Progress indication
using (var progress = new ProgressBar())
{
    for (int i = 0; i <= 100; i++)
    {
        progress.Report((double)i / 100);
        Thread.Sleep(20);
    }
}
```

### Rust Equivalent

```rust
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

// Basic colors
println!("{}", "Error message".red());
println!("{}", "Success message".green().bold());

// Advanced formatting
println!("This is {} and {}", "red".red(), "green".green());

// Progress bars
let pb = ProgressBar::new(100);
pb.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap());

for i in 0..100 {
    pb.set_message(format!("Processing item {}", i));
    pb.inc(1);
    std::thread::sleep(std::time::Duration::from_millis(20));
}
pb.finish_with_message("Done!");
```

**Key Differences:**
- **C#**: Console.ForegroundColor approach, external libraries for advanced features
- **Rust**: Method chaining with colored crate, built-in progress bar support
- **C#**: More verbose color management
- **Rust**: More fluent API design

## 📁 File Operations

### C# Approach

```csharp
// Reading files
var content = File.ReadAllText(path);
var lines = File.ReadAllLines(path);

// Writing files
File.WriteAllText(path, content);
File.WriteAllLines(path, lines);

// Directory operations
var files = Directory.GetFiles(path, "*.txt", SearchOption.AllDirectories);
Directory.CreateDirectory(outputPath);

// Path manipulation
var fullPath = Path.Combine(basePath, "subfolder", "file.txt");
var extension = Path.GetExtension(filename);
var directory = Path.GetDirectoryName(fullPath);
```

### Rust Equivalent

```rust
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

// Reading files
let content = fs::read_to_string(path)?;
let lines: Vec<String> = fs::read_to_string(path)?
    .lines()
    .map(|s| s.to_string())
    .collect();

// Writing files
fs::write(path, content)?;
fs::write(path, lines.join("\n"))?;

// Directory operations
let files: Vec<PathBuf> = WalkDir::new(path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "txt"))
    .map(|e| e.path().to_path_buf())
    .collect();

fs::create_dir_all(output_path)?;

// Path manipulation
let full_path = PathBuf::from(base_path).join("subfolder").join("file.txt");
let extension = path.extension();
let directory = path.parent();
```

**Key Differences:**
- **C#**: Static methods on File/Directory/Path classes
- **Rust**: Functions in std::fs module, PathBuf for path manipulation
- **C#**: Exceptions for error handling
- **Rust**: Result types with `?` operator
- **C#**: Built-in recursive directory search
- **Rust**: External crate (walkdir) for advanced directory traversal

## 🔄 Async Operations

### C# Approach

```csharp
public async Task<string> ProcessFileAsync(string path)
{
    var content = await File.ReadAllTextAsync(path);
    var result = await ProcessContentAsync(content);
    await File.WriteAllTextAsync(outputPath, result);
    return result;
}

// HTTP client
using var client = new HttpClient();
var response = await client.GetStringAsync("https://api.example.com/data");
var data = JsonSerializer.Deserialize<ApiResponse>(response);
```

### Rust Equivalent

```rust
use tokio::fs;
use reqwest;

async fn process_file_async(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path).await?;
    let result = process_content_async(&content).await?;
    fs::write(output_path, &result).await?;
    Ok(result)
}

// HTTP client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.example.com/data")
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;
    
    Ok(())
}
```

**Key Differences:**
- **C#**: Built-in async/await with Task types
- **Rust**: External runtime (tokio) with async/await
- **C#**: Async methods throughout standard library
- **Rust**: Separate async versions of standard functions

## 🧪 Testing

### C# Approach

```csharp
[TestClass]
public class ProcessorTests
{
    [TestMethod]
    public void ProcessFile_ValidInput_ReturnsSuccess()
    {
        // Arrange
        var processor = new FileProcessor();
        var input = "test content";
        
        // Act
        var result = processor.Process(input);
        
        // Assert
        Assert.AreEqual("processed: test content", result);
    }
    
    [TestMethod]
    public void ProcessFile_EmptyInput_ThrowsException()
    {
        var processor = new FileProcessor();
        
        Assert.ThrowsException<ArgumentException>(() => 
            processor.Process(""));
    }
}
```

### Rust Equivalent

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn process_file_valid_input_returns_success() {
        // Arrange
        let processor = FileProcessor::new();
        let input = "test content";
        
        // Act
        let result = processor.process(input).unwrap();
        
        // Assert
        assert_eq!("processed: test content", result);
    }
    
    #[test]
    fn process_file_empty_input_returns_error() {
        let processor = FileProcessor::new();
        
        let result = processor.process("");
        assert!(result.is_err());
    }
    
    #[test]
    fn process_file_integration_test() {
        use tempfile::NamedTempFile;
        use std::io::Write;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test content").unwrap();
        
        let result = process_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
    }
}
```

**Key Differences:**
- **C#**: Attribute-based testing with MSTest/NUnit/xUnit
- **Rust**: Built-in test framework with `#[test]` attribute
- **C#**: Exception-based assertions
- **Rust**: Result-based assertions with `assert!` macros

## 🛠️ Dependency Management

### C# Approach

```xml
<!-- .csproj -->
<PackageReference Include="CommandLineParser" Version="2.9.1" />
<PackageReference Include="Newtonsoft.Json" Version="13.0.1" />
<PackageReference Include="Serilog" Version="2.12.0" />
```

### Rust Equivalent

```toml
# Cargo.toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

**Key Differences:**
- **C#**: XML-based project files with NuGet packages
- **Rust**: TOML-based configuration with crates.io packages
- **C#**: Feature flags through conditional compilation
- **Rust**: Feature flags built into dependency specification

## 🚀 Key Takeaways

### Mental Model Mapping

| C# Concept | Rust Equivalent | Notes |
|-----------|-----------------|-------|
| `try-catch` | `Result<T, E>` with `?` | Explicit error handling |
| `using` statement | `Drop` trait | Automatic resource cleanup |
| `async/await` | `async/await` with tokio | External runtime required |
| Properties | Public fields or methods | Simpler, more explicit |
| LINQ | Iterator chains | Functional programming style |
| Exceptions | `Result` and `Option` | No exceptions, explicit handling |
| Inheritance | Traits | Composition over inheritance |
| Generics | Generics | Similar but more powerful |

### Best Practices Translation

1. **Error Handling**: Move from exception-based to Result-based thinking
2. **Memory Management**: Leverage ownership instead of GC
3. **Concurrency**: Use async/await patterns but understand the differences
4. **Testing**: Embrace Rust's built-in testing framework
5. **Dependencies**: Use Cargo.toml features instead of conditional compilation

### Learning Path

1. **Start with CLI parsing** - clap is similar to CommandLineParser
2. **Learn Result handling** - Essential for robust CLI tools
3. **Master file operations** - Foundation for most CLI tools
4. **Add configuration** - TOML + serde works like appsettings.json
5. **Enhance with colors and progress** - Polish the user experience
6. **Test thoroughly** - Rust's testing tools are excellent

The transition from C# to Rust CLI development builds on your existing knowledge while introducing powerful new concepts like ownership and explicit error handling. Focus on one concept at a time and practice with real CLI tools!