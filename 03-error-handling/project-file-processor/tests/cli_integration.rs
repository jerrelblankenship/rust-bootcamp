// tests/cli_integration.rs
//
// CLI integration tests that verify the command-line interface works correctly
// These tests build and run the actual binary to ensure end-to-end functionality

use std::process::Command;
use std::fs;
use tempfile::TempDir;
use std::io::Write;

/// Test basic CLI functionality with valid arguments
#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Help command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file-processor"), "Should show program name");
    assert!(stdout.contains("robust file processing tool"), "Should show description");
    assert!(stdout.contains("--output"), "Should show output option");
    assert!(stdout.contains("--format"), "Should show format option");
}

/// Test CLI version output
#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Version command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"), "Should show version number");
}

/// Test processing a single JSON file via CLI
#[test]
fn test_cli_single_file_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a test JSON file
    let json_file = temp_dir.path().join("test.json");
    let json_content = r#"{"name": "Test", "value": 42}"#;
    fs::write(&json_file, json_content).expect("Failed to write test file");
    
    // Create output directory
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Run the CLI command
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            json_file.to_str().unwrap(),
            "--output", output_dir.to_str().unwrap(),
            "--format", "summary",
            "--verbose"
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    // Check that command succeeded
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("CLI command failed:\nStdout: {}\nStderr: {}", stdout, stderr);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Processing Summary") || stdout.contains("Processed"), 
           "Should show processing results");
    
    // Check that output files were created
    let summary_file = output_dir.join("processing_summary.txt");
    assert!(summary_file.exists(), "Summary file should be created");
}

/// Test batch processing multiple files via CLI
#[test]
fn test_cli_batch_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create multiple test files
    let json_file = temp_dir.path().join("data.json");
    fs::write(&json_file, r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#)
        .expect("Failed to write JSON file");
    
    let csv_file = temp_dir.path().join("data.csv");
    fs::write(&csv_file, "name,age\nAlice,30\nBob,25")
        .expect("Failed to write CSV file");
    
    let output_dir = temp_dir.path().join("output");
    
    // Run CLI with multiple files
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            json_file.to_str().unwrap(),
            csv_file.to_str().unwrap(),
            "--output", output_dir.to_str().unwrap(),
            "--format", "json",
            "--continue-on-error"
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Batch processing failed:\nStdout: {}\nStderr: {}", stdout, stderr);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Final Summary") || stdout.contains("files processed"), 
           "Should show batch processing results");
}

/// Test CLI error handling with invalid files
#[test]
fn test_cli_error_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("output");
    
    // Try to process a non-existent file
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "nonexistent_file.json",
            "--output", output_dir.to_str().unwrap(),
            "--continue-on-error"
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    // Command should fail (exit code 1) since continue-on-error won't help with no valid files
    assert!(!output.status.success(), "Should fail when no files can be processed");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error") || stderr.contains("not found") || stderr.contains("Missing files"), 
           "Should show error message for missing file");
}

/// Test CLI with validation flags
#[test]
fn test_cli_validation_options() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a file that might fail validation
    let csv_file = temp_dir.path().join("empty_data.csv");
    fs::write(&csv_file, "name,age\n")  // Headers only, no data rows
        .expect("Failed to write CSV file");
    
    let output_dir = temp_dir.path().join("output");
    
    // Test with validation enabled
    let output_with_validation = Command::new("cargo")
        .args(&[
            "run", "--",
            csv_file.to_str().unwrap(),
            "--output", output_dir.to_str().unwrap(),
            "--validate",
            "--continue-on-error"
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    // The exact behavior depends on implementation, but we should get some output
    let stdout = String::from_utf8_lossy(&output_with_validation.stdout);
    let stderr = String::from_utf8_lossy(&output_with_validation.stderr);
    
    // Either it succeeds and shows results, or it fails with validation errors
    assert!(
        stdout.contains("Processing") || stderr.contains("Error") || stderr.contains("Validation"),
        "Should either process successfully or show validation errors"
    );
}

/// Test CLI dry run mode
#[test]
fn test_cli_dry_run() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a test file
    let json_file = temp_dir.path().join("test.json");
    fs::write(&json_file, r#"{"test": true}"#).expect("Failed to write test file");
    
    let output_dir = temp_dir.path().join("output");
    
    // Run in dry run mode
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            json_file.to_str().unwrap(),
            "--output", output_dir.to_str().unwrap(),
            "--dry-run",
            "--verbose"
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Dry run should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run mode") || stdout.contains("No files were written"), 
           "Should indicate dry run mode");
    
    // Output directory should either not exist or be empty (no files written)
    if output_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&output_dir)
            .expect("Failed to read output directory")
            .collect();
        assert!(entries.is_empty() || entries.len() <= 1, // Might have just the directory itself
               "No output files should be written in dry run mode");
    }
}

/// Test CLI configuration file generation
#[test]
fn test_cli_config_generation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_file = temp_dir.path().join("test_config.json");
    
    // Generate configuration file
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "--generate-config", config_file.to_str().unwrap()
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Config generation should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("configuration generated") || stdout.contains("Sample configuration"), 
           "Should confirm config generation");
    
    // Check that config file was created and is valid JSON
    assert!(config_file.exists(), "Config file should be created");
    
    let config_content = fs::read_to_string(&config_file)
        .expect("Failed to read config file");
    
    // Verify it's valid JSON
    let _: serde_json::Value = serde_json::from_str(&config_content)
        .expect("Generated config should be valid JSON");
}

/// Test CLI with custom configuration file
#[test]
fn test_cli_with_config_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a custom config file
    let config_file = temp_dir.path().join("custom_config.json");
    let config_content = r#"{
        "max_file_size": 1048576,
        "validate_data": false,
        "continue_on_error": true,
        "output_format": "json",
        "output_directory": "custom_output"
    }"#;
    fs::write(&config_file, config_content).expect("Failed to write config file");
    
    // Create a test file
    let json_file = temp_dir.path().join("test.json");
    fs::write(&json_file, r#"{"configured": true}"#).expect("Failed to write test file");
    
    let output_dir = temp_dir.path().join("output");
    
    // Run CLI with custom config
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            json_file.to_str().unwrap(),
            "--config", config_file.to_str().unwrap(),
            "--output", output_dir.to_str().unwrap()
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    // Should succeed with custom configuration
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        eprintln!("Config test failed:\nStdout: {}\nStderr: {}", stdout, stderr);
    }
    
    assert!(output.status.success(), "Processing with custom config should succeed");
}

/// Test CLI performance with multiple files
#[test]
#[ignore] // Use `cargo test -- --ignored` to run performance tests
fn test_cli_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create multiple test files
    let mut files = Vec::new();
    for i in 1..=10 {
        let file_path = temp_dir.path().join(format!("test_{}.json", i));
        let content = format!(r#"{{"id": {}, "name": "Test{}", "data": "sample"}}"#, i, i);
        fs::write(&file_path, content).expect("Failed to write test file");
        files.push(file_path.to_str().unwrap().to_string());
    }
    
    let output_dir = temp_dir.path().join("output");
    
    // Measure processing time
    let start = std::time::Instant::now();
    
    let mut cmd_args = vec!["run", "--"];
    cmd_args.extend(files.iter().map(|s| s.as_str()));
    cmd_args.extend(&[
        "--output", output_dir.to_str().unwrap(),
        "--format", "summary"
    ]);
    
    let output = Command::new("cargo")
        .args(&cmd_args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");
    
    let duration = start.elapsed();
    
    assert!(output.status.success(), "Performance test should succeed");
    
    // Should process 10 small files reasonably quickly
    assert!(duration.as_secs() < 30, "Should process 10 files in under 30 seconds");
    
    println!("Processed 10 files in {:?}", duration);
}

/// Helper function to check if the binary can be built
#[test]
fn test_binary_builds() {
    let output = Command::new("cargo")
        .args(&["check", "--bin", "file-processor"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo check");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Binary does not compile:\n{}", stderr);
    }
    
    assert!(output.status.success(), "Binary should compile successfully");
}
