# Exercise 06 - Property Tests: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete Property Identification
```rust
use proptest::prelude::*;

// String manipulation properties
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

proptest! {
    #[test]
    fn test_reverse_twice_is_identity(s: String) {
        let reversed_twice = reverse_string(&reverse_string(&s));
        prop_assert_eq!(reversed_twice, s);
    }
    
    #[test]
    fn test_reverse_preserves_length(s: String) {
        let reversed = reverse_string(&s);
        prop_assert_eq!(reversed.len(), s.len());
    }
    
    #[test]
    fn test_reverse_preserves_chars(s: String) {
        let reversed = reverse_string(&s);
        let mut original_chars: Vec<char> = s.chars().collect();
        let mut reversed_chars: Vec<char> = reversed.chars().collect();
        original_chars.sort();
        reversed_chars.sort();
        prop_assert_eq!(original_chars, reversed_chars);
    }
}

// Arithmetic properties
pub fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().sum::<f64>() / numbers.len() as f64)
    }
}

proptest! {
    #[test]
    fn test_average_is_bounded(numbers: Vec<f64>) {
        prop_assume!(!numbers.is_empty());
        prop_assume!(numbers.iter().all(|n| n.is_finite()));
        
        let avg = calculate_average(&numbers).unwrap();
        let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        prop_assert!(avg >= min);
        prop_assert!(avg <= max);
    }
}
```

### Checkpoint 2: Complete Proptest Setup
```rust
// In Cargo.toml:
/*
[dev-dependencies]
proptest = "1.0"
proptest-derive = "0.3"  # For deriving Arbitrary
*/

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;
    
    // Import common strategies
    use proptest::collection::{vec, hash_map};
    use proptest::option;
    use proptest::result;
    use proptest::string::string_regex;
    
    proptest! {
        #[test]
        fn test_json_parser(
            json in string_regex(r#"\{"[a-z]+": (true|false|[0-9]+)\}"#).unwrap()
        ) {
            let parsed = parse_json(&json);
            prop_assert!(parsed.is_ok(), "Failed to parse: {}", json);
        }
    }
}
```

### Checkpoint 3: Complete Custom Strategy Implementation
```rust
use proptest::prelude::*;
use proptest_derive::Arbitrary;

// Email type with custom generation
#[derive(Debug, Clone, PartialEq)]
struct Email(String);

fn arb_email() -> impl Strategy<Value = Email> {
    (
        "[a-z]{1,10}",           // username
        "[a-z]{1,10}",           // domain
        "[a-z]{2,4}"             // tld
    ).prop_map(|(user, domain, tld)| {
        Email(format!("{}@{}.{}", user, domain, tld))
    })
}

// Complex type with derive
#[derive(Debug, Clone, PartialEq, Arbitrary)]
struct Order {
    #[proptest(strategy = "1..=1000000u64")]
    id: u64,
    
    #[proptest(strategy = "arb_email()")]
    customer_email: Email,
    
    #[proptest(strategy = "vec(arb_order_item(), 1..=10)")]
    items: Vec<OrderItem>,
    
    #[proptest(strategy = "option::of(0.0..=100.0)")]
    discount_percentage: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
struct OrderItem {
    name: String,
    quantity: u32,
    price: f64,
}

fn arb_order_item() -> impl Strategy<Value = OrderItem> {
    (
        "[A-Za-z ]{5,20}",      // product name
        1..=100u32,             // quantity  
        0.01..=1000.0f64        // price
    ).prop_map(|(name, quantity, price)| {
        OrderItem { name, quantity, price }
    })
}

proptest! {
    #[test]
    fn test_order_total_calculation(order in any::<Order>()) {
        let total = calculate_order_total(&order);
        
        // Total should be sum of items minus discount
        let subtotal: f64 = order.items.iter()
            .map(|item| item.quantity as f64 * item.price)
            .sum();
            
        let expected = match order.discount_percentage {
            Some(discount) => subtotal * (1.0 - discount / 100.0),
            None => subtotal,
        };
        
        prop_assert!((total - expected).abs() < 0.01);
    }
}
```

### Checkpoint 4: Complete Test Configuration
```rust
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

// Method 1: Attribute configuration
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 500,
        max_shrink_iters: 100,
        failure_persistence: Some(Box::new(proptest::test_runner::FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    })]
    
    #[test]
    fn test_with_custom_config(data: Vec<u8>) {
        process_data(&data);
    }
}

// Method 2: Environment-based configuration
#[test]
fn test_with_env_config() {
    // Set PROPTEST_CASES=1000 to run more cases
    let config = Config::default();
    let mut runner = TestRunner::new(config);
    
    runner.run(&any::<String>(), |s| {
        prop_assert!(validate_string(&s).is_ok());
        Ok(())
    }).unwrap();
}

// Method 3: proptest.toml file
/*
# proptest.toml in project root
cases = 256
max_shrink_iters = 500
max_shrink_time = 60000  # milliseconds
max_global_rejects = 65536
fork = false
timeout = 0
max_threads = 0
failure_persistence = "WithSource"
source_file = "regressions"
*/

// Method 4: Quick tests for CI
#[cfg(ci)]
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    
    #[test]
    fn quick_test_for_ci(x: i32) {
        // Runs only 10 cases in CI
    }
}
```

### Checkpoint 5: Complete Shrinking Implementation
```rust
use proptest::prelude::*;
use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::TestRunner;

// Custom type that needs shrinking
#[derive(Debug, Clone, PartialEq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

// Implement custom strategy with shrinking
fn arb_version() -> impl Strategy<Value = Version> {
    (0..=10u32, 0..=20u32, 0..=100u32)
        .prop_map(|(major, minor, patch)| Version { major, minor, patch })
}

// Example showing shrinking in action
fn buggy_version_compare(v1: &Version, v2: &Version) -> bool {
    // Bug: doesn't handle patch version correctly
    if v1.major != v2.major {
        v1.major < v2.major
    } else if v1.minor != v2.minor {
        v1.minor < v2.minor  
    } else {
        false  // BUG: should compare patch!
    }
}

proptest! {
    #[test]
    fn test_version_ordering_is_transitive(
        v1 in arb_version(),
        v2 in arb_version(),
        v3 in arb_version()
    ) {
        prop_assume!(v1 != v2 && v2 != v3 && v1 != v3);
        
        if buggy_version_compare(&v1, &v2) && buggy_version_compare(&v2, &v3) {
            prop_assert!(
                buggy_version_compare(&v1, &v3),
                "Transitivity failed: {:?} < {:?} < {:?}", v1, v2, v3
            );
        }
    }
}

// When this fails, proptest will shrink to minimal case like:
// v1 = Version { major: 0, minor: 0, patch: 1 }
// v2 = Version { major: 0, minor: 0, patch: 2 }
// v3 = Version { major: 0, minor: 0, patch: 3 }
```

### Checkpoint 6: Complete Strong Properties
```rust
use proptest::prelude::*;

// Serialization round-trip with validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
    enabled: bool,
    limits: Vec<u32>,
}

fn arb_valid_config() -> impl Strategy<Value = Config> {
    (
        "[a-zA-Z_][a-zA-Z0-9_]{0,19}",  // Valid identifier
        1024..=65535u16,                  // Non-privileged ports
        any::<bool>(),
        vec(1..=1000u32, 0..=5)          // Reasonable limits
    ).prop_map(|(name, port, enabled, limits)| {
        Config { name, port, enabled, limits }
    })
}

proptest! {
    #[test]
    fn test_config_serialization_roundtrip(config in arb_valid_config()) {
        // Property 1: Serialization round-trip
        let json = serde_json::to_string(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(config, parsed);
        
        // Property 2: JSON is valid
        prop_assert!(json.starts_with('{') && json.ends_with('}'));
        prop_assert!(json.contains(&format!("\"name\":\"{}\"", config.name)));
        
        // Property 3: Validation still passes
        prop_assert!(validate_config(&parsed).is_ok());
    }
    
    #[test]
    fn test_data_structure_invariants(mut operations: Vec<DataOperation>) {
        let mut data_store = DataStore::new();
        
        for op in operations {
            match op {
                DataOperation::Insert(key, value) => {
                    data_store.insert(key.clone(), value);
                    // Invariant 1: Key should exist after insert
                    prop_assert!(data_store.contains(&key));
                },
                DataOperation::Remove(key) => {
                    data_store.remove(&key);
                    // Invariant 2: Key should not exist after remove
                    prop_assert!(!data_store.contains(&key));
                },
                DataOperation::Clear => {
                    data_store.clear();
                    // Invariant 3: Store should be empty after clear
                    prop_assert_eq!(data_store.len(), 0);
                }
            }
            
            // Global invariant: Size is consistent
            prop_assert_eq!(data_store.len(), data_store.keys().count());
        }
    }
    
    #[test]
    fn test_parser_security_properties(input: String) {
        let start_time = std::time::Instant::now();
        let result = parse_untrusted_input(&input);
        let elapsed = start_time.elapsed();
        
        // Property 1: Parser completes in reasonable time (no DoS)
        prop_assert!(elapsed < std::time::Duration::from_secs(1));
        
        // Property 2: No panic on any input
        match result {
            Ok(parsed) => {
                // Property 3: Output is sanitized
                prop_assert!(!contains_script_tags(&parsed));
                prop_assert!(is_valid_utf8(&parsed));
            },
            Err(_) => {
                // Property 4: Error messages don't leak internals
                prop_assert!(!result.unwrap_err().contains("stack"));
                prop_assert!(!result.unwrap_err().contains("panic"));
            }
        }
    }
}
```

## 🎉 Congratulations!

You've mastered property-based testing! Key takeaways:
- Properties test invariants that hold for all inputs
- Custom strategies generate domain-specific test data
- Shrinking finds minimal failing examples automatically
- Configuration allows balancing thoroughness vs speed
- Strong properties catch bugs that example-based tests miss
- Think like a mathematician: what must always be true?

Property testing is like having millions of QA testers working for you!