# Exercise 08 - Coverage Gaps: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Measuring Test Coverage
```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Or use llvm-cov (newer approach)
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

```rust
// Look for uncovered lines marked in red:
pub fn calculate_discount(price: f64, user_type: UserType) -> f64 {
    if price <= 0.0 {
        return 0.0;  // ❌ Not tested!
    }
    
    match user_type {
        UserType::Premium => price * 0.2,    // ✅ Tested
        UserType::Regular => price * 0.1,    // ✅ Tested  
        UserType::Guest => 0.0,              // ❌ Not tested!
    }
}

// Add missing tests:
#[test]
fn test_negative_price() {
    assert_eq!(calculate_discount(-10.0, UserType::Regular), 0.0);
}

#[test]
fn test_guest_user_discount() {
    assert_eq!(calculate_discount(100.0, UserType::Guest), 0.0);
}
```

### Checkpoint 2: Testing Error Scenarios
```rust
// Function with multiple error cases
pub fn process_payment(amount: f64, card: &CreditCard) -> Result<PaymentResult, PaymentError> {
    if amount <= 0.0 {
        return Err(PaymentError::InvalidAmount);  // Test this!
    }
    
    if card.is_expired() {
        return Err(PaymentError::ExpiredCard);    // Test this!
    }
    
    if amount > card.credit_limit {
        return Err(PaymentError::InsufficientFunds);  // Test this!
    }
    
    // Process payment...
    Ok(PaymentResult::Success)
}

// Comprehensive error testing:
#[test]
fn test_payment_errors() {
    let valid_card = CreditCard::new("4111111111111111", "12/25", 1000.0);
    let expired_card = CreditCard::new("4111111111111111", "12/20", 1000.0);
    
    // Test all error conditions
    assert!(matches!(
        process_payment(-10.0, &valid_card),
        Err(PaymentError::InvalidAmount)
    ));
    
    assert!(matches!(
        process_payment(100.0, &expired_card),
        Err(PaymentError::ExpiredCard)
    ));
    
    assert!(matches!(
        process_payment(2000.0, &valid_card),
        Err(PaymentError::InsufficientFunds)
    ));
    
    // Test success case too
    assert!(process_payment(100.0, &valid_card).is_ok());
}
```

### Checkpoint 3: Edge Case Testing
```rust
// Function with edge cases
pub fn find_median(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;  // Edge case: empty input
    }
    
    let mut sorted = numbers.to_vec();
    sorted.sort();
    
    let len = sorted.len();
    
    if len % 2 == 0 {
        // Even length: average of two middle elements
        let mid1 = sorted[len / 2 - 1] as f64;
        let mid2 = sorted[len / 2] as f64;
        Some((mid1 + mid2) / 2.0)
    } else {
        // Odd length: middle element
        Some(sorted[len / 2] as f64)
    }
}

// Comprehensive edge case testing:
#[test]
fn test_median_edge_cases() {
    // Empty collection
    assert_eq!(find_median(&[]), None);
    
    // Single element
    assert_eq!(find_median(&[42]), Some(42.0));
    
    // Two elements
    assert_eq!(find_median(&[1, 3]), Some(2.0));
    
    // Odd number of elements
    assert_eq!(find_median(&[1, 2, 3]), Some(2.0));
    
    // Even number of elements
    assert_eq!(find_median(&[1, 2, 3, 4]), Some(2.5));
    
    // Negative numbers
    assert_eq!(find_median(&[-5, -1, 0, 2]), Some(-0.5));
    
    // Duplicate values
    assert_eq!(find_median(&[1, 2, 2, 3]), Some(2.0));
    
    // Large numbers
    assert_eq!(find_median(&[i32::MAX]), Some(i32::MAX as f64));
}
```

### Checkpoint 4: Complex Condition Testing
```rust
// Function with complex logic
pub fn should_send_notification(
    user: &User,
    notification_type: NotificationType,
    time_of_day: u8,  // 0-23
) -> bool {
    // Complex conditions requiring thorough testing
    if !user.is_active || user.notifications_disabled {
        return false;
    }
    
    match notification_type {
        NotificationType::Marketing => {
            user.marketing_opt_in && 
            time_of_day >= 9 && time_of_day <= 21 &&
            !user.is_premium  // Premium users don't get marketing
        },
        NotificationType::Security => {
            true  // Always send security notifications
        },
        NotificationType::Social => {
            user.social_notifications_enabled &&
            (time_of_day >= 8 && time_of_day <= 22)
        }
    }
}

// Test all combinations:
#[test]
fn test_notification_logic_coverage() {
    // Create test users with different states
    let active_user = User {
        is_active: true,
        notifications_disabled: false,
        marketing_opt_in: true,
        is_premium: false,
        social_notifications_enabled: true,
    };
    
    let inactive_user = User { is_active: false, ..active_user.clone() };
    let disabled_notifications = User { notifications_disabled: true, ..active_user.clone() };
    let premium_user = User { is_premium: true, ..active_user.clone() };
    
    // Test all branches for inactive user
    assert!(!should_send_notification(&inactive_user, NotificationType::Marketing, 10));
    assert!(!should_send_notification(&inactive_user, NotificationType::Security, 10));
    assert!(!should_send_notification(&inactive_user, NotificationType::Social, 10));
    
    // Test marketing notifications
    assert!(should_send_notification(&active_user, NotificationType::Marketing, 15));
    assert!(!should_send_notification(&active_user, NotificationType::Marketing, 7));   // Too early
    assert!(!should_send_notification(&active_user, NotificationType::Marketing, 23));  // Too late
    assert!(!should_send_notification(&premium_user, NotificationType::Marketing, 15)); // Premium user
    
    // Test security notifications (always true for active users)
    assert!(should_send_notification(&active_user, NotificationType::Security, 3));
    assert!(!should_send_notification(&inactive_user, NotificationType::Security, 15));
    
    // Test social notifications
    assert!(should_send_notification(&active_user, NotificationType::Social, 12));
    assert!(!should_send_notification(&active_user, NotificationType::Social, 5));   // Too early
    assert!(!should_send_notification(&active_user, NotificationType::Social, 23));  // Too late
}
```

### Checkpoint 5: State Transition Testing
```rust
// State machine requiring transition testing
#[derive(Debug, PartialEq, Clone)]
pub enum OrderState {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}

pub struct Order {
    pub state: OrderState,
    pub items: Vec<OrderItem>,
}

impl Order {
    pub fn confirm(&mut self) -> Result<(), String> {
        match self.state {
            OrderState::Pending => {
                self.state = OrderState::Confirmed;
                Ok(())
            },
            _ => Err("Can only confirm pending orders".to_string())
        }
    }
    
    pub fn ship(&mut self) -> Result<(), String> {
        match self.state {
            OrderState::Confirmed => {
                self.state = OrderState::Shipped;
                Ok(())
            },
            _ => Err("Can only ship confirmed orders".to_string())
        }
    }
    
    pub fn cancel(&mut self) -> Result<(), String> {
        match self.state {
            OrderState::Pending | OrderState::Confirmed => {
                self.state = OrderState::Cancelled;
                Ok(())
            },
            _ => Err("Cannot cancel shipped or delivered orders".to_string())
        }
    }
}

// Test all valid and invalid transitions:
#[test]
fn test_order_state_transitions() {
    let mut order = Order { 
        state: OrderState::Pending, 
        items: vec![] 
    };
    
    // Test valid transitions
    assert!(order.confirm().is_ok());
    assert_eq!(order.state, OrderState::Confirmed);
    
    assert!(order.ship().is_ok());
    assert_eq!(order.state, OrderState::Shipped);
    
    // Test invalid transitions
    let mut pending_order = Order { state: OrderState::Pending, items: vec![] };
    assert!(pending_order.ship().is_err());  // Can't ship pending order
    
    let mut shipped_order = Order { state: OrderState::Shipped, items: vec![] };
    assert!(shipped_order.cancel().is_err());  // Can't cancel shipped order
    assert!(shipped_order.confirm().is_err()); // Can't re-confirm
}
```

### Checkpoint 6: Integration Path Testing
```rust
// Service with multiple dependencies
pub struct OrderService {
    payment_service: Box<dyn PaymentService>,
    inventory_service: Box<dyn InventoryService>,
    notification_service: Box<dyn NotificationService>,
}

impl OrderService {
    pub fn process_order(&self, order: &Order) -> Result<OrderResult, OrderError> {
        // Multiple integration points to test
        
        // 1. Check inventory
        for item in &order.items {
            if !self.inventory_service.is_available(&item.product_id, item.quantity)? {
                return Err(OrderError::InsufficientInventory);
            }
        }
        
        // 2. Process payment
        let payment_result = self.payment_service.charge(
            &order.payment_method,
            order.total_amount(),
        )?;
        
        // 3. Reserve inventory
        for item in &order.items {
            self.inventory_service.reserve(&item.product_id, item.quantity)?;
        }
        
        // 4. Send confirmation
        self.notification_service.send_order_confirmation(&order.customer_email, order)?;
        
        Ok(OrderResult {
            order_id: order.id,
            payment_id: payment_result.transaction_id,
            status: "confirmed".to_string(),
        })
    }
}

// Test all integration failure scenarios:
#[test]
fn test_order_processing_integration_failures() {
    // Test inventory failure
    let mut inventory_mock = MockInventoryService::new();
    inventory_mock.expect_is_available()
        .returning(|_, _| Ok(false));  // Not available
    
    let service = OrderService::new(
        Box::new(MockPaymentService::new()),
        Box::new(inventory_mock),
        Box::new(MockNotificationService::new())
    );
    
    let order = create_test_order();
    assert!(matches!(
        service.process_order(&order),
        Err(OrderError::InsufficientInventory)
    ));
    
    // Test payment failure
    let mut payment_mock = MockPaymentService::new();
    payment_mock.expect_charge()
        .returning(|_, _| Err(PaymentError::CardDeclined));
    
    // ... test other failure scenarios
}
```

## 🎯 Pattern Recognition

Key insights:
- Coverage tools show what code isn't tested
- Error paths are often missed in testing
- Edge cases reveal boundary condition bugs  
- Complex conditions need systematic testing
- State machines require transition coverage
- Integration points need failure scenario testing

Ready for Level 3 if you need complete solutions!