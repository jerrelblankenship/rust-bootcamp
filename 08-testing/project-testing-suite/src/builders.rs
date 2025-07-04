// Test Data Builders Module - Fix the Broken Builder Patterns!
//
// This module provides builder patterns for creating complex test data
// Currently BROKEN - incomplete implementations and missing patterns

use std::collections::HashMap;

// FIXME: This User struct is missing fields!
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub age: u32,
    // FIXME: Add more fields like id, created_at, preferences, etc.
}

// FIXME: This builder is incomplete!
pub struct UserBuilder {
    username: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    // FIXME: Missing builder fields for new User properties
}

impl UserBuilder {
    // FIXME: Constructor doesn't set any defaults!
    pub fn new() -> Self {
        Self {
            username: None,
            email: None, 
            age: None,
        }
    }

    // FIXME: These builder methods don't validate input!
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    // FIXME: Build method has no validation and poor defaults!
    pub fn build(self) -> User {
        User {
            username: self.username.unwrap_or_else(|| "unknown".to_string()),
            email: self.email.unwrap_or_else(|| "default@example.com".to_string()),
            age: self.age.unwrap_or(18),
        }
    }
}

// FIXME: This Order struct is missing critical fields!
#[derive(Debug, Clone)]
pub struct Order {
    pub customer: User,
    pub items: Vec<OrderItem>,
    // FIXME: Missing order_id, total_price, status, created_at, etc.
}

// FIXME: OrderItem is too simple!
#[derive(Debug, Clone)]
pub struct OrderItem {
    pub name: String,
    pub quantity: u32,
    pub unit_price: f64,
    // FIXME: Missing item_id, category, weight, etc.
}

// FIXME: OrderBuilder is incomplete!
pub struct OrderBuilder {
    customer: Option<User>,
    items: Vec<OrderItem>,
    // FIXME: Missing builder fields for new Order properties
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self {
            customer: None,
            items: Vec::new(),
        }
    }

    pub fn customer(mut self, customer: User) -> Self {
        self.customer = Some(customer);
        self
    }

    // FIXME: This method doesn't validate prices or quantities!
    pub fn add_item(mut self, name: &str, quantity: u32, unit_price: f64) -> Self {
        self.items.push(OrderItem {
            name: name.to_string(),
            quantity,
            unit_price,
        });
        self
    }

    // FIXME: Build has no validation!
    pub fn build(self) -> Order {
        Order {
            customer: self.customer.unwrap_or_else(|| UserBuilder::new().build()),
            items: self.items,
        }
    }
}

// FIXME: These builder helpers don't exist!
/*
pub struct TestDataSet {
    // Collection of related test objects
}

impl TestDataSet {
    pub fn new() -> Self {
        // TODO: Initialize test data collection
    }
    
    pub fn add_users(&mut self, count: usize) -> &mut Self {
        // TODO: Add multiple users with variations
    }
    
    pub fn add_orders(&mut self, count: usize) -> &mut Self {
        // TODO: Add multiple orders with variations
    }
}
*/

// FIXME: Random data generators are missing!
/*
pub mod random {
    use super::*;
    
    pub fn random_user() -> User {
        // TODO: Generate user with random but valid data
    }
    
    pub fn random_email() -> String {
        // TODO: Generate valid random email
    }
    
    pub fn random_username() -> String {
        // TODO: Generate valid random username
    }
}
*/

// FIXME: Template-based builders don't exist!
/*
pub mod templates {
    use super::*;
    
    pub fn admin_user() -> UserBuilder {
        // TODO: Return pre-configured admin user builder
    }
    
    pub fn guest_user() -> UserBuilder {
        // TODO: Return pre-configured guest user builder  
    }
    
    pub fn large_order() -> OrderBuilder {
        // TODO: Return pre-configured large order builder
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: Tests are incomplete and don't cover edge cases!
    #[test]
    fn test_user_builder_basic() {
        let user = UserBuilder::new()
            .username("testuser")
            .email("test@example.com")
            .age(25)
            .build();

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_user_builder_defaults() {
        let user = UserBuilder::new().build();
        
        // FIXME: Are these good defaults for testing?
        assert_eq!(user.username, "unknown");
        assert_eq!(user.email, "default@example.com");
        assert_eq!(user.age, 18);
    }

    // FIXME: Missing tests for:
    // - Order builder validation
    // - Invalid input handling
    // - Complex object relationships
    // - Builder method chaining
    // - Default value behavior
    // - Template builders
    // - Random data generators
}