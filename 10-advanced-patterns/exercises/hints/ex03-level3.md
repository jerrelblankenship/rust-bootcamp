# Exercise 03 - Level 3 Hints: Trait Object Trouble

## 🎯 Complete Solutions

Here are the fixes for each checkpoint:

## 🔧 Checkpoint 1-3: Object Safety Fixes

### Fixed Processor Trait
```rust
// Split into object-safe and non-object-safe parts
trait Processor {
    fn process(&self, data: &str) -> String;
}

// Separate trait for cloning
trait ProcessorClone {
    fn clone_processor(&self) -> Box<dyn Processor>;
}

// Separate trait for typed processing
trait TypedProcessor {
    fn process_typed<T>(&self, data: T) -> String
    where
        T: Debug;
}
```

### Fixed DataSource Trait
```rust
// Make it concrete instead of generic
trait DataSource {
    fn next(&mut self) -> Result<String, String>;
    fn reset(&mut self);
    fn get_all(&self) -> Vec<String>;
}
```

### Fixed Factory Trait
```rust
// Remove static methods for object safety
trait Factory {
    fn configure(&mut self, config: &str);
}

// Static methods go in separate trait or impl blocks
impl StringProcessor {
    fn create() -> Self {
        Self { prefix: "default".to_string() }
    }
}
```

## 🔧 Checkpoint 4: Container Fix

```rust
struct Container {
    processor: Box<dyn Processor>,
    data_source: Box<dyn DataSource>,
    factory: Box<dyn Factory>,
}
```

## 🔧 Checkpoint 5-6: Function Fixes

```rust
// Fix function signatures
fn process_data(processor: &dyn Processor, data: &str) -> String {
    processor.process(data)
}

fn batch_process(processors: Vec<Box<dyn Processor>>, data: Vec<&str>) -> Vec<String> {
    let mut results = Vec::new();
    for processor in processors {
        for item in &data {
            results.push(processor.process(item));
        }
    }
    results
}
```

## 🔧 Checkpoint 7: Downcasting Fix

```rust
// Add Any trait for downcasting
trait Component: Debug {
    fn name(&self) -> &str;
    fn update(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Component for Button {
    fn name(&self) -> &str {
        &self.label
    }
    
    fn update(&mut self) {
        self.clicked = !self.clicked;
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn handle_component(component: &mut dyn Component) {
    println!("Handling component: {}", component.name());
    component.update();
    
    // Fixed downcasting
    if let Some(button) = component.as_any_mut().downcast_mut::<Button>() {
        println!("Button clicked: {}", button.clicked);
    }
}
```

## 🔧 Checkpoint 8: Lifetime Fix

```rust
// Fix lifetime issues
fn create_handler(name: String) -> Box<dyn EventHandler> {
    Box::new(Logger { name })
}

// Or use static lifetime
struct Logger {
    name: String,  // Own the string instead of borrowing
}

impl EventHandler for Logger {
    fn handle_event(&self, event: &str) {
        println!("[{}] {}", self.name, event);
    }
}
```

## 🔧 Updated Implementation

```rust
impl Processor for StringProcessor {
    fn process(&self, data: &str) -> String {
        format!("{}: {}", self.prefix, data)
    }
}

impl ProcessorClone for StringProcessor {
    fn clone_processor(&self) -> Box<dyn Processor> {
        Box::new(Self {
            prefix: self.prefix.clone(),
        })
    }
}

impl TypedProcessor for StringProcessor {
    fn process_typed<T>(&self, data: T) -> String
    where
        T: Debug,
    {
        format!("{}: {:?}", self.prefix, data)
    }
}
```

## 🎮 Testing Your Solution

```rust
fn main() {
    // Test trait object creation
    let processor: Box<dyn Processor> = Box::new(StringProcessor {
        prefix: "test".to_string(),
    });
    
    let result = processor.process("hello");
    println!("Result: {}", result);
    
    // Test downcasting
    let mut button = Button {
        label: "Click me".to_string(),
        clicked: false,
    };
    
    handle_component(&mut button);
}
```

## 🔍 Key Takeaways

1. **Object safety requires specific rules** - no generics, no Self returns, no static methods
2. **Use `dyn` keyword** for all trait objects
3. **Split traits when needed** - separate object-safe from non-object-safe methods
4. **Downcasting requires Any trait** - implement it manually when needed
5. **Lifetime issues** - own data instead of borrowing when possible

## 🎯 Verification

Your code should now compile and run without errors. The trait objects should work correctly with dynamic dispatch!