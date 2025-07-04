// Exercise 01: Macro Madness - Declarative Macros Gone Wrong
// 
// EXERCISE PROGRESS: [░░░░░░░░] 0% Complete (8 checkpoints to fix)
// 
// Your task: Make this advanced macro code compile and work correctly by fixing
// all the macro syntax errors and implementation issues.
//
// INSTRUCTIONS:
// 1. Fix ONE macro at a time - don't try to fix everything at once!
// 2. Compile after each fix: `rustc ex01-macro-madness.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (basic macro syntax)
// - Each checkpoint introduces more complex macro concepts
// - Celebrate each successful compilation - these are genuinely difficult!
//
// C# ANALOGY: Like building Source Generators that create code at compile time,
// but with pattern matching on syntax instead of analyzing semantic symbols.
//
// COMPLETED CONCEPTS:
// [] Basic macro syntax patterns
// [] Pattern matching with repetition
// [] Macro hygiene and variable capture
// [] Fragment specifiers (expr, block, ident)
// [] Recursive macro expansion
// [] Complex pattern matching
// [] Macro export and visibility
// [] Integration with the Rust type system

use std::collections::HashMap;

// ✅ CHECKPOINT 1: Basic Macro Syntax Error
// This macro should create a simple greeting function
// C# equivalent: A source generator that creates methods based on attributes
macro_rules! make_greeter {
    ($name:ident) => {
        fn $name() {
            println!("Hello from {}!", stringify!($name));
        }
    }
}

// ✅ CHECKPOINT 2: Pattern Matching Problems
// This macro should handle multiple patterns for different greetings
// C# equivalent: Switch expressions with pattern matching
macro_rules! greet {
    (hello $name:expr) => {
        println!("Hello, {}!", $name)
    };
    (goodbye $name:expr) => {
        println!("Goodbye, {}!", $name)
    };
    // FIXME: This pattern is broken - missing braces around the expansion
    (shout $name:expr) => 
        println!("HEY {}!!!", $name.to_uppercase())
    ;

// ✅ CHECKPOINT 2: Run `rustc ex01-macro-madness.rs` - should show macro syntax error
// If the greet macro syntax error is fixed, move to the next section

// ✅ CHECKPOINT 3: Repetition Syntax Issues
// This macro should generate multiple variable declarations
// C# equivalent: Using StringBuilder to generate multiple field declarations
macro_rules! declare_vars {
    ($($name:ident : $type:ty),*) => {
        $(
            let $name: $type = Default::default();
        )*
    };
}

// ✅ CHECKPOINT 4: Hygiene Problems
// This macro has variable capture issues
// C# equivalent: Variable scoping issues in generated code
macro_rules! create_counter {
    () => {
        let count = 0;
        move || {
            count += 1;
            count
        }
    };
}

// ✅ CHECKPOINT 5: Fragment Specifier Errors
// This macro should accept different types of expressions
// C# equivalent: Generic constraints in source generators
macro_rules! debug_print {
    ($val:expr) => {
        println!("{} = {:?}", stringify!($val), $val);
    };
    // This should accept a block, not an expression
    ($block:expr) => {
        println!("Executing block...");
        $block
    };
}

// ✅ CHECKPOINT 6: Macro Recursion Issues
// This macro should generate nested function calls
// C# equivalent: Recursive template generation
macro_rules! nest_calls {
    ($func:ident, 1) => {
        $func()
    };
    ($func:ident, $n:expr) => {
        $func();
        nest_calls!($func, $n - 1)
    };
}

// ✅ CHECKPOINT 7: Complex Pattern Matching
// This macro should build a HashMap with different syntaxes
// C# equivalent: Dictionary initialization with complex patterns
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
    // This pattern should handle type annotations but is broken
    ($($key:expr => $value:expr),+ : $key_type:ty, $value_type:ty) => {
        {
            let mut map: HashMap<$key_type, $value_type> = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

// ✅ CHECKPOINT 8: Macro Export and Usage Issues
// This macro should be usable from other modules
// C# equivalent: Public source generator attributes
macro_rules! create_struct {
    (pub $name:ident { $($field:ident: $type:ty),* }) => {
        pub struct $name {
            $(pub $field: $type),*
        }
        
        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self { $($field),* }
            }
        }
    };
}

fn main() {
    println!("=== Macro Madness Exercise ===");
    
    // Test Checkpoint 1: Basic macro
    make_greeter!(say_hello);
    say_hello();
    
    // Test Checkpoint 2: Pattern matching
    greet!(hello "Alice");
    greet!(goodbye "Bob");
    greet!(shout "charlie");
    
    // Test Checkpoint 3: Repetition
    declare_vars!(x: i32, y: String, z: bool);
    println!("Declared variables: x={}, y={:?}, z={}", x, y, z);
    
    // Test Checkpoint 4: Hygiene (this won't compile due to capture issues)
    let counter = create_counter!();
    println!("Counter: {}", counter());
    println!("Counter: {}", counter());
    
    // Test Checkpoint 5: Fragment specifiers
    debug_print!(42);
    debug_print!({
        let temp = 10 + 5;
        temp * 2
    });
    
    // Test Checkpoint 6: Recursion
    fn print_number() {
        println!("Number!");
    }
    nest_calls!(print_number, 3);
    
    // Test Checkpoint 7: Complex patterns
    let map1 = hashmap!();
    let map2 = hashmap!("a" => 1, "b" => 2);
    let map3 = hashmap!("x" => 10, "y" => 20 : &str, i32);
    
    println!("Maps created: {} {} {}", map1.len(), map2.len(), map3.len());
    
    // Test Checkpoint 8: Struct creation
    create_struct!(pub Person { name: String, age: u32 });
    let person = Person::new("Alice".to_string(), 30);
    println!("Created person: {} (age {})", person.name, person.age);
    
    println!("🎉 All macros working correctly!");
}

// C# Comparison Notes:
// 
// 1. Declarative macros are like C# Source Generators but work at the token level
// 2. Pattern matching in macros is similar to switch expressions on syntax
// 3. Macro hygiene prevents variable capture (like closure capture rules)
// 4. Fragment specifiers are like generic type constraints
// 5. Macro recursion is like recursive template instantiation
// 6. Complex patterns enable DSL-like syntax (like LINQ expression trees)
// 7. Macro export is like making generators available across assemblies
// 8. Debugging macros is like debugging generated code - requires expansion tools

// Key Differences from C#:
// - Macros work on tokens, not semantic analysis
// - Pattern matching is structural, not type-based
// - Hygiene prevents accidental variable capture
// - No runtime overhead - purely compile-time
// - More powerful than C# preprocessor, less than Source Generators