// Integration tests for the web scraper
// These tests will help verify your fixes work correctly

use std::process::Command;

#[test]
fn test_scraper_compiles() {
    // First test: Does the code even compile?
    let output = Command::new("cargo")
        .args(&["check", "--bin", "scraper"])
        .current_dir("..")
        .output()
        .expect("Failed to run cargo check");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Scraper failed to compile:\n{}", stderr);
    }
}

#[test]
fn test_scraper_runs_without_panic() {
    // Second test: Does it run without panicking?
    let output = Command::new("cargo")
        .args(&["run", "--bin", "scraper"])
        .current_dir("..")
        .output()
        .expect("Failed to run scraper");

    // Should exit successfully (code 0)
    assert!(output.status.success(), 
        "Scraper exited with error code: {}\nStderr: {}", 
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_scraper_produces_output() {
    // Third test: Does it produce expected output?
    let output = Command::new("cargo")
        .args(&["run", "--bin", "scraper"])
        .current_dir("..")
        .output()
        .expect("Failed to run scraper");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should contain scraping results
    assert!(stdout.contains("Scraping Results"), 
        "Output should contain 'Scraping Results'\nActual output: {}", stdout);
    
    // Should show completed message
    assert!(stdout.contains("Scraping completed"), 
        "Output should contain 'Scraping completed'\nActual output: {}", stdout);
}

// TODO: Add more specific tests once the basic functionality works
// - Test with invalid URLs
// - Test thread safety
// - Test error handling
// - Test performance with many URLs