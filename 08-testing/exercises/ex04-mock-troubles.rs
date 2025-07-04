// Exercise 4: Mock Troubles - Fix the Broken Mocking and Dependency Injection!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken mocking patterns and dependency injection issues
//
// INSTRUCTIONS:
// 1. Fix ONE mocking error at a time
// 2. Run tests: `cargo test --bin ex04-mock-troubles`
// 3. Learn Rust trait-based mocking vs C# interface mocking
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (no trait for mocking)
// - Learn how to make code testable with dependency injection
// - Fix trait design and mock implementation issues
//
// COMPLETED CONCEPTS:
// [] Trait-based dependency injection
// [] Manual mock implementations
// [] Using mockall crate for automatic mocks
// [] Testing with mocked dependencies
// [] Verifying mock interactions
// [] Mock configuration and expectations

// External service that we want to mock
pub struct EmailService {
    api_key: String,
}

impl EmailService {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String> {
        // In real implementation, this would call external API
        if self.api_key.is_empty() {
            return Err("Invalid API key".to_string());
        }
        Ok(format!("Email sent to {} with subject: {}", to, subject))
    }
}

// User notification system that depends on email service
pub struct NotificationService {
    email_service: EmailService,  // FIXME: Hard-coded dependency!
}

impl NotificationService {
    pub fn new(email_service: EmailService) -> Self {
        Self { email_service }
    }

    pub fn send_welcome_email(&self, user_email: &str, username: &str) -> Result<(), String> {
        let subject = "Welcome!";
        let body = format!("Welcome {}, thanks for signing up!", username);
        
        self.email_service.send_email(user_email, subject, &body)
            .map(|_| ())
            .map_err(|e| format!("Failed to send welcome email: {}", e))
    }

    pub fn send_password_reset(&self, user_email: &str) -> Result<(), String> {
        let subject = "Password Reset";
        let body = "Click here to reset your password";
        
        self.email_service.send_email(user_email, subject, body)
            .map(|_| ())
            .map_err(|e| format!("Failed to send password reset: {}", e))
    }
}

// BROKEN MOCKING TESTS TO FIX!

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 4.1: Fix unmockable dependency
    // FIXME: Can't mock EmailService - it's a concrete type!
    #[test]
    fn test_welcome_email_with_mock() {
        // FIXME: How do we mock EmailService?
        let email_service = EmailService::new("test-key".to_string());
        let notification_service = NotificationService::new(email_service);
        
        // We want to test NotificationService without actually sending emails!
        let result = notification_service.send_welcome_email("test@example.com", "testuser");
        
        // FIXME: This will try to "send" an email! We need a mock!
        assert!(result.is_ok());
        // How do we verify the email service was called with correct parameters?
    }
    // In C#, you'd use an interface: Mock<IEmailService>
    
    // ✅ CHECKPOINT 1: Create trait for EmailService to enable mocking

    // Exercise 4.2: Fix manual mock implementation issues
    // Let's try a manual mock (after creating the trait)
    struct MockEmailService {
        should_fail: bool,
        call_count: std::cell::RefCell<usize>,  // FIXME: Interior mutability issues!
    }

    impl MockEmailService {
        fn new(should_fail: bool) -> Self {
            Self {
                should_fail,
                call_count: std::cell::RefCell::new(0),
            }
        }

        fn get_call_count(&self) -> usize {
            *self.call_count.borrow()
        }
    }

    // FIXME: Can't implement trait that doesn't exist yet!
    /*
    impl EmailSender for MockEmailService {
        fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String> {
            *self.call_count.borrow_mut() += 1;
            if self.should_fail {
                Err("Mock email failure".to_string())
            } else {
                Ok(format!("Mock email sent to {}", to))
            }
        }
    }
    */

    #[test]
    fn test_with_manual_mock() {
        // FIXME: This won't compile until we have the trait and proper dependency injection
        /*
        let mock_service = MockEmailService::new(false);
        let notification_service = NotificationService::new(Box::new(mock_service));
        
        let result = notification_service.send_welcome_email("test@example.com", "testuser");
        assert!(result.is_ok());
        assert_eq!(mock_service.get_call_count(), 1);
        */
    }
    
    // ✅ CHECKPOINT 2: Fix trait implementation and dependency injection

    // Exercise 4.3: Fix automatic mock generation issues
    // Using mockall crate (uncomment when trait is ready)
    /*
    use mockall::predicate::*;
    use mockall::mock;

    // FIXME: Need to derive mockall traits properly
    mock! {
        EmailService {}
        
        // FIXME: Wrong trait implementation syntax!
        impl EmailSender for EmailService {
            fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String>;
        }
    }
    */

    #[test]
    fn test_with_mockall() {
        // FIXME: This won't work until mockall is set up correctly
        /*
        let mut mock = MockEmailService::new();
        mock.expect_send_email()
            .with(eq("test@example.com"), eq("Welcome!"), function(|s: &str| s.contains("Welcome")))
            .times(1)
            .returning(|_, _, _| Ok("Mock success".to_string()));

        let notification_service = NotificationService::new(Box::new(mock));
        let result = notification_service.send_welcome_email("test@example.com", "testuser");
        assert!(result.is_ok());
        */
    }
    
    // ✅ CHECKPOINT 3: Set up mockall properly with correct trait syntax

    // Exercise 4.4: Fix mock verification issues
    #[test]
    fn test_password_reset_calls_email_service() {
        // FIXME: Can't verify mock interactions without proper setup
        /*
        let mut mock = MockEmailService::new();
        
        // FIXME: Incorrect expectation syntax!
        mock.expect_send_email()
            .with("user@example.com", "Password Reset", "Click here to reset your password")
            .times(1)
            .returning(|_, _, _| Ok("Sent".to_string()));

        let notification_service = NotificationService::new(Box::new(mock));
        let result = notification_service.send_password_reset("user@example.com");
        
        assert!(result.is_ok());
        // Mock expectations are automatically verified when mock is dropped
        */
    }
    
    // ✅ CHECKPOINT 4: Fix mock expectation syntax and verification

    // Exercise 4.5: Fix error case testing with mocks
    #[test]
    fn test_email_service_failure_handling() {
        // FIXME: Need to test how NotificationService handles email service failures
        /*
        let mut mock = MockEmailService::new();
        mock.expect_send_email()
            .returning(|_, _, _| Err("Email service unavailable".to_string()));

        let notification_service = NotificationService::new(Box::new(mock));
        let result = notification_service.send_welcome_email("test@example.com", "testuser");
        
        // FIXME: What should happen when email service fails?
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to send welcome email"));
        */
    }
    
    // ✅ CHECKPOINT 5: Test error propagation through mocked dependencies

    // Exercise 4.6: Fix complex mock scenarios
    #[test]
    fn test_multiple_email_calls() {
        // FIXME: Testing scenarios with multiple mock calls
        /*
        let mut mock = MockEmailService::new();
        
        // First call succeeds
        mock.expect_send_email()
            .with(eq("user1@example.com"), contains("Welcome"), anything())
            .times(1)
            .returning(|_, _, _| Ok("Sent 1".to_string()));
        
        // Second call fails
        mock.expect_send_email()
            .with(eq("user2@example.com"), contains("Welcome"), anything())
            .times(1)
            .returning(|_, _, _| Err("Service down".to_string()));

        let notification_service = NotificationService::new(Box::new(mock));
        
        let result1 = notification_service.send_welcome_email("user1@example.com", "user1");
        let result2 = notification_service.send_welcome_email("user2@example.com", "user2");
        
        assert!(result1.is_ok());
        assert!(result2.is_err());
        */
    }
    
    // ✅ CHECKPOINT 6: Handle complex mock interaction scenarios
}

// COMPILATION CHALLENGES:
// 1. Create trait abstraction for EmailService
// 2. Modify NotificationService to accept trait objects
// 3. Implement manual mock with proper interior mutability
// 4. Set up mockall crate with correct trait derivation
// 5. Fix mock expectation and verification syntax
// 6. Test error cases and complex mock scenarios
//
// LEARNING OBJECTIVES:
// - Understanding trait-based dependency injection in Rust
// - Creating mockable abstractions for external dependencies
// - Manual mock implementation with RefCell for mutation
// - Using mockall crate for automatic mock generation
// - Mock expectations, verification, and interaction testing
// - Testing error scenarios with mocked failures
//
// C# COMPARISON:
// C#: Mock<IInterface> with Moq
// Rust: mockall with trait objects
//
// C#: Setup().Returns() / Setup().Throws()
// Rust: expect_method().returning() / expect_method().returning(Err(...))
//
// C#: Verify() calls at end of test
// Rust: Automatic verification when mock is dropped
//
// C#: It.IsAny<T>() for flexible matching
// Rust: anything() and predicate functions
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Trait abstraction): [ ] Complete
// Checkpoint 2 (Manual mock): [ ] Complete  
// Checkpoint 3 (Mockall setup): [ ] Complete
// Checkpoint 4 (Mock verification): [ ] Complete
// Checkpoint 5 (Error testing): [ ] Complete
// Checkpoint 6 (Complex scenarios): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Trait-based dependency injection for testability
// ✅ Manual and automatic mock implementations
// ✅ Mock configuration and expectation setting
// ✅ Interaction verification and error scenario testing
// ✅ Complex mock scenarios and edge cases
// ✅ Rust mocking patterns vs C# mocking frameworks
//
// 🚀 Ready for the next challenge?
// Move on to ex05-async-tests.rs to learn async testing patterns!
// Or check your work with: `cargo test --bin ex04-mock-troubles`