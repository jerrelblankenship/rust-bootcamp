// Integration tests for the calculator project
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_addition() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["5", "+", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5 + 3 = 8"));
}

#[test]
fn test_basic_subtraction() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["10", "-", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("10 - 4 = 6"));
}

#[test]
fn test_basic_multiplication() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["6", "*", "7"])
        .assert()
        .success()
        .stdout(predicate::str::contains("6 * 7 = 42"));
}

#[test]
fn test_basic_division() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["15", "/", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("15 / 3 = 5"));
}

#[test]
fn test_floating_point_numbers() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["3.14", "+", "2.86"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3.14 + 2.86 = 6"));
}

#[test]
fn test_negative_numbers() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["-5", "+", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-5 + 3 = -2"));
}

#[test]
fn test_division_by_zero() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["10", "/", "0"])
        .assert()
        .failure();  // Should fail, don't require specific error message
}

#[test]
fn test_invalid_number() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["abc", "+", "3"])
        .assert()
        .failure();  // Should fail, don't require specific error message
}

#[test]
fn test_invalid_operation() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["5", "^", "3"])
        .assert()
        .failure();  // Should fail, don't require specific error message
}

#[test]
fn test_wrong_number_of_arguments() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["5", "+"])
        .assert()
        .failure();  // Should fail, don't require specific error message
}

#[test]
fn test_zero_operations() {
    // Test adding zero
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["42", "+", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("42 + 0 = 42"));
    
    // Test multiplying by zero
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["42", "*", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("42 * 0 = 0"));
}

#[test]
fn test_large_numbers() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["999999", "+", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("999999 + 1 = 1000000"));
}

#[test]
fn test_very_small_numbers() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.args(&["0.0001", "*", "10000"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0.0001 * 10000 = 1"));
}