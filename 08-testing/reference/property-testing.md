# Property-Based Testing in Rust 🔄

*A comprehensive guide to property-based testing with proptest and beyond*

## 🚀 Quick Reference

### Essential Property Test Patterns (Copy & Paste Ready)

```rust
// 1. Basic property test
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_reverse_property(input: Vec<i32>) {
        let reversed = reverse(input.clone());
        let double_reversed = reverse(reversed);
        prop_assert_eq!(input, double_reversed);
    }
}

// 2. Roundtrip property (most common)
proptest! {
    #[test]
    fn test_serialization_roundtrip(data: MyStruct) {
        let serialized = serialize(&data);
        let deserialized = deserialize(&serialized).unwrap();
        prop_assert_eq!(data, deserialized);
    }
}

// 3. Custom strategies
fn arb_email() -> impl Strategy<Value = String> {
    "[a-z]{3,10}@[a-z]{3,10}\\.[a-z]{2,4}"
}

proptest! {
    #[test]
    fn test_with_custom_strategy(email in arb_email()) {
        prop_assert!(validate_email(&email));
    }
}

// 4. Invariant testing
proptest! {
    #[test]
    fn test_sort_preserves_length(input: Vec<i32>) {
        let sorted = sort_function(input.clone());
        prop_assert_eq!(input.len(), sorted.len());
    }
}
```

### Emergency Property Test Setup

| **Property Type** | **Use Case** | **Pattern** |
|-------------------|-------------|------------|
| Roundtrip | Serialize/deserialize, encode/decode | `prop_assert_eq!(original, decode(encode(original)))` |
| Idempotent | Normalization, formatting | `prop_assert_eq!(f(x), f(f(x)))` |
| Invariant | Data structure properties | `prop_assert!(property_holds_after_operation)` |
| Commutative | Order-independent operations | `prop_assert_eq!(f(a,b), f(b,a))` |
| Oracle | Compare with known good impl | `prop_assert_eq!(my_impl(x), stdlib_impl(x))` |

### Cargo.toml Setup

```toml
[dev-dependencies]
proptest = "1.0"
proptest-derive = "0.3"    # For #[derive(Arbitrary)]
```

### Quick Strategy Patterns

```rust
// Built-in strategies
proptest! {
    #[test]
    fn test_with_built_ins(
        s: String,                    // Any string
        i: 1..100i32,                // Range
        v: Vec<i32>,                 // Any vector
        opt: Option<String>,         // Optional values
    ) { /* test */ }
}

// Custom strategies
fn arb_positive() -> impl Strategy<Value = i32> { 1..1000 }
fn arb_user() -> impl Strategy<Value = User> {
    (arb_name(), arb_email(), 18..100u32)
        .prop_map(|(name, email, age)| User { name, email, age })
}

// Conditional strategies
fn arb_conditional() -> impl Strategy<Value = (String, u32)> {
    "[a-z]{1,10}".prop_flat_map(|name| {
        let age_range = if name.len() > 5 { 18..65u32 } else { 0..18u32 };
        (Just(name), age_range)
    })
}
```

---

## Introduction

Property-based testing is a powerful testing methodology that automatically generates test cases to verify that your code satisfies certain properties. Instead of writing specific examples, you define properties that should always hold true, and the testing framework generates hundreds or thousands of test cases to verify them.

## Core Concepts

### What is Property-Based Testing?

```rust
// Traditional example-based test
#[test]
fn test_reverse_specific_case() {
    let input = vec![1, 2, 3];
    let result = reverse(input.clone());
    assert_eq!(result, vec![3, 2, 1]);
}

// Property-based test
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_reverse_property(input: Vec<i32>) {
        let reversed = reverse(input.clone());
        let double_reversed = reverse(reversed);
        
        // Property: reversing twice gives original
        prop_assert_eq!(input, double_reversed);
    }
}
```

### Properties vs Examples

| Example-Based Testing | Property-Based Testing |
|----------------------|------------------------|
| Tests specific inputs | Tests general rules |
| Limited coverage | Extensive coverage |
| Manual case selection | Automatic generation |
| Easy to write | Requires thinking about invariants |
| Focused debugging | May find edge cases you missed |

## Setting Up proptest

### Basic Setup

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.0"
proptest-derive = "0.3"  # For deriving Arbitrary
```

```rust
// Basic proptest usage
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_my_property(x: i32, y: i32) {
        // Your property test here
        prop_assert!(my_function(x, y) >= 0);
    }
}
```

### Configuration

```rust
// Custom configuration
proptest! {
    // Run 1000 test cases instead of default 256
    #![proptest_config(ProptestConfig::with_cases(1000))]
    
    #[test]
    fn test_with_more_cases(input: String) {
        prop_assert!(validate_string(&input).is_ok());
    }
}

// Global configuration in proptest.toml
/*
cases = 512
max_shrink_iters = 1000
max_shrink_time = 30000  # milliseconds
*/
```

## Common Property Patterns

### 1. Roundtrip Properties

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

proptest! {
    #[test]
    fn test_json_serialization_roundtrip(
        name in "[a-zA-Z ]{1,50}",
        age in 0u32..150,
        email in "[a-z0-9]+@[a-z0-9]+\\.[a-z]{2,5}"
    ) {
        let user = User { name, age, email };
        
        // Serialize to JSON
        let json = serde_json::to_string(&user).unwrap();
        
        // Deserialize back
        let deserialized: User = serde_json::from_str(&json).unwrap();
        
        // Property: roundtrip should preserve data
        prop_assert_eq!(user, deserialized);
    }
}

proptest! {
    #[test]
    fn test_base64_encoding_roundtrip(data: Vec<u8>) {
        let encoded = base64::encode(&data);
        let decoded = base64::decode(&encoded).unwrap();
        
        // Property: encoding then decoding gives original data
        prop_assert_eq!(data, decoded);
    }
}
```

### 2. Idempotent Properties

```rust
fn normalize_string(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

proptest! {
    #[test]
    fn test_string_normalization_idempotent(input: String) {
        let normalized_once = normalize_string(&input);
        let normalized_twice = normalize_string(&normalized_once);
        
        // Property: normalizing is idempotent
        prop_assert_eq!(normalized_once, normalized_twice);
    }
}

fn sort_vector(mut vec: Vec<i32>) -> Vec<i32> {
    vec.sort();
    vec
}

proptest! {
    #[test]
    fn test_sort_idempotent(input: Vec<i32>) {
        let sorted_once = sort_vector(input.clone());
        let sorted_twice = sort_vector(sorted_once.clone());
        
        // Property: sorting is idempotent
        prop_assert_eq!(sorted_once, sorted_twice);
    }
}
```

### 3. Invariant Properties

```rust
proptest! {
    #[test]
    fn test_sort_preserves_length(input: Vec<i32>) {
        let sorted = sort_vector(input.clone());
        
        // Property: sorting preserves length
        prop_assert_eq!(input.len(), sorted.len());
    }
    
    #[test]
    fn test_sort_preserves_elements(input: Vec<i32>) {
        let mut original = input.clone();
        let sorted = sort_vector(input);
        
        original.sort();
        
        // Property: sorting preserves all elements
        prop_assert_eq!(original, sorted);
    }
    
    #[test]
    fn test_sort_is_ordered(input: Vec<i32>) {
        let sorted = sort_vector(input);
        
        // Property: result is ordered
        for window in sorted.windows(2) {
            prop_assert!(window[0] <= window[1]);
        }
    }
}
```

### 4. Commutativity Properties

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

proptest! {
    #[test]
    fn test_addition_commutative(a: i32, b: i32) {
        // Property: a + b = b + a
        prop_assert_eq!(add(a, b), add(b, a));
    }
    
    #[test]
    fn test_multiplication_commutative(a: i32, b: i32) {
        // Property: a * b = b * a
        prop_assert_eq!(multiply(a, b), multiply(b, a));
    }
}

// Set operations
use std::collections::HashSet;

fn set_union(a: &HashSet<i32>, b: &HashSet<i32>) -> HashSet<i32> {
    a.union(b).cloned().collect()
}

proptest! {
    #[test]
    fn test_set_union_commutative(
        a: Vec<i32>,
        b: Vec<i32>
    ) {
        let set_a: HashSet<i32> = a.into_iter().collect();
        let set_b: HashSet<i32> = b.into_iter().collect();
        
        let union_ab = set_union(&set_a, &set_b);
        let union_ba = set_union(&set_b, &set_a);
        
        // Property: A ∪ B = B ∪ A
        prop_assert_eq!(union_ab, union_ba);
    }
}
```

### 5. Associativity Properties

```rust
proptest! {
    #[test]
    fn test_addition_associative(a: i32, b: i32, c: i32) {
        // Property: (a + b) + c = a + (b + c)
        prop_assert_eq!(add(add(a, b), c), add(a, add(b, c)));
    }
    
    #[test]
    fn test_string_concatenation_associative(
        a: String,
        b: String, 
        c: String
    ) {
        let left = format!("{}{}", format!("{}{}", a, b), c);
        let right = format!("{}{}", a, format!("{}{}", b, c));
        
        // Property: (a + b) + c = a + (b + c)
        prop_assert_eq!(left, right);
    }
}
```

## Custom Strategies

### Basic Custom Strategies

```rust
use proptest::strategy::{Strategy, Just};
use proptest::collection::vec;

// Custom strategy for valid email addresses
fn arb_email() -> impl Strategy<Value = String> {
    (
        "[a-z]{3,10}",      // username
        "[a-z]{3,10}",      // domain
        "[a-z]{2,4}"        // tld
    ).prop_map(|(user, domain, tld)| {
        format!("{}@{}.{}", user, domain, tld)
    })
}

// Custom strategy for positive integers
fn arb_positive_int() -> impl Strategy<Value = i32> {
    1..=1000i32
}

// Custom strategy for non-empty strings
fn arb_non_empty_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9]{1,50}"
}

proptest! {
    #[test]
    fn test_with_custom_strategies(
        email in arb_email(),
        count in arb_positive_int(),
        name in arb_non_empty_string()
    ) {
        let user = User { name, email, age: count as u32 };
        prop_assert!(validate_user(&user).is_ok());
    }
}
```

### Complex Custom Types

```rust
use proptest_derive::Arbitrary;

#[derive(Debug, Clone, PartialEq, Arbitrary)]
struct Point {
    #[proptest(strategy = "-100.0..100.0")]
    x: f64,
    #[proptest(strategy = "-100.0..100.0")]
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

// Custom strategy for valid rectangles
fn arb_rectangle() -> impl Strategy<Value = Rectangle> {
    (
        any::<Point>(),
        0.1..100.0f64,  // positive width
        0.1..100.0f64   // positive height
    ).prop_map(|(top_left, width, height)| {
        Rectangle { top_left, width, height }
    })
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

proptest! {
    #[test]
    fn test_rectangle_area_positive(rect in arb_rectangle()) {
        // Property: area is always positive for valid rectangles
        prop_assert!(rect.area() > 0.0);
    }
    
    #[test]
    fn test_rectangle_perimeter_positive(rect in arb_rectangle()) {
        // Property: perimeter is always positive
        prop_assert!(rect.perimeter() > 0.0);
    }
    
    #[test]
    fn test_scaling_rectangle(
        rect in arb_rectangle(),
        scale in 0.1..10.0f64
    ) {
        let original_area = rect.area();
        let scaled_rect = Rectangle {
            top_left: rect.top_left,
            width: rect.width * scale,
            height: rect.height * scale,
        };
        let scaled_area = scaled_rect.area();
        
        // Property: scaling by factor s multiplies area by s²
        let expected_area = original_area * scale * scale;
        prop_assert!((scaled_area - expected_area).abs() < 0.001);
    }
}
```

### Conditional Strategies

```rust
use proptest::strategy::Strategy;

// Strategy that depends on other values
fn arb_user_and_valid_age() -> impl Strategy<Value = (String, u32)> {
    arb_non_empty_string().prop_flat_map(|name| {
        let age_range = if name.len() > 10 {
            18..65u32  // Adults for long names
        } else {
            0..18u32   // Minors for short names
        };
        (Just(name), age_range)
    })
}

// Filtered strategies
proptest! {
    #[test]
    fn test_with_filtered_input(
        input in "[a-z]{1,100}"
            .prop_filter("Must contain vowel", |s| {
                s.chars().any(|c| "aeiou".contains(c))
            })
    ) {
        prop_assert!(has_vowel(&input));
    }
}

// Weighted choices
fn arb_error_type() -> impl Strategy<Value = ErrorType> {
    prop_oneof![
        9 => Just(ErrorType::Network),     // 90% network errors
        1 => Just(ErrorType::Database),    // 10% database errors
    ]
}
```

## Advanced Property Testing Patterns

### 1. State Machine Testing

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
    
    fn size(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Set(String, String),
    Get(String),
    Delete(String),
}

fn arb_operation() -> impl Strategy<Value = Operation> {
    prop_oneof![
        (arb_key(), arb_value()).prop_map(|(k, v)| Operation::Set(k, v)),
        arb_key().prop_map(Operation::Get),
        arb_key().prop_map(Operation::Delete),
    ]
}

fn arb_key() -> impl Strategy<Value = String> {
    "[a-z]{1,5}"
}

fn arb_value() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9]{1,10}"
}

proptest! {
    #[test]
    fn test_kv_store_properties(operations: Vec<Operation>) {
        let mut store = KeyValueStore::new();
        let mut reference_data = HashMap::new();
        
        for op in operations {
            match op {
                Operation::Set(key, value) => {
                    store.set(key.clone(), value.clone());
                    reference_data.insert(key, value);
                }
                Operation::Get(key) => {
                    let store_result = store.get(&key);
                    let reference_result = reference_data.get(&key);
                    prop_assert_eq!(store_result, reference_result);
                }
                Operation::Delete(key) => {
                    let store_deleted = store.delete(&key);
                    let reference_deleted = reference_data.remove(&key).is_some();
                    prop_assert_eq!(store_deleted, reference_deleted);
                }
            }
            
            // Invariant: size matches reference
            prop_assert_eq!(store.size(), reference_data.len());
        }
    }
}
```

### 2. Metamorphic Testing

```rust
// Testing properties by transformation
fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let len = vec.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
    vec
}

proptest! {
    #[test]
    fn test_sort_metamorphic_properties(input: Vec<i32>) {
        let sorted = bubble_sort(input.clone());
        
        // Metamorphic property 1: sorting twice gives same result
        let sorted_twice = bubble_sort(sorted.clone());
        prop_assert_eq!(sorted, sorted_twice);
        
        // Metamorphic property 2: sorting reversed input gives same result
        let mut reversed_input = input.clone();
        reversed_input.reverse();
        let sorted_reversed = bubble_sort(reversed_input);
        prop_assert_eq!(sorted, sorted_reversed);
        
        // Metamorphic property 3: adding element and sorting
        // should contain all original elements plus the new one
        let new_element = 42;
        let mut extended_input = input.clone();
        extended_input.push(new_element);
        let sorted_extended = bubble_sort(extended_input);
        
        // Check that all original elements are present
        for &element in &input {
            prop_assert!(sorted_extended.contains(&element));
        }
        
        // Check that new element is present
        prop_assert!(sorted_extended.contains(&new_element));
    }
}
```

### 3. Oracle-Based Testing

```rust
// Use a simpler, obviously correct implementation as oracle
fn simple_sort(mut vec: Vec<i32>) -> Vec<i32> {
    // Use standard library as oracle
    vec.sort();
    vec
}

proptest! {
    #[test]
    fn test_bubble_sort_against_oracle(input: Vec<i32>) {
        let bubble_result = bubble_sort(input.clone());
        let oracle_result = simple_sort(input);
        
        // Property: our implementation should match the oracle
        prop_assert_eq!(bubble_result, oracle_result);
    }
}

// Cross-validation with multiple implementations
fn quick_sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec;
    }
    
    let pivot = vec.len() / 2;
    let pivot_value = vec[pivot];
    
    let less: Vec<i32> = vec.iter().filter(|&&x| x < pivot_value).cloned().collect();
    let equal: Vec<i32> = vec.iter().filter(|&&x| x == pivot_value).cloned().collect();
    let greater: Vec<i32> = vec.iter().filter(|&&x| x > pivot_value).cloned().collect();
    
    let mut result = quick_sort(less);
    result.extend(equal);
    result.extend(quick_sort(greater));
    result
}

proptest! {
    #[test]
    fn test_multiple_sort_implementations_agree(input: Vec<i32>) {
        let bubble_result = bubble_sort(input.clone());
        let quick_result = quick_sort(input.clone());
        let std_result = simple_sort(input);
        
        // Property: all implementations should produce same result
        prop_assert_eq!(bubble_result, quick_result);
        prop_assert_eq!(quick_result, std_result);
    }
}
```

## Regression Testing with proptest

### Failure Persistence

```rust
// proptest automatically saves failing cases
// Configuration to persist failures
use proptest::test_runner::{Config, FileFailurePersistence};

proptest! {
    #![proptest_config(Config {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        .. Config::default()
    })]
    
    #[test]
    fn test_with_regression_protection(input: String) {
        // If this test fails, the input will be saved
        // and re-tested in future runs
        prop_assert!(complex_function(&input).is_ok());
    }
}
```

### Custom Failure Analysis

```rust
use proptest::test_runner::TestCaseResult;

fn custom_property_check(input: &[i32]) -> TestCaseResult {
    let result = risky_function(input);
    
    match result {
        Ok(output) => {
            // Additional checks beyond just "doesn't panic"
            if output.len() != input.len() {
                return Err(proptest::test_runner::TestCaseError::Fail(
                    format!("Length mismatch: input={}, output={}", 
                           input.len(), output.len()).into()
                ));
            }
            
            // Check output invariants
            for (i, &value) in output.iter().enumerate() {
                if value < 0 {
                    return Err(proptest::test_runner::TestCaseError::Fail(
                        format!("Negative value at index {}: {}", i, value).into()
                    ));
                }
            }
            
            Ok(())
        }
        Err(e) => {
            Err(proptest::test_runner::TestCaseError::Fail(
                format!("Function failed: {}", e).into()
            ))
        }
    }
}

proptest! {
    #[test]
    fn test_with_custom_analysis(input: Vec<i32>) {
        custom_property_check(&input)?;
    }
}
```

## Performance Property Testing

### Complexity Properties

```rust
use std::time::Instant;

proptest! {
    #[test]
    fn test_sorting_performance_properties(input: Vec<i32>) {
        // Skip very large inputs to avoid test timeouts
        prop_assume!(input.len() <= 10000);
        
        let start = Instant::now();
        let _result = bubble_sort(input.clone());
        let duration = start.elapsed();
        
        // Property: bubble sort should be O(n²)
        // For small inputs, should complete quickly
        if input.len() <= 100 {
            prop_assert!(duration.as_millis() < 100);
        }
        
        // Property: time should be roughly proportional to n²
        let expected_max_ms = (input.len() * input.len()) as u128 / 10000;
        prop_assert!(duration.as_millis() <= expected_max_ms + 100);
    }
}
```

### Memory Usage Properties

```rust
use std::mem;

proptest! {
    #[test]
    fn test_memory_efficiency(input: Vec<i32>) {
        let original_size = mem::size_of_val(&input) + 
                           input.capacity() * mem::size_of::<i32>();
        
        let result = efficient_process(&input);
        let result_size = mem::size_of_val(&result) + 
                         result.capacity() * mem::size_of::<i32>();
        
        // Property: result shouldn't use significantly more memory
        prop_assert!(result_size <= original_size * 2);
    }
}
```

## Integration with Other Testing Tools

### Combining with Unit Tests

```rust
// Regular unit tests for specific cases
#[test]
fn test_empty_vector() {
    assert_eq!(bubble_sort(vec![]), vec![]);
}

#[test]
fn test_single_element() {
    assert_eq!(bubble_sort(vec![42]), vec![42]);
}

#[test]
fn test_already_sorted() {
    assert_eq!(bubble_sort(vec![1, 2, 3]), vec![1, 2, 3]);
}

// Property tests for general properties
proptest! {
    #[test]
    fn test_sort_general_properties(input: Vec<i32>) {
        let result = bubble_sort(input.clone());
        
        // Property: length preserved
        prop_assert_eq!(result.len(), input.len());
        
        // Property: result is sorted
        for window in result.windows(2) {
            prop_assert!(window[0] <= window[1]);
        }
    }
}
```

### Property Testing with Async Code

```rust
use tokio_test;

// Note: proptest doesn't directly support async, but you can work around it
proptest! {
    #[test]
    fn test_async_properties(input: String) {
        // Use tokio_test to run async code in property tests
        tokio_test::block_on(async {
            let result = async_process(&input).await;
            prop_assert!(result.is_ok());
        });
    }
}

// Alternative: use regular async test with property-like loops
#[tokio::test]
async fn test_async_with_generated_data() {
    use proptest::strategy::Strategy;
    use proptest::test_runner::TestRunner;
    
    let mut runner = TestRunner::default();
    
    let strategy = proptest::string::string_regex("[a-z]{1,100}").unwrap();
    
    for _ in 0..100 {
        let input = strategy.new_tree(&mut runner).unwrap().current();
        let result = async_process(&input).await;
        assert!(result.is_ok());
    }
}
```

## Best Practices

### 1. Choosing Good Properties

✅ **Good Properties:**
- Roundtrip properties (encode/decode)
- Invariants (data structure properties)
- Commutativity (order independence)
- Idempotence (repeated application)
- Conservation properties (nothing lost/gained)

❌ **Avoid:**
- Properties that are too specific (like unit tests)
- Properties that duplicate the implementation
- Properties that are too weak to catch bugs

### 2. Strategy Design

```rust
// Good: Focused strategy for domain
fn arb_valid_credit_card() -> impl Strategy<Value = String> {
    "[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{4}"
}

// Better: Strategy that generates actually valid cards
fn arb_credit_card_with_valid_checksum() -> impl Strategy<Value = String> {
    (0u64..9999, 0u64..9999, 0u64..9999, 0u64..9)
        .prop_map(|(a, b, c, d)| {
            let partial = format!("{:04}-{:04}-{:04}-{:03}", a, b, c, d);
            let checksum = calculate_luhn_checksum(&partial);
            format!("{}{}", partial, checksum)
        })
}
```

### 3. Debugging Failed Properties

```rust
proptest! {
    #[test]
    fn test_with_debugging(input: Vec<i32>) {
        // Add debugging information to failures
        let result = complex_function(&input);
        
        prop_assert!(
            result.is_ok(),
            "Function failed for input: {:?}\nError: {:?}",
            input,
            result.err()
        );
        
        let output = result.unwrap();
        
        prop_assert!(
            output.len() == input.len(),
            "Length mismatch for input: {:?}\nInput length: {}\nOutput length: {}",
            input,
            input.len(),
            output.len()
        );
    }
}
```

### 4. Performance Considerations

```rust
proptest! {
    // Limit test case complexity for performance
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn test_expensive_operation(
        input in proptest::collection::vec(any::<i32>(), 0..1000)
    ) {
        // Use prop_assume to skip expensive cases
        prop_assume!(input.len() <= 500);
        
        let result = expensive_operation(&input);
        prop_assert!(result.is_ok());
    }
}
```

## Common Patterns and Recipes

### Testing Parsers

```rust
proptest! {
    #[test]
    fn test_json_parser_roundtrip(value: serde_json::Value) {
        let serialized = serde_json::to_string(&value).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        prop_assert_eq!(value, parsed);
    }
}
```

### Testing Compression

```rust
proptest! {
    #[test]
    fn test_compression_properties(data: Vec<u8>) {
        let compressed = compress(&data);
        let decompressed = decompress(&compressed).unwrap();
        
        // Property 1: Lossless
        prop_assert_eq!(data, decompressed);
        
        // Property 2: Compression ratio reasonable
        if data.len() > 1000 {
            prop_assert!(compressed.len() <= data.len());
        }
    }
}
```

### Testing Caches

```rust
proptest! {
    #[test]
    fn test_cache_properties(operations: Vec<CacheOperation>) {
        let mut cache = Cache::new(100);
        let mut reference = HashMap::new();
        
        for op in operations {
            match op {
                CacheOperation::Put(k, v) => {
                    cache.put(k, v);
                    reference.insert(k, v);
                }
                CacheOperation::Get(k) => {
                    let cache_result = cache.get(&k);
                    let reference_result = reference.get(&k);
                    
                    // Recently accessed items should be in cache
                    if reference_result.is_some() {
                        prop_assert_eq!(cache_result, reference_result.copied());
                    }
                }
            }
        }
    }
}
```

## Conclusion

Property-based testing in Rust with proptest provides:

1. **Automatic test case generation** that finds edge cases you missed
2. **Higher confidence** through testing general properties
3. **Better bug discovery** through extensive input coverage
4. **Documentation** of your code's invariants and properties
5. **Regression protection** through failure persistence

Key principles for effective property-based testing:

- Think about **invariants** your code should maintain
- Design **good strategies** for generating test data
- Use **multiple property types** (roundtrip, idempotent, etc.)
- **Combine** with traditional unit tests
- **Debug systematically** when properties fail

Property-based testing complements traditional testing by ensuring your code works correctly across a much broader range of inputs than you could manually test.