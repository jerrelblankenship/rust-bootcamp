# File Processor Project Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working file processor project. Here's the full solution with all components implemented and comprehensive error handling throughout.

## 📝 Complete Project Implementation

This level provides the complete implementation of a production-ready file processor demonstrating all error handling patterns from Module 03.

### Complete src/main.rs
```rust
// src/main.rs - Complete CLI implementation with comprehensive error handling
use clap::{Parser, ValueEnum};
use file_processor::{
    Config, FileProcessorEngine, ProcessingOptions, OutputFormat,
    error::FileProcessorError,
    reporting::ReportGenerator,
};
use std::process;

#[derive(Parser)]
#[command(name = "file-processor")]
#[command(version = "1.0.0")]
#[command(about = "A robust file processing tool with comprehensive error handling")]
#[command(long_about = "Process various file formats (JSON, CSV, Text) with production-grade error handling, \
validation, and reporting capabilities.")]
struct Cli {
    /// Input file patterns (supports glob patterns)
    #[arg(value_name = "PATTERNS", help = "File patterns to process (e.g., '*.json' 'data/*.csv')")]
    patterns: Vec<String>,
    
    /// Output directory for processed files and reports
    #[arg(short, long, value_name = "DIR", help = "Output directory (default: ./output)")]
    output: Option<String>,
    
    /// Output format for reports
    #[arg(short, long, value_enum, help = "Report output format")]
    format: Option<CliOutputFormat>,
    
    /// Enable verbose output with detailed progress information
    #[arg(short, long, help = "Enable verbose logging")]
    verbose: bool,
    
    /// Disable parallel processing (process files sequentially)
    #[arg(long, help = "Disable parallel processing")]
    no_parallel: bool,
    
    /// Skip data validation (faster but less safe)
    #[arg(long, help = "Skip data validation steps")]
    no_validate: bool,
    
    /// Maximum file size in bytes (default: 10MB)
    #[arg(long, value_name = "BYTES", help = "Maximum file size to process")]
    max_size: Option<usize>,
    
    /// Configuration file path
    #[arg(short, long, value_name = "FILE", help = "Configuration file path")]
    config: Option<String>,
    
    /// Dry run mode (analyze files without processing)
    #[arg(long, help = "Analyze files without actually processing them")]
    dry_run: bool,
}

#[derive(Clone, ValueEnum)]
enum CliOutputFormat {
    Json,
    Csv, 
    Text,
    Summary,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(cli_format: CliOutputFormat) -> Self {
        match cli_format {
            CliOutputFormat::Json => OutputFormat::Json,
            CliOutputFormat::Csv => OutputFormat::Csv,
            CliOutputFormat::Text => OutputFormat::Text,
            CliOutputFormat::Summary => OutputFormat::Summary,
        }
    }
}

fn main() {
    // Set up panic hook for graceful error handling
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("💥 Fatal error occurred:");
        eprintln!("   {}", panic_info);
        eprintln!("   This indicates a bug in the file processor.");
        eprintln!("   Please report this issue with the command you ran.");
        process::exit(2);
    }));
    
    // Run application with comprehensive error handling
    if let Err(e) = run() {
        print_error_with_guidance(&e);
        
        // Exit with appropriate code based on error type
        let exit_code = match &e {
            FileProcessorError::InvalidArguments { .. } => 1,
            FileProcessorError::ConfigurationError { .. } => 2,
            FileProcessorError::FileNotFound { .. } => 3,
            FileProcessorError::PermissionDenied { .. } => 4,
            FileProcessorError::FormatNotSupported { .. } => 5,
            _ => 1,
        };
        
        process::exit(exit_code);
    }
}

fn run() -> Result<(), FileProcessorError> {
    let cli = Cli::parse();
    
    // Validate command line arguments
    if cli.patterns.is_empty() {
        return Err(FileProcessorError::InvalidArguments {
            message: "At least one file pattern must be specified. Use --help for usage information.".to_string(),
        });
    }
    
    // Load configuration from multiple sources
    let mut config = match &cli.config {
        Some(path) => Config::from_file(path)?,
        None => Config::load()?,
    };
    
    // Apply CLI overrides
    apply_cli_overrides(&mut config, &cli)?;
    
    // Create processing options
    let options = ProcessingOptions {
        validate_data: !cli.no_validate,
        parallel_processing: !cli.no_parallel,
        dry_run: cli.dry_run,
        verbose: cli.verbose,
    };
    
    // Initialize file processor engine
    let engine = FileProcessorEngine::new(config.clone())?;
    
    if cli.verbose {
        println!("🚀 Starting file processor with configuration:");
        println!("   Input patterns: {:?}", cli.patterns);
        println!("   Output directory: {:?}", config.output_dir);
        println!("   Max file size: {} bytes", config.max_file_size);
        println!("   Parallel processing: {}", options.parallel_processing);
        println!("   Data validation: {}", options.validate_data);
        println!("   Dry run mode: {}", options.dry_run);
        println!();
    }
    
    // Process files with comprehensive error handling
    let batch_result = engine.process_files(&cli.patterns, &options)?;
    
    // Generate and display report
    let report_generator = ReportGenerator::new(&config);
    let report = report_generator.generate_report(&batch_result)?;
    
    // Display results
    report_generator.display_report(&report, &config.output_format)?;
    
    // Write detailed report to file
    if !cli.dry_run {
        report_generator.write_report_file(&report, &config.output_dir)?;
    }
    
    // Exit with error if any files failed to process
    if !batch_result.failures.is_empty() {
        if cli.verbose {
            println!("\n⚠️  Some files failed to process. Check the detailed report for more information.");
        }
        return Err(FileProcessorError::ProcessingFailed {
            file: "batch".to_string(),
            reason: format!("{} out of {} files failed to process", 
                          batch_result.failures.len(), 
                          batch_result.total_files),
        });
    }
    
    if cli.verbose {
        println!("✅ All files processed successfully!");
    }
    
    Ok(())
}

fn apply_cli_overrides(config: &mut Config, cli: &Cli) -> Result<(), FileProcessorError> {
    if let Some(output_dir) = &cli.output {
        config.output_dir = output_dir.into();
    }
    
    if let Some(format) = &cli.format {
        config.output_format = format.clone().into();
    }
    
    if let Some(max_size) = cli.max_size {
        if max_size == 0 {
            return Err(FileProcessorError::InvalidArguments {
                message: "Maximum file size must be greater than 0".to_string(),
            });
        }
        config.max_file_size = max_size;
    }
    
    config.verbose = cli.verbose;
    
    Ok(())
}

fn print_error_with_guidance(error: &FileProcessorError) {
    eprintln!("❌ Error: {}", error);
    
    // Print error chain
    let mut source = error.source();
    while let Some(err) = source {
        eprintln!("   Caused by: {}", err);
        source = err.source();
    }
    
    // Provide specific guidance based on error type
    eprintln!("\n💡 Guidance:");
    match error {
        FileProcessorError::FileNotFound { path } => {
            eprintln!("   • Check that the file path '{}' is correct", path);
            eprintln!("   • Verify you have read permissions for the file");
            eprintln!("   • If using patterns, ensure files matching the pattern exist");
        }
        FileProcessorError::PermissionDenied { path, operation } => {
            eprintln!("   • Check file permissions for '{}'", path);
            eprintln!("   • Ensure you have rights to {} this file", operation);
            eprintln!("   • Try running with elevated permissions if appropriate");
        }
        FileProcessorError::FormatNotSupported { format, file } => {
            eprintln!("   • Supported formats: JSON (.json, .jsonl), CSV (.csv, .tsv), Text (.txt, .log)");
            eprintln!("   • Check if '{}' has the correct file extension", file);
            eprintln!("   • Verify the file content matches the expected format");
        }
        FileProcessorError::InvalidArguments { .. } => {
            eprintln!("   • Run 'file-processor --help' for usage information");
            eprintln!("   • Check the command-line arguments and options");
            eprintln!("   • Ensure all required arguments are provided");
        }
        FileProcessorError::ConfigurationError { .. } => {
            eprintln!("   • Check your configuration file syntax");
            eprintln!("   • Verify all configuration values are valid");
            eprintln!("   • Try running without a config file to use defaults");
        }
        FileProcessorError::FileSizeExceeded { size, limit, .. } => {
            eprintln!("   • File size ({} bytes) exceeds limit ({} bytes)", size, limit);
            eprintln!("   • Use --max-size to increase the limit");
            eprintln!("   • Consider processing the file in chunks");
        }
        _ => {
            eprintln!("   • Check the file format and content");
            eprintln!("   • Verify input parameters are correct");
            eprintln!("   • Try with --verbose for more detailed information");
        }
    }
    
    eprintln!("\nFor more help, run: file-processor --help");
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::{File, create_dir_all};
    use std::io::Write;
    use std::path::Path;
    
    #[test]
    fn test_cli_help_message() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_file-processor"))
            .arg("--help")
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
        let help_text = String::from_utf8(output.stdout).unwrap();
        assert!(help_text.contains("file processing tool"));
        assert!(help_text.contains("--output"));
        assert!(help_text.contains("--format"));
    }
    
    #[test]
    fn test_cli_with_valid_json_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"name": "test", "value": 42}}"#).unwrap();
        
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_file-processor"))
            .arg(file_path.to_str().unwrap())
            .arg("--output")
            .arg(temp_dir.path().join("output").to_str().unwrap())
            .arg("--format")
            .arg("summary")
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success(), 
               "Command failed with stderr: {}", 
               String::from_utf8_lossy(&output.stderr));
    }
    
    #[test]
    fn test_cli_with_nonexistent_file() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_file-processor"))
            .arg("nonexistent_file.json")
            .output()
            .expect("Failed to execute command");
        
        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("File not found") || stderr.contains("No files found"));
    }
    
    #[test]
    fn test_cli_invalid_arguments() {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_file-processor"))
            .arg("--invalid-flag")
            .output()
            .expect("Failed to execute command");
        
        assert!(!output.status.success());
    }
    
    #[test]
    fn test_cli_dry_run_mode() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"name": "test", "value": 42}}"#).unwrap();
        
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_file-processor"))
            .arg(file_path.to_str().unwrap())
            .arg("--dry-run")
            .arg("--verbose")
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Dry run mode") || stdout.contains("analyzing"));
    }
}
```

### Complete Production Error Handling Patterns

This implementation demonstrates enterprise-grade error handling:

#### 1. **Comprehensive Error Recovery**
```rust
// Automatic retry with exponential backoff
impl FileProcessorEngine {
    async fn process_with_retry<F, T>(&self, operation: F, max_attempts: u32) -> Result<T>
    where
        F: Fn() -> Result<T> + Send + Sync,
        T: Send,
    {
        let mut last_error = None;
        
        for attempt in 1..=max_attempts {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) if self.should_retry(&e) && attempt < max_attempts => {
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
                    tokio::time::sleep(delay).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(last_error.unwrap())
    }
    
    fn should_retry(&self, error: &FileProcessorError) -> bool {
        matches!(error, 
            FileProcessorError::IoError(_) |
            FileProcessorError::DiskSpaceInsufficient { .. }
        )
    }
}
```

#### 2. **Error Observability and Metrics**
```rust
use tracing::{error, warn, info, debug, instrument};
use metrics::{counter, histogram, gauge};

impl FileProcessorEngine {
    #[instrument(skip(self, file_content))]
    async fn process_file_with_observability(&self, path: &Path) -> Result<ProcessingResult> {
        let start_time = Instant::now();
        let file_size = std::fs::metadata(path)?.len();
        
        // Record metrics
        counter!("files_processed_total").increment(1);
        gauge!("current_file_size_bytes").set(file_size as f64);
        
        let result = self.process_file_internal(path).await;
        
        match &result {
            Ok(processing_result) => {
                counter!("files_processed_success").increment(1);
                histogram!("file_processing_duration_ms")
                    .record(start_time.elapsed().as_millis() as f64);
                
                info!(
                    file_path = %path.display(),
                    records_processed = processing_result.record_count,
                    processing_time_ms = processing_result.processing_time_ms,
                    "File processed successfully"
                );
            }
            Err(e) => {
                counter!("files_processed_error").increment(1);
                counter!("processing_errors", "error_type" => error_type_name(e)).increment(1);
                
                error!(
                    file_path = %path.display(),
                    error = %e,
                    error_source = ?e.source(),
                    "File processing failed"
                );
            }
        }
        
        result
    }
}

fn error_type_name(error: &FileProcessorError) -> &'static str {
    match error {
        FileProcessorError::FileNotFound { .. } => "file_not_found",
        FileProcessorError::PermissionDenied { .. } => "permission_denied",
        FileProcessorError::FormatNotSupported { .. } => "format_not_supported",
        FileProcessorError::JsonError { .. } => "json_parse_error",
        FileProcessorError::CsvError { .. } => "csv_parse_error",
        FileProcessorError::InvalidData { .. } => "invalid_data",
        _ => "unknown",
    }
}
```

#### 3. **Circuit Breaker for Resilient Processing**
```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
enum CircuitState {
    Closed,   // Normal operation
    Open,     // Failing, reject requests
    HalfOpen, // Testing recovery
}

struct CircuitBreaker {
    state: Arc<Mutex<CircuitBreakerState>>,
    failure_threshold: u32,
    timeout: Duration,
}

struct CircuitBreakerState {
    state: CircuitState,
    failure_count: u32,
    last_failure_time: Option<Instant>,
    success_count: u32,
}

impl CircuitBreaker {
    fn new(failure_threshold: u32, timeout: Duration) -> Self {
        CircuitBreaker {
            state: Arc::new(Mutex::new(CircuitBreakerState {
                state: CircuitState::Closed,
                failure_count: 0,
                last_failure_time: None,
                success_count: 0,
            })),
            failure_threshold,
            timeout,
        }
    }
    
    async fn execute<F, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: Future<Output = Result<T, E>>,
    {
        // Check circuit state
        {
            let mut state = self.state.lock().unwrap();
            match state.state {
                CircuitState::Open => {
                    if let Some(last_failure) = state.last_failure_time {
                        if last_failure.elapsed() >= self.timeout {
                            state.state = CircuitState::HalfOpen;
                            state.success_count = 0;
                        } else {
                            return Err(CircuitBreakerError::CircuitOpen);
                        }
                    }
                }
                CircuitState::HalfOpen => {
                    // Allow limited requests through
                }
                CircuitState::Closed => {
                    // Normal operation
                }
            }
        }
        
        // Execute operation
        match operation.await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(error) => {
                self.on_failure();
                Err(CircuitBreakerError::OperationFailed(error))
            }
        }
    }
    
    fn on_success(&self) {
        let mut state = self.state.lock().unwrap();
        state.failure_count = 0;
        state.success_count += 1;
        
        match state.state {
            CircuitState::HalfOpen if state.success_count >= 3 => {
                state.state = CircuitState::Closed;
                info!("Circuit breaker closed - service recovered");
            }
            _ => {}
        }
    }
    
    fn on_failure(&self) {
        let mut state = self.state.lock().unwrap();
        state.failure_count += 1;
        state.last_failure_time = Some(Instant::now());
        
        if state.failure_count >= self.failure_threshold {
            state.state = CircuitState::Open;
            warn!("Circuit breaker opened due to {} failures", state.failure_count);
        }
    }
}

#[derive(Debug)]
enum CircuitBreakerError<E> {
    CircuitOpen,
    OperationFailed(E),
}
```

#### 4. **Comprehensive Error Testing**
```rust
#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::{File, set_permissions, Permissions};
    use std::os::unix::fs::PermissionsExt;
    
    #[tokio::test]
    async fn test_file_not_found_error() {
        let config = Config::default();
        let engine = FileProcessorEngine::new(config).unwrap();
        
        let result = engine.process_files(&["nonexistent.json".to_string()], &ProcessingOptions::default()).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            FileProcessorError::ProcessingFailed { reason, .. } => {
                assert!(reason.contains("No files found"));
            }
            other => panic!("Expected ProcessingFailed, got: {:?}", other),
        }
    }
    
    #[tokio::test]
    async fn test_permission_denied_error() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("readonly.json");
        
        // Create file and make it unreadable
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"test": true}}"#).unwrap();
        drop(file);
        
        set_permissions(&file_path, Permissions::from_mode(0o000)).unwrap();
        
        let config = Config::default();
        let engine = FileProcessorEngine::new(config).unwrap();
        
        let result = engine.process_files(&[file_path.to_string_lossy().to_string()], &ProcessingOptions::default()).await;
        
        // Should handle permission error gracefully
        assert!(result.is_ok()); // Batch processing continues despite individual failures
        let batch_result = result.unwrap();
        assert!(!batch_result.failures.is_empty());
    }
    
    #[tokio::test]
    async fn test_malformed_json_error() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("malformed.json");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"incomplete": json"#).unwrap(); // Missing closing brace
        
        let config = Config::default();
        let engine = FileProcessorEngine::new(config).unwrap();
        
        let result = engine.process_files(&[file_path.to_string_lossy().to_string()], &ProcessingOptions::default()).await;
        
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert_eq!(batch_result.failures.len(), 1);
        assert!(batch_result.failures[0].error.contains("JSON"));
    }
    
    #[tokio::test]
    async fn test_file_size_exceeded_error() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("large.json");
        
        // Create file larger than default limit
        let mut file = File::create(&file_path).unwrap();
        let large_content = "x".repeat(15_000_000); // 15MB
        writeln!(file, r#"{{"data": "{}"}}"#, large_content).unwrap();
        
        let config = Config::default(); // Default max size is 10MB
        let engine = FileProcessorEngine::new(config).unwrap();
        
        let result = engine.process_files(&[file_path.to_string_lossy().to_string()], &ProcessingOptions::default()).await;
        
        assert!(result.is_ok());
        let batch_result = result.unwrap();
        assert!(!batch_result.failures.is_empty());
        assert!(batch_result.failures[0].error.contains("size") && batch_result.failures[0].error.contains("exceeds"));
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_functionality() {
        let circuit_breaker = CircuitBreaker::new(3, Duration::from_millis(100));
        
        // Simulate failures to trigger circuit breaker
        for _ in 0..3 {
            let result = circuit_breaker.execute(async { Err::<(), &str>("simulated failure") }).await;
            assert!(matches!(result, Err(CircuitBreakerError::OperationFailed(_))));
        }
        
        // Next request should be rejected due to open circuit
        let result = circuit_breaker.execute(async { Ok::<(), &str>(()) }).await;
        assert!(matches!(result, Err(CircuitBreakerError::CircuitOpen)));
        
        // Wait for timeout and test half-open state
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should allow request through (half-open state)
        let result = circuit_breaker.execute(async { Ok::<(), &str>(()) }).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_error_recovery_with_retry() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        
        // Simulate a file that becomes available after retry
        std::thread::spawn({
            let file_path = file_path.clone();
            move || {
                std::thread::sleep(Duration::from_millis(50));
                let mut file = File::create(&file_path).unwrap();
                writeln!(file, r#"{{"test": "data"}}"#).unwrap();
            }
        });
        
        let config = Config::default();
        let engine = FileProcessorEngine::new(config).unwrap();
        
        // Should succeed after retry
        let result = engine.process_with_retry(
            || engine.process_files(&[file_path.to_string_lossy().to_string()], &ProcessingOptions::default()),
            3
        ).await;
        
        assert!(result.is_ok());
    }
}
```

## 🏆 Production Deployment Checklist

### Error Handling Completeness
- ✅ All possible failure modes are handled
- ✅ Error messages are actionable and user-friendly
- ✅ Error chains preserve debugging information
- ✅ Automatic error recovery where appropriate
- ✅ Graceful degradation for non-critical failures

### Observability and Monitoring
- ✅ Structured logging with appropriate log levels
- ✅ Error metrics collection and alerting
- ✅ Performance metrics (latency, throughput)
- ✅ Health checks and service status endpoints

### Reliability Patterns
- ✅ Circuit breaker for external dependencies
- ✅ Retry logic with exponential backoff
- ✅ Timeout handling for long-running operations
- ✅ Resource cleanup in error scenarios

### User Experience
- ✅ Clear error messages with guidance
- ✅ Appropriate exit codes for automation
- ✅ Progress reporting for long operations
- ✅ Verbose mode for debugging

## 🎓 Mastery Achievement

You've successfully implemented production-grade error handling that demonstrates:

### Core Competencies
- **Comprehensive Error Types**: Domain-specific errors with rich context
- **Error Recovery**: Retry, fallback, and circuit breaker patterns
- **User Experience**: Actionable error messages and guidance
- **Observability**: Logging, metrics, and debugging support
- **Testing**: Comprehensive error scenario coverage

### Production Readiness
- **Robustness**: Handles all failure modes gracefully
- **Performance**: Efficient error handling with minimal overhead
- **Maintainability**: Clear error type organization and documentation
- **Scalability**: Patterns that work from single files to large batches

### Enterprise Features
- **Configuration Management**: Multi-source configuration with validation
- **CLI Excellence**: Professional command-line interface
- **Reporting**: Detailed processing reports and statistics
- **Integration**: Patterns suitable for larger systems

## 🚀 Next Steps

You've now mastered comprehensive error handling in Rust! Apply these patterns to:

1. **Your Own Projects**: Use these error handling patterns in real applications
2. **Team Standards**: Establish error handling guidelines for your team
3. **Library Development**: Create robust APIs with excellent error types
4. **Production Systems**: Build resilient services and applications

Congratulations on completing the File Processor project and mastering production-grade error handling in Rust! 🦀