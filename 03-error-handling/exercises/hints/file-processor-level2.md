# File Processor Project Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Production Error Handling

You've tried Level 1 hints but need concrete implementation guidance. Here are specific solutions for building the file processor with comprehensive error handling.

## 🔧 Step 1: Complete Error Type Implementation

**Problem**: Fix all compilation errors in `src/error.rs`.

**Specific Solution:**
```rust
// src/error.rs - Complete error type implementation
use std::fmt;
use std::error::Error;
use std::io;
use serde_json;
use csv;

#[derive(Debug)]
pub enum FileProcessorError {
    // File system errors
    FileNotFound { path: String },
    PermissionDenied { path: String, operation: String },
    IoError(io::Error),
    
    // Processing errors
    FormatNotSupported { format: String, file: String },
    ProcessingFailed { file: String, reason: String },
    InvalidData { file: String, line: Option<usize>, message: String },
    
    // Configuration errors
    ConfigurationError { message: String },
    InvalidArguments { message: String },
    
    // Format-specific errors
    JsonError { file: String, error: serde_json::Error },
    CsvError { file: String, error: csv::Error },
    
    // System errors
    FileSizeExceeded { file: String, size: usize, limit: usize },
    DiskSpaceInsufficient { required: usize, available: usize },
}

impl fmt::Display for FileProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileProcessorError::FileNotFound { path } => {
                write!(f, "File not found: '{}'", path)
            }
            FileProcessorError::PermissionDenied { path, operation } => {
                write!(f, "Permission denied: cannot {} '{}'", operation, path)
            }
            FileProcessorError::IoError(e) => {
                write!(f, "I/O error: {}", e)
            }
            FileProcessorError::FormatNotSupported { format, file } => {
                write!(f, "Unsupported format '{}' for file '{}'", format, file)
            }
            FileProcessorError::ProcessingFailed { file, reason } => {
                write!(f, "Failed to process '{}': {}", file, reason)
            }
            FileProcessorError::InvalidData { file, line, message } => {
                match line {
                    Some(n) => write!(f, "Invalid data in '{}' at line {}: {}", file, n, message),
                    None => write!(f, "Invalid data in '{}': {}", file, message),
                }
            }
            FileProcessorError::ConfigurationError { message } => {
                write!(f, "Configuration error: {}", message)
            }
            FileProcessorError::InvalidArguments { message } => {
                write!(f, "Invalid arguments: {}", message)
            }
            FileProcessorError::JsonError { file, error } => {
                write!(f, "JSON error in '{}': {}", file, error)
            }
            FileProcessorError::CsvError { file, error } => {
                write!(f, "CSV error in '{}': {}", file, error)
            }
            FileProcessorError::FileSizeExceeded { file, size, limit } => {
                write!(f, "File '{}' size ({} bytes) exceeds limit ({} bytes)", file, size, limit)
            }
            FileProcessorError::DiskSpaceInsufficient { required, available } => {
                write!(f, "Insufficient disk space: need {} bytes, have {} bytes", required, available)
            }
        }
    }
}

impl Error for FileProcessorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileProcessorError::IoError(e) => Some(e),
            FileProcessorError::JsonError { error, .. } => Some(error),
            FileProcessorError::CsvError { error, .. } => Some(error),
            _ => None,
        }
    }
}

// Automatic error conversions
impl From<io::Error> for FileProcessorError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => {
                // Try to extract path from error message if possible
                FileProcessorError::FileNotFound { 
                    path: "unknown".to_string() 
                }
            }
            io::ErrorKind::PermissionDenied => {
                FileProcessorError::PermissionDenied {
                    path: "unknown".to_string(),
                    operation: "access".to_string(),
                }
            }
            _ => FileProcessorError::IoError(err),
        }
    }
}

impl From<serde_json::Error> for FileProcessorError {
    fn from(err: serde_json::Error) -> Self {
        FileProcessorError::JsonError {
            file: "unknown".to_string(),
            error: err,
        }
    }
}

impl From<csv::Error> for FileProcessorError {
    fn from(err: csv::Error) -> Self {
        FileProcessorError::CsvError {
            file: "unknown".to_string(),
            error: err,
        }
    }
}

// Result type alias for convenience
pub type Result<T> = std::result::Result<T, FileProcessorError>;
```

**Key Learning**: Comprehensive error types with specific variants and automatic conversions.

## 🔧 Step 2: Configuration Loading Implementation

**Problem**: Implement configuration loading in `src/config.rs`.

**Specific Solution:**
```rust
// src/config.rs - Complete configuration implementation
use crate::error::{FileProcessorError, Result};
use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub input_patterns: Vec<String>,
    pub output_dir: PathBuf,
    pub max_file_size: usize,
    pub parallel_processing: bool,
    pub verbose: bool,
    pub output_format: OutputFormat,
    pub validate_data: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Csv,
    Text,
    Summary,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            input_patterns: vec!["*.txt".to_string()],
            output_dir: PathBuf::from("output"),
            max_file_size: 10_000_000, // 10MB
            parallel_processing: true,
            verbose: false,
            output_format: OutputFormat::Summary,
            validate_data: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::default();
        
        // Load from environment variables
        config.load_from_env()?;
        
        // Load from config file if it exists
        if let Ok(file_config) = config.load_from_file() {
            config.merge_with(file_config)?;
        }
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn from_args(args: &[String]) -> Result<Self> {
        let mut config = Config::load()?;
        config.parse_command_line(args)?;
        config.validate()?;
        Ok(config)
    }
    
    fn load_from_env(&mut self) -> Result<()> {
        if let Ok(max_size) = env::var("FILE_PROCESSOR_MAX_SIZE") {
            self.max_file_size = max_size.parse().map_err(|_| {
                FileProcessorError::ConfigurationError {
                    message: format!("Invalid FILE_PROCESSOR_MAX_SIZE: {}", max_size),
                }
            })?;
        }
        
        if let Ok(output_dir) = env::var("FILE_PROCESSOR_OUTPUT_DIR") {
            self.output_dir = PathBuf::from(output_dir);
        }
        
        if let Ok(parallel) = env::var("FILE_PROCESSOR_PARALLEL") {
            self.parallel_processing = parallel.parse().map_err(|_| {
                FileProcessorError::ConfigurationError {
                    message: format!("Invalid FILE_PROCESSOR_PARALLEL: {}", parallel),
                }
            })?;
        }
        
        Ok(())
    }
    
    fn load_from_file(&self) -> Result<Config> {
        let config_paths = [
            "file-processor.json",
            "config/file-processor.json",
            &format!("{}/.config/file-processor.json", env::var("HOME").unwrap_or_default()),
        ];
        
        for path in &config_paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                return self.parse_config_file(&content, path);
            }
        }
        
        // No config file found, return default
        Ok(Config::default())
    }
    
    fn parse_config_file(&self, content: &str, path: &str) -> Result<Config> {
        let config_value: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| FileProcessorError::JsonError {
                file: path.to_string(),
                error: e,
            })?;
        
        let mut config = Config::default();
        
        if let Some(max_size) = config_value.get("max_file_size").and_then(|v| v.as_u64()) {
            config.max_file_size = max_size as usize;
        }
        
        if let Some(output_dir) = config_value.get("output_dir").and_then(|v| v.as_str()) {
            config.output_dir = PathBuf::from(output_dir);
        }
        
        if let Some(parallel) = config_value.get("parallel_processing").and_then(|v| v.as_bool()) {
            config.parallel_processing = parallel;
        }
        
        if let Some(format_str) = config_value.get("output_format").and_then(|v| v.as_str()) {
            config.output_format = match format_str {
                "json" => OutputFormat::Json,
                "csv" => OutputFormat::Csv,
                "text" => OutputFormat::Text,
                "summary" => OutputFormat::Summary,
                _ => return Err(FileProcessorError::ConfigurationError {
                    message: format!("Invalid output format: {}", format_str),
                }),
            };
        }
        
        Ok(config)
    }
    
    fn parse_command_line(&mut self, args: &[String]) -> Result<()> {
        let mut i = 1; // Skip program name
        
        while i < args.len() {
            match args[i].as_str() {
                "--output" | "-o" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(FileProcessorError::InvalidArguments {
                            message: "--output requires a directory path".to_string(),
                        });
                    }
                    self.output_dir = PathBuf::from(&args[i]);
                }
                "--format" | "-f" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(FileProcessorError::InvalidArguments {
                            message: "--format requires a format (json|csv|text|summary)".to_string(),
                        });
                    }
                    self.output_format = match args[i].as_str() {
                        "json" => OutputFormat::Json,
                        "csv" => OutputFormat::Csv,
                        "text" => OutputFormat::Text,
                        "summary" => OutputFormat::Summary,
                        format => return Err(FileProcessorError::InvalidArguments {
                            message: format!("Unknown format: {}", format),
                        }),
                    };
                }
                "--verbose" | "-v" => {
                    self.verbose = true;
                }
                "--no-parallel" => {
                    self.parallel_processing = false;
                }
                "--no-validate" => {
                    self.validate_data = false;
                }
                "--max-size" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(FileProcessorError::InvalidArguments {
                            message: "--max-size requires a size in bytes".to_string(),
                        });
                    }
                    self.max_file_size = args[i].parse().map_err(|_| {
                        FileProcessorError::InvalidArguments {
                            message: format!("Invalid size: {}", args[i]),
                        }
                    })?;
                }
                arg if arg.starts_with("--") => {
                    return Err(FileProcessorError::InvalidArguments {
                        message: format!("Unknown option: {}", arg),
                    });
                }
                pattern => {
                    self.input_patterns.push(pattern.to_string());
                }
            }
            i += 1;
        }
        
        Ok(())
    }
    
    fn merge_with(&mut self, other: Config) -> Result<()> {
        // Command line args take precedence, so only merge non-default values
        if other.max_file_size != Config::default().max_file_size {
            self.max_file_size = other.max_file_size;
        }
        if other.output_dir != Config::default().output_dir {
            self.output_dir = other.output_dir;
        }
        // Continue for other fields...
        Ok(())
    }
    
    fn validate(&self) -> Result<()> {
        // Validate max file size
        if self.max_file_size == 0 {
            return Err(FileProcessorError::ConfigurationError {
                message: "max_file_size must be greater than 0".to_string(),
            });
        }
        
        // Validate output directory can be created
        if let Some(parent) = self.output_dir.parent() {
            if !parent.exists() {
                return Err(FileProcessorError::ConfigurationError {
                    message: format!("Output directory parent does not exist: {:?}", parent),
                });
            }
        }
        
        // Validate input patterns
        if self.input_patterns.is_empty() {
            return Err(FileProcessorError::ConfigurationError {
                message: "At least one input pattern must be specified".to_string(),
            });
        }
        
        Ok(())
    }
}
```

**Key Learning**: Configuration loading with multiple sources, validation, and error handling.

## 🔧 Step 3: Core Processing Engine

**Problem**: Implement the main processing engine in `src/processor.rs`.

**Specific Solution:**
```rust
// src/processor.rs - Core processing engine
use crate::error::{FileProcessorError, Result};
use crate::config::{Config, OutputFormat};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub source_path: PathBuf,
    pub record_count: usize,
    pub file_size: usize,
    pub processing_time_ms: u64,
    pub format_detected: String,
}

#[derive(Debug, Clone)]
pub struct ProcessingFailure {
    pub path: PathBuf,
    pub error: String,
}

#[derive(Debug)]
pub struct BatchResult {
    pub successes: Vec<ProcessingResult>,
    pub failures: Vec<ProcessingFailure>,
    pub total_files: usize,
    pub total_processing_time_ms: u64,
}

pub struct FileProcessorEngine {
    config: Config,
}

impl FileProcessorEngine {
    pub fn new(config: Config) -> Self {
        FileProcessorEngine { config }
    }
    
    pub fn process_files(&self, patterns: &[String]) -> Result<BatchResult> {
        let start_time = Instant::now();
        let files = self.collect_files(patterns)?;
        
        if files.is_empty() {
            return Err(FileProcessorError::ProcessingFailed {
                file: "".to_string(),
                reason: "No files found matching the specified patterns".to_string(),
            });
        }
        
        // Create output directory
        self.ensure_output_directory()?;
        
        let mut successes = Vec::new();
        let mut failures = Vec::new();
        
        for file_path in files {
            match self.process_single_file(&file_path) {
                Ok(result) => {
                    if self.config.verbose {
                        println!("✅ Processed: {} ({} records)", 
                               result.source_path.display(), result.record_count);
                    }
                    successes.push(result);
                }
                Err(e) => {
                    if self.config.verbose {
                        eprintln!("❌ Failed: {} - {}", file_path.display(), e);
                    }
                    failures.push(ProcessingFailure {
                        path: file_path,
                        error: e.to_string(),
                    });
                }
            }
        }
        
        Ok(BatchResult {
            total_files: successes.len() + failures.len(),
            successes,
            failures,
            total_processing_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }
    
    fn collect_files(&self, patterns: &[String]) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for pattern in patterns {
            if Path::new(pattern).exists() {
                // Direct file path
                files.push(PathBuf::from(pattern));
            } else {
                // Pattern matching
                let matched = glob::glob(pattern).map_err(|e| {
                    FileProcessorError::InvalidArguments {
                        message: format!("Invalid pattern '{}': {}", pattern, e),
                    }
                })?;
                
                for entry in matched {
                    match entry {
                        Ok(path) => {
                            if path.is_file() {
                                files.push(path);
                            }
                        }
                        Err(e) => {
                            if self.config.verbose {
                                eprintln!("Warning: Error processing glob entry: {}", e);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    fn process_single_file(&self, path: &Path) -> Result<ProcessingResult> {
        let start_time = Instant::now();
        
        // Check file size
        let metadata = fs::metadata(path).map_err(|e| {
            FileProcessorError::ProcessingFailed {
                file: path.display().to_string(),
                reason: format!("Cannot read file metadata: {}", e),
            }
        })?;
        
        let file_size = metadata.len() as usize;
        if file_size > self.config.max_file_size {
            return Err(FileProcessorError::FileSizeExceeded {
                file: path.display().to_string(),
                size: file_size,
                limit: self.config.max_file_size,
            });
        }
        
        // Detect format
        let format = self.detect_file_format(path)?;
        
        // Process based on format
        let record_count = match format.as_str() {
            "json" => self.process_json_file(path)?,
            "csv" => self.process_csv_file(path)?,
            "text" => self.process_text_file(path)?,
            _ => {
                return Err(FileProcessorError::FormatNotSupported {
                    format: format.clone(),
                    file: path.display().to_string(),
                });
            }
        };
        
        Ok(ProcessingResult {
            source_path: path.to_path_buf(),
            record_count,
            file_size,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            format_detected: format,
        })
    }
    
    fn detect_file_format(&self, path: &Path) -> Result<String> {
        // First try extension
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            match extension.to_lowercase().as_str() {
                "json" | "jsonl" => return Ok("json".to_string()),
                "csv" | "tsv" => return Ok("csv".to_string()),
                "txt" | "log" => return Ok("text".to_string()),
                _ => {}
            }
        }
        
        // Try content-based detection
        let content = fs::read_to_string(path).map_err(|e| {
            FileProcessorError::ProcessingFailed {
                file: path.display().to_string(),
                reason: format!("Cannot read file for format detection: {}", e),
            }
        })?;
        
        let trimmed = content.trim();
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            Ok("json".to_string())
        } else if trimmed.contains(',') && trimmed.lines().count() > 1 {
            Ok("csv".to_string())
        } else {
            Ok("text".to_string())
        }
    }
    
    fn process_json_file(&self, path: &Path) -> Result<usize> {
        use crate::formats::json::JsonProcessor;
        let processor = JsonProcessor::new(&self.config);
        processor.process_file(path)
    }
    
    fn process_csv_file(&self, path: &Path) -> Result<usize> {
        use crate::formats::csv::CsvProcessor;
        let processor = CsvProcessor::new(&self.config);
        processor.process_file(path)
    }
    
    fn process_text_file(&self, path: &Path) -> Result<usize> {
        use crate::formats::text::TextProcessor;
        let processor = TextProcessor::new(&self.config);
        processor.process_file(path)
    }
    
    fn ensure_output_directory(&self) -> Result<()> {
        if !self.config.output_dir.exists() {
            fs::create_dir_all(&self.config.output_dir).map_err(|e| {
                FileProcessorError::ProcessingFailed {
                    file: self.config.output_dir.display().to_string(),
                    reason: format!("Cannot create output directory: {}", e),
                }
            })?;
        }
        Ok(())
    }
}
```

**Key Learning**: Comprehensive processing engine with format detection, validation, and error recovery.

## 🔧 Step 4: Format Processor Implementation

**Problem**: Implement format-specific processors in `src/formats/`.

**Specific Solution for JSON:**
```rust
// src/formats/json.rs - JSON format processor
use crate::error::{FileProcessorError, Result};
use crate::config::Config;
use std::path::Path;
use std::fs;
use serde_json::{Value, Map};

pub struct JsonProcessor<'a> {
    config: &'a Config,
}

impl<'a> JsonProcessor<'a> {
    pub fn new(config: &'a Config) -> Self {
        JsonProcessor { config }
    }
    
    pub fn process_file(&self, path: &Path) -> Result<usize> {
        let content = fs::read_to_string(path).map_err(|e| {
            FileProcessorError::ProcessingFailed {
                file: path.display().to_string(),
                reason: format!("Cannot read JSON file: {}", e),
            }
        })?;
        
        if content.trim().is_empty() {
            return Ok(0);
        }
        
        // Try parsing as JSON Lines first
        if let Ok(count) = self.process_json_lines(&content, path) {
            return Ok(count);
        }
        
        // Try parsing as single JSON object/array
        self.process_single_json(&content, path)
    }
    
    fn process_json_lines(&self, content: &str, path: &Path) -> Result<usize> {
        let mut count = 0;
        
        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            let parsed: Value = serde_json::from_str(trimmed).map_err(|e| {
                FileProcessorError::InvalidData {
                    file: path.display().to_string(),
                    line: Some(line_num + 1),
                    message: format!("Invalid JSON: {}", e),
                }
            })?;
            
            if self.config.validate_data {
                self.validate_json_record(&parsed, path, Some(line_num + 1))?;
            }
            
            count += 1;
        }
        
        Ok(count)
    }
    
    fn process_single_json(&self, content: &str, path: &Path) -> Result<usize> {
        let parsed: Value = serde_json::from_str(content).map_err(|e| {
            FileProcessorError::JsonError {
                file: path.display().to_string(),
                error: e,
            }
        })?;
        
        let count = match &parsed {
            Value::Array(arr) => {
                if self.config.validate_data {
                    for (i, item) in arr.iter().enumerate() {
                        self.validate_json_record(item, path, Some(i + 1))?;
                    }
                }
                arr.len()
            }
            Value::Object(_) => {
                if self.config.validate_data {
                    self.validate_json_record(&parsed, path, None)?;
                }
                1
            }
            _ => {
                return Err(FileProcessorError::InvalidData {
                    file: path.display().to_string(),
                    line: None,
                    message: "JSON must be an object or array".to_string(),
                });
            }
        };
        
        Ok(count)
    }
    
    fn validate_json_record(&self, value: &Value, path: &Path, line: Option<usize>) -> Result<()> {
        match value {
            Value::Object(obj) => {
                // Basic validation - ensure it's not empty
                if obj.is_empty() {
                    return Err(FileProcessorError::InvalidData {
                        file: path.display().to_string(),
                        line,
                        message: "Empty JSON object".to_string(),
                    });
                }
                
                // Additional validation rules can be added here
                Ok(())
            }
            _ => {
                Err(FileProcessorError::InvalidData {
                    file: path.display().to_string(),
                    line,
                    message: "JSON record must be an object".to_string(),
                })
            }
        }
    }
}
```

## 💡 Integration Tips

### CLI Interface Pattern
```rust
// src/main.rs - Complete CLI implementation
use clap::Parser;
use file_processor::{Config, FileProcessorEngine, error::FileProcessorError};

#[derive(Parser)]
#[command(name = "file-processor")]
#[command(about = "A robust file processing tool with comprehensive error handling")]
struct Cli {
    /// Input file patterns
    #[arg(value_name = "PATTERNS")]
    patterns: Vec<String>,
    
    /// Output directory
    #[arg(short, long, value_name = "DIR")]
    output: Option<String>,
    
    /// Output format
    #[arg(short, long, value_enum)]
    format: Option<OutputFormat>,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        
        // Print error chain
        let mut source = e.source();
        while let Some(err) = source {
            eprintln!("  Caused by: {}", err);
            source = err.source();
        }
        
        std::process::exit(1);
    }
}

fn run() -> Result<(), FileProcessorError> {
    let cli = Cli::parse();
    
    let mut config = Config::load()?;
    config.apply_cli_args(&cli)?;
    
    let engine = FileProcessorEngine::new(config);
    let result = engine.process_files(&cli.patterns)?;
    
    // Print results
    print_processing_summary(&result);
    
    Ok(())
}
```

### Testing Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;
    
    #[test]
    fn test_json_processing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"name": "test", "value": 42}}"#).unwrap();
        
        let config = Config::default();
        let processor = JsonProcessor::new(&config);
        
        let result = processor.process_file(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
    
    #[test]
    fn test_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"name": "test", invalid json"#).unwrap();
        
        let config = Config::default();
        let processor = JsonProcessor::new(&config);
        
        let result = processor.process_file(&file_path);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            FileProcessorError::JsonError { .. } => {}, // Expected
            other => panic!("Expected JsonError, got: {:?}", other),
        }
    }
}
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](file-processor-level3.md) for full solution code.

## 🎓 Understanding Check

You should now understand:
1. **Production error type design**: Comprehensive error enums with specific variants
2. **Configuration management**: Multi-source configuration loading with validation
3. **Processing architecture**: Modular design with format-specific processors
4. **Error recovery**: Graceful handling of failures in batch operations
5. **CLI integration**: Professional command-line interface with error handling
6. **Testing strategies**: Comprehensive error scenario testing

Ready to build production-ready Rust applications! 🦀