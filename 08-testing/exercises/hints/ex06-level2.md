# Exercise 06 - Property Tests: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Identifying Properties to Test
```rust
// Traditional test: specific example
#[test]
fn test_sort_example() {
    assert_eq!(sort(vec![3, 1, 2]), vec![1, 2, 3]);
}

// Property test: general property
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_sort_preserves_length(v: Vec<i32>) {
        let original_len = v.len();
        let sorted = sort(v);
        assert_eq!(sorted.len(), original_len);
    }
    
    #[test]
    fn test_sort_is_ordered(v: Vec<i32>) {
        let sorted = sort(v);
        for window in sorted.windows(2) {
            assert!(window[0] <= window[1]);
        }
    }
}
```

Common properties to test:
- Roundtrip: `decode(encode(x)) == x`
- Idempotence: `f(f(x)) == f(x)`
- Commutativity: `f(a, b) == f(b, a)`
- Associativity: `f(f(a, b), c) == f(a, f(b, c))`

### Checkpoint 2: Setting Up Proptest
```toml
# In Cargo.toml:
[dependencies]
# Your regular dependencies

[dev-dependencies]
proptest = "1.0"
```

Then in your test file:
```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn my_property_test(x: i32) {
            // Test properties with random x values
        }
    }
}
```

### Checkpoint 3: Creating Custom Strategies
```rust
use proptest::prelude::*;

// Basic strategies
proptest! {
    #[test]
    fn test_with_ranges(
        x in 0..100i32,                    // Range
        y in -50..=50i32,                  // Inclusive range
        name in "[a-z]+",                  // Regex string
        opt in prop::option::of(0..10),    // Option<i32>
        vec in prop::collection::vec(0..5, 0..10)  // Vec with length 0-10
    ) {
        // Test with these values
    }
}

// Custom type strategy
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
}

// Define how to generate Users
fn arb_user() -> impl Strategy<Value = User> {
    ("[a-zA-Z]+", 18..100u8).prop_map(|(name, age)| {
        User { name, age }
    })
}

proptest! {
    #[test]
    fn test_user_validation(user in arb_user()) {
        assert!(validate_user(&user).is_ok());
    }
}
```

### Checkpoint 4: Configuring Test Runs
```rust
use proptest::prelude::*;

// Configure number of test cases
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn expensive_test(x: i32) {
        // This will run 1000 times instead of default 256
    }
}

// Or configure globally in proptest.toml:
// cases = 100
// max_shrink_iters = 50

// For CI, use environment variable:
// PROPTEST_CASES=10000 cargo test
```

### Checkpoint 5: Implementing Shrinking
```rust
use proptest::prelude::*;

// Proptest automatically shrinks built-in types
// For custom types, derive Arbitrary:

#[derive(Debug, Clone, Arbitrary)]
struct Point {
    #[proptest(strategy = "-100..100")]
    x: i32,
    #[proptest(strategy = "-100..100")]  
    y: i32,
}

// Or implement custom shrinking:
fn arb_custom_type() -> impl Strategy<Value = MyType> {
    (0..100, ".*").prop_map(|(n, s)| MyType { n, s })
        .prop_filter("non-empty string", |t| !t.s.is_empty())
}

// When test fails, proptest will try smaller inputs
proptest! {
    #[test]
    fn test_will_find_minimal_case(points: Vec<Point>) {
        // If this fails, proptest shrinks to minimal failing case
        assert!(process_points(&points).len() < 1000);
    }
}
```

### Checkpoint 6: Stronger Properties
```rust
// Weak property (doesn't catch much):
proptest! {
    #[test]
    fn test_parse_doesnt_crash(s: String) {
        let _ = parse_config(&s);  // Just checking it doesn't panic
    }
}

// Strong properties:
proptest! {
    #[test]
    fn test_parse_validate_roundtrip(config: ValidConfig) {
        let serialized = config.to_string();
        let parsed = parse_config(&serialized).unwrap();
        assert_eq!(parsed, config);
    }
    
    #[test]
    fn test_compression_properties(data: Vec<u8>) {
        let compressed = compress(&data);
        let decompressed = decompress(&compressed).unwrap();
        
        // Property 1: Lossless
        assert_eq!(data, decompressed);
        
        // Property 2: Actually compresses repeating data
        if has_repetition(&data) {
            assert!(compressed.len() < data.len());
        }
    }
    
    #[test]
    fn test_sort_is_permutation(mut original: Vec<i32>) {
        let sorted = sort(original.clone());
        
        // Same elements, different order
        original.sort_unstable();
        let mut sorted_clone = sorted.clone();
        sorted_clone.sort_unstable();
        assert_eq!(original, sorted_clone);
    }
}
```

## 🎯 Pattern Recognition

Key insights:
- Property tests verify invariants, not specific examples
- Strategies generate random test data within constraints
- Shrinking finds minimal failing cases automatically
- Strong properties catch more bugs than weak ones

Ready for Level 3 if you need complete solutions!