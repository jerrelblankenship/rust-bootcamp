// Integration tests for the Memory Visualizer
//
// These tests verify that the complete application works correctly

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("memory management visualizer"));
}

#[test]
fn test_ownership_demo() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("ownership")
        .assert()
        .success()
        .stdout(predicate::str::contains("Ownership Demonstration"));
}

#[test]
fn test_borrowing_demo() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("borrowing")
        .assert()
        .success()
        .stdout(predicate::str::contains("Borrowing Demonstration"));
}

#[test]
fn test_smart_pointers_demo() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("smart-pointers")
        .assert()
        .success()
        .stdout(predicate::str::contains("Smart Pointers Demonstration"));
}

#[test]
fn test_csharp_comparison() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("compare-csharp")
        .assert()
        .success()
        .stdout(predicate::str::contains("C# vs Rust Comparison"));
}

#[test]
fn test_default_overview() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Memory Visualizer"))
        .stdout(predicate::str::contains("Quick overview"));
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.args(&["ownership", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Memory Operations Summary"));
}

// TODO: Add tests for interactive mode
// Note: Interactive tests are more complex and might need special handling

#[test]
fn test_memory_tracking() {
    // TODO: Test that memory operations are tracked correctly
    // This would require examining the output for allocation/deallocation messages
}

#[test]
fn test_visualization_output() {
    // TODO: Test that visualization functions produce expected ASCII art
}

// Performance tests (marked with ignore so they don't run by default)
#[test]
#[ignore]
fn test_performance_large_operations() {
    // TODO: Test performance with many memory operations
}

#[test]
#[ignore]
fn test_memory_usage() {
    // TODO: Test that the visualizer itself doesn't use excessive memory
}
