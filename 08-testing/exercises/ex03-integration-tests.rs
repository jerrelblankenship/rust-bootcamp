// Exercise 3: Integration Testing - Fix the Broken Integration Test Patterns!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken integration test setup and public API testing
//
// INSTRUCTIONS:
// 1. Fix ONE integration test issue at a time
// 2. Run tests: `cargo test --bin ex03-integration-tests`
// 3. Learn the difference between unit tests and integration tests
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (trying to access private internals)
// - Learn what integration tests should and shouldn't test
// - Fix test setup and teardown issues
//
// COMPLETED CONCEPTS:
// [] Integration vs unit test differences
// [] Testing only public APIs
// [] Test setup and teardown patterns
// [] External dependency simulation
// [] File system and database testing
// [] End-to-end workflow testing

use std::fs;
use std::path::Path;
use std::collections::HashMap;

// A simple file-based user management system
pub struct UserManager {
    users: HashMap<String, User>,
    file_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
}

impl UserManager {
    pub fn new(file_path: &str) -> Self {
        Self {
            users: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.username) {
            return Err("User already exists".to_string());
        }
        self.users.insert(user.username.clone(), user);
        self.save_to_file()
    }

    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }

    pub fn remove_user(&mut self, username: &str) -> Result<(), String> {
        if self.users.remove(username).is_some() {
            self.save_to_file()
        } else {
            Err("User not found".to_string())
        }
    }

    pub fn load_from_file(&mut self) -> Result<(), String> {
        if !Path::new(&self.file_path).exists() {
            return Ok(()); // File doesn't exist yet, that's fine
        }
        
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        self.users.clear();
        for line in content.lines() {
            if let Some(user) = self.parse_user_line(line) {
                self.users.insert(user.username.clone(), user);
            }
        }
        Ok(())
    }

    // Private helper method - should NOT be tested directly in integration tests
    fn save_to_file(&self) -> Result<(), String> {
        let content: String = self.users
            .values()
            .map(|user| format!("{}:{}:{}", user.username, user.email, user.active))
            .collect::<Vec<_>>()
            .join("\n");
        
        fs::write(&self.file_path, content)
            .map_err(|e| format!("Failed to write file: {}", e))
    }

    // Private helper method - should NOT be tested directly in integration tests
    fn parse_user_line(&self, line: &str) -> Option<User> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            return None;
        }
        
        Some(User {
            username: parts[0].to_string(),
            email: parts[1].to_string(),
            active: parts[2].parse().unwrap_or(false),
        })
    }
}

// BROKEN INTEGRATION TESTS TO FIX!

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;

    // Exercise 3.1: Fix trying to test private implementation details
    // FIXME: Integration tests should only test public API!
    #[test]
    fn test_save_to_file_internal() {
        let mut manager = UserManager::new("test_output.txt");
        let user = User {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            active: true,
        };
        manager.users.insert(user.username.clone(), user);  // COMPILE ERROR: 'users' field is private!
        
        // FIXME: Calling private method directly!
        let result = manager.save_to_file();  // COMPILE ERROR: Method is private!
        assert!(result.is_ok());
    }
    // In C#, you might use InternalsVisibleTo, but integration tests should test public API
    
    // ✅ CHECKPOINT 1: Test public API only, not private implementation details

    #[test]
    fn test_parse_user_line_internal() {
        let manager = UserManager::new("test.txt");
        // FIXME: Testing private method directly!
        let result = manager.parse_user_line("john:john@example.com:true");  // COMPILE ERROR: Method is private!
        assert!(result.is_some());
    }
    
    // ✅ CHECKPOINT 2: Remove tests of private methods - test through public API

    // Exercise 3.2: Fix file system test interference
    // FIXME: Tests will interfere with each other!
    #[test]
    fn test_user_persistence() {
        let mut manager = UserManager::new("users.txt");  // BAD: Same file name!
        
        let user = User {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            active: true,
        };
        
        manager.add_user(user.clone()).unwrap();
        
        // Create new manager to test persistence
        let mut new_manager = UserManager::new("users.txt");
        new_manager.load_from_file().unwrap();
        
        let loaded_user = new_manager.get_user("alice");
        assert_eq!(loaded_user, Some(&user));
        
        // FIXME: Not cleaning up test file!
        // File will persist and interfere with other tests!
    }
    
    // ✅ CHECKPOINT 3: Use unique file names and clean up test files

    #[test]
    fn test_multiple_users_persistence() {
        let mut manager = UserManager::new("users.txt");  // BAD: Same file name again!
        
        let user1 = User {
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            active: true,
        };
        
        let user2 = User {
            username: "charlie".to_string(),
            email: "charlie@example.com".to_string(),
            active: false,
        };
        
        manager.add_user(user1).unwrap();
        manager.add_user(user2).unwrap();
        
        // FIXME: This test depends on the previous test's state!
        let mut new_manager = UserManager::new("users.txt");
        new_manager.load_from_file().unwrap();
        
        // This might fail if previous test ran and left alice in the file!
        assert_eq!(new_manager.get_user("alice"), None);  // FLAKY: Depends on test execution order!
    }
    
    // ✅ CHECKPOINT 4: Make tests independent with proper setup/teardown

    // Exercise 3.3: Fix inadequate test cleanup
    #[test]
    fn test_user_removal_persistence() {
        let test_file = "test_removal.txt";
        let mut manager = UserManager::new(test_file);
        
        let user = User {
            username: "remove_me".to_string(),
            email: "remove@example.com".to_string(),
            active: true,
        };
        
        manager.add_user(user).unwrap();
        manager.remove_user("remove_me").unwrap();
        
        // Test persistence of removal
        let mut new_manager = UserManager::new(test_file);
        new_manager.load_from_file().unwrap();
        
        assert_eq!(new_manager.get_user("remove_me"), None);
        
        // FIXME: Still not cleaning up!
        // Test file will remain on disk
    }
    
    // ✅ CHECKPOINT 5: Implement proper test cleanup pattern

    // Exercise 3.4: Fix missing error case testing
    #[test]
    fn test_happy_path_only() {
        let test_file = "happy_test.txt";
        let mut manager = UserManager::new(test_file);
        
        let user = User {
            username: "happy".to_string(),
            email: "happy@example.com".to_string(),
            active: true,
        };
        
        // Only testing success case
        let result = manager.add_user(user);
        assert!(result.is_ok());
        
        // FIXME: Not testing error cases like:
        // - Adding duplicate users
        // - Loading from non-existent files
        // - Invalid file permissions
        // - Malformed file content
        
        // FIXME: Still no cleanup!
    }
    
    // ✅ CHECKPOINT 6: Add comprehensive error case testing and cleanup

    // Helper function that should be used for cleanup
    fn cleanup_test_file(file_path: &str) {
        if Path::new(file_path).exists() {
            let _ = fs::remove_file(file_path);
        }
    }
}

// COMPILATION CHALLENGES:
// 1. Remove tests that access private fields and methods
// 2. Test only through public API methods
// 3. Use unique file names for each test
// 4. Implement proper test cleanup (setup/teardown)
// 5. Make tests independent and non-interfering
// 6. Add comprehensive error case testing
//
// LEARNING OBJECTIVES:
// - Understanding integration test scope (public API only)
// - Creating isolated integration tests
// - Proper file system testing with cleanup
// - Testing both happy path and error cases
// - Test independence and parallel execution
// - Integration test vs unit test differences
//
// C# COMPARISON:
// C#: [TestClass] with [TestInitialize]/[TestCleanup]
// Rust: Manual setup/teardown in each test
//
// C#: Integration tests in separate projects
// Rust: Integration tests test public API only
//
// C#: InternalsVisibleTo for testing internals
// Rust: Private methods not accessible, test through public API
//
// C#: File/database testing with disposable patterns
// Rust: Manual cleanup or use temporary directories
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Public API only): [ ] Complete
// Checkpoint 2 (Remove private tests): [ ] Complete
// Checkpoint 3 (Unique file names): [ ] Complete
// Checkpoint 4 (Test independence): [ ] Complete
// Checkpoint 5 (Proper cleanup): [ ] Complete
// Checkpoint 6 (Error case testing): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Integration testing scope and boundaries
// ✅ Testing only public APIs
// ✅ Proper test isolation and cleanup
// ✅ File system testing patterns
// ✅ Comprehensive error case testing
// ✅ Test independence and reliability
//
// 🚀 Ready for the next challenge?
// Move on to ex04-mock-troubles.rs to learn mocking and dependency injection!
// Or check your work with: `cargo test --bin ex03-integration-tests`