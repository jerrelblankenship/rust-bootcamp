// Exercise 7: Test Fixtures and Data Builders - Fix the Broken Test Data Patterns!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken test fixture and data builder patterns
//
// INSTRUCTIONS:
// 1. Fix ONE test data pattern error at a time
// 2. Run tests: `cargo test --bin ex07-test-fixtures`
// 3. Learn test data builders vs hard-coded test data
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (hard-coded test data duplication)
// - Learn builder patterns for flexible test data creation
// - Fix test data setup and maintenance issues
//
// COMPLETED CONCEPTS:
// [] Test data builder patterns
// [] Fixture setup and teardown
// [] Test data factories and generators
// [] Default values with customization
// [] Complex object graph creation
// [] Test data maintenance and reusability

use std::collections::HashMap;

// Domain models for testing
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub roles: Vec<String>,
    pub profile: UserProfile,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserProfile {
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub settings: UserSettings,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserSettings {
    pub theme: String,
    pub notifications_enabled: bool,
    pub language: String,
    pub timezone: String,
}

// Business logic to test
pub struct UserService {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_user(&mut self, user: User) -> Result<u32, String> {
        if self.users.values().any(|u| u.username == user.username) {
            return Err("Username already exists".to_string());
        }

        if self.users.values().any(|u| u.email == user.email) {
            return Err("Email already exists".to_string());
        }

        let id = self.next_id;
        self.next_id += 1;
        
        let mut user_with_id = user;
        user_with_id.id = id;
        
        self.users.insert(id, user_with_id);
        Ok(id)
    }

    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn update_user(&mut self, id: u32, user: User) -> Result<(), String> {
        if !self.users.contains_key(&id) {
            return Err("User not found".to_string());
        }

        let mut updated_user = user;
        updated_user.id = id;
        self.users.insert(id, updated_user);
        Ok(())
    }

    pub fn get_active_users(&self) -> Vec<&User> {
        self.users.values().filter(|u| u.active).collect()
    }

    pub fn get_users_by_role(&self, role: &str) -> Vec<&User> {
        self.users.values()
            .filter(|u| u.roles.contains(&role.to_string()))
            .collect()
    }
}

// BROKEN TEST DATA PATTERNS TO FIX!

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 7.1: Fix hard-coded test data duplication
    #[test]
    fn test_create_user_success() {
        let mut service = UserService::new();
        
        // FIXME: Hard-coded test data duplicated across tests!
        let user = User {
            id: 0, // Will be set by service
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
            active: true,
            roles: vec!["user".to_string()],
            profile: UserProfile {
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                bio: "A test user".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };
        
        let result = service.create_user(user);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_user_duplicate_username() {
        let mut service = UserService::new();
        
        // FIXME: Same hard-coded data copied and pasted!
        let user1 = User {
            id: 0,
            username: "testuser".to_string(),
            email: "test1@example.com".to_string(),
            age: 25,
            active: true,
            roles: vec!["user".to_string()],
            profile: UserProfile {
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                bio: "A test user".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };

        let user2 = User {
            id: 0,
            username: "testuser".to_string(),  // Same username
            email: "test2@example.com".to_string(),  // Different email
            age: 30,  // Different age
            active: true,
            roles: vec!["admin".to_string()],  // Different role
            profile: UserProfile {
                first_name: "Another".to_string(),
                last_name: "User".to_string(),
                bio: "Another test user".to_string(),
                avatar_url: Some("http://example.com/avatar.jpg".to_string()),
                settings: UserSettings {
                    theme: "light".to_string(),
                    notifications_enabled: false,
                    language: "es".to_string(),
                    timezone: "PST".to_string(),
                },
            },
        };
        
        service.create_user(user1).unwrap();
        let result = service.create_user(user2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Username already exists"));
    }
    
    // ✅ CHECKPOINT 1: Create a test data builder to eliminate duplication

    // Exercise 7.2: Fix inflexible test data creation
    #[test]
    fn test_create_admin_user() {
        let mut service = UserService::new();
        
        // FIXME: Hard to create variations of test data!
        // What if I want a user with admin role but different email?
        // What if I want to test with different ages?
        // What if I want to test with different settings?
        
        // Currently, I have to copy-paste and modify the entire struct!
        let admin_user = User {
            id: 0,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            age: 35,
            active: true,
            roles: vec!["admin".to_string(), "user".to_string()],  // Admin has multiple roles
            profile: UserProfile {
                first_name: "Admin".to_string(),
                last_name: "User".to_string(),
                bio: "An admin user".to_string(),
                avatar_url: Some("http://example.com/admin.jpg".to_string()),
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };
        
        let result = service.create_user(admin_user);
        assert!(result.is_ok());
        
        let user_id = result.unwrap();
        let user = service.get_user(user_id).unwrap();
        assert!(user.roles.contains(&"admin".to_string()));
    }
    
    // ✅ CHECKPOINT 2: Create flexible builder methods for different user types

    // Exercise 7.3: Fix complex object graph creation
    #[test]
    fn test_user_with_complex_profile() {
        let mut service = UserService::new();
        
        // FIXME: Creating complex nested objects is verbose and error-prone!
        let complex_user = User {
            id: 0,
            username: "poweruser".to_string(),
            email: "power@example.com".to_string(),
            age: 28,
            active: true,
            roles: vec!["user".to_string(), "moderator".to_string(), "beta_tester".to_string()],
            profile: UserProfile {
                first_name: "Power".to_string(),
                last_name: "User".to_string(),
                bio: "A very long bio that describes this user in great detail, including their background, interests, and what makes them special in our application.".to_string(),
                avatar_url: Some("https://secure.example.com/avatars/poweruser_512x512.png".to_string()),
                settings: UserSettings {
                    theme: "custom_dark_blue".to_string(),
                    notifications_enabled: false,  // Power users don't want notifications
                    language: "en_US".to_string(),
                    timezone: "America/New_York".to_string(),
                },
            },
        };
        
        let result = service.create_user(complex_user);
        assert!(result.is_ok());
        
        // How do we easily create variations of this complex object?
        // How do we test edge cases with different nested values?
    }
    
    // ✅ CHECKPOINT 3: Create builders for complex nested object graphs

    // Exercise 7.4: Fix test data setup and teardown
    #[test]
    fn test_get_active_users() {
        let mut service = UserService::new();
        
        // FIXME: Need to set up multiple users for this test!
        // This is getting very verbose...
        
        let active_user1 = User {
            id: 0,
            username: "active1".to_string(),
            email: "active1@example.com".to_string(),
            age: 25,
            active: true,  // Active
            roles: vec!["user".to_string()],
            profile: UserProfile {
                first_name: "Active".to_string(),
                last_name: "One".to_string(),
                bio: "First active user".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };

        let inactive_user = User {
            id: 0,
            username: "inactive".to_string(),
            email: "inactive@example.com".to_string(),
            age: 30,
            active: false,  // Inactive
            roles: vec!["user".to_string()],
            profile: UserProfile {
                first_name: "Inactive".to_string(),
                last_name: "User".to_string(),
                bio: "An inactive user".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "light".to_string(),
                    notifications_enabled: false,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };

        let active_user2 = User {
            id: 0,
            username: "active2".to_string(),
            email: "active2@example.com".to_string(),
            age: 35,
            active: true,  // Active
            roles: vec!["user".to_string()],
            profile: UserProfile {
                first_name: "Active".to_string(),
                last_name: "Two".to_string(),
                bio: "Second active user".to_string(),
                avatar_url: Some("http://example.com/active2.jpg".to_string()),
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };
        
        service.create_user(active_user1).unwrap();
        service.create_user(inactive_user).unwrap();
        service.create_user(active_user2).unwrap();
        
        let active_users = service.get_active_users();
        assert_eq!(active_users.len(), 2);
        
        // This test setup is getting unwieldy!
        // How do we create multiple test users efficiently?
    }
    
    // ✅ CHECKPOINT 4: Create test fixtures for multiple object setup

    // Exercise 7.5: Fix test data maintenance issues
    #[test]
    fn test_get_users_by_role() {
        let mut service = UserService::new();
        
        // FIXME: If the User struct changes, ALL these tests break!
        // This is maintenance nightmare!
        
        // What if we add a new required field to UserSettings?
        // What if UserProfile gets new mandatory fields?
        // Every test will need updates!
        
        let user_with_admin_role = User {
            id: 0,
            username: "admin_user".to_string(),
            email: "admin_user@example.com".to_string(),
            age: 40,
            active: true,
            roles: vec!["admin".to_string()],
            profile: UserProfile {
                first_name: "Admin".to_string(),
                last_name: "User".to_string(),
                bio: "An admin".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };

        let user_with_moderator_role = User {
            id: 0,
            username: "mod_user".to_string(),
            email: "mod_user@example.com".to_string(),
            age: 28,
            active: true,
            roles: vec!["moderator".to_string()],
            profile: UserProfile {
                first_name: "Mod".to_string(),
                last_name: "User".to_string(),
                bio: "A moderator".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "light".to_string(),
                    notifications_enabled: false,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        };
        
        service.create_user(user_with_admin_role).unwrap();
        service.create_user(user_with_moderator_role).unwrap();
        
        let admin_users = service.get_users_by_role("admin");
        assert_eq!(admin_users.len(), 1);
        
        let mod_users = service.get_users_by_role("moderator");
        assert_eq!(mod_users.len(), 1);
    }
    
    // ✅ CHECKPOINT 5: Create maintainable test data with sensible defaults

    // Exercise 7.6: Fix test data reusability across test modules
    // FIXME: These test data creation functions are only available in this module!
    // What if other test modules need similar test users?
    // How do we share test data builders across multiple test files?
    
    fn create_basic_user() -> User {
        // This helper is not reusable outside this module!
        User {
            id: 0,
            username: "basic".to_string(),
            email: "basic@example.com".to_string(),
            age: 25,
            active: true,
            roles: vec!["user".to_string()],
            profile: UserProfile {
                first_name: "Basic".to_string(),
                last_name: "User".to_string(),
                bio: "A basic user".to_string(),
                avatar_url: None,
                settings: UserSettings {
                    theme: "dark".to_string(),
                    notifications_enabled: true,
                    language: "en".to_string(),
                    timezone: "UTC".to_string(),
                },
            },
        }
    }
    
    #[test]
    fn test_with_basic_user() {
        let mut service = UserService::new();
        let user = create_basic_user();
        
        let result = service.create_user(user);
        assert!(result.is_ok());
    }
    
    // ✅ CHECKPOINT 6: Create reusable test builders across modules
}

// COMPILATION CHALLENGES:
// 1. Create a UserBuilder struct with builder pattern methods
// 2. Implement default values with customization options
// 3. Add specialized builder methods for different user types
// 4. Create fixture setup functions for multiple objects
// 5. Implement maintainable defaults that adapt to struct changes
// 6. Make test builders reusable across test modules
//
// LEARNING OBJECTIVES:
// - Understanding test data builder patterns vs hard-coded data
// - Creating flexible and maintainable test fixtures
// - Building complex object graphs efficiently
// - Test data setup and teardown patterns
// - Reusable test builders across multiple test files
// - Comparing with C# test data patterns and builders
//
// C# COMPARISON:
// C#: Object initializers and anonymous objects
// Rust: Builder pattern with method chaining
//
// C#: Factory methods and test data builders
// Rust: impl blocks with fluent interfaces
//
// C#: [SetUp] and [TearDown] attributes
// Rust: Manual setup/teardown functions
//
// C#: Test data shared via base classes
// Rust: Test utilities in separate modules
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Eliminate duplication): [ ] Complete
// Checkpoint 2 (Flexible builders): [ ] Complete
// Checkpoint 3 (Complex object graphs): [ ] Complete
// Checkpoint 4 (Test fixtures): [ ] Complete
// Checkpoint 5 (Maintainable defaults): [ ] Complete
// Checkpoint 6 (Reusable builders): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Test data builder patterns and implementations
// ✅ Flexible and maintainable test fixture creation
// ✅ Complex object graph building strategies
// ✅ Test setup and teardown patterns
// ✅ Reusable test utilities across modules
// ✅ Rust test data patterns vs C# approaches
//
// 🚀 Ready for the next challenge?
// Move on to ex08-coverage-gaps.rs to learn test coverage analysis!
// Or check your work with: `cargo test --bin ex07-test-fixtures`