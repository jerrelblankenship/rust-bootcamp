// Exercise 04: HRTB Headaches - Higher-Ranked Trait Bounds Gone Wrong
//
// Learning Objectives:
// - Understand higher-ranked trait bounds (for<'a>)
// - Debug lifetime polymorphism issues
// - Learn advanced lifetime relationships
// - Compare with C# generic constraints and delegates
//
// C# Analogy: Like generic constraints that work with any lifetime,
// similar to Func<T> delegates that can work with any input type.
//
// Your Mission: Fix the broken higher-ranked trait bounds to enable
// lifetime polymorphism. Each checkpoint deals with different HRTB scenarios.

use std::fmt::Debug;

// ❌ CHECKPOINT 1: Basic HRTB Syntax Issues
// This function should accept closures that work with any lifetime
// C# equivalent: Generic delegate that works with any reference type
fn process_with_closure<F>(data: &str, processor: F) -> String
where
    // ❌ This HRTB syntax is wrong
    F: Fn(&str) -> String,
{
    processor(data)
}

// ❌ CHECKPOINT 2: HRTB with Multiple Lifetimes
// This trait should work with any combination of lifetimes
// C# equivalent: Generic interface with multiple type parameters
trait Combiner {
    // ❌ This signature needs HRTB to work with any lifetimes
    fn combine(&self, left: &str, right: &str) -> String;
}

struct StringCombiner;

impl Combiner for StringCombiner {
    fn combine(&self, left: &str, right: &str) -> String {
        format!("{} + {}", left, right)
    }
}

// ❌ CHECKPOINT 3: HRTB with Generic Functions
// This function should work with any closure that accepts any lifetime
// C# equivalent: Higher-order function with generic delegate constraints
fn apply_to_all<F>(items: Vec<&str>, func: F) -> Vec<String>
where
    // ❌ This HRTB constraint is incorrect
    F: Fn(&str) -> String,
{
    items.into_iter().map(func).collect()
}

// ❌ CHECKPOINT 4: HRTB with Associated Types
// This trait should work with associated types that have lifetime parameters
// C# equivalent: Generic interface with associated generic types
trait Parser {
    type Output;
    
    // ❌ This method needs HRTB for the associated type
    fn parse(&self, input: &str) -> Self::Output;
}

// ❌ CHECKPOINT 5: HRTB with Trait Objects
// This trait object should work with any lifetime
// C# equivalent: Interface reference that works with any generic parameter
trait Validator {
    fn validate(&self, input: &str) -> bool;
}

struct EmailValidator;

impl Validator for EmailValidator {
    fn validate(&self, input: &str) -> bool {
        input.contains('@')
    }
}

// ❌ This function should accept validators that work with any lifetime
fn validate_all(
    items: Vec<&str>,
    validator: &dyn Validator, // This signature is wrong
) -> Vec<bool> {
    items.into_iter().map(|item| validator.validate(item)).collect()
}

// ❌ CHECKPOINT 6: HRTB with Complex Lifetime Relationships
// This should handle complex lifetime relationships
// C# equivalent: Generic constraints with multiple where clauses
trait Transform {
    // ❌ This method needs HRTB for complex lifetime relationships
    fn transform(&self, input: &str, context: &str) -> String;
}

struct PrefixTransform {
    prefix: String,
}

impl Transform for PrefixTransform {
    fn transform(&self, input: &str, context: &str) -> String {
        format!("{}: {} ({})", self.prefix, input, context)
    }
}

// ❌ CHECKPOINT 7: HRTB with Async Functions
// This should work with async closures (when they exist)
// C# equivalent: Async delegates with generic constraints
use std::future::Future;
use std::pin::Pin;

// ❌ This function should accept async closures with HRTB
fn process_async<F, Fut>(data: &str, processor: F) -> Pin<Box<dyn Future<Output = String>>>
where
    // ❌ This HRTB constraint for async is wrong
    F: Fn(&str) -> Fut,
    Fut: Future<Output = String>,
{
    Box::pin(processor(data))
}

// ❌ CHECKPOINT 8: HRTB with Iterators
// This should work with iterators that have lifetime parameters
// C# equivalent: LINQ expressions with generic constraints
trait IteratorProcessor {
    // ❌ This method needs HRTB for iterator lifetimes
    fn process_iter<'a, I>(&self, iter: I) -> Vec<String>
    where
        I: Iterator<Item = &'a str>;
}

struct UppercaseProcessor;

impl IteratorProcessor for UppercaseProcessor {
    fn process_iter<'a, I>(&self, iter: I) -> Vec<String>
    where
        I: Iterator<Item = &'a str>,
    {
        iter.map(|s| s.to_uppercase()).collect()
    }
}

// ❌ This function should accept processors that work with any iterator lifetime
fn batch_process<P>(processor: P, batches: Vec<Vec<&str>>) -> Vec<Vec<String>>
where
    // ❌ This HRTB constraint is incomplete
    P: IteratorProcessor,
{
    batches
        .into_iter()
        .map(|batch| processor.process_iter(batch.into_iter()))
        .collect()
}

// Helper functions that should work with HRTB
fn create_processor() -> impl Fn(&str) -> String {
    |s| s.to_uppercase()
}

fn create_combiner() -> impl Combiner {
    StringCombiner
}

fn create_validator() -> impl Validator {
    EmailValidator
}

// ❌ CHECKPOINT 9: HRTB with Error Handling
// This should work with error handling across lifetimes
// C# equivalent: Generic error handling with constraints
trait ErrorHandler {
    type Error;
    
    // ❌ This method needs HRTB for error lifetimes
    fn handle_error(&self, error: &str) -> Result<String, Self::Error>;
}

struct LoggingErrorHandler;

impl ErrorHandler for LoggingErrorHandler {
    type Error = String;
    
    fn handle_error(&self, error: &str) -> Result<String, Self::Error> {
        println!("Error: {}", error);
        Err(format!("Handled: {}", error))
    }
}

// ❌ This function should work with error handlers for any lifetime
fn safe_process<H>(
    data: Vec<&str>,
    handler: H,
) -> Result<Vec<String>, H::Error>
where
    // ❌ This HRTB constraint is missing
    H: ErrorHandler,
{
    let mut results = Vec::new();
    for item in data {
        match handler.handle_error(item) {
            Ok(result) => results.push(result),
            Err(e) => return Err(e),
        }
    }
    Ok(results)
}

fn main() {
    println!("=== HRTB Headaches Exercise ===");
    
    // Test Checkpoint 1: Basic HRTB
    let processor = create_processor();
    let result = process_with_closure("hello", processor);
    println!("Basic HRTB result: {}", result);
    
    // Test Checkpoint 2: Multiple lifetimes
    let combiner = create_combiner();
    let combined = combiner.combine("left", "right");
    println!("Combined: {}", combined);
    
    // Test Checkpoint 3: Generic functions
    let items = vec!["one", "two", "three"];
    let results = apply_to_all(items, |s| s.to_uppercase());
    println!("Applied to all: {:?}", results);
    
    // Test Checkpoint 4: Associated types
    // This will be tested once Parser is properly implemented
    
    // Test Checkpoint 5: Trait objects
    let validator = create_validator();
    let test_items = vec!["valid@email.com", "invalid", "another@valid.com"];
    let validation_results = validate_all(test_items, &validator);
    println!("Validation results: {:?}", validation_results);
    
    // Test Checkpoint 6: Complex relationships
    let transform = PrefixTransform {
        prefix: "TRANSFORM".to_string(),
    };
    let transformed = transform.transform("input", "context");
    println!("Transformed: {}", transformed);
    
    // Test Checkpoint 7: Async functions (simulated)
    // This would be tested with actual async runtime
    
    // Test Checkpoint 8: Iterators
    let processor = UppercaseProcessor;
    let batches = vec![
        vec!["hello", "world"],
        vec!["rust", "programming"],
    ];
    let processed_batches = batch_process(processor, batches);
    println!("Processed batches: {:?}", processed_batches);
    
    // Test Checkpoint 9: Error handling
    let error_handler = LoggingErrorHandler;
    let error_data = vec!["error1", "error2"];
    let error_result = safe_process(error_data, error_handler);
    println!("Error handling result: {:?}", error_result);
    
    println!("🎉 HRTB concepts demonstrated!");
}

// C# Comparison Notes:
//
// 1. HRTB is like generic constraints that work with any type parameter
// 2. for<'a> is like <T> where T can be any type
// 3. Lifetime polymorphism is like type parameter variance
// 4. Higher-ranked bounds enable more flexible generic programming
// 5. Similar to C# delegates that can work with any reference type
// 6. Async HRTB is like async delegates with generic constraints
// 7. Iterator HRTB is like LINQ with generic type constraints
// 8. Error handling HRTB is like generic exception handling
// 9. More compile-time guarantees than C# generic constraints

// Key Differences from C#:
// - Explicit lifetime management vs garbage collection
// - More precise control over memory and borrowing
// - Better performance characteristics
// - More complex syntax but safer guarantees
// - No runtime type checking needed