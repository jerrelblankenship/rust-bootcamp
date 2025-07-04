// Exercise 03: Trait Object Trouble - Dynamic Dispatch Gone Wrong
//
// EXERCISE PROGRESS: [░░░░░░░░] 0% Complete (8 checkpoints to fix)
// 
// Your task: Make these trait objects work by fixing object safety violations
// and dynamic dispatch issues.
//
// INSTRUCTIONS:
// 1. Fix ONE object safety violation at a time
// 2. Compile after each fix: `rustc ex03-trait-object-trouble.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (object safety rules)
// - Each checkpoint introduces different object safety concepts
// - Think about C# interface limitations and how Rust is stricter
//
// C# ANALOGY: Like working with interfaces and virtual methods, but with
// stricter rules about what can be made into an "interface object".
//
// COMPLETED CONCEPTS:
// [] Object safety rules and violations
// [] Generic methods and trait objects
// [] Associated types and object safety
// [] Static methods and object safety
// [] Trait object syntax (dyn Trait)
// [] Downcasting with Any trait
// [] Dynamic dispatch performance
// [] Safe abstractions over trait objects

use std::fmt::Debug;
use std::any::Any;

// ✅ CHECKPOINT 1: Object Safety Violations - Generic Methods
// This trait can't be made into a trait object because of generic methods
// C# equivalent: Interface with generic methods can't be used as object reference
trait Processor {
    fn process(&self, data: &str) -> String;
    
    // ❌ This generic method breaks object safety
    fn process_typed<T>(&self, data: T) -> String
    where
        T: Debug;
    
    // ❌ This method with Self return type breaks object safety
    fn clone_processor(&self) -> Self;
}

// ✅ CHECKPOINT 2: Object Safety Violations - Associated Types
// This trait can't be made into a trait object because of associated types
// C# equivalent: Interface with generic type parameters
trait DataSource {
    type Item;
    type Error;
    
    fn next(&mut self) -> Result<Self::Item, Self::Error>;
    fn reset(&mut self);
    
    // ❌ This method returning associated type breaks object safety
    fn get_all(&self) -> Vec<Self::Item>;
}

// ✅ CHECKPOINT 3: Object Safety Violations - Static Methods
// This trait can't be made into a trait object because of static methods
// C# equivalent: Interface with static methods (not allowed until C# 8)
trait Factory {
    fn create() -> Self;
    
    // ❌ This static method breaks object safety
    fn create_default() -> Self
    where
        Self: Default;
        
    fn configure(&mut self, config: &str);
}

// ✅ CHECKPOINT 4: Wrong Trait Object Syntax
// These trait object declarations have syntax errors
// C# equivalent: Incorrect interface reference syntax
struct Container {
    // ❌ Missing dyn keyword
    processor: Box<Processor>,
    
    // ❌ Incorrect trait bound syntax
    data_source: Box<dyn DataSource<Item = String>>,
    
    // ❌ Missing size annotation
    factory: Factory,
}

// ✅ CHECKPOINT 5: Trait Object Coercion Issues
// These implementations can't be converted to trait objects
// C# equivalent: Classes that implement interfaces but can't be cast
struct StringProcessor {
    prefix: String,
}

impl Processor for StringProcessor {
    fn process(&self, data: &str) -> String {
        format!("{}: {}", self.prefix, data)
    }
    
    // ❌ This breaks object safety but we need to implement it
    fn process_typed<T>(&self, data: T) -> String
    where
        T: Debug,
    {
        format!("{}: {:?}", self.prefix, data)
    }
    
    // ❌ This breaks object safety but we need to implement it
    fn clone_processor(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
        }
    }
}

// ✅ CHECKPOINT 6: Dynamic Dispatch Function Issues
// These functions can't work with trait objects as written
// C# equivalent: Methods that can't accept interface parameters
fn process_data(processor: Processor, data: &str) -> String {
    processor.process(data)
}

fn batch_process(processors: Vec<Box<Processor>>, data: Vec<&str>) -> Vec<String> {
    let mut results = Vec::new();
    for processor in processors {
        for item in &data {
            results.push(processor.process(item));
        }
    }
    results
}

// ✅ CHECKPOINT 7: Trait Object Downcasting Problems
// This downcasting code has multiple issues
// C# equivalent: Casting interface references back to concrete types
trait Component: Debug {
    fn name(&self) -> &str;
    fn update(&mut self);
}

#[derive(Debug)]
struct Button {
    label: String,
    clicked: bool,
}

impl Component for Button {
    fn name(&self) -> &str {
        &self.label
    }
    
    fn update(&mut self) {
        self.clicked = !self.clicked;
    }
}

fn handle_component(component: &mut dyn Component) {
    println!("Handling component: {}", component.name());
    component.update();
    
    // ❌ This downcasting is broken
    if let Some(button) = component.as_any().downcast_ref::<Button>() {
        println!("Button clicked: {}", button.clicked);
    }
}

// ✅ CHECKPOINT 8: Trait Object Lifetime Issues
// These trait objects have lifetime problems
// C# equivalent: Object lifetime and garbage collection issues
trait EventHandler {
    fn handle_event(&self, event: &str);
}

struct Logger<'a> {
    name: &'a str,
}

impl<'a> EventHandler for Logger<'a> {
    fn handle_event(&self, event: &str) {
        println!("[{}] {}", self.name, event);
    }
}

// ❌ This function has lifetime issues with trait objects
fn create_handler(name: &str) -> Box<dyn EventHandler> {
    Box::new(Logger { name })
}

fn setup_event_system() -> Vec<Box<dyn EventHandler>> {
    let mut handlers = Vec::new();
    
    // ❌ This creates dangling references
    let logger_name = "main_logger".to_string();
    let handler = create_handler(&logger_name);
    handlers.push(handler);
    
    handlers
}

// Helper implementations (these need to be fixed too)
impl DataSource for StringProcessor {
    type Item = String;
    type Error = String;
    
    fn next(&mut self) -> Result<Self::Item, Self::Error> {
        Ok("processed".to_string())
    }
    
    fn reset(&mut self) {
        // Reset logic
    }
    
    fn get_all(&self) -> Vec<Self::Item> {
        vec!["item1".to_string(), "item2".to_string()]
    }
}

impl Factory for StringProcessor {
    fn create() -> Self {
        Self {
            prefix: "default".to_string(),
        }
    }
    
    fn create_default() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
    
    fn configure(&mut self, config: &str) {
        self.prefix = config.to_string();
    }
}

impl Default for StringProcessor {
    fn default() -> Self {
        Self {
            prefix: "default".to_string(),
        }
    }
}

// ❌ Missing Any implementation for downcasting
trait ComponentAny: Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl ComponentAny for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    println!("=== Trait Object Trouble Exercise ===");
    
    // Test Checkpoint 1-3: Object safety violations
    // These lines should compile once traits are fixed
    /*
    let processor: Box<dyn Processor> = Box::new(StringProcessor {
        prefix: "test".to_string(),
    });
    
    let data_source: Box<dyn DataSource<Item = String, Error = String>> = 
        Box::new(StringProcessor {
            prefix: "source".to_string(),
        });
    
    let factory: Box<dyn Factory> = Box::new(StringProcessor {
        prefix: "factory".to_string(),
    });
    */
    
    // Test Checkpoint 4: Container with trait objects
    /*
    let container = Container {
        processor: Box::new(StringProcessor {
            prefix: "container".to_string(),
        }),
        data_source: Box::new(StringProcessor {
            prefix: "data".to_string(),
        }),
        factory: StringProcessor {
            prefix: "fact".to_string(),
        },
    };
    */
    
    // Test Checkpoint 5: Trait object coercion
    let string_processor = StringProcessor {
        prefix: "coercion".to_string(),
    };
    
    // Test Checkpoint 6: Dynamic dispatch
    let result = process_data(string_processor, "test data");
    println!("Processed: {}", result);
    
    // Test Checkpoint 7: Downcasting
    let mut button = Button {
        label: "Click me".to_string(),
        clicked: false,
    };
    
    handle_component(&mut button);
    
    // Test Checkpoint 8: Lifetime issues
    let handlers = setup_event_system();
    for handler in &handlers {
        handler.handle_event("test event");
    }
    
    println!("🎉 Trait object concepts demonstrated!");
}

// C# Comparison Notes:
//
// 1. Trait objects are like interface references with virtual method calls
// 2. Object safety ensures interface can be used as object reference
// 3. Generic methods break object safety (like generic interface methods)
// 4. Associated types break object safety (like generic type parameters)
// 5. Static methods break object safety (like static interface methods)
// 6. Downcasting is like casting interface to concrete type
// 7. Lifetime issues are like object lifetime management
// 8. Dynamic dispatch has performance cost (like virtual method calls)

// Key Differences from C#:
// - Stricter object safety rules than C# interfaces
// - No inheritance, only composition through traits
// - Explicit lifetime management vs garbage collection
// - More predictable performance characteristics
// - Better type safety with less runtime overhead