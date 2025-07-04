// Exercise 2: Test Organization - Fix the Broken Test Structure!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken test module organization and isolation issues
//
// INSTRUCTIONS:
// 1. Fix ONE organizational error at a time
// 2. Run tests: `cargo test --bin ex02-test-organization`
// 3. Watch tests run in isolation without interference
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (missing cfg attribute)
// - Learn how Rust organizes tests vs C# test classes
// - Fix test isolation and shared state issues
//
// COMPLETED CONCEPTS:
// [] Test module organization (#[cfg(test)])
// [] Test isolation and independence
// [] Shared test utilities and helpers
// [] Conditional compilation for tests
// [] Test module hierarchy
// [] Private vs public test helpers

// A simple bank account system to test
pub struct BankAccount {
    balance: f64,
    account_number: String,
}

impl BankAccount {
    pub fn new(account_number: String, initial_balance: f64) -> Self {
        Self {
            balance: initial_balance,
            account_number,
        }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_account_number(&self) -> &str {
        &self.account_number
    }
}

// Global counter for testing (BAD PRACTICE - will cause issues!)
static mut GLOBAL_TEST_COUNTER: i32 = 0;

// BROKEN TEST ORGANIZATION TO FIX!

// Exercise 2.1: Fix missing cfg attribute
// FIXME: This module will be compiled in release builds!
mod tests {  // COMPILE ERROR: Missing #[cfg(test)] attribute!
    use super::*;

    // Exercise 2.2: Fix test isolation issue
    // FIXME: This test modifies global state!
    #[test]
    fn test_account_creation_with_counter() {
        unsafe {
            GLOBAL_TEST_COUNTER += 1;  // BAD: Modifies global state!
        }
        let account = BankAccount::new("123".to_string(), 100.0);
        assert_eq!(account.get_balance(), 100.0);
        
        unsafe {
            // FIXME: This will fail if tests run in parallel!
            assert_eq!(GLOBAL_TEST_COUNTER, 1);  // FLAKY: Depends on test execution order!
        }
    }
    // In C#, you'd use [TestInitialize] or constructor, but avoid shared state
    
    // ✅ CHECKPOINT 1: Add #[cfg(test)] to module and fix isolation

    #[test]
    fn test_another_counter_test() {
        unsafe {
            GLOBAL_TEST_COUNTER += 1;  // BAD: More global state modification!
        }
        let account = BankAccount::new("456".to_string(), 200.0);
        assert_eq!(account.get_balance(), 200.0);
        
        unsafe {
            // FIXME: This will fail when run with other tests!
            assert_eq!(GLOBAL_TEST_COUNTER, 2);  // FLAKY: Assumes specific execution order!
        }
    }
    
    // ✅ CHECKPOINT 2: Remove global state dependency and make tests independent

    // Exercise 2.3: Fix test helper organization
    // FIXME: Helper function in wrong place!
    fn create_test_account() -> BankAccount {  // BAD: Not accessible to other test modules!
        BankAccount::new("TEST123".to_string(), 1000.0)
    }
    
    #[test]
    fn test_deposit_valid_amount() {
        let mut account = create_test_account();
        let result = account.deposit(50.0);
        assert!(result.is_ok());
        assert_eq!(account.get_balance(), 1050.0);
    }
    
    // ✅ CHECKPOINT 3: Move helper to proper location for reuse
}

// Exercise 2.4: Fix nested test module issues
// FIXME: Wrong module structure for nested tests!
mod account_validation_tests {  // MISSING: #[cfg(test)] attribute!
    use super::*;
    
    #[test]
    fn test_negative_deposit() {
        let mut account = BankAccount::new("NEG001".to_string(), 100.0);
        let result = account.deposit(-10.0);
        assert!(result.is_err());
    }
    
    #[test] 
    fn test_zero_deposit() {
        let mut account = BankAccount::new("ZERO001".to_string(), 100.0);
        let result = account.deposit(0.0);
        assert!(result.is_err());
    }
    
    // FIXME: Can't access the helper function from the other module!
    #[test]
    fn test_withdrawal_from_test_account() {
        let mut account = create_test_account();  // COMPILE ERROR: Function not accessible!
        let result = account.withdraw(100.0);
        assert!(result.is_ok());
    }
}

// ✅ CHECKPOINT 4: Fix nested module structure and helper access

// Exercise 2.5: Fix test utility organization
// FIXME: Test utilities mixed with production code!
pub fn create_account_with_balance(balance: f64) -> BankAccount {  // BAD: Public in production!
    BankAccount::new(format!("TEST{}", balance), balance)
}

pub fn assert_account_balance(account: &BankAccount, expected: f64) {  // BAD: Test helper in production!
    assert_eq!(account.get_balance(), expected);
}

// ✅ CHECKPOINT 5: Move test utilities to appropriate test-only location

// Exercise 2.6: Fix conditional compilation issues
#[test]  // FIXME: Test outside of #[cfg(test)] module!
fn test_account_number_format() {
    let account = BankAccount::new("FORMAT123".to_string(), 0.0);
    assert_eq!(account.get_account_number(), "FORMAT123");
}

// This should only be available during testing
#[cfg(test)]
fn debug_account_state(account: &BankAccount) {
    println!("Account: {} has balance: {}", 
             account.get_account_number(), 
             account.get_balance());
}

// ✅ CHECKPOINT 6: Organize conditional compilation properly

// COMPILATION CHALLENGES:
// 1. Add #[cfg(test)] attributes to test modules
// 2. Remove global state dependencies between tests
// 3. Organize test helpers for proper accessibility
// 4. Fix nested test module structure
// 5. Move test utilities to test-only sections
// 6. Ensure tests can run independently and in parallel
//
// LEARNING OBJECTIVES:
// - Understanding Rust test module organization with #[cfg(test)]
// - Creating isolated, independent tests
// - Organizing test helpers and utilities
// - Proper conditional compilation for test code
// - Test module hierarchy and accessibility
// - Comparing with C# test class organization
//
// C# COMPARISON:
// C#: [TestClass] with [TestMethod] attributes
// Rust: #[cfg(test)] modules with #[test] functions
//
// C#: Test classes naturally isolated
// Rust: Tests run in parallel, need manual isolation
//
// C#: [TestInitialize] for setup
// Rust: Helper functions and test builders
//
// C#: Separate test projects
// Rust: #[cfg(test)] conditional compilation
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (cfg attribute): [ ] Complete
// Checkpoint 2 (Test isolation): [ ] Complete
// Checkpoint 3 (Helper organization): [ ] Complete
// Checkpoint 4 (Nested modules): [ ] Complete
// Checkpoint 5 (Test utilities): [ ] Complete
// Checkpoint 6 (Conditional compilation): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Rust test module organization and structure
// ✅ Test isolation and independence
// ✅ Proper test helper organization
// ✅ Conditional compilation for tests
// ✅ Test module hierarchy and accessibility
//
// 🚀 Ready for the next challenge?
// Move on to ex03-integration-tests.rs to learn integration testing!
// Or check your work with: `cargo test --bin ex02-test-organization`