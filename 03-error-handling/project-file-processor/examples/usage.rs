// examples/usage.rs
//
// This example demonstrates how to use the file processor library
// in your own Rust applications.

use file_processor::{
    FileProcessorEngine, ProcessingOptions, OutputFormat,
    process_file, process_files,
    error::FileProcessorError,
    reporting::ReportGenerator,
    config::Config,
};
use std::fs;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 File Processor Library Usage Examples\n");
    
    // Create temporary directory and test files
    let temp_dir = TempDir::new()?;
    setup_test_files(&temp_dir)?;
    
    // Example 1: Quick single file processing
    example_1_quick_processing(&temp_dir)?;
    
    // Example 2: Batch processing with custom options
    example_2_batch_processing(&temp_dir)?;
    
    // Example 3: Error handling and recovery
    example_3_error_handling(&temp_dir)?;
    
    // Example 4: Custom configuration
    example_4_custom_configuration(&temp_dir)?;
    
    // Example 5: Report generation
    example_5_report_generation(&temp_dir)?;
    
    // Example 6: Advanced processing engine usage
    example_6_advanced_usage(&temp_dir)?;
    
    println!("✅ All examples completed successfully!");
    Ok(())
}

/// Example 1: Quick single file processing using convenience functions
fn example_1_quick_processing(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("📄 Example 1: Quick Single File Processing");
    
    let json_file = temp_dir.path().join("example.json");
    
    // Use the convenience function for quick processing
    match process_file(json_file.to_str().unwrap()) {
        Ok(data) => {
            println!("  ✅ Processed {} ({} records)", data.source_path, data.record_count);
            println!("     Format: {}", data.format);
            println!("     Metadata: {:?}", data.metadata);
        }
        Err(e) => {
            println!("  ❌ Error: {}", e);
        }
    }
    
    println!();
    Ok(())
}

/// Example 2: Batch processing with custom options
fn example_2_batch_processing(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Example 2: Batch Processing with Custom Options");
    
    // Create a custom processing engine
    let engine = FileProcessorEngine::new();
    
    // Configure processing options
    let options = ProcessingOptions {
        validate_data: true,
        convert_encoding: false,
        max_file_size: 1024 * 1024, // 1MB limit
        output_format: OutputFormat::Json,
    };
    
    // Collect all test files
    let files = vec![
        temp_dir.path().join("example.json").to_str().unwrap().to_string(),
        temp_dir.path().join("example.csv").to_str().unwrap().to_string(),
        temp_dir.path().join("example.txt").to_str().unwrap().to_string(),
        "nonexistent.txt".to_string(), // This will fail
    ];
    
    // Process all files
    let result = engine.process_batch(&files, &options);
    
    println!("  📊 Batch Processing Results:");
    println!("     Total files: {}", files.len());
    println!("     Successful: {}", result.successes.len());
    println!("     Failed: {}", result.failures.len());
    println!("     Success rate: {:.1}%", result.success_rate() * 100.0);
    println!("     Total records: {}", result.total_records_processed());
    
    // Show details of successful processing
    for success in &result.successes {
        println!("     ✅ {} -> {} records", success.source_path, success.record_count);
    }
    
    // Show details of failures
    for failure in &result.failures {
        println!("     ❌ {}: {}", failure.path, failure.error);
    }
    
    println!();
    Ok(())
}

/// Example 3: Error handling and recovery
fn example_3_error_handling(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Example 3: Error Handling and Recovery");
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    // Try to process various files that may fail
    let test_cases = vec![
        ("existing_file", temp_dir.path().join("example.json").to_str().unwrap()),
        ("missing_file", "does_not_exist.json"),
        ("unsupported_format", "test.xyz"),
    ];
    
    for (case_name, file_path) in test_cases {
        println!("  🧪 Testing {}: {}", case_name, file_path);
        
        match engine.process_file(file_path, &options) {
            Ok(data) => {
                println!("     ✅ Success: {} records processed", data.record_count);
            }
            Err(e) => {
                println!("     ❌ Error: {}", e);
                
                // Demonstrate error type matching for different handling strategies
                match e {
                    FileProcessorError::FileNotFound { path } => {
                        println!("     💡 Recovery: Could create file '{}' or skip", path);
                    }
                    FileProcessorError::UnsupportedFormat { format, .. } => {
                        println!("     💡 Recovery: Format '{}' not supported, could convert or skip", format);
                    }
                    FileProcessorError::ValidationError { message, .. } => {
                        println!("     💡 Recovery: Validation failed ({}), could fix data", message);
                    }
                    _ => {
                        println!("     💡 Recovery: General error, could retry or abort");
                    }
                }
            }
        }
    }
    
    println!();
    Ok(())
}

/// Example 4: Custom configuration
fn example_4_custom_configuration(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️ Example 4: Custom Configuration");
    
    // Create a custom configuration
    let mut config = Config::default();
    config.max_file_size = 512 * 1024; // 512KB limit
    config.validate_data = false; // Disable validation for performance
    config.continue_on_error = true; // Continue processing even if some files fail
    config.output_format = "json".to_string();
    
    println!("  📋 Configuration:");
    println!("     Max file size: {} bytes", config.max_file_size);
    println!("     Validate data: {}", config.validate_data);
    println!("     Continue on error: {}", config.continue_on_error);
    println!("     Output format: {}", config.output_format);
    
    // Validate configuration
    match config.validate() {
        Ok(()) => println!("     ✅ Configuration is valid"),
        Err(e) => println!("     ❌ Configuration error: {}", e),
    }
    
    // Save configuration to file
    let config_file = temp_dir.path().join("custom_config.json");
    config.save_to_file(&config_file)?;
    println!("     💾 Configuration saved to: {:?}", config_file);
    
    // Load configuration from file
    let loaded_config = Config::from_file(&config_file)?;
    println!("     📖 Configuration loaded successfully");
    println!("     🔍 Loaded max file size: {}", loaded_config.max_file_size);
    
    println!();
    Ok(())
}

/// Example 5: Report generation
fn example_5_report_generation(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Example 5: Report Generation");
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    // Process multiple files to generate a meaningful report
    let files = vec![
        temp_dir.path().join("example.json").to_str().unwrap().to_string(),
        temp_dir.path().join("example.csv").to_str().unwrap().to_string(),
        "missing.txt".to_string(),
    ];
    
    let start_time = std::time::Instant::now();
    let result = engine.process_batch(&files, &options);
    let processing_time = start_time.elapsed();
    
    // Generate comprehensive report
    let report_generator = ReportGenerator::new();
    let report = report_generator.generate_report(&result, processing_time.as_millis() as u64);
    
    // Display report summary
    println!("  📈 Processing Report:");
    println!("     Generated at: {}", report.generated_at);
    println!("     Processing time: {}ms", report.processing_time_ms);
    println!("     Total files: {}", report.summary.total_files);
    println!("     Success rate: {:.1}%", report.summary.success_rate * 100.0);
    println!("     Total records: {}", report.summary.total_records);
    
    // Save report in different formats
    let output_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&output_dir)?;
    
    // JSON report
    let json_report = output_dir.join("report.json");
    report_generator.save_report(&report, &json_report, &OutputFormat::Json)?;
    println!("     💾 JSON report saved: {:?}", json_report);
    
    // CSV report
    let csv_report = output_dir.join("report.csv");
    report_generator.save_report(&report, &csv_report, &OutputFormat::Csv)?;
    println!("     💾 CSV report saved: {:?}", csv_report);
    
    // Text summary
    let text_report = output_dir.join("summary.txt");
    report_generator.save_report(&report, &text_report, &OutputFormat::Summary)?;
    println!("     💾 Text summary saved: {:?}", text_report);
    
    println!();
    Ok(())
}

/// Example 6: Advanced processing engine usage
fn example_6_advanced_usage(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Example 6: Advanced Processing Engine Usage");
    
    let engine = FileProcessorEngine::new();
    
    // Show supported extensions
    let extensions = engine.get_supported_extensions();
    println!("  📋 Supported file extensions: {:?}", extensions);
    
    // Process files with different validation settings
    let csv_file = temp_dir.path().join("example.csv");
    
    // Process with validation enabled
    let mut strict_options = ProcessingOptions::default();
    strict_options.validate_data = true;
    
    println!("  🔍 Processing with validation enabled:");
    match engine.process_file(csv_file.to_str().unwrap(), &strict_options) {
        Ok(data) => {
            println!("     ✅ Validation passed: {} records", data.record_count);
            
            // Show some metadata
            for (key, value) in &data.metadata {
                println!("     📊 {}: {}", key, value);
            }
        }
        Err(e) => {
            println!("     ❌ Validation failed: {}", e);
        }
    }
    
    // Process with validation disabled
    let mut lenient_options = ProcessingOptions::default();
    lenient_options.validate_data = false;
    
    println!("  🔍 Processing with validation disabled:");
    match engine.process_file(csv_file.to_str().unwrap(), &lenient_options) {
        Ok(data) => {
            println!("     ✅ Processing completed: {} records", data.record_count);
        }
        Err(e) => {
            println!("     ❌ Processing failed: {}", e);
        }
    }
    
    println!();
    Ok(())
}

/// Helper function to set up test files for examples
fn setup_test_files(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    // Create JSON test file
    let json_file = temp_dir.path().join("example.json");
    let json_content = r#"{
        "products": [
            {"id": 1, "name": "Laptop", "price": 999.99, "category": "Electronics"},
            {"id": 2, "name": "Mouse", "price": 29.99, "category": "Electronics"},
            {"id": 3, "name": "Keyboard", "price": 79.99, "category": "Electronics"}
        ],
        "meta": {
            "total": 3,
            "currency": "USD"
        }
    }"#;
    fs::write(&json_file, json_content)?;
    
    // Create CSV test file
    let csv_file = temp_dir.path().join("example.csv");
    let csv_content = "id,name,email,department,salary\n1,Alice,alice@company.com,Engineering,85000\n2,Bob,bob@company.com,Marketing,65000\n3,Carol,carol@company.com,Sales,72000";
    fs::write(&csv_file, csv_content)?;
    
    // Create text test file
    let text_file = temp_dir.path().join("example.txt");
    let text_content = "# System Log\n\nINFO: Application started\nWARN: High memory usage\nERROR: Database connection failed\nINFO: Retrying connection\nINFO: Connection restored";
    fs::write(&text_file, text_content)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_setup() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_files(&temp_dir).unwrap();
        
        // Verify files were created
        assert!(temp_dir.path().join("example.json").exists());
        assert!(temp_dir.path().join("example.csv").exists());
        assert!(temp_dir.path().join("example.txt").exists());
    }
    
    #[test]
    fn test_quick_processing_example() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_files(&temp_dir).unwrap();
        
        let json_file = temp_dir.path().join("example.json");
        let result = process_file(json_file.to_str().unwrap());
        
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.format, "json");
        assert!(data.record_count > 0);
    }
    
    #[test]
    fn test_batch_processing_example() {
        let temp_dir = TempDir::new().unwrap();
        setup_test_files(&temp_dir).unwrap();
        
        let files = vec![
            temp_dir.path().join("example.json").to_str().unwrap().to_string(),
            temp_dir.path().join("example.csv").to_str().unwrap().to_string(),
        ];
        
        let result = process_files(&files);
        
        assert_eq!(result.successes.len(), 2);
        assert_eq!(result.failures.len(), 0);
        assert!(result.is_success());
    }
}
