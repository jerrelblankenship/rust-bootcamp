// Exercise 6: Property-Based Testing - Fix the Broken Property Tests!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix broken property-based testing patterns and generators
//
// INSTRUCTIONS:
// 1. Fix ONE property test error at a time
// 2. Run tests: `cargo test --bin ex06-property-tests`
// 3. Learn property-based testing vs example-based testing
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (missing proptest dependency)
// - Learn to write properties instead of specific examples
// - Fix generator and shrinking issues
//
// COMPLETED CONCEPTS:
// [] Property-based testing fundamentals
// [] Writing testable properties vs examples
// [] Custom generators and strategies
// [] Shrinking and minimal test cases
// [] Property test configuration
// [] Combining multiple properties

// Math utilities to test with properties
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn sort_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers
}

pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

// String manipulation utilities
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn count_words(s: &str) -> usize {
    if s.trim().is_empty() {
        0
    } else {
        s.split_whitespace().count()
    }
}

// BROKEN PROPERTY TESTS TO FIX!

#[cfg(test)]
mod tests {
    use super::*;
    // FIXME: Missing proptest import!
    // use proptest::prelude::*;

    // Exercise 6.1: Fix missing proptest setup
    // FIXME: This won't compile without proptest!
    /*
    proptest! {
        #[test]
        fn test_add_commutative(a in any::<i32>(), b in any::<i32>()) {
            // Property: addition is commutative (a + b = b + a)
            assert_eq!(add(a, b), add(b, a));
        }
    }
    */
    
    // Temporary example-based test (should be replaced with property test)
    #[test]
    fn test_add_commutative_examples() {
        // FIXME: This is example-based, not property-based!
        assert_eq!(add(2, 3), add(3, 2));
        assert_eq!(add(-1, 5), add(5, -1));
        assert_eq!(add(0, 0), add(0, 0));
        // But what about all other possible integer combinations?
    }
    // In C# you might use FsCheck or similar property testing libraries
    
    // ✅ CHECKPOINT 1: Set up proptest and convert to property-based test

    // Exercise 6.2: Fix inadequate property testing
    #[test]
    fn test_multiply_properties_incomplete() {
        // FIXME: Testing only specific examples, not properties!
        
        // Testing identity property with only one example
        assert_eq!(multiply(5, 1), 5);
        
        // Testing zero property with only one example  
        assert_eq!(multiply(7, 0), 0);
        
        // FIXME: What about:
        // - Commutativity: a * b = b * a for ALL integers?
        // - Associativity: (a * b) * c = a * (b * c) for ALL integers?
        // - Distributivity: a * (b + c) = a * b + a * c for ALL integers?
        
        // How do we test these properties for thousands of random inputs?
    }
    
    // ✅ CHECKPOINT 2: Write comprehensive property tests for multiplication

    // Exercise 6.3: Fix incorrect property definitions
    /*
    proptest! {
        #[test]
        fn test_reverse_string_property_wrong(s in ".*") {
            // FIXME: Wrong property! This is not always true!
            let reversed = reverse_string(&s);
            assert_eq!(s, reversed);  // WRONG: Only true for palindromes!
        }
    }
    */
    
    #[test]
    fn test_reverse_string_wrong_property() {
        // FIXME: This shows the wrong property
        let s = "hello";
        let reversed = reverse_string(s);
        // This assertion will fail because "hello" != "olleh"
        // assert_eq!(s, reversed);  // This would fail!
        
        // What's the CORRECT property for string reversal?
        // Hint: reverse(reverse(s)) = s
    }
    
    // ✅ CHECKPOINT 3: Write correct properties for string reversal

    // Exercise 6.4: Fix generator and strategy issues
    /*
    proptest! {
        #[test]
        fn test_factorial_with_bad_generator(n in any::<u32>()) {
            // FIXME: This will panic! factorial(50) is too large!
            let result = factorial(n);
            assert!(result >= 1);  // PANIC: Integer overflow for large n!
        }
    }
    */
    
    #[test]
    fn test_factorial_overflow_issue() {
        // FIXME: This demonstrates the overflow problem
        // factorial(50) will panic due to integer overflow
        // We need to limit the input range for testing
        
        // What's a reasonable range for factorial testing?
        // How do we create custom generators?
    }
    
    // ✅ CHECKPOINT 4: Create proper generators with appropriate ranges

    // Exercise 6.5: Fix shrinking and minimal test case issues
    /*
    proptest! {
        #[test]
        fn test_sort_with_poor_shrinking(numbers in prop::collection::vec(any::<i32>(), 0..1000)) {
            let sorted = sort_numbers(numbers.clone());
            
            // FIXME: When this fails, we get a huge vector that's hard to debug!
            // Property: sorted array should have same length
            assert_eq!(sorted.len(), numbers.len());
            
            // Property: sorted array should be in order
            for i in 1..sorted.len() {
                assert!(sorted[i-1] <= sorted[i]);
            }
        }
    }
    */
    
    #[test]
    fn test_sort_properties_without_shrinking() {
        // FIXME: When testing fails, we don't get minimal failing examples
        let numbers = vec![5, 2, 8, 1, 9, 3];  // Fixed example
        let sorted = sort_numbers(numbers.clone());
        
        assert_eq!(sorted.len(), numbers.len());
        
        for i in 1..sorted.len() {
            assert!(sorted[i-1] <= sorted[i]);
        }
        
        // But what if there's a bug? How do we find the SMALLEST input that fails?
    }
    
    // ✅ CHECKPOINT 5: Configure shrinking for better debugging

    // Exercise 6.6: Fix missing property combinations
    #[test]
    fn test_whitespace_normalization_incomplete() {
        // FIXME: Testing only one aspect, not comprehensive properties!
        
        let input = "hello    world";
        let normalized = normalize_whitespace(input);
        assert_eq!(normalized, "hello world");
        
        // FIXME: Missing properties:
        // - normalize(normalize(s)) = normalize(s) (idempotent)
        // - word count should be preserved
        // - no leading/trailing whitespace
        // - single spaces between words
        
        // How do we test all these properties together?
        // How do we generate strings with various whitespace patterns?
    }
    
    // ✅ CHECKPOINT 6: Write comprehensive combined properties

    // Helper function for manual property testing (before proptest is set up)
    fn test_property_manually<F>(test_fn: F, iterations: usize) 
    where 
        F: Fn(i32) -> bool 
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        for i in 0..iterations {
            // Simple pseudo-random number generation for testing
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            let random_val = (hasher.finish() as i32) % 1000 - 500;
            
            if !test_fn(random_val) {
                panic!("Property failed for input: {}", random_val);
            }
        }
    }

    #[test]
    fn test_add_identity_manually() {
        // Manual property test before proptest is set up
        test_property_manually(|x| add(x, 0) == x, 100);
    }
}

// COMPILATION CHALLENGES:
// 1. Add proptest dependency and imports
// 2. Convert example-based tests to property-based tests
// 3. Write correct mathematical properties
// 4. Create appropriate generators with proper ranges
// 5. Configure shrinking for better failure diagnosis
// 6. Combine multiple properties in comprehensive tests
//
// LEARNING OBJECTIVES:
// - Understanding property-based vs example-based testing
// - Writing mathematical properties that should always hold
// - Creating custom generators and input strategies
// - Using shrinking to find minimal failing examples
// - Combining multiple properties for comprehensive testing
// - Property testing in Rust vs C# (FsCheck) or other languages
//
// C# COMPARISON:
// C#: FsCheck with Arbitrary<T> and Property.ForAll()
// Rust: proptest with proptest! macro and strategies
//
// C#: Gen.Choose() for custom generators
// Rust: Strategy trait and prop::collection, prop::num ranges
//
// C#: Shrinking happens automatically
// Rust: Shrinking configurable per strategy
//
// C#: [Property] attribute
// Rust: proptest! { #[test] ... } block
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Proptest setup): [ ] Complete
// Checkpoint 2 (Multiplication properties): [ ] Complete
// Checkpoint 3 (Correct properties): [ ] Complete
// Checkpoint 4 (Proper generators): [ ] Complete
// Checkpoint 5 (Shrinking configuration): [ ] Complete
// Checkpoint 6 (Combined properties): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Property-based testing fundamentals and philosophy
// ✅ Writing mathematical properties for code verification
// ✅ Custom generators and input strategies
// ✅ Shrinking configuration for better debugging
// ✅ Comprehensive property testing patterns
// ✅ Property testing in Rust vs other languages
//
// 🚀 Ready for the next challenge?
// Move on to ex07-test-fixtures.rs to learn test data builders!
// Or check your work with: `cargo test --bin ex06-property-tests`