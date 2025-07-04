# Exercise 08 - Coverage Gaps: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete Coverage Analysis Setup
```bash
# Comprehensive coverage setup script
#!/bin/bash

# Install coverage tools
cargo install cargo-tarpaulin
cargo install cargo-llvm-cov

# Generate HTML coverage report
echo "Generating coverage report with tarpaulin..."
cargo tarpaulin --out Html --output-dir coverage/

# Alternative: using llvm-cov (more accurate)
echo "Generating coverage report with llvm-cov..."
cargo llvm-cov --html --output-dir coverage-llvm/

# Open reports
if command -v open &> /dev/null; then
    open coverage/tarpaulin-report.html
    open coverage-llvm/index.html
fi
```

```rust
// Example function with coverage gaps highlighted
pub fn calculate_tax(
    income: f64,
    filing_status: FilingStatus,
    deductions: &[Deduction],
) -> Result<TaxCalculation, TaxError> {
    if income < 0.0 {
        return Err(TaxError::NegativeIncome);  // ❌ NOT TESTED
    }
    
    let standard_deduction = match filing_status {
        FilingStatus::Single => 12950.0,        // ✅ Tested
        FilingStatus::MarriedJoint => 25900.0,  // ✅ Tested  
        FilingStatus::MarriedSeparate => 12950.0, // ❌ NOT TESTED
        FilingStatus::HeadOfHousehold => 19400.0, // ❌ NOT TESTED
    };
    
    let total_deductions: f64 = deductions.iter()
        .map(|d| d.amount)
        .sum();
    
    let itemized_deductions = if total_deductions > standard_deduction {
        total_deductions  // ❌ NOT TESTED (complex condition)
    } else {
        standard_deduction
    };
    
    let taxable_income = (income - itemized_deductions).max(0.0);
    
    let tax = calculate_progressive_tax(taxable_income, filing_status)?;
    
    Ok(TaxCalculation {
        gross_income: income,
        deductions: itemized_deductions,
        taxable_income,
        tax_owed: tax,
    })
}

// Complete test coverage:
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_negative_income_error() {
        let result = calculate_tax(-1000.0, FilingStatus::Single, &[]);
        assert!(matches!(result, Err(TaxError::NegativeIncome)));
    }
    
    #[test]
    fn test_all_filing_statuses() {
        let income = 50000.0;
        let no_deductions = &[];
        
        // Test each filing status
        let single = calculate_tax(income, FilingStatus::Single, no_deductions).unwrap();
        let married_joint = calculate_tax(income, FilingStatus::MarriedJoint, no_deductions).unwrap();
        let married_separate = calculate_tax(income, FilingStatus::MarriedSeparate, no_deductions).unwrap();
        let head_of_household = calculate_tax(income, FilingStatus::HeadOfHousehold, no_deductions).unwrap();
        
        // Verify different standard deductions
        assert_eq!(single.deductions, 12950.0);
        assert_eq!(married_joint.deductions, 25900.0);
        assert_eq!(married_separate.deductions, 12950.0);
        assert_eq!(head_of_household.deductions, 19400.0);
    }
    
    #[test]
    fn test_itemized_vs_standard_deductions() {
        let income = 100000.0;
        
        // Test with low itemized deductions (should use standard)
        let low_deductions = vec![
            Deduction { amount: 5000.0, category: "Charity".to_string() }
        ];
        let result_standard = calculate_tax(income, FilingStatus::Single, &low_deductions).unwrap();
        assert_eq!(result_standard.deductions, 12950.0);  // Standard deduction
        
        // Test with high itemized deductions (should use itemized)
        let high_deductions = vec![
            Deduction { amount: 10000.0, category: "State Tax".to_string() },
            Deduction { amount: 8000.0, category: "Charity".to_string() },
        ];
        let result_itemized = calculate_tax(income, FilingStatus::Single, &high_deductions).unwrap();
        assert_eq!(result_itemized.deductions, 18000.0);  // Itemized total
    }
}
```

### Checkpoint 2: Complete Error Path Testing
```rust
use std::fs::File;
use std::io::{self, Read, Write};

// File processor with multiple error scenarios
pub struct FileProcessor;

impl FileProcessor {
    pub fn process_config_file(path: &str) -> Result<Config, ProcessingError> {
        // Error path 1: File not found
        let mut file = File::open(path)
            .map_err(|e| ProcessingError::FileNotFound(path.to_string()))?;
        
        // Error path 2: Read failure
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| ProcessingError::ReadError(e.to_string()))?;
        
        // Error path 3: Empty file
        if contents.trim().is_empty() {
            return Err(ProcessingError::EmptyFile);
        }
        
        // Error path 4: Invalid JSON
        let raw_config: serde_json::Value = serde_json::from_str(&contents)
            .map_err(|e| ProcessingError::InvalidJson(e.to_string()))?;
        
        // Error path 5: Missing required fields
        let name = raw_config.get("name")
            .and_then(|v| v.as_str())
            .ok_or(ProcessingError::MissingField("name".to_string()))?;
        
        let port = raw_config.get("port")
            .and_then(|v| v.as_u64())
            .ok_or(ProcessingError::MissingField("port".to_string()))?;
        
        // Error path 6: Invalid port range
        if port == 0 || port > 65535 {
            return Err(ProcessingError::InvalidPort(port));
        }
        
        // Error path 7: Name validation
        if name.len() < 3 || name.len() > 50 {
            return Err(ProcessingError::InvalidName(name.to_string()));
        }
        
        Ok(Config {
            name: name.to_string(),
            port: port as u16,
        })
    }
}

// Comprehensive error testing:
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    #[test]
    fn test_file_not_found() {
        let result = FileProcessor::process_config_file("nonexistent.json");
        assert!(matches!(result, Err(ProcessingError::FileNotFound(_))));
    }
    
    #[test]
    fn test_empty_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "   ").unwrap();  // Only whitespace
        
        let result = FileProcessor::process_config_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(ProcessingError::EmptyFile)));
    }
    
    #[test]
    fn test_invalid_json() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{{ invalid json").unwrap();
        
        let result = FileProcessor::process_config_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(ProcessingError::InvalidJson(_))));
    }
    
    #[test]
    fn test_missing_fields() {
        // Missing name field
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"port": 8080}}"#).unwrap();
        
        let result = FileProcessor::process_config_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(ProcessingError::MissingField(ref field)) if field == "name"));
        
        // Missing port field
        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, r#"{{"name": "test"}}"#).unwrap();
        
        let result2 = FileProcessor::process_config_file(temp_file2.path().to_str().unwrap());
        assert!(matches!(result2, Err(ProcessingError::MissingField(ref field)) if field == "port"));
    }
    
    #[test]
    fn test_invalid_port_range() {
        // Port 0
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"name": "test", "port": 0}}"#).unwrap();
        
        let result = FileProcessor::process_config_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(ProcessingError::InvalidPort(0))));
        
        // Port too high
        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, r#"{{"name": "test", "port": 70000}}"#).unwrap();
        
        let result2 = FileProcessor::process_config_file(temp_file2.path().to_str().unwrap());
        assert!(matches!(result2, Err(ProcessingError::InvalidPort(70000))));
    }
    
    #[test]
    fn test_invalid_name_length() {
        // Name too short
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"name": "ab", "port": 8080}}"#).unwrap();
        
        let result = FileProcessor::process_config_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(ProcessingError::InvalidName(_))));
        
        // Name too long
        let long_name = "a".repeat(51);
        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, r#"{{"name": "{}", "port": 8080}}"#, long_name).unwrap();
        
        let result2 = FileProcessor::process_config_file(temp_file2.path().to_str().unwrap());
        assert!(matches!(result2, Err(ProcessingError::InvalidName(_))));
    }
    
    #[test]
    fn test_successful_processing() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"name": "test-server", "port": 8080}}"#).unwrap();
        
        let result = FileProcessor::process_config_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.name, "test-server");
        assert_eq!(config.port, 8080);
    }
}
```

### Checkpoint 3: Complete Edge Case Testing
```rust
// Function with many edge cases
pub struct Calculator;

impl Calculator {
    pub fn calculate_payment(
        principal: f64,
        annual_rate: f64,
        years: u32,
        extra_payment: Option<f64>,
    ) -> Result<PaymentCalculation, CalculationError> {
        // Edge case: Zero or negative principal
        if principal <= 0.0 {
            return Err(CalculationError::InvalidPrincipal);
        }
        
        // Edge case: Negative interest rate
        if annual_rate < 0.0 {
            return Err(CalculationError::InvalidRate);
        }
        
        // Edge case: Zero years
        if years == 0 {
            return Err(CalculationError::InvalidTerm);
        }
        
        // Edge case: Zero interest rate (special calculation)
        if annual_rate == 0.0 {
            let monthly_payment = principal / (years as f64 * 12.0);
            return Ok(PaymentCalculation {
                monthly_payment,
                total_interest: 0.0,
                total_paid: principal,
                months_to_payoff: years * 12,
            });
        }
        
        let monthly_rate = annual_rate / 12.0 / 100.0;
        let num_payments = years * 12;
        
        // Standard loan calculation
        let monthly_payment = principal * 
            (monthly_rate * (1.0 + monthly_rate).powi(num_payments as i32)) /
            ((1.0 + monthly_rate).powi(num_payments as i32) - 1.0);
        
        // Handle extra payments
        let (total_interest, actual_months) = if let Some(extra) = extra_payment {
            if extra < 0.0 {
                return Err(CalculationError::InvalidExtraPayment);
            }
            
            self.calculate_with_extra_payments(principal, monthly_rate, monthly_payment + extra)
        } else {
            let total_paid = monthly_payment * num_payments as f64;
            (total_paid - principal, num_payments)
        };
        
        Ok(PaymentCalculation {
            monthly_payment: if extra_payment.is_some() { 
                monthly_payment + extra_payment.unwrap() 
            } else { 
                monthly_payment 
            },
            total_interest,
            total_paid: principal + total_interest,
            months_to_payoff: actual_months,
        })
    }
}

// Comprehensive edge case testing:
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_edge_case_zero_negative_principal() {
        assert!(matches!(
            Calculator::calculate_payment(0.0, 5.0, 30, None),
            Err(CalculationError::InvalidPrincipal)
        ));
        
        assert!(matches!(
            Calculator::calculate_payment(-1000.0, 5.0, 30, None),
            Err(CalculationError::InvalidPrincipal)
        ));
    }
    
    #[test]
    fn test_edge_case_negative_interest_rate() {
        assert!(matches!(
            Calculator::calculate_payment(100000.0, -1.0, 30, None),
            Err(CalculationError::InvalidRate)
        ));
    }
    
    #[test]
    fn test_edge_case_zero_years() {
        assert!(matches!(
            Calculator::calculate_payment(100000.0, 5.0, 0, None),
            Err(CalculationError::InvalidTerm)
        ));
    }
    
    #[test]
    fn test_edge_case_zero_interest_rate() {
        let result = Calculator::calculate_payment(120000.0, 0.0, 10, None).unwrap();
        
        // With 0% interest, payment is just principal divided by months
        assert_relative_eq!(result.monthly_payment, 1000.0, epsilon = 0.01);
        assert_relative_eq!(result.total_interest, 0.0, epsilon = 0.01);
        assert_eq!(result.months_to_payoff, 120);
    }
    
    #[test]
    fn test_edge_case_very_high_interest_rate() {
        let result = Calculator::calculate_payment(100000.0, 99.0, 1, None).unwrap();
        
        // With very high interest, most payment goes to interest
        assert!(result.monthly_payment > 8000.0);  // Should be very high
        assert!(result.total_interest > result.monthly_payment * 12.0 - 100000.0);
    }
    
    #[test]
    fn test_edge_case_very_small_loan() {
        let result = Calculator::calculate_payment(0.01, 5.0, 1, None).unwrap();
        
        // Should handle tiny loans without panicking
        assert!(result.monthly_payment > 0.0);
        assert!(result.monthly_payment < 1.0);
    }
    
    #[test]
    fn test_edge_case_very_large_loan() {
        let result = Calculator::calculate_payment(1_000_000_000.0, 5.0, 30, None).unwrap();
        
        // Should handle very large loans
        assert!(result.monthly_payment > 5_000_000.0);
        assert!(result.total_paid > 1_000_000_000.0);
    }
    
    #[test]
    fn test_edge_case_negative_extra_payment() {
        assert!(matches!(
            Calculator::calculate_payment(100000.0, 5.0, 30, Some(-100.0)),
            Err(CalculationError::InvalidExtraPayment)
        ));
    }
    
    #[test]
    fn test_edge_case_huge_extra_payment() {
        // Extra payment larger than principal
        let result = Calculator::calculate_payment(1000.0, 5.0, 30, Some(2000.0)).unwrap();
        
        // Should pay off very quickly
        assert!(result.months_to_payoff <= 1);
        assert!(result.total_interest < 10.0);  // Very little interest
    }
    
    #[test]
    fn test_edge_case_boundary_values() {
        // Test with f64 precision boundaries
        let result1 = Calculator::calculate_payment(
            f64::MIN_POSITIVE, 
            f64::MIN_POSITIVE, 
            1, 
            None
        ).unwrap();
        assert!(result1.monthly_payment.is_finite());
        
        // Test with max reasonable values
        let result2 = Calculator::calculate_payment(
            1e12,  // 1 trillion 
            100.0, // 100% APR
            100,   // 100 years
            None
        ).unwrap();
        assert!(result2.monthly_payment.is_finite());
        assert!(!result2.monthly_payment.is_nan());
    }
}
```

### Checkpoint 4: Complete Complex Condition Testing
```rust
// Authentication system with complex logic
pub struct AuthManager;

impl AuthManager {
    pub fn authenticate(
        &self,
        credentials: &Credentials,
        user: &User,
        security_context: &SecurityContext,
    ) -> AuthResult {
        // Complex multi-factor authentication logic
        
        // Base authentication
        if !self.verify_password(&credentials.password, &user.password_hash) {
            return AuthResult::Failed(AuthError::InvalidPassword);
        }
        
        // Account status checks
        if !user.is_active {
            return AuthResult::Failed(AuthError::AccountDisabled);
        }
        
        if user.is_locked && !self.is_lock_expired(&user.locked_until) {
            return AuthResult::Failed(AuthError::AccountLocked);
        }
        
        // Security context evaluation
        let risk_score = self.calculate_risk_score(&credentials, &user, &security_context);
        
        let requires_mfa = 
            risk_score >= 0.7 ||                                    // High risk
            user.role == UserRole::Admin ||                         // Admin always needs MFA
            security_context.is_new_device ||                       // New device
            (security_context.is_foreign_ip && !user.travel_mode);  // Foreign IP without travel mode
        
        if requires_mfa {
            if credentials.mfa_token.is_none() {
                return AuthResult::RequiresMFA(self.generate_mfa_challenge(&user));
            }
            
            let mfa_token = credentials.mfa_token.as_ref().unwrap();
            if !self.verify_mfa_token(mfa_token, &user) {
                return AuthResult::Failed(AuthError::InvalidMFA);
            }
        }
        
        // Additional checks for high-privilege accounts
        if user.role == UserRole::Admin && security_context.office_hours_only {
            if !self.is_office_hours(&security_context.timestamp) {
                return AuthResult::Failed(AuthError::OutsideOfficeHours);
            }
        }
        
        // Success with appropriate privileges
        let session_duration = if user.role == UserRole::Admin {
            Duration::from_secs(3600)  // 1 hour for admins
        } else if requires_mfa {
            Duration::from_secs(28800) // 8 hours for MFA users
        } else {
            Duration::from_secs(14400) // 4 hours for regular users
        };
        
        AuthResult::Success(AuthSession {
            user_id: user.id,
            session_duration,
            requires_mfa: requires_mfa,
            risk_score,
        })
    }
}

// Systematic testing of all condition combinations:
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, TimeZone};
    
    fn create_base_user() -> User {
        User {
            id: 1,
            is_active: true,
            is_locked: false,
            locked_until: None,
            password_hash: "hashed_password".to_string(),
            role: UserRole::Regular,
            travel_mode: false,
        }
    }
    
    fn create_base_credentials() -> Credentials {
        Credentials {
            password: "correct_password".to_string(),
            mfa_token: None,
        }
    }
    
    fn create_base_context() -> SecurityContext {
        SecurityContext {
            is_new_device: false,
            is_foreign_ip: false,
            office_hours_only: false,
            timestamp: Utc::now(),
        }
    }
    
    #[test]
    fn test_basic_authentication_scenarios() {
        let auth = AuthManager::new();
        let user = create_base_user();
        let credentials = create_base_credentials();
        let context = create_base_context();
        
        // Successful basic auth
        let result = auth.authenticate(&credentials, &user, &context);
        assert!(matches!(result, AuthResult::Success(_)));
        
        // Wrong password
        let wrong_creds = Credentials {
            password: "wrong_password".to_string(),
            ..credentials
        };
        let result = auth.authenticate(&wrong_creds, &user, &context);
        assert!(matches!(result, AuthResult::Failed(AuthError::InvalidPassword)));
        
        // Inactive account
        let inactive_user = User { is_active: false, ..user };
        let result = auth.authenticate(&credentials, &inactive_user, &context);
        assert!(matches!(result, AuthResult::Failed(AuthError::AccountDisabled)));
        
        // Locked account
        let locked_user = User {
            is_locked: true,
            locked_until: Some(Utc::now() + Duration::from_secs(3600)),
            ..user
        };
        let result = auth.authenticate(&credentials, &locked_user, &context);
        assert!(matches!(result, AuthResult::Failed(AuthError::AccountLocked)));
    }
    
    #[test]
    fn test_mfa_requirements_combinations() {
        let auth = AuthManager::new();
        let credentials = create_base_credentials();
        
        // Test: Admin always requires MFA
        let admin_user = User { role: UserRole::Admin, ..create_base_user() };
        let context = create_base_context();
        
        let result = auth.authenticate(&credentials, &admin_user, &context);
        assert!(matches!(result, AuthResult::RequiresMFA(_)));
        
        // Test: New device requires MFA
        let user = create_base_user();
        let new_device_context = SecurityContext { is_new_device: true, ..context };
        
        let result = auth.authenticate(&credentials, &user, &new_device_context);
        assert!(matches!(result, AuthResult::RequiresMFA(_)));
        
        // Test: Foreign IP without travel mode requires MFA
        let foreign_ip_context = SecurityContext { is_foreign_ip: true, ..context };
        
        let result = auth.authenticate(&credentials, &user, &foreign_ip_context);
        assert!(matches!(result, AuthResult::RequiresMFA(_)));
        
        // Test: Foreign IP with travel mode doesn't require MFA
        let travel_user = User { travel_mode: true, ..user };
        
        let result = auth.authenticate(&credentials, &travel_user, &foreign_ip_context);
        assert!(matches!(result, AuthResult::Success(_)));
    }
    
    #[test]
    fn test_mfa_verification() {
        let auth = AuthManager::new();
        let admin_user = User { role: UserRole::Admin, ..create_base_user() };
        let context = create_base_context();
        
        // Valid MFA token
        let valid_mfa_creds = Credentials {
            mfa_token: Some("123456".to_string()),
            ..create_base_credentials()
        };
        
        let result = auth.authenticate(&valid_mfa_creds, &admin_user, &context);
        assert!(matches!(result, AuthResult::Success(_)));
        
        // Invalid MFA token
        let invalid_mfa_creds = Credentials {
            mfa_token: Some("000000".to_string()),
            ..create_base_credentials()
        };
        
        let result = auth.authenticate(&invalid_mfa_creds, &admin_user, &context);
        assert!(matches!(result, AuthResult::Failed(AuthError::InvalidMFA)));
    }
    
    #[test]
    fn test_office_hours_restriction() {
        let auth = AuthManager::new();
        let admin_user = User { role: UserRole::Admin, ..create_base_user() };
        let mfa_credentials = Credentials {
            mfa_token: Some("123456".to_string()),
            ..create_base_credentials()
        };
        
        // During office hours (9 AM)
        let office_context = SecurityContext {
            office_hours_only: true,
            timestamp: Utc.ymd(2023, 1, 1).and_hms(17, 0, 0), // 9 AM PST
            ..create_base_context()
        };
        
        let result = auth.authenticate(&mfa_credentials, &admin_user, &office_context);
        assert!(matches!(result, AuthResult::Success(_)));
        
        // Outside office hours (2 AM)
        let night_context = SecurityContext {
            timestamp: Utc.ymd(2023, 1, 1).and_hms(10, 0, 0), // 2 AM PST
            ..office_context
        };
        
        let result = auth.authenticate(&mfa_credentials, &admin_user, &night_context);
        assert!(matches!(result, AuthResult::Failed(AuthError::OutsideOfficeHours)));
    }
    
    #[test]
    fn test_session_duration_logic() {
        let auth = AuthManager::new();
        let credentials = create_base_credentials();
        let context = create_base_context();
        
        // Regular user - 4 hour session
        let user = create_base_user();
        if let AuthResult::Success(session) = auth.authenticate(&credentials, &user, &context) {
            assert_eq!(session.session_duration, Duration::from_secs(14400));
        }
        
        // Admin user - 1 hour session
        let admin_user = User { role: UserRole::Admin, ..user };
        let mfa_creds = Credentials {
            mfa_token: Some("123456".to_string()),
            ..credentials
        };
        
        if let AuthResult::Success(session) = auth.authenticate(&mfa_creds, &admin_user, &context) {
            assert_eq!(session.session_duration, Duration::from_secs(3600));
        }
        
        // MFA user - 8 hour session
        let new_device_context = SecurityContext { is_new_device: true, ..context };
        
        if let AuthResult::Success(session) = auth.authenticate(&mfa_creds, &user, &new_device_context) {
            assert_eq!(session.session_duration, Duration::from_secs(28800));
        }
    }
}
```

### Checkpoint 5: Complete State Machine Testing
```rust
// Complex state machine: Order fulfillment system
#[derive(Debug, PartialEq, Clone)]
pub enum OrderState {
    Draft,
    Submitted,
    PaymentPending,
    PaymentFailed,
    Confirmed,
    InProgress,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

pub struct OrderStateMachine {
    pub current_state: OrderState,
    pub history: Vec<StateTransition>,
}

impl OrderStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: OrderState::Draft,
            history: vec![],
        }
    }
    
    pub fn submit(&mut self) -> Result<(), StateError> {
        match self.current_state {
            OrderState::Draft => {
                self.transition_to(OrderState::Submitted, "Order submitted by customer")?;
                Ok(())
            },
            _ => Err(StateError::InvalidTransition {
                from: self.current_state.clone(),
                to: OrderState::Submitted,
                reason: "Can only submit draft orders".to_string(),
            })
        }
    }
    
    pub fn process_payment(&mut self, success: bool) -> Result<(), StateError> {
        match (&self.current_state, success) {
            (OrderState::Submitted, true) => {
                self.transition_to(OrderState::Confirmed, "Payment successful")?;
            },
            (OrderState::Submitted, false) => {
                self.transition_to(OrderState::PaymentFailed, "Payment failed")?;
            },
            (OrderState::PaymentFailed, true) => {
                self.transition_to(OrderState::Confirmed, "Payment retry successful")?;
            },
            (OrderState::PaymentFailed, false) => {
                // Stay in PaymentFailed state
                self.add_history_entry("Payment retry failed")?;
            },
            _ => return Err(StateError::InvalidTransition {
                from: self.current_state.clone(),
                to: if success { OrderState::Confirmed } else { OrderState::PaymentFailed },
                reason: "Payment processing not allowed in current state".to_string(),
            })
        }
        Ok(())
    }
    
    pub fn cancel(&mut self, reason: &str) -> Result<(), StateError> {
        match self.current_state {
            OrderState::Draft | 
            OrderState::Submitted | 
            OrderState::PaymentFailed |
            OrderState::Confirmed => {
                self.transition_to(OrderState::Cancelled, reason)?;
                Ok(())
            },
            OrderState::InProgress | OrderState::Shipped => {
                Err(StateError::InvalidTransition {
                    from: self.current_state.clone(),
                    to: OrderState::Cancelled,
                    reason: "Cannot cancel orders that are in progress or shipped".to_string(),
                })
            },
            _ => Err(StateError::InvalidTransition {
                from: self.current_state.clone(),
                to: OrderState::Cancelled,
                reason: "Cannot cancel order in current state".to_string(),
            })
        }
    }
}

// Comprehensive state machine testing:
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complete_happy_path() {
        let mut order = OrderStateMachine::new();
        
        // Draft -> Submitted
        assert!(order.submit().is_ok());
        assert_eq!(order.current_state, OrderState::Submitted);
        
        // Submitted -> Confirmed (successful payment)
        assert!(order.process_payment(true).is_ok());
        assert_eq!(order.current_state, OrderState::Confirmed);
        
        // Confirmed -> InProgress
        assert!(order.start_fulfillment().is_ok());
        assert_eq!(order.current_state, OrderState::InProgress);
        
        // InProgress -> Shipped
        assert!(order.ship().is_ok());
        assert_eq!(order.current_state, OrderState::Shipped);
        
        // Shipped -> Delivered
        assert!(order.deliver().is_ok());
        assert_eq!(order.current_state, OrderState::Delivered);
        
        // Verify transition history
        assert_eq!(order.history.len(), 5);
    }
    
    #[test]
    fn test_payment_failure_recovery() {
        let mut order = OrderStateMachine::new();
        
        order.submit().unwrap();
        
        // Payment fails
        assert!(order.process_payment(false).is_ok());
        assert_eq!(order.current_state, OrderState::PaymentFailed);
        
        // Payment retry fails (stays in same state)
        assert!(order.process_payment(false).is_ok());
        assert_eq!(order.current_state, OrderState::PaymentFailed);
        
        // Payment retry succeeds
        assert!(order.process_payment(true).is_ok());
        assert_eq!(order.current_state, OrderState::Confirmed);
    }
    
    #[test]
    fn test_invalid_transitions() {
        let mut order = OrderStateMachine::new();
        
        // Cannot ship without submitting first
        assert!(order.ship().is_err());
        
        // Cannot process payment on draft order
        assert!(order.process_payment(true).is_err());
        
        // Submit order
        order.submit().unwrap();
        order.process_payment(true).unwrap();
        order.start_fulfillment().unwrap();
        
        // Cannot cancel in-progress order
        assert!(order.cancel("Customer request").is_err());
        
        order.ship().unwrap();
        
        // Cannot cancel shipped order
        assert!(order.cancel("Customer request").is_err());
    }
    
    #[test]
    fn test_cancellation_windows() {
        // Test cancellation at each valid state
        let cancellable_states = vec![
            OrderState::Draft,
            OrderState::Submitted,
            OrderState::PaymentFailed,
            OrderState::Confirmed,
        ];
        
        for initial_state in cancellable_states {
            let mut order = OrderStateMachine::new();
            order.current_state = initial_state.clone();
            
            let result = order.cancel("Test cancellation");
            assert!(result.is_ok(), "Should be able to cancel from {:?}", initial_state);
            assert_eq!(order.current_state, OrderState::Cancelled);
        }
        
        // Test non-cancellable states
        let non_cancellable_states = vec![
            OrderState::InProgress,
            OrderState::Shipped,
            OrderState::Delivered,
            OrderState::Cancelled,
            OrderState::Refunded,
        ];
        
        for initial_state in non_cancellable_states {
            let mut order = OrderStateMachine::new();
            order.current_state = initial_state.clone();
            
            let result = order.cancel("Test cancellation");
            assert!(result.is_err(), "Should not be able to cancel from {:?}", initial_state);
        }
    }
    
    #[test]
    fn test_state_transition_history() {
        let mut order = OrderStateMachine::new();
        
        order.submit().unwrap();
        order.process_payment(false).unwrap();  // Fail
        order.process_payment(false).unwrap();  // Fail again
        order.process_payment(true).unwrap();   // Succeed
        order.cancel("Customer changed mind").unwrap();
        
        // Verify complete history is recorded
        assert_eq!(order.history.len(), 5);
        assert_eq!(order.history[0].from_state, OrderState::Draft);
        assert_eq!(order.history[0].to_state, OrderState::Submitted);
        assert_eq!(order.history[4].to_state, OrderState::Cancelled);
        assert!(order.history[4].reason.contains("Customer changed mind"));
    }
    
    #[test]
    fn test_concurrent_state_changes() {
        // Test that state machine handles concurrent modifications safely
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let order = Arc::new(Mutex::new(OrderStateMachine::new()));
        
        // Simulate concurrent access
        let handles: Vec<_> = (0..10).map(|i| {
            let order_clone = Arc::clone(&order);
            thread::spawn(move || {
                let mut order = order_clone.lock().unwrap();
                if i == 0 {
                    // Only first thread should succeed in submitting
                    order.submit()
                } else {
                    // Other threads should fail
                    order.submit()
                }
            })
        }).collect();
        
        let results: Vec<_> = handles.into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        
        // Exactly one thread should succeed
        let successes = results.iter().filter(|r| r.is_ok()).count();
        assert_eq!(successes, 1);
        
        let order = order.lock().unwrap();
        assert_eq!(order.current_state, OrderState::Submitted);
    }
}
```

### Checkpoint 6: Complete Integration Testing
```rust
// Full system integration test with all failure modes
use mockall::{automock, predicate::*};
use tokio;

// Complete e-commerce order processing system
pub struct OrderProcessingService {
    inventory: Box<dyn InventoryService>,
    payment: Box<dyn PaymentService>,
    shipping: Box<dyn ShippingService>,
    notification: Box<dyn NotificationService>,
    audit: Box<dyn AuditService>,
}

impl OrderProcessingService {
    pub async fn process_order(&self, order: Order) -> Result<OrderConfirmation, ProcessingError> {
        // Step 1: Audit order received
        self.audit.log_event("order_received", &order.id).await?;
        
        // Step 2: Validate inventory
        for item in &order.items {
            if !self.inventory.check_availability(&item.sku, item.quantity).await? {
                self.audit.log_event("inventory_insufficient", &order.id).await?;
                return Err(ProcessingError::InsufficientInventory(item.sku.clone()));
            }
        }
        
        // Step 3: Reserve inventory
        let mut reservations = Vec::new();
        for item in &order.items {
            let reservation_id = self.inventory.reserve(&item.sku, item.quantity).await?;
            reservations.push(reservation_id);
        }
        
        // Step 4: Process payment
        let payment_result = match self.payment.charge(&order.payment_info, order.total).await {
            Ok(result) => result,
            Err(e) => {
                // Rollback reservations on payment failure
                for reservation_id in reservations {
                    let _ = self.inventory.release_reservation(reservation_id).await;
                }
                self.audit.log_event("payment_failed", &order.id).await?;
                return Err(ProcessingError::PaymentFailed(e));
            }
        };
        
        // Step 5: Create shipping label
        let shipping_label = match self.shipping.create_label(&order.shipping_address, &order.items).await {
            Ok(label) => label,
            Err(e) => {
                // Rollback payment and reservations
                let _ = self.payment.refund(&payment_result.transaction_id).await;
                for reservation_id in reservations {
                    let _ = self.inventory.release_reservation(reservation_id).await;
                }
                self.audit.log_event("shipping_failed", &order.id).await?;
                return Err(ProcessingError::ShippingFailed(e));
            }
        };
        
        // Step 6: Send confirmation
        if let Err(e) = self.notification.send_confirmation(&order.customer_email, &order).await {
            // Log but don't fail the order for notification errors
            self.audit.log_event("notification_failed", &order.id).await?;
        }
        
        // Step 7: Final audit log
        self.audit.log_event("order_completed", &order.id).await?;
        
        Ok(OrderConfirmation {
            order_id: order.id,
            payment_id: payment_result.transaction_id,
            tracking_number: shipping_label.tracking_number,
            estimated_delivery: shipping_label.estimated_delivery,
        })
    }
}

// Complete integration testing with all failure scenarios:
#[cfg(test)]
mod integration_tests {
    use super::*;
    use mockall::Sequence;
    
    fn create_test_order() -> Order {
        Order {
            id: "ORDER-123".to_string(),
            customer_email: "customer@example.com".to_string(),
            items: vec![
                OrderItem { sku: "WIDGET-1".to_string(), quantity: 2 },
                OrderItem { sku: "GADGET-1".to_string(), quantity: 1 },
            ],
            total: 99.99,
            payment_info: PaymentInfo {
                card_number: "4111111111111111".to_string(),
                exp_month: 12,
                exp_year: 2025,
            },
            shipping_address: Address {
                street: "123 Main St".to_string(),
                city: "Anytown".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string(),
            },
        }
    }
    
    #[tokio::test]
    async fn test_successful_order_processing() {
        // Setup all mocks for success scenario
        let mut inventory = MockInventoryService::new();
        let mut payment = MockPaymentService::new();
        let mut shipping = MockShippingService::new();
        let mut notification = MockNotificationService::new();
        let mut audit = MockAuditService::new();
        
        let mut seq = Sequence::new();
        
        // Expected call sequence for successful order
        audit.expect_log_event()
            .with(eq("order_received"), eq("ORDER-123"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(()));
        
        inventory.expect_check_availability()
            .with(eq("WIDGET-1"), eq(2))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(true));
        
        inventory.expect_check_availability()
            .with(eq("GADGET-1"), eq(1))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(true));
        
        inventory.expect_reserve()
            .with(eq("WIDGET-1"), eq(2))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok("RES-1".to_string()));
        
        inventory.expect_reserve()
            .with(eq("GADGET-1"), eq(1))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok("RES-2".to_string()));
        
        payment.expect_charge()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(PaymentResult {
                transaction_id: "TXN-123".to_string(),
                amount: 99.99,
            }));
        
        shipping.expect_create_label()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(ShippingLabel {
                tracking_number: "TRACK-123".to_string(),
                estimated_delivery: "2023-12-01".to_string(),
            }));
        
        notification.expect_send_confirmation()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(()));
        
        audit.expect_log_event()
            .with(eq("order_completed"), eq("ORDER-123"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(()));
        
        let service = OrderProcessingService {
            inventory: Box::new(inventory),
            payment: Box::new(payment),
            shipping: Box::new(shipping),
            notification: Box::new(notification),
            audit: Box::new(audit),
        };
        
        let order = create_test_order();
        let result = service.process_order(order).await;
        
        assert!(result.is_ok());
        let confirmation = result.unwrap();
        assert_eq!(confirmation.order_id, "ORDER-123");
        assert_eq!(confirmation.payment_id, "TXN-123");
        assert_eq!(confirmation.tracking_number, "TRACK-123");
    }
    
    #[tokio::test]
    async fn test_inventory_failure_rollback() {
        let mut inventory = MockInventoryService::new();
        let mut audit = MockAuditService::new();
        
        audit.expect_log_event()
            .with(eq("order_received"), eq("ORDER-123"))
            .returning(|_, _| Ok(()));
        
        // First item available, second item not available
        inventory.expect_check_availability()
            .with(eq("WIDGET-1"), eq(2))
            .returning(|_, _| Ok(true));
        
        inventory.expect_check_availability()
            .with(eq("GADGET-1"), eq(1))
            .returning(|_, _| Ok(false));  // Not available!
        
        audit.expect_log_event()
            .with(eq("inventory_insufficient"), eq("ORDER-123"))
            .returning(|_, _| Ok(()));
        
        let service = OrderProcessingService {
            inventory: Box::new(inventory),
            payment: Box::new(MockPaymentService::new()),
            shipping: Box::new(MockShippingService::new()),
            notification: Box::new(MockNotificationService::new()),
            audit: Box::new(audit),
        };
        
        let order = create_test_order();
        let result = service.process_order(order).await;
        
        assert!(matches!(result, Err(ProcessingError::InsufficientInventory(_))));
    }
    
    #[tokio::test]
    async fn test_payment_failure_rollback() {
        let mut inventory = MockInventoryService::new();
        let mut payment = MockPaymentService::new();
        let mut audit = MockAuditService::new();
        
        let mut seq = Sequence::new();
        
        // Setup successful inventory checks and reservations
        audit.expect_log_event()
            .with(eq("order_received"), eq("ORDER-123"))
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(()));
        
        inventory.expect_check_availability()
            .returning(|_, _| Ok(true))
            .times(2);
        
        inventory.expect_reserve()
            .with(eq("WIDGET-1"), eq(2))
            .in_sequence(&mut seq)
            .returning(|_, _| Ok("RES-1".to_string()));
        
        inventory.expect_reserve()
            .with(eq("GADGET-1"), eq(1))
            .in_sequence(&mut seq)
            .returning(|_, _| Ok("RES-2".to_string()));
        
        // Payment fails
        payment.expect_charge()
            .in_sequence(&mut seq)
            .returning(|_, _| Err(PaymentError::CardDeclined));
        
        // Rollback reservations
        inventory.expect_release_reservation()
            .with(eq("RES-1"))
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));
        
        inventory.expect_release_reservation()
            .with(eq("RES-2"))
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));
        
        audit.expect_log_event()
            .with(eq("payment_failed"), eq("ORDER-123"))
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(()));
        
        let service = OrderProcessingService {
            inventory: Box::new(inventory),
            payment: Box::new(payment),
            shipping: Box::new(MockShippingService::new()),
            notification: Box::new(MockNotificationService::new()),
            audit: Box::new(audit),
        };
        
        let order = create_test_order();
        let result = service.process_order(order).await;
        
        assert!(matches!(result, Err(ProcessingError::PaymentFailed(_))));
    }
    
    #[tokio::test]
    async fn test_shipping_failure_rollback() {
        let mut inventory = MockInventoryService::new();
        let mut payment = MockPaymentService::new();
        let mut shipping = MockShippingService::new();
        let mut audit = MockAuditService::new();
        
        // Setup successful flow until shipping
        audit.expect_log_event().returning(|_, _| Ok(())).times(2);
        inventory.expect_check_availability().returning(|_, _| Ok(true)).times(2);
        inventory.expect_reserve().returning(|_, _| Ok("RES-1".to_string()));
        inventory.expect_reserve().returning(|_, _| Ok("RES-2".to_string()));
        
        payment.expect_charge()
            .returning(|_, _| Ok(PaymentResult {
                transaction_id: "TXN-123".to_string(),
                amount: 99.99,
            }));
        
        // Shipping fails
        shipping.expect_create_label()
            .returning(|_, _| Err(ShippingError::AddressInvalid));
        
        // Rollback payment
        payment.expect_refund()
            .with(eq("TXN-123"))
            .returning(|_| Ok(()));
        
        // Rollback reservations
        inventory.expect_release_reservation().returning(|_| Ok(())).times(2);
        
        let service = OrderProcessingService {
            inventory: Box::new(inventory),
            payment: Box::new(payment),
            shipping: Box::new(shipping),
            notification: Box::new(MockNotificationService::new()),
            audit: Box::new(audit),
        };
        
        let order = create_test_order();
        let result = service.process_order(order).await;
        
        assert!(matches!(result, Err(ProcessingError::ShippingFailed(_))));
    }
    
    #[tokio::test]
    async fn test_notification_failure_doesnt_fail_order() {
        // Setup successful flow except notification fails
        let mut notification = MockNotificationService::new();
        
        // All other services succeed (shortened setup for brevity)
        let service = create_successful_service_except_notification();
        
        notification.expect_send_confirmation()
            .returning(|_, _| Err(NotificationError::EmailServiceDown));
        
        let order = create_test_order();
        let result = service.process_order(order).await;
        
        // Order should still succeed even if notification fails
        assert!(result.is_ok());
    }
}
```

## 🎉 Congratulations!

You've mastered comprehensive test coverage analysis! Key takeaways:
- Use coverage tools to identify untested code paths
- Systematically test all error scenarios
- Test edge cases and boundary values
- Verify complex conditional logic combinations
- Ensure all state transitions are covered
- Test integration failure and rollback scenarios
- Remember: 100% coverage doesn't guarantee bug-free code, but it's a great foundation

Your testing skills are now comprehensive enough to catch bugs before they reach production!