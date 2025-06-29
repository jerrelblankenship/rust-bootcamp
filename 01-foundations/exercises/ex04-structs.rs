// Exercise 4: Structs and Enums - Fix the Broken Code!
//
// Your task: Make all the struct and enum definitions compile and work correctly
// by fixing compilation errors and implementing missing functionality.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `rustc ex04-structs.rs && ./ex04-structs` to test
// 4. Understand Rust's approach to custom types vs C# classes

fn main() {
    println!("=== Exercise 4: Structs and Enums (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each section as you implement the types
    test_basic_structs();
    // test_struct_methods();
    // test_enums();
    // test_pattern_matching();
    // test_generic_types();
    // test_builder_pattern();
    
    println!("\n🎉 All struct and enum exercises completed!");
}

fn test_basic_structs() {
    println!("Testing basic structs...");
    
    // Exercise 4.1: Create and use Person struct
    // FIXME: Person struct doesn't exist yet
    let person = Person {  // COMPILE ERROR: Person not defined!
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    
    println!("Person: {} is {} years old", person.name, person.age);
    
    // Exercise 4.2: Create Point struct with methods
    // FIXME: Point struct and methods don't exist
    let point1 = Point::new(3.0, 4.0);  // COMPILE ERROR: Point not defined!
    let point2 = Point::origin();  // COMPILE ERROR: origin method not defined!
    
    println!("Point1: {:?}", point1);
    println!("Distance from origin: {}", point1.distance_from_origin());  // COMPILE ERROR!
    println!("Distance between points: {}", point1.distance_to(&point2));  // COMPILE ERROR!
}

fn test_struct_methods() {
    println!("Testing struct methods...");
    
    // Exercise 4.3: Rectangle with area calculation
    let rect = Rectangle::new(10.0, 5.0);  // COMPILE ERROR: Rectangle not defined!
    
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());  // COMPILE ERROR: area method not defined!
    println!("Perimeter: {}", rect.perimeter());  // COMPILE ERROR: perimeter method not defined!
    
    // Exercise 4.4: Mutable struct methods
    let mut circle = Circle::new(5.0);  // COMPILE ERROR: Circle not defined!
    println!("Original circle: {:?}", circle);
    
    circle.scale(2.0);  // COMPILE ERROR: scale method not defined!
    println!("Scaled circle: {:?}", circle);
    
    circle.set_radius(10.0);  // COMPILE ERROR: set_radius method not defined!
    println!("Modified circle: {:?}", circle);
}

fn test_enums() {
    println!("Testing enums...");
    
    // Exercise 4.5: Color enum
    let red = Color::Red;  // COMPILE ERROR: Color enum not defined!
    let custom = Color::Custom(128, 64, 192);  // COMPILE ERROR: Custom variant not defined!
    
    println!("Red color: {:?}", red);
    println!("Custom color: {:?}", custom);
    
    // Exercise 4.6: Shape enum with different variants
    let shapes = vec![
        Shape::Circle(5.0),  // COMPILE ERROR: Shape enum not defined!
        Shape::Rectangle(4.0, 6.0),  // COMPILE ERROR: Rectangle variant not defined!
        Shape::Triangle(3.0, 4.0, 5.0),  // COMPILE ERROR: Triangle variant not defined!
    ];
    
    for shape in shapes {
        println!("Shape area: {}", shape.area());  // COMPILE ERROR: area method not defined!
    }
}

fn test_pattern_matching() {
    println!("Testing pattern matching...");
    
    // Exercise 4.7: Matching on enums
    let status = ConnectionStatus::Connected("192.168.1.1".to_string());  // COMPILE ERROR!
    
    match status {
        ConnectionStatus::Connected(ip) => println!("Connected to {}", ip),
        ConnectionStatus::Disconnected => println!("Not connected"),
        ConnectionStatus::Error(code) => println!("Connection error: {}", code),
    }
    
    // Exercise 4.8: Processing different message types
    let messages = vec![
        Message::Text("Hello".to_string()),  // COMPILE ERROR: Message enum not defined!
        Message::Image("photo.jpg".to_string(), 1920, 1080),
        Message::File("document.pdf".to_string(), 2048),
    ];
    
    for msg in messages {
        process_message(&msg);  // COMPILE ERROR: process_message not defined!
    }
}

fn test_generic_types() {
    println!("Testing generic types...");
    
    // Exercise 4.9: Generic Container
    let int_container = Container::new(42);  // COMPILE ERROR: Container not defined!
    let string_container = Container::new("Hello".to_string());
    
    println!("Int container: {:?}", int_container.get());  // COMPILE ERROR!
    println!("String container: {:?}", string_container.get());
    
    // Exercise 4.10: Generic Result-like enum
    let success: MyResult<i32, String> = MyResult::Ok(100);  // COMPILE ERROR!
    let failure: MyResult<i32, String> = MyResult::Err("Something went wrong".to_string());
    
    handle_result(success);  // COMPILE ERROR: handle_result not defined!
    handle_result(failure);
}

fn test_builder_pattern() {
    println!("Testing builder pattern...");
    
    // Exercise 4.11: Person builder
    let person = PersonBuilder::new()  // COMPILE ERROR: PersonBuilder not defined!
        .name("Bob".to_string())
        .age(25)
        .email("bob@example.com".to_string())
        .build();  // COMPILE ERROR: Methods not defined!
    
    println!("Built person: {:?}", person);
}

// ============================================================================
// TODO: Implement all the types below by replacing the todo!() macros
// ============================================================================

// Exercise 4.1: Basic struct definition
// TODO: Define Person struct with name, age, and email fields
// HINT: Use String for name and email, u32 for age
struct Person {
    // TODO: Add fields
    // HINT: name: String, age: u32, email: String
}

// Exercise 4.2: Struct with methods
// TODO: Define Point struct with x and y coordinates
#[derive(Debug)]
struct Point {
    // TODO: Add x and y fields (f64)
}

impl Point {
    // TODO: Associated function to create new point
    fn new(x: f64, y: f64) -> Self {
        // TODO: Create and return Point instance
        todo!("Implement Point::new")
    }
    
    // TODO: Associated function for origin point (0, 0)
    fn origin() -> Self {
        // TODO: Return Point at origin
        todo!("Implement Point::origin")
    }
    
    // TODO: Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        // TODO: Use Pythagorean theorem: sqrt(x² + y²)
        // HINT: Use f64::sqrt() and f64::powi()
        todo!("Implement distance_from_origin")
    }
    
    // TODO: Method to calculate distance to another point
    fn distance_to(&self, other: &Point) -> f64 {
        // TODO: Distance between two points: sqrt((x2-x1)² + (y2-y1)²)
        todo!("Implement distance_to")
    }
}

// Exercise 4.3: Rectangle with area calculation
#[derive(Debug)]
struct Rectangle {
    // TODO: Add width and height fields (f64)
}

impl Rectangle {
    // TODO: Constructor
    fn new(width: f64, height: f64) -> Self {
        todo!("Implement Rectangle::new")
    }
    
    // TODO: Calculate area
    fn area(&self) -> f64 {
        todo!("Implement area calculation")
    }
    
    // TODO: Calculate perimeter
    fn perimeter(&self) -> f64 {
        todo!("Implement perimeter calculation")
    }
}

// Exercise 4.4: Mutable methods
#[derive(Debug)]
struct Circle {
    // TODO: Add radius field (f64)
}

impl Circle {
    // TODO: Constructor
    fn new(radius: f64) -> Self {
        todo!("Implement Circle::new")
    }
    
    // TODO: Mutable method to scale the circle
    fn scale(&mut self, factor: f64) {
        // TODO: Multiply radius by factor
        todo!("Implement scale method")
    }
    
    // TODO: Setter method for radius
    fn set_radius(&mut self, new_radius: f64) {
        todo!("Implement set_radius")
    }
    
    // TODO: Area calculation
    fn area(&self) -> f64 {
        // TODO: π × r²
        // HINT: Use std::f64::consts::PI
        todo!("Implement circle area")
    }
}

// Exercise 4.5: Basic enum
#[derive(Debug)]
enum Color {
    // TODO: Add color variants
    // HINT: Red, Green, Blue, Custom(u8, u8, u8)
}

// Exercise 4.6: Enum with methods
#[derive(Debug)]
enum Shape {
    // TODO: Add shape variants that hold data
    // HINT: Circle(f64), Rectangle(f64, f64), Triangle(f64, f64, f64)
}

impl Shape {
    // TODO: Calculate area for different shapes
    fn area(&self) -> f64 {
        // TODO: Match on self and calculate area for each shape
        // Circle: π × r²
        // Rectangle: width × height  
        // Triangle: use Heron's formula or assume right triangle
        todo!("Implement Shape::area")
    }
}

// Exercise 4.7: Enum for connection status
#[derive(Debug)]
enum ConnectionStatus {
    // TODO: Add variants
    // HINT: Connected(String), Disconnected, Error(u32)
}

// Exercise 4.8: Message enum and processing
#[derive(Debug)]
enum Message {
    // TODO: Add message variants
    // HINT: Text(String), Image(String, u32, u32), File(String, u64)
}

// TODO: Function to process different message types
fn process_message(msg: &Message) {
    // TODO: Match on message type and print appropriate info
    // Text: print the text
    // Image: print filename and dimensions
    // File: print filename and size
    todo!("Implement message processing")
}

// Exercise 4.9: Generic types
#[derive(Debug)]
struct Container<T> {
    // TODO: Add a field to hold value of type T
}

impl<T> Container<T> {
    // TODO: Constructor for generic container
    fn new(value: T) -> Self {
        todo!("Implement Container::new")
    }
    
    // TODO: Getter method
    fn get(&self) -> &T {
        todo!("Implement get method")
    }
}

// Exercise 4.10: Generic enum (like Result)
#[derive(Debug)]
enum MyResult<T, E> {
    // TODO: Add Ok and Err variants
    // HINT: Ok(T), Err(E)
}

// TODO: Function to handle MyResult
fn handle_result<T: std::fmt::Debug, E: std::fmt::Debug>(result: MyResult<T, E>) {
    // TODO: Match on result and print success or error
    todo!("Implement result handling")
}

// Exercise 4.11: Builder pattern
#[derive(Debug)]
struct PersonBuilder {
    // TODO: Add optional fields for building Person
    // HINT: Use Option<T> for each field
}

impl PersonBuilder {
    // TODO: Constructor for builder
    fn new() -> Self {
        todo!("Implement PersonBuilder::new")
    }
    
    // TODO: Builder methods that return Self
    fn name(self, name: String) -> Self {
        todo!("Implement name builder method")
    }
    
    fn age(self, age: u32) -> Self {
        todo!("Implement age builder method")
    }
    
    fn email(self, email: String) -> Self {
        todo!("Implement email builder method")
    }
    
    // TODO: Build method that returns Person
    fn build(self) -> Person {
        // TODO: Create Person from builder fields
        // HINT: Use unwrap_or_default() or provide defaults
        todo!("Implement build method")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_creation() {
        // TODO: Uncomment when Person is implemented
        // let person = Person {
        //     name: "Test".to_string(),
        //     age: 25,
        //     email: "test@example.com".to_string(),
        // };
        // assert_eq!(person.name, "Test");
        // assert_eq!(person.age, 25);
    }
    
    #[test]
    fn test_point_distance() {
        // TODO: Uncomment when Point is implemented
        // let origin = Point::origin();
        // let point = Point::new(3.0, 4.0);
        // assert_eq!(point.distance_from_origin(), 5.0);
        // assert_eq!(origin.distance_to(&point), 5.0);
    }
    
    #[test]
    fn test_rectangle_area() {
        // TODO: Uncomment when Rectangle is implemented
        // let rect = Rectangle::new(4.0, 5.0);
        // assert_eq!(rect.area(), 20.0);
        // assert_eq!(rect.perimeter(), 18.0);
    }
    
    #[test]
    fn test_circle_operations() {
        // TODO: Uncomment when Circle is implemented
        // let mut circle = Circle::new(2.0);
        // circle.scale(2.0);
        // assert_eq!(circle.radius, 4.0);
    }
    
    #[test]
    fn test_shape_areas() {
        // TODO: Uncomment when Shape is implemented
        // let circle = Shape::Circle(2.0);
        // let rect = Shape::Rectangle(4.0, 5.0);
        // assert!((circle.area() - (std::f64::consts::PI * 4.0)).abs() < 0.001);
        // assert_eq!(rect.area(), 20.0);
    }
    
    #[test]
    fn test_generic_container() {
        // TODO: Uncomment when Container is implemented
        // let container = Container::new(42);
        // assert_eq!(*container.get(), 42);
    }
    
    #[test]
    fn test_builder_pattern() {
        // TODO: Uncomment when PersonBuilder is implemented
        // let person = PersonBuilder::new()
        //     .name("Builder Test".to_string())
        //     .age(30)
        //     .build();
        // assert_eq!(person.name, "Builder Test");
        // assert_eq!(person.age, 30);
    }
}

// COMPILATION CHALLENGES:
// 1. Define structs with appropriate fields
// 2. Implement associated functions (::new) vs methods (&self)
// 3. Understand mutable methods (&mut self)
// 4. Create enums with data-carrying variants
// 5. Pattern match on enum variants
// 6. Use generics in structs and enums
// 7. Implement the builder pattern with method chaining
// 8. Handle ownership in methods (self vs &self vs &mut self)
//
// LEARNING OBJECTIVES FOR C# DEVELOPERS:
// - Structs vs classes: structs are value types by default
// - No inheritance: use composition and traits instead
// - Associated functions vs instance methods
// - Pattern matching on enums (more powerful than C# switch)
// - Ownership in method signatures
// - Generic types similar to C# generics
// - Builder pattern for optional parameters (no method overloading)
//
// KEY DIFFERENCES FROM C#:
// C#: public class Person { public string Name { get; set; } }
// Rust: struct Person { name: String } (no getters/setters by default)
//
// C#: public static Person Create() => new Person();
// Rust: impl Person { fn new() -> Self { Person { ... } } }
//
// C#: public void SetRadius(double r) { this.radius = r; }
// Rust: fn set_radius(&mut self, r: f64) { self.radius = r; }
//
// C#: enum Color { Red, Green, Blue }
// Rust: enum Color { Red, Green, Blue, Custom(u8, u8, u8) }
//
// C#: switch (shape) { case Circle c => c.Radius * c.Radius * Math.PI; }
// Rust: match shape { Shape::Circle(r) => std::f64::consts::PI * r * r, }
