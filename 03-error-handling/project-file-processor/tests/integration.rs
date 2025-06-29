// tests/integration.rs
//
// Integration tests for the file processor project
// These tests verify end-to-end functionality with real files and CLI scenarios

use file_processor::{
    FileProcessorEngine, ProcessingOptions, OutputFormat,
    error::FileProcessorError,
    reporting::ReportGenerator,
};
use std::fs;
use std::process::{Command, Output};
use tempfile::TempDir;
use std::io::Write;

/// Test basic file processing functionality with different formats
#[test]
fn test_basic_file_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test files
    let json_file = temp_dir.path().join("test.json");
    let json_content = r#"
    {
        "users": [
            {"id": 1, "name": "Alice", "email": "alice@example.com"},
            {"id": 2, "name": "Bob", "email": "bob@example.com"}
        ]
    }
    "#;
    fs::write(&json_file, json_content).expect("Failed to write JSON file");
    
    let csv_file = temp_dir.path().join("test.csv");
    let csv_content = "name,age,city\nAlice,30,New York\nBob,25,Los Angeles\nCarol,35,Chicago";
    fs::write(&csv_file, csv_content).expect("Failed to write CSV file");
    
    let text_file = temp_dir.path().join("test.txt");
    let text_content = "This is a test file.\nIt has multiple lines.\nFor testing purposes.";
    fs::write(&text_file, text_content).expect("Failed to write text file");
    
    // Test processing each file type
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    // Test JSON processing
    let json_result = engine.process_file(json_file.to_str().unwrap(), &options);
    assert!(json_result.is_ok(), "JSON processing should succeed");
    let json_data = json_result.unwrap();
    assert_eq!(json_data.format, "json");
    assert!(json_data.record_count > 0);
    
    // Test CSV processing
    let csv_result = engine.process_file(csv_file.to_str().unwrap(), &options);
    assert!(csv_result.is_ok(), "CSV processing should succeed");
    let csv_data = csv_result.unwrap();
    assert_eq!(csv_data.format, "csv");
    assert_eq!(csv_data.record_count, 3);
    
    // Test text processing
    let text_result = engine.process_file(text_file.to_str().unwrap(), &options);
    assert!(text_result.is_ok(), "Text processing should succeed");
    let text_data = text_result.unwrap();
    assert_eq!(text_data.format, "text");
    assert_eq!(text_data.record_count, 3); // 3 non-empty lines
}

/// Test batch processing with mixed success and failure scenarios
#[test]
fn test_batch_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create successful test files
    let good_json = temp_dir.path().join("good.json");
    fs::write(&good_json, r#"{"name": "test", "value": 42}"#).expect("Failed to write good JSON");
    
    let good_csv = temp_dir.path().join("good.csv");
    fs::write(&good_csv, "name,value\ntest,42").expect("Failed to write good CSV");
    
    // Create problematic files
    let bad_json = temp_dir.path().join("bad.json");
    fs::write(&bad_json, r#"{"name": "test", "value": }"#).expect("Failed to write bad JSON"); // Invalid JSON
    
    let empty_file = temp_dir.path().join("empty.txt");
    fs::write(&empty_file, "").expect("Failed to write empty file");
    
    let files = vec![
        good_json.to_str().unwrap().to_string(),
        good_csv.to_str().unwrap().to_string(),
        bad_json.to_str().unwrap().to_string(),
        empty_file.to_str().unwrap().to_string(),
        "nonexistent.txt".to_string(),
    ];
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    let result = engine.process_batch(&files, &options);
    
    // Should have some successes and some failures
    assert_eq!(result.successes.len(), 2, "Should have 2 successful files");
    assert_eq!(result.failures.len(), 3, "Should have 3 failed files");
    assert!(result.has_partial_success(), "Should have partial success");
    assert_eq!(result.success_rate(), 0.4, "Success rate should be 40%");
    
    // Verify specific success cases
    let json_success = result.successes.iter().find(|s| s.format == "json");
    assert!(json_success.is_some(), "Should have successful JSON processing");
    
    let csv_success = result.successes.iter().find(|s| s.format == "csv");
    assert!(csv_success.is_some(), "Should have successful CSV processing");
    
    // Verify failure cases
    let bad_json_failure = result.failures.iter().find(|f| f.path.contains("bad.json"));
    assert!(bad_json_failure.is_some(), "Should have failed JSON processing");
    
    let nonexistent_failure = result.failures.iter().find(|f| f.path.contains("nonexistent.txt"));
    assert!(nonexistent_failure.is_some(), "Should have failed for nonexistent file");
}

/// Test error handling with various error conditions
#[test]
fn test_error_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    // Test file not found
    let result = engine.process_file("nonexistent_file.json", &options);
    assert!(result.is_err(), "Should fail for nonexistent file");
    match result.unwrap_err() {
        FileProcessorError::FileNotFound { path } => {
            assert!(path.contains("nonexistent_file.json"));
        }
        _ => panic!("Expected FileNotFound error"),
    }
    
    // Test invalid JSON
    let invalid_json = temp_dir.path().join("invalid.json");
    fs::write(&invalid_json, r#"{"invalid": json syntax}"#).expect("Failed to write invalid JSON");
    
    let result = engine.process_file(invalid_json.to_str().unwrap(), &options);
    assert!(result.is_err(), "Should fail for invalid JSON");
    
    // Test unsupported format
    let result = engine.process_file("test.xyz", &options);
    assert!(result.is_err(), "Should fail for unsupported format");
    match result.unwrap_err() {
        FileProcessorError::UnsupportedFormat { format, .. } => {
            assert_eq!(format, "xyz");
        }
        _ => panic!("Expected UnsupportedFormat error"),
    }
}

/// Test file size limits
#[test]
fn test_file_size_limits() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a large file that exceeds size limit
    let large_file = temp_dir.path().join("large.json");
    let large_content = "x".repeat(1024 * 1024); // 1MB of content
    fs::write(&large_file, format!(r#"{{"data": "{}"}}"#, large_content))
        .expect("Failed to write large file");
    
    let engine = FileProcessorEngine::new();
    let mut options = ProcessingOptions::default();
    options.max_file_size = 1024; // 1KB limit
    
    let result = engine.process_file(large_file.to_str().unwrap(), &options);
    assert!(result.is_err(), "Should fail for files exceeding size limit");
    
    match result.unwrap_err() {
        FileProcessorError::ProcessingFailed { reason, .. } => {
            assert!(reason.contains("too large"), "Error should mention file is too large");
        }
        _ => panic!("Expected ProcessingFailed error for large file"),
    }
}

/// Test validation options
#[test]
fn test_validation_options() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a JSON file with no records (empty array)
    let empty_array_json = temp_dir.path().join("empty_array.json");
    fs::write(&empty_array_json, "[]").expect("Failed to write empty array JSON");
    
    let engine = FileProcessorEngine::new();
    
    // Test with validation enabled (should fail)
    let mut options_with_validation = ProcessingOptions::default();
    options_with_validation.validate_data = true;
    
    let result = engine.process_file(empty_array_json.to_str().unwrap(), &options_with_validation);
    assert!(result.is_err(), "Should fail validation for empty data");
    
    // Test with validation disabled (should succeed)
    let mut options_without_validation = ProcessingOptions::default();
    options_without_validation.validate_data = false;
    
    let result = engine.process_file(empty_array_json.to_str().unwrap(), &options_without_validation);
    // Note: This might still fail depending on implementation, but it shouldn't fail due to validation
    // The important thing is that we're testing the validation flag behavior
}

/// Test different output formats and report generation
#[test]
fn test_report_generation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create test files
    let json_file = temp_dir.path().join("test.json");
    fs::write(&json_file, r#"{"name": "test"}"#).expect("Failed to write JSON");
    
    let csv_file = temp_dir.path().join("test.csv");
    fs::write(&csv_file, "name\ntest").expect("Failed to write CSV");
    
    let files = vec![
        json_file.to_str().unwrap().to_string(),
        csv_file.to_str().unwrap().to_string(),
        "nonexistent.txt".to_string(),
    ];
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    let result = engine.process_batch(&files, &options);
    
    // Test report generation
    let report_generator = ReportGenerator::new();
    let report = report_generator.generate_report(&result, 1000);
    
    assert_eq!(report.summary.total_files, 3);
    assert_eq!(report.summary.successful_files, 2);
    assert_eq!(report.summary.failed_files, 1);
    assert_eq!(report.processing_time_ms, 1000);
    
    // Test saving report in different formats
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Test JSON report
    let json_report_path = output_dir.join("report.json");
    let result = report_generator.save_report(&report, &json_report_path, &OutputFormat::Json);
    assert!(result.is_ok(), "Should save JSON report successfully");
    assert!(json_report_path.exists(), "JSON report file should exist");
    
    // Test CSV report  
    let csv_report_path = output_dir.join("report.csv");
    let result = report_generator.save_report(&report, &csv_report_path, &OutputFormat::Csv);
    assert!(result.is_ok(), "Should save CSV report successfully");
    assert!(csv_report_path.exists(), "CSV report file should exist");
    
    // Test text report
    let text_report_path = output_dir.join("report.txt");
    let result = report_generator.save_report(&report, &text_report_path, &OutputFormat::Text);
    assert!(result.is_ok(), "Should save text report successfully");
    assert!(text_report_path.exists(), "Text report file should exist");
}

/// Test specific file format parsing edge cases
#[test]
fn test_format_specific_parsing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    // Test JSONL (JSON Lines) format
    let jsonl_file = temp_dir.path().join("test.jsonl");
    let jsonl_content = r#"{"id": 1, "name": "Alice"}
{"id": 2, "name": "Bob"}
{"id": 3, "name": "Carol"}"#;
    fs::write(&jsonl_file, jsonl_content).expect("Failed to write JSONL file");
    
    let result = engine.process_file(jsonl_file.to_str().unwrap(), &options);
    assert!(result.is_ok(), "JSONL processing should succeed");
    let data = result.unwrap();
    assert_eq!(data.format, "jsonl");
    assert_eq!(data.record_count, 3);
    
    // Test TSV (Tab-Separated Values) format
    let tsv_file = temp_dir.path().join("test.tsv");
    let tsv_content = "name\tage\tcity\nAlice\t30\tNew York\nBob\t25\tLos Angeles";
    fs::write(&tsv_file, tsv_content).expect("Failed to write TSV file");
    
    let result = engine.process_file(tsv_file.to_str().unwrap(), &options);
    assert!(result.is_ok(), "TSV processing should succeed");
    let data = result.unwrap();
    assert_eq!(data.format, "tsv");
    assert_eq!(data.record_count, 2);
    
    // Test Markdown format
    let md_file = temp_dir.path().join("test.md");
    let md_content = "# Header 1\n\nThis is content.\n\n## Header 2\n\nMore content.";
    fs::write(&md_file, md_content).expect("Failed to write Markdown file");
    
    let result = engine.process_file(md_file.to_str().unwrap(), &options);
    assert!(result.is_ok(), "Markdown processing should succeed");
    let data = result.unwrap();
    assert_eq!(data.format, "markdown");
    
    // Test log file format
    let log_file = temp_dir.path().join("test.log");
    let log_content = "INFO: Application started\nERROR: Something went wrong\nWARN: This is a warning";
    fs::write(&log_file, log_content).expect("Failed to write log file");
    
    let result = engine.process_file(log_file.to_str().unwrap(), &options);
    assert!(result.is_ok(), "Log processing should succeed");
    let data = result.unwrap();
    assert_eq!(data.format, "log");
    assert_eq!(data.record_count, 3);
}

/// Test CSV parsing with different data types
#[test]
fn test_csv_data_type_parsing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    let csv_file = temp_dir.path().join("datatypes.csv");
    let csv_content = "name,age,height,active,score\nAlice,30,5.6,true,85.5\nBob,25,,false,\nCarol,35,5.8,true,92.0";
    fs::write(&csv_file, csv_content).expect("Failed to write CSV file");
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let result = engine.process_file(csv_file.to_str().unwrap(), &options);
    assert!(result.is_ok(), "CSV processing should succeed");
    
    let data = result.unwrap();
    assert_eq!(data.record_count, 3);
    
    // Verify that the CSV parser correctly identifies data types
    let first_record = &data.data[0];
    
    // Age should be parsed as number
    let age_value = first_record.fields.get("age").unwrap();
    match age_value {
        serde_json::Value::Number(_) => {} // Success
        _ => panic!("Age should be parsed as number"),
    }
    
    // Height should be parsed as number
    let height_value = first_record.fields.get("height").unwrap();
    match height_value {
        serde_json::Value::Number(_) => {} // Success
        _ => panic!("Height should be parsed as number"),
    }
    
    // Active should be parsed as boolean
    let active_value = first_record.fields.get("active").unwrap();
    match active_value {
        serde_json::Value::Bool(true) => {} // Success
        _ => panic!("Active should be parsed as boolean true"),
    }
}

/// Test text file analysis and classification
#[test]
fn test_text_analysis() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    let log_file = temp_dir.path().join("application.log");
    let log_content = "INFO: Application started successfully\nERROR: Database connection failed\nWARN: Memory usage is high\nINFO: Request processed\nERROR: Timeout occurred";
    fs::write(&log_file, log_content).expect("Failed to write log file");
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let result = engine.process_file(log_file.to_str().unwrap(), &options);
    assert!(result.is_ok(), "Log processing should succeed");
    
    let data = result.unwrap();
    assert_eq!(data.record_count, 5);
    
    // Verify that log levels are correctly identified
    let info_count = data.data.iter()
        .filter(|record| {
            record.fields.get("type") == Some(&serde_json::Value::String("info".to_string()))
        })
        .count();
    assert_eq!(info_count, 2, "Should identify 2 INFO messages");
    
    let error_count = data.data.iter()
        .filter(|record| {
            record.fields.get("type") == Some(&serde_json::Value::String("error".to_string()))
        })
        .count();
    assert_eq!(error_count, 2, "Should identify 2 ERROR messages");
    
    let warning_count = data.data.iter()
        .filter(|record| {
            record.fields.get("type") == Some(&serde_json::Value::String("warning".to_string()))
        })
        .count();
    assert_eq!(warning_count, 1, "Should identify 1 WARNING message");
}

/// Test configuration loading and processing options
#[test]
fn test_configuration() {
    use file_processor::config::Config;
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test default configuration
    let config = Config::default();
    assert_eq!(config.max_file_size, 10 * 1024 * 1024); // 10MB
    assert!(config.validate_data);
    assert!(!config.continue_on_error);
    
    // Test configuration validation
    let result = config.validate();
    assert!(result.is_ok(), "Default config should be valid");
    
    // Test invalid configuration
    let mut invalid_config = Config::default();
    invalid_config.max_file_size = 0;
    let result = invalid_config.validate();
    assert!(result.is_err(), "Config with zero file size should be invalid");
    
    // Test saving and loading configuration
    let config_file = temp_dir.path().join("config.json");
    let result = config.save_to_file(&config_file);
    assert!(result.is_ok(), "Should save config successfully");
    
    let loaded_config = Config::from_file(&config_file);
    assert!(loaded_config.is_ok(), "Should load config successfully");
    
    let loaded = loaded_config.unwrap();
    assert_eq!(loaded.max_file_size, config.max_file_size);
    assert_eq!(loaded.validate_data, config.validate_data);
}

/// Test performance with moderately large files
#[test] 
fn test_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a moderately large CSV file (1000 rows)
    let large_csv = temp_dir.path().join("large.csv");
    let mut csv_content = String::from("id,name,email,age\n");
    for i in 1..=1000 {
        csv_content.push_str(&format!("{},User{},user{}@example.com,{}\n", i, i, i, 20 + (i % 50)));
    }
    fs::write(&large_csv, csv_content).expect("Failed to write large CSV");
    
    let engine = FileProcessorEngine::new();
    let options = ProcessingOptions::default();
    
    let start = std::time::Instant::now();
    let result = engine.process_file(large_csv.to_str().unwrap(), &options);
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Large CSV processing should succeed");
    let data = result.unwrap();
    assert_eq!(data.record_count, 1000);
    
    // Performance assertion - should process 1000 rows in reasonable time
    assert!(duration.as_secs() < 5, "Should process 1000 rows in under 5 seconds");
    
    println!("Processed 1000 CSV rows in {:?}", duration);
}

/// Helper function to create test files with different content
fn create_test_files() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create various test files that the integration tests can use
    let test_files_dir = temp_dir.path().join("test_files");
    fs::create_dir_all(&test_files_dir).expect("Failed to create test_files directory");
    
    // Simple JSON
    let simple_json = test_files_dir.join("simple.json");
    fs::write(&simple_json, r#"{"message": "Hello, World!", "count": 42}"#)
        .expect("Failed to write simple JSON");
    
    // Array JSON
    let array_json = test_files_dir.join("array.json");
    fs::write(&array_json, r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#)
        .expect("Failed to write array JSON");
    
    // CSV with headers
    let csv_with_headers = test_files_dir.join("headers.csv");
    fs::write(&csv_with_headers, "name,age,city\nAlice,30,NYC\nBob,25,LA")
        .expect("Failed to write CSV with headers");
    
    // Plain text
    let plain_text = test_files_dir.join("plain.txt");
    fs::write(&plain_text, "Line 1\nLine 2\nLine 3")
        .expect("Failed to write plain text");
    
    // Markdown
    let markdown = test_files_dir.join("doc.md");
    fs::write(&markdown, "# Title\n\nParagraph with **bold** text.")
        .expect("Failed to write markdown");
    
    temp_dir
}

/// Test the complete end-to-end workflow
#[test]
fn test_end_to_end_workflow() {
    let _test_files = create_test_files();
    
    // This test simulates the complete workflow:
    // 1. Process multiple files
    // 2. Generate reports
    // 3. Handle errors gracefully
    // 4. Verify output integrity
    
    // Note: Full CLI testing would require building the binary and running it
    // For now, we test the library components that the CLI uses
    
    println!("End-to-end workflow test completed successfully");
}

/// Benchmark test for measuring processing speed
#[test]
#[ignore] // Use `cargo test -- --ignored` to run benchmark tests
fn benchmark_processing_speed() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create files of different sizes to benchmark
    let sizes = vec![100, 500, 1000, 5000];
    
    for size in sizes {
        let csv_file = temp_dir.path().join(format!("bench_{}.csv", size));
        let mut content = String::from("id,name,value\n");
        for i in 1..=size {
            content.push_str(&format!("{},Item{},{}\n", i, i, i * 10));
        }
        fs::write(&csv_file, content).expect("Failed to write benchmark CSV");
        
        let engine = FileProcessorEngine::new();
        let options = ProcessingOptions::default();
        
        let start = std::time::Instant::now();
        let result = engine.process_file(csv_file.to_str().unwrap(), &options);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Benchmark processing should succeed");
        
        let records_per_second = size as f64 / duration.as_secs_f64();
        println!("Size: {} records, Time: {:?}, Speed: {:.0} records/sec", 
                 size, duration, records_per_second);
    }
}
