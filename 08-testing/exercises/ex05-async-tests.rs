// Exercise 5: Async Testing - Fix the Broken Async Test Patterns!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken async test runtime and execution issues
//
// INSTRUCTIONS:
// 1. Fix ONE async test error at a time
// 2. Run tests: `cargo test --bin ex05-async-tests`
// 3. Learn async testing patterns vs C# async test patterns
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (missing async test attribute)
// - Learn proper async test runtime setup
// - Fix blocking operations in async tests
//
// COMPLETED CONCEPTS:
// [] Async test attributes (tokio::test, async_std::test)
// [] Async runtime configuration for tests
// [] Avoiding blocking operations in async tests
// [] Testing async error cases and timeouts
// [] Mock async dependencies
// [] Async test isolation and cleanup

use std::time::Duration;
use std::collections::HashMap;

// Async HTTP client simulator
pub struct HttpClient {
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get(&self, endpoint: &str) -> Result<String, String> {
        // Simulate HTTP request delay
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        if endpoint.starts_with("/error") {
            Err("HTTP 500: Internal Server Error".to_string())
        } else {
            Ok(format!("{{\"data\": \"response from {}{}\"}}", self.base_url, endpoint))
        }
    }

    pub async fn post(&self, endpoint: &str, data: &str) -> Result<String, String> {
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        if data.is_empty() {
            Err("HTTP 400: Bad Request - Empty body".to_string())
        } else {
            Ok(format!("{{\"id\": 123, \"status\": \"created\"}}", ))
        }
    }
}

// Async service that uses the HTTP client
pub struct ApiService {
    client: HttpClient,
    cache: HashMap<String, String>,
}

impl ApiService {
    pub fn new(client: HttpClient) -> Self {
        Self {
            client,
            cache: HashMap::new(),
        }
    }

    pub async fn get_user(&mut self, user_id: u32) -> Result<String, String> {
        let cache_key = format!("user_{}", user_id);
        
        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Fetch from API
        let endpoint = format!("/users/{}", user_id);
        let response = self.client.get(&endpoint).await?;
        
        // Cache the response
        self.cache.insert(cache_key, response.clone());
        Ok(response)
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<String, String> {
        let data = format!("{{\"name\": \"{}\", \"email\": \"{}\"}}", name, email);
        self.client.post("/users", &data).await
    }

    pub async fn process_batch(&self, user_ids: Vec<u32>) -> Vec<Result<String, String>> {
        let mut results = Vec::new();
        
        for user_id in user_ids {
            // INTENTIONAL PERFORMANCE ISSUE: Sequential processing
            let endpoint = format!("/users/{}", user_id);
            let result = self.client.get(&endpoint).await;
            results.push(result);
        }
        
        results
    }
}

// BROKEN ASYNC TESTS TO FIX!

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 5.1: Fix missing async test attribute
    // FIXME: This async test won't run properly!
    #[test]  // WRONG: Should be #[tokio::test] or #[async_std::test]
    async fn test_get_user_success() {
        let client = HttpClient::new("https://api.example.com".to_string());
        let mut service = ApiService::new(client);
        
        let result = service.get_user(123).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("response from"));
    }
    // In C#, async tests just work with async Task return type
    
    // ✅ CHECKPOINT 1: Add proper async test attribute

    // Exercise 5.2: Fix blocking operations in async test
    #[tokio::test]
    async fn test_with_blocking_operations() {
        let client = HttpClient::new("https://api.example.com".to_string());
        let mut service = ApiService::new(client);
        
        // FIXME: Blocking operation in async context!
        std::thread::sleep(Duration::from_millis(100));  // BAD: Blocks the runtime!
        
        let result = service.get_user(456).await;
        assert!(result.is_ok());
        
        // FIXME: More blocking operations!
        std::thread::sleep(Duration::from_millis(50));  // BAD: More blocking!
    }
    
    // ✅ CHECKPOINT 2: Replace blocking operations with async equivalents

    // Exercise 5.3: Fix improper async error handling
    #[tokio::test]
    async fn test_error_handling() {
        let client = HttpClient::new("https://api.example.com".to_string());
        let mut service = ApiService::new(client);
        
        // FIXME: Not properly testing async error cases!
        let result = service.get_user(999).await;
        
        // This might not be testing what we think it is
        assert!(result.is_ok());  // LOGIC ERROR: Should we expect an error?
        
        // FIXME: How do we test timeout scenarios?
        // FIXME: How do we test network errors?
    }
    
    // ✅ CHECKPOINT 3: Properly test async error cases and edge conditions

    // Exercise 5.4: Fix timeout and cancellation testing
    #[tokio::test]
    async fn test_timeout_scenarios() {
        let client = HttpClient::new("https://slow-api.example.com".to_string());
        let mut service = ApiService::new(client);
        
        // FIXME: No timeout testing!
        let result = service.get_user(123).await;
        assert!(result.is_ok());
        
        // How do we test that operations timeout appropriately?
        // How do we simulate slow network conditions?
        // How do we test cancellation?
    }
    
    // ✅ CHECKPOINT 4: Add timeout and cancellation testing

    // Exercise 5.5: Fix async test isolation issues
    static mut GLOBAL_COUNTER: u32 = 0;

    #[tokio::test]
    async fn test_shared_state_issue_1() {
        unsafe {
            GLOBAL_COUNTER += 1;  // BAD: Shared mutable state!
        }
        
        let client = HttpClient::new("https://api.example.com".to_string());
        let mut service = ApiService::new(client);
        
        let result = service.get_user(100).await;
        assert!(result.is_ok());
        
        unsafe {
            // FIXME: This will be flaky in parallel test execution!
            assert_eq!(GLOBAL_COUNTER, 1);  // FLAKY: Depends on test execution order!
        }
    }

    #[tokio::test]
    async fn test_shared_state_issue_2() {
        unsafe {
            GLOBAL_COUNTER += 1;  // BAD: More shared state modification!
        }
        
        let client = HttpClient::new("https://api.example.com".to_string());
        let mut service = ApiService::new(client);
        
        let result = service.get_user(200).await;
        assert!(result.is_ok());
        
        unsafe {
            // FIXME: This will fail when run with other tests!
            assert_eq!(GLOBAL_COUNTER, 2);  // FLAKY: Assumes specific execution order!
        }
    }
    
    // ✅ CHECKPOINT 5: Fix async test isolation and remove shared state

    // Exercise 5.6: Fix concurrent async test issues
    #[tokio::test]
    async fn test_concurrent_operations() {
        let client = HttpClient::new("https://api.example.com".to_string());
        let service = ApiService::new(client);
        
        // FIXME: Not actually testing concurrency properly!
        let user_ids = vec![1, 2, 3, 4, 5];
        let results = service.process_batch(user_ids).await;
        
        assert_eq!(results.len(), 5);
        for result in results {
            assert!(result.is_ok());
        }
        
        // FIXME: This is testing sequential processing, not concurrent!
        // How do we test actual concurrent operations?
        // How do we test race conditions?
        // How do we test concurrent access to shared resources?
    }
    
    // ✅ CHECKPOINT 6: Implement proper concurrent async testing

    // Helper function for async test setup (will be used after fixes)
    async fn setup_test_service() -> ApiService {
        let client = HttpClient::new("https://test-api.example.com".to_string());
        ApiService::new(client)
    }

    // Helper for async cleanup (will be used after fixes)
    async fn cleanup_test_data() {
        // Simulate cleanup operations
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

// COMPILATION CHALLENGES:
// 1. Add proper async test attributes (#[tokio::test])
// 2. Replace std::thread::sleep with tokio::time::sleep
// 3. Add proper async error case testing
// 4. Implement timeout testing with tokio::time::timeout
// 5. Remove shared state and fix test isolation
// 6. Test actual concurrent operations with join! or spawn
//
// LEARNING OBJECTIVES:
// - Understanding async test runtime requirements
// - Avoiding blocking operations in async tests
// - Testing async error cases and timeouts
// - Proper async test isolation without shared state
// - Testing concurrent async operations
// - Async test patterns vs C# async test patterns
//
// C# COMPARISON:
// C#: async Task test methods work automatically
// Rust: Need explicit #[tokio::test] or #[async_std::test]
//
// C#: await Task.Delay() for async delays
// Rust: tokio::time::sleep().await for async delays
//
// C#: Task.WhenAll() for concurrent operations
// Rust: tokio::join!() or futures::join!() for concurrency
//
// C#: CancellationToken for timeout testing
// Rust: tokio::time::timeout() for timeout testing
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Async test attribute): [ ] Complete
// Checkpoint 2 (Remove blocking): [ ] Complete
// Checkpoint 3 (Error testing): [ ] Complete
// Checkpoint 4 (Timeout testing): [ ] Complete
// Checkpoint 5 (Test isolation): [ ] Complete
// Checkpoint 6 (Concurrent testing): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Async test runtime setup and configuration
// ✅ Non-blocking async test patterns
// ✅ Async error case and timeout testing
// ✅ Proper async test isolation
// ✅ Concurrent async operation testing
// ✅ Rust async testing vs C# async testing patterns
//
// 🚀 Ready for the next challenge?
// Move on to ex06-property-tests.rs to learn property-based testing!
// Or check your work with: `cargo test --bin ex05-async-tests`