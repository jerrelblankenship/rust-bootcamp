# Exercise 07 - Test Fixtures: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Eliminating Duplicate Setup
```rust
// Instead of repeating in every test:
#[test]
fn test_user_validation() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 25,
        verified: true,
    };
    // test...
}

// Create a fixture function:
fn create_test_user() -> User {
    User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 25,
        verified: true,
    }
}

#[test]
fn test_user_validation() {
    let user = create_test_user();
    // test...
}
```

### Checkpoint 2: Test Data Builders
```rust
// Builder pattern for test data
struct UserBuilder {
    id: u64,
    name: String,
    email: String,
    age: u8,
    verified: bool,
}

impl UserBuilder {
    fn new() -> Self {
        // Sensible defaults
        Self {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
            verified: true,
        }
    }
    
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    
    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }
    
    fn unverified(mut self) -> Self {
        self.verified = false;
        self
    }
    
    fn build(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            age: self.age,
            verified: self.verified,
        }
    }
}

// Usage:
let young_user = UserBuilder::new()
    .with_age(17)
    .build();
```

### Checkpoint 3: Object Mother Pattern
```rust
// Object Mother provides pre-configured objects
mod fixtures {
    use super::*;
    
    pub struct UserFixtures;
    
    impl UserFixtures {
        pub fn admin() -> User {
            UserBuilder::new()
                .with_name("Admin")
                .with_email("admin@example.com")
                .build()
        }
        
        pub fn regular_user() -> User {
            UserBuilder::new()
                .with_name("John Doe")
                .build()
        }
        
        pub fn unverified_user() -> User {
            UserBuilder::new()
                .unverified()
                .build()
        }
        
        pub fn minor() -> User {
            UserBuilder::new()
                .with_age(16)
                .build()
        }
    }
}

// Usage:
#[test]
fn test_admin_permissions() {
    let admin = fixtures::UserFixtures::admin();
    assert!(has_admin_rights(&admin));
}
```

### Checkpoint 4: Test Cleanup with Drop
```rust
// Automatic cleanup using Drop
struct TestDatabase {
    name: String,
    connection: Connection,
}

impl TestDatabase {
    fn new() -> Self {
        let name = format!("test_db_{}", uuid::Uuid::new_v4());
        let connection = create_database(&name);
        Self { name, connection }
    }
    
    fn connection(&self) -> &Connection {
        &self.connection
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Automatic cleanup when test ends
        drop_database(&self.name);
    }
}

#[test]
fn test_with_database() {
    let test_db = TestDatabase::new();
    // Use test_db.connection()
    // Cleanup happens automatically!
}
```

### Checkpoint 5: Composable Fixtures
```rust
// Fixtures that build on each other
struct OrderBuilder {
    customer: Option<User>,
    items: Vec<OrderItem>,
    discount: Option<f64>,
}

impl OrderBuilder {
    fn new() -> Self {
        Self {
            customer: None,
            items: vec![],
            discount: None,
        }
    }
    
    fn for_customer(mut self, customer: User) -> Self {
        self.customer = Some(customer);
        self
    }
    
    fn with_item(mut self, name: &str, price: f64, quantity: u32) -> Self {
        self.items.push(OrderItem {
            name: name.to_string(),
            price,
            quantity,
        });
        self
    }
    
    fn with_discount(mut self, discount: f64) -> Self {
        self.discount = Some(discount);
        self
    }
    
    fn build(self) -> Order {
        Order {
            customer: self.customer.unwrap_or_else(|| UserBuilder::new().build()),
            items: self.items,
            discount: self.discount,
        }
    }
}

// Compose fixtures:
let customer = UserBuilder::new()
    .with_name("Premium Customer")
    .build();

let order = OrderBuilder::new()
    .for_customer(customer)
    .with_item("Widget", 9.99, 2)
    .with_item("Gadget", 19.99, 1)
    .with_discount(10.0)
    .build();
```

### Checkpoint 6: Parameterized Fixtures
```rust
// Factory functions with parameters
mod fixtures {
    pub fn create_users(count: usize) -> Vec<User> {
        (0..count)
            .map(|i| UserBuilder::new()
                .with_name(&format!("User {}", i))
                .with_email(&format!("user{}@example.com", i))
                .build())
            .collect()
    }
    
    pub fn create_order_with_items(item_count: usize) -> Order {
        let mut builder = OrderBuilder::new();
        
        for i in 0..item_count {
            builder = builder.with_item(
                &format!("Item {}", i),
                10.0 * (i as f64 + 1.0),
                1
            );
        }
        
        builder.build()
    }
    
    pub fn create_aged_users(ages: &[u8]) -> Vec<User> {
        ages.iter()
            .enumerate()
            .map(|(i, &age)| UserBuilder::new()
                .with_name(&format!("User {}", i))
                .with_age(age)
                .build())
            .collect()
    }
}

// Usage:
#[test]
fn test_bulk_processing() {
    let users = fixtures::create_users(100);
    let results = process_users(&users);
    assert_eq!(results.len(), 100);
}
```

## 🎯 Pattern Recognition

Key insights:
- Builders provide flexible test data construction
- Object Mothers offer pre-configured scenarios
- Drop trait enables automatic cleanup
- Composable fixtures reduce duplication
- Parameterized factories create variations

Ready for Level 3 if you need complete solutions!