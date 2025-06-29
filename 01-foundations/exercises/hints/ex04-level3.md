# Exercise 4 Hints - Level 3 (Nearly Complete Solutions)

## 🎯 Complete Implementations

You've worked through the earlier levels but need to see the complete implementations. Here are the nearly-complete solutions with detailed explanations:

## 📝 Complete Struct Implementations

### Person Struct
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}
```

**Explanation**: Simple struct with three fields. `#[derive(Debug)]` allows printing with `{:?}`.

### Point Struct with Methods
```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
        // Alternative: Point::new(0.0, 0.0)
    }
    
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
        // Using Pythagorean theorem: √(x² + y²)
    }
    
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
```

**Key Points**:
- `Self` is shorthand for the struct type
- `&self` borrows the instance (read-only)
- `powi(2)` squares a number, `sqrt()` takes square root

### Rectangle with Area Calculation
```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
```

### Circle with Mutable Methods
```rust
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
    
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
        // &mut self allows modifying the instance
    }
    
    fn set_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
        // π × r²
    }
}
```

**Key Point**: `&mut self` is required to modify the struct's fields.

## 📝 Complete Enum Implementations

### Color Enum
```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),  // RGB values
}
```

### Shape Enum with Methods
```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),                    // radius
    Rectangle(f64, f64),           // width, height
    Triangle(f64, f64, f64),       // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius.powi(2),
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula: √(s(s-a)(s-b)(s-c)) where s = (a+b+c)/2
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}
```

**Key Points**:
- Enum variants can hold different types of data
- Pattern matching extracts data from variants
- Each match arm must handle one variant

### Connection Status Enum
```rust
#[derive(Debug)]
enum ConnectionStatus {
    Connected(String),    // IP address
    Disconnected,
    Error(u32),          // Error code
}
```

### Message Enum and Processing
```rust
#[derive(Debug)]
enum Message {
    Text(String),
    Image(String, u32, u32),    // filename, width, height
    File(String, u64),          // filename, size in bytes
}

fn process_message(msg: &Message) {
    match msg {
        Message::Text(content) => {
            println!("Text message: {}", content);
        }
        Message::Image(filename, width, height) => {
            println!("Image: {} ({}x{})", filename, width, height);
        }
        Message::File(filename, size) => {
            println!("File: {} ({} bytes)", filename, size);
        }
    }
}
```

## 📝 Generic Type Implementations

### Generic Container
```rust
#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}
```

**Key Points**:
- `<T>` declares a generic type parameter
- `T` can be any type
- `&T` returns a reference to avoid moving the value

### Generic Result-like Enum
```rust
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn handle_result<T: std::fmt::Debug, E: std::fmt::Debug>(result: MyResult<T, E>) {
    match result {
        MyResult::Ok(value) => println!("Success: {:?}", value),
        MyResult::Err(error) => println!("Error: {:?}", error),
    }
}
```

**Key Points**:
- `T: std::fmt::Debug` means T must implement Debug trait
- This allows using `{:?}` formatting

## 📝 Builder Pattern Implementation

### PersonBuilder
```rust
#[derive(Debug)]
struct PersonBuilder {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
}

impl PersonBuilder {
    fn new() -> Self {
        PersonBuilder {
            name: None,
            age: None,
            email: None,
        }
    }
    
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self  // Return self for method chaining
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    fn build(self) -> Person {
        Person {
            name: self.name.unwrap_or_else(|| "Unknown".to_string()),
            age: self.age.unwrap_or(0),
            email: self.email.unwrap_or_else(|| "no-email".to_string()),
        }
    }
}
```

**Key Points**:
- Builder methods take `self` by value (not `&self`)
- This enables method chaining: `builder.name(...).age(...).build()`
- `unwrap_or_else()` provides defaults for missing values

## 🎓 Core Learning Concepts

### Struct vs Class (C# Comparison)
**C#**:
```csharp
public class Person {
    public string Name { get; set; }
    public int Age { get; set; }
    
    public Person(string name, int age) {
        Name = name;
        Age = age;
    }
    
    public void Greet() => Console.WriteLine($"Hello, I'm {Name}");
}
```

**Rust**:
```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, I'm {}", self.name);
    }
}
```

### Method Types
- **Associated functions**: `Point::new()` - called on the type
- **Instance methods**: `point.distance_from_origin()` - called on instances
- **Mutable methods**: `circle.scale(2.0)` - can modify the instance

### Ownership in Methods
- `self` - takes ownership (consumes the value)
- `&self` - borrows immutably (read-only)
- `&mut self` - borrows mutably (can modify)

## 🚀 Testing Your Implementation

Run this to test everything:
```bash
rustc ex04-structs.rs && ./ex04-structs
```

All functions should compile and run successfully when you use these implementations.

## 💡 Next Steps

Once this exercise works:
1. Understand why each method signature uses `self`, `&self`, or `&mut self`
2. Experiment with adding your own methods
3. Try creating your own structs and enums
4. Move on to Exercise 5 for more advanced patterns

You've mastered Rust's fundamental type system - structs and enums are the building blocks of all Rust programs! 🦀