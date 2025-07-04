# Exercise 07 - Test Fixtures: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete DRY Test Setup
```rust
// Test fixture module
mod fixtures {
    use super::*;
    use once_cell::sync::Lazy;
    
    // Shared test configuration
    static TEST_CONFIG: Lazy<Config> = Lazy::new(|| {
        Config {
            database_url: "postgres://localhost/test_db".to_string(),
            api_key: "test_api_key_12345".to_string(),
            timeout_ms: 5000,
            retry_count: 3,
        }
    });
    
    pub fn test_config() -> &'static Config {
        &TEST_CONFIG
    }
    
    // Fixture creation functions
    pub fn create_test_user() -> User {
        User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: hash_password("password123"),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_active: true,
            role: UserRole::Regular,
        }
    }
    
    pub fn create_test_users(count: usize) -> Vec<User> {
        (1..=count)
            .map(|i| User {
                id: i as u64,
                username: format!("user{}", i),
                email: format!("user{}@example.com", i),
                ..create_test_user()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures;
    
    #[test]
    fn test_user_authentication() {
        let user = fixtures::create_test_user();
        let service = AuthService::new(fixtures::test_config());
        
        let result = service.authenticate(&user.email, "password123");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_bulk_user_import() {
        let users = fixtures::create_test_users(50);
        let service = UserService::new(fixtures::test_config());
        
        let imported = service.bulk_import(users).unwrap();
        assert_eq!(imported.len(), 50);
    }
}
```

### Checkpoint 2: Complete Builder Pattern
```rust
#[derive(Debug, Clone)]
pub struct UserBuilder {
    id: Option<u64>,
    username: String,
    email: String,
    password: String,
    age: u8,
    country: String,
    is_verified: bool,
    is_premium: bool,
    created_at: DateTime<Utc>,
    preferences: UserPreferences,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: None,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            age: 25,
            country: "US".to_string(),
            is_verified: true,
            is_premium: false,
            created_at: Utc::now(),
            preferences: UserPreferences::default(),
        }
    }
}

impl UserBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
    
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }
    
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }
    
    pub fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }
    
    pub fn from_country(mut self, country: impl Into<String>) -> Self {
        self.country = country.into();
        self
    }
    
    pub fn unverified(mut self) -> Self {
        self.is_verified = false;
        self
    }
    
    pub fn premium(mut self) -> Self {
        self.is_premium = true;
        self
    }
    
    pub fn created_days_ago(mut self, days: i64) -> Self {
        self.created_at = Utc::now() - Duration::days(days);
        self
    }
    
    pub fn with_preferences(mut self, f: impl FnOnce(&mut UserPreferences)) -> Self {
        f(&mut self.preferences);
        self
    }
    
    pub fn build(self) -> User {
        User {
            id: self.id.unwrap_or_else(|| rand::random()),
            username: self.username,
            email: self.email,
            password_hash: hash_password(&self.password),
            age: self.age,
            country: self.country,
            is_verified: self.is_verified,
            is_premium: self.is_premium,
            created_at: self.created_at,
            updated_at: self.created_at,
            preferences: self.preferences,
        }
    }
}

// Usage examples:
#[test]
fn test_premium_user_features() {
    let premium_user = UserBuilder::new()
        .with_username("premium_customer")
        .premium()
        .with_preferences(|prefs| {
            prefs.theme = "dark".to_string();
            prefs.notifications_enabled = true;
        })
        .build();
    
    assert!(has_premium_features(&premium_user));
}

#[test]
fn test_new_user_verification() {
    let new_user = UserBuilder::new()
        .unverified()
        .created_days_ago(0)
        .build();
    
    assert!(needs_verification(&new_user));
}
```

### Checkpoint 3: Complete Object Mother
```rust
pub mod test_objects {
    use super::*;
    
    pub struct Users;
    
    impl Users {
        pub fn alice() -> User {
            UserBuilder::new()
                .with_id(1)
                .with_username("alice")
                .with_email("alice@example.com")
                .with_age(28)
                .from_country("UK")
                .premium()
                .build()
        }
        
        pub fn bob() -> User {
            UserBuilder::new()
                .with_id(2)
                .with_username("bob")
                .with_email("bob@example.com")
                .with_age(35)
                .from_country("US")
                .build()
        }
        
        pub fn unverified_charlie() -> User {
            UserBuilder::new()
                .with_id(3)
                .with_username("charlie")
                .with_email("charlie@example.com")
                .unverified()
                .created_days_ago(1)
                .build()
        }
        
        pub fn suspended_dave() -> User {
            let mut user = UserBuilder::new()
                .with_id(4)
                .with_username("dave")
                .build();
            user.suspend("Violation of terms");
            user
        }
    }
    
    pub struct Orders;
    
    impl Orders {
        pub fn small_order() -> Order {
            OrderBuilder::new()
                .for_customer(Users::alice())
                .with_item("Book", 15.99, 1)
                .build()
        }
        
        pub fn large_order() -> Order {
            OrderBuilder::new()
                .for_customer(Users::bob())
                .with_item("Laptop", 999.99, 1)
                .with_item("Mouse", 29.99, 2)
                .with_item("Keyboard", 79.99, 1)
                .with_discount(10.0)
                .build()
        }
        
        pub fn cancelled_order() -> Order {
            let mut order = Orders::small_order();
            order.cancel("Customer request");
            order
        }
    }
    
    pub struct Scenarios;
    
    impl Scenarios {
        pub fn user_makes_first_purchase() -> (User, Order) {
            let user = Users::unverified_charlie();
            let order = OrderBuilder::new()
                .for_customer(user.clone())
                .with_item("Starter Pack", 9.99, 1)
                .build();
            (user, order)
        }
        
        pub fn premium_user_bulk_order() -> (User, Vec<Order>) {
            let user = Users::alice();
            let orders = vec![
                Orders::small_order(),
                Orders::large_order(),
            ];
            (user, orders)
        }
    }
}

// Usage:
#[test]
fn test_order_cancellation() {
    let order = test_objects::Orders::cancelled_order();
    assert_eq!(order.status, OrderStatus::Cancelled);
}

#[test]
fn test_first_purchase_flow() {
    let (user, order) = test_objects::Scenarios::user_makes_first_purchase();
    let result = process_first_purchase(&user, &order);
    assert!(result.includes_welcome_bonus);
}
```

### Checkpoint 4: Complete Test Cleanup
```rust
use std::sync::Arc;
use tempfile::TempDir;

// Automatic cleanup with RAII
pub struct TestEnvironment {
    pub config: Config,
    pub database: TestDatabase,
    pub temp_dir: TempDir,
    pub mock_server: MockServer,
}

impl TestEnvironment {
    pub async fn new() -> Self {
        // Create temporary directory
        let temp_dir = TempDir::new().unwrap();
        
        // Setup test database
        let database = TestDatabase::new().await;
        
        // Start mock server
        let mock_server = MockServer::start().await;
        
        // Create config pointing to test resources
        let config = Config {
            database_url: database.url(),
            storage_path: temp_dir.path().to_str().unwrap().to_string(),
            api_base_url: mock_server.uri(),
            ..Default::default()
        };
        
        Self {
            config,
            database,
            temp_dir,
            mock_server,
        }
    }
    
    pub fn create_test_file(&self, name: &str, content: &str) -> PathBuf {
        let path = self.temp_dir.path().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}

// TestDatabase with automatic cleanup
pub struct TestDatabase {
    name: String,
    pool: PgPool,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let name = format!("test_db_{}", Uuid::new_v4());
        
        // Create database
        let conn = PgConnection::connect("postgres://localhost/postgres").await.unwrap();
        sqlx::query(&format!("CREATE DATABASE {}", name))
            .execute(&conn)
            .await
            .unwrap();
        
        // Connect to new database
        let pool = PgPool::connect(&format!("postgres://localhost/{}", name))
            .await
            .unwrap();
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .unwrap();
        
        Self { name, pool }
    }
    
    pub fn url(&self) -> String {
        format!("postgres://localhost/{}", self.name)
    }
    
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Schedule cleanup (can't be async in Drop)
        let name = self.name.clone();
        tokio::spawn(async move {
            if let Ok(conn) = PgConnection::connect("postgres://localhost/postgres").await {
                let _ = sqlx::query(&format!("DROP DATABASE IF EXISTS {}", name))
                    .execute(&conn)
                    .await;
            }
        });
    }
}

// Usage:
#[tokio::test]
async fn test_with_full_environment() {
    let env = TestEnvironment::new().await;
    
    // Setup mock responses
    Mock::given(method("GET"))
        .and(path("/api/data"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ok"
        })))
        .mount(&env.mock_server)
        .await;
    
    // Create test file
    let test_file = env.create_test_file("input.txt", "test data");
    
    // Run test with all resources
    let service = MyService::new(env.config.clone());
    let result = service.process_file(test_file).await.unwrap();
    
    assert_eq!(result.status, "ok");
    
    // Everything cleaned up automatically!
}
```

### Checkpoint 5: Complete Composable Fixtures
```rust
// Trait for composable fixtures
trait Fixture: Sized {
    type Output;
    
    fn build(self) -> Self::Output;
    
    fn and_then<F, Other>(self, f: F) -> Combined<Self, Other>
    where
        F: FnOnce(Self::Output) -> Other,
        Other: Fixture,
    {
        Combined {
            first: self,
            transform: Box::new(f),
        }
    }
}

// Product catalog fixtures
pub struct ProductBuilder {
    name: String,
    price: f64,
    category: String,
    in_stock: bool,
}

impl ProductBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            price: 9.99,
            category: "General".to_string(),
            in_stock: true,
        }
    }
    
    pub fn priced_at(mut self, price: f64) -> Self {
        self.price = price;
        self
    }
    
    pub fn in_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }
    
    pub fn out_of_stock(mut self) -> Self {
        self.in_stock = false;
        self
    }
}

// Shopping cart fixtures
pub struct CartBuilder {
    user: User,
    items: Vec<(Product, u32)>,
}

impl CartBuilder {
    pub fn for_user(user: User) -> Self {
        Self {
            user,
            items: vec![],
        }
    }
    
    pub fn with_product(mut self, product: Product, quantity: u32) -> Self {
        self.items.push((product, quantity));
        self
    }
    
    pub fn with_products(mut self, products: Vec<Product>) -> Self {
        for product in products {
            self.items.push((product, 1));
        }
        self
    }
}

// Compose fixtures for complex scenarios
pub mod scenarios {
    use super::*;
    
    pub fn user_abandons_expensive_cart() -> (User, Cart) {
        let user = UserBuilder::new()
            .with_username("hesitant_shopper")
            .build();
        
        let expensive_items = vec![
            ProductBuilder::new("Gaming Laptop").priced_at(1999.99).build(),
            ProductBuilder::new("4K Monitor").priced_at(599.99).build(),
            ProductBuilder::new("Mechanical Keyboard").priced_at(149.99).build(),
        ];
        
        let cart = CartBuilder::for_user(user.clone())
            .with_products(expensive_items)
            .build();
        
        (user, cart)
    }
    
    pub fn bulk_order_scenario() -> OrderScenario {
        let business_user = UserBuilder::new()
            .with_username("business_buyer")
            .premium()
            .build();
        
        let bulk_products: Vec<_> = (1..=5)
            .map(|i| ProductBuilder::new(format!("Bulk Item {}", i))
                .priced_at(5.99)
                .build())
            .collect();
        
        let cart = CartBuilder::for_user(business_user.clone())
            .with_products(bulk_products.clone())
            .build();
        
        OrderScenario {
            user: business_user,
            cart,
            expected_discount: 0.15, // 15% bulk discount
        }
    }
}

// Usage:
#[test]
fn test_abandoned_cart_recovery() {
    let (user, cart) = scenarios::user_abandons_expensive_cart();
    
    let recovery_email = generate_recovery_email(&user, &cart);
    assert!(recovery_email.includes_discount_code());
    assert!(recovery_email.total_value() > 2000.0);
}
```

### Checkpoint 6: Complete Parameterized Fixtures
```rust
// Flexible fixture generation
pub struct FixtureFactory;

impl FixtureFactory {
    pub fn users_with_ages(ages: &[u8]) -> Vec<User> {
        ages.iter()
            .enumerate()
            .map(|(idx, &age)| {
                UserBuilder::new()
                    .with_id(idx as u64 + 1)
                    .with_username(format!("user_{}", idx))
                    .with_age(age)
                    .build()
            })
            .collect()
    }
    
    pub fn users_by_country(countries: &[(&str, usize)]) -> Vec<User> {
        countries.iter()
            .flat_map(|(country, count)| {
                (0..*count).map(move |i| {
                    UserBuilder::new()
                        .with_username(format!("{}_{}", country.to_lowercase(), i))
                        .from_country(*country)
                        .build()
                })
            })
            .collect()
    }
    
    pub fn products_in_price_range(
        count: usize,
        min_price: f64,
        max_price: f64,
    ) -> Vec<Product> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        (0..count)
            .map(|i| {
                let price = rng.gen_range(min_price..=max_price);
                ProductBuilder::new(format!("Product {}", i))
                    .priced_at((price * 100.0).round() / 100.0) // Round to cents
                    .build()
            })
            .collect()
    }
    
    pub fn orders_over_time(
        user: User,
        count: usize,
        days_span: i64,
    ) -> Vec<Order> {
        (0..count)
            .map(|i| {
                let days_ago = days_span - (i as i64 * days_span / count as i64);
                let order_date = Utc::now() - Duration::days(days_ago);
                
                OrderBuilder::new()
                    .for_customer(user.clone())
                    .with_item(&format!("Item {}", i), 10.0 + i as f64, 1)
                    .placed_at(order_date)
                    .build()
            })
            .collect()
    }
}

// Scenario generators
pub fn create_test_marketplace(
    num_sellers: usize,
    products_per_seller: usize,
) -> Marketplace {
    let sellers = FixtureFactory::users_with_roles(
        vec![UserRole::Seller; num_sellers]
    );
    
    let mut marketplace = Marketplace::new();
    
    for seller in sellers {
        let products = (0..products_per_seller)
            .map(|i| {
                ProductBuilder::new(format!("{}'s Product {}", seller.username, i))
                    .by_seller(seller.clone())
                    .build()
            })
            .collect();
        
        marketplace.add_seller_with_products(seller, products);
    }
    
    marketplace
}

// Usage:
#[test]
fn test_age_based_recommendations() {
    let users = FixtureFactory::users_with_ages(&[15, 25, 35, 45, 65]);
    let recommendations = users.iter()
        .map(|user| recommend_products(user))
        .collect::<Vec<_>>();
    
    // Verify age-appropriate recommendations
    assert!(recommendations[0].all_rated_for_teens());
    assert!(recommendations[4].includes_senior_discounts());
}

#[test]
fn test_international_shipping() {
    let users = FixtureFactory::users_by_country(&[
        ("US", 5),
        ("UK", 3),
        ("JP", 2),
    ]);
    
    for user in users {
        let shipping = calculate_shipping(&user);
        match user.country.as_str() {
            "US" => assert_eq!(shipping.base_rate, 5.00),
            "UK" => assert_eq!(shipping.base_rate, 12.00),
            "JP" => assert_eq!(shipping.base_rate, 15.00),
            _ => unreachable!(),
        }
    }
}
```

## 🎉 Congratulations!

You've mastered test fixtures and data builders! Key takeaways:
- Builders provide flexible, maintainable test data
- Object Mothers offer named, meaningful test scenarios
- RAII pattern ensures automatic cleanup
- Composable fixtures enable complex test setups
- Parameterized factories generate varied test data
- Good fixtures make tests readable and focused on behavior

Your tests are now as clean and maintainable as your production code!