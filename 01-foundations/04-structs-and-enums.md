# Lesson 04: Structs and Enums

Rust's structs and enums form the foundation of its type system. Coming from C#, you'll find structs familiar but more powerful, while enums are far more capable than C#'s simple enumerations.

## 🎯 Learning Objectives

- Create and use structs for data organization
- Implement methods and associated functions
- Master enums with data variants
- Understand Option and Result types
- Apply pattern matching with custom types

## 📝 Structs - Custom Data Types

### Basic Struct Definition

```rust
// C# class
// public class User {
//     public string Username { get; set; }
//     public string Email { get; set; }
//     public bool Active { get; set; }
// }

// Rust struct
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    // Create instance
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
    };
    
    // Access fields
    println!("User: {}", user1.username);
    
    // Mutable instance
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
    };
    
    user2.email = String::from("newemail@example.com");
}
```

### Struct Update Syntax

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
    };
    
    // Create new instance based on existing
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // Copy remaining fields from user1
    };
    
    // Note: user1.username has been moved to user2
    // println!("{}", user1.username);  // ERROR!
    println!("{}", user1.active);  // OK - bool implements Copy
}
```

### Tuple Structs

```rust
// Named tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Access by index
    println!("Red value: {}", black.0);
    
    // Type safety - Color and Point are different types
    // let color: Color = origin;  // ERROR!
}
```

### Unit-Like Structs

```rust
// No fields (like C# marker interfaces)
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    // Useful for traits without data
}
```

## 🔧 Methods and Associated Functions

### Implementing Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Implementation block
impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // Mutable method
    fn double_size(&mut self) {
        self.width *= 2.0;
        self.height *= 2.0;
    }
    
    // Consuming method (takes self)
    fn destroy(self) {
        println!("Rectangle destroyed!");
        // self is moved and dropped
    }
    
    // Associated function (no self) - like static method
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Associated function - alternative constructor
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // Call associated function
    let rect1 = Rectangle::new(30.0, 50.0);
    let rect2 = Rectangle::square(20.0);
    
    // Call methods
    println!("Area: {}", rect1.area());
    println!("Can hold? {}", rect1.can_hold(&rect2));
    
    // Debug print
    println!("Rectangle: {:?}", rect1);
    
    // Mutable method
    let mut rect3 = Rectangle::square(10.0);
    rect3.double_size();
    println!("Doubled: {:?}", rect3);
}
```

### Multiple impl Blocks

```rust
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
```

## 🎨 Enums - Sum Types

### Basic Enums

```rust
// C# enum (simple)
// public enum Status { Pending, Approved, Rejected }

// Rust enum (can hold data!)
enum Status {
    Pending,
    Approved(String),    // With approval message
    Rejected { reason: String, code: u32 },  // Named fields
}

fn main() {
    let status1 = Status::Pending;
    let status2 = Status::Approved(String::from("Looks good!"));
    let status3 = Status::Rejected {
        reason: String::from("Missing documentation"),
        code: 400,
    };
    
    // Pattern matching
    match status2 {
        Status::Pending => println!("Waiting..."),
        Status::Approved(msg) => println!("Approved: {}", msg),
        Status::Rejected { reason, code } => {
            println!("Rejected ({}): {}", code, reason)
        }
    }
}
```

### Enums with Methods

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b)
            }
        }
    }
}

fn main() {
    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];
    
    for msg in messages {
        msg.process();
    }
}
```

## 🔍 The Option Type

Rust's answer to null references:

```rust
// Definition in standard library
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    // No null in Rust!
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    
    // Must handle both cases
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
    
    // Convenience methods
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    // unwrap_or provides default
    println!("x: {}", x.unwrap_or(0));  // 5
    println!("y: {}", y.unwrap_or(0));  // 0
    
    // map transforms the value if Some
    let doubled = x.map(|n| n * 2);  // Some(10)
    
    // and_then for chaining
    let result = x.and_then(|n| {
        if n > 0 {
            Some(n * 2)
        } else {
            None
        }
    });
}
```

## ⚠️ The Result Type

For error handling without exceptions:

```rust
// Definition in standard library
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // File operations return Result
    let file_result = File::open("hello.txt");
    
    // Pattern matching
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found!");
                return;
            }
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };
    
    // Using ? operator (like C#'s throw)
    fn read_username() -> Result<String, std::io::Error> {
        let mut file = File::open("username.txt")?;
        let mut username = String::new();
        use std::io::Read;
        file.read_to_string(&mut username)?;
        Ok(username)
    }
    
    // Result methods
    let result: Result<i32, &str> = Ok(2);
    
    // Transform success value
    let squared = result.map(|x| x * x);  // Ok(4)
    
    // Provide default
    let value = result.unwrap_or(0);  // 2
    
    // Chain operations
    let final_result = result
        .and_then(|x| Ok(x * 2))
        .and_then(|x| Ok(x + 1));  // Ok(5)
}
```

## 🏗️ Builder Pattern

Common pattern in Rust:

```rust
#[derive(Debug, Default)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_seconds: u64,
}

// Builder
struct ServerConfigBuilder {
    config: ServerConfig,
}

impl ServerConfigBuilder {
    fn new() -> Self {
        ServerConfigBuilder {
            config: ServerConfig::default(),
        }
    }
    
    fn host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }
    
    fn max_connections(mut self, max: usize) -> Self {
        self.config.max_connections = max;
        self
    }
    
    fn timeout_seconds(mut self, timeout: u64) -> Self {
        self.config.timeout_seconds = timeout;
        self
    }
    
    fn build(self) -> ServerConfig {
        self.config
    }
}

fn main() {
    let config = ServerConfigBuilder::new()
        .host("localhost")
        .port(8080)
        .max_connections(1000)
        .timeout_seconds(30)
        .build();
    
    println!("Server config: {:?}", config);
}
```

## 💻 Practice Exercises

### Exercise 1: Shape Calculator

```rust
// Define an enum for different shapes
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    // Implement area calculation
    fn area(&self) -> f64 {
        // Your code here
    }
    
    // Implement perimeter calculation
    fn perimeter(&self) -> f64 {
        // Your code here
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 10.0, height: 20.0 },
        Shape::Triangle { base: 8.0, height: 6.0 },
    ];
    
    for shape in shapes {
        println!("Area: {:.2}", shape.area());
        println!("Perimeter: {:.2}", shape.perimeter());
        println!("---");
    }
}
```

### Exercise 2: Task Manager

```rust
#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        // Initialize empty task manager
    }
    
    fn add_task(&mut self, title: String) -> u32 {
        // Add task and return its ID
    }
    
    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        // Mark task as complete or return error
    }
    
    fn list_tasks(&self) {
        // Print all tasks
    }
}

fn main() {
    let mut manager = TaskManager::new();
    
    // Test your implementation
    let id1 = manager.add_task(String::from("Learn Rust"));
    let id2 = manager.add_task(String::from("Build project"));
    
    manager.list_tasks();
    
    match manager.complete_task(id1) {
        Ok(()) => println!("Task {} completed!", id1),
        Err(e) => println!("Error: {}", e),
    }
    
    manager.list_tasks();
}
```

## 🚀 Mini-Project: Game State

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    health: i32,
    score: u32,
}

#[derive(Debug)]
enum GameEvent {
    PlayerJoined(String),
    PlayerLeft(String),
    PlayerScored { name: String, points: u32 },
    PlayerDamaged { name: String, damage: i32 },
}

struct GameState {
    players: HashMap<String, Player>,
    events: Vec<GameEvent>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            players: HashMap::new(),
            events: Vec::new(),
        }
    }
    
    fn process_event(&mut self, event: GameEvent) {
        self.events.push(event.clone());
        
        match event {
            GameEvent::PlayerJoined(name) => {
                let player = Player {
                    name: name.clone(),
                    health: 100,
                    score: 0,
                };
                self.players.insert(name, player);
            }
            GameEvent::PlayerLeft(name) => {
                self.players.remove(&name);
            }
            GameEvent::PlayerScored { name, points } => {
                if let Some(player) = self.players.get_mut(&name) {
                    player.score += points;
                }
            }
            GameEvent::PlayerDamaged { name, damage } => {
                if let Some(player) = self.players.get_mut(&name) {
                    player.health -= damage;
                    if player.health <= 0 {
                        println!("{} has been eliminated!", name);
                        self.players.remove(&name);
                    }
                }
            }
        }
    }
    
    fn get_leaderboard(&self) -> Vec<(&String, u32)> {
        let mut scores: Vec<_> = self.players
            .iter()
            .map(|(name, player)| (name, player.score))
            .collect();
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        scores
    }
}

fn main() {
    let mut game = GameState::new();
    
    // Simulate game events
    game.process_event(GameEvent::PlayerJoined("Alice".to_string()));
    game.process_event(GameEvent::PlayerJoined("Bob".to_string()));
    game.process_event(GameEvent::PlayerScored {
        name: "Alice".to_string(),
        points: 100,
    });
    game.process_event(GameEvent::PlayerDamaged {
        name: "Bob".to_string(),
        damage: 30,
    });
    game.process_event(GameEvent::PlayerScored {
        name: "Bob".to_string(),
        points: 150,
    });
    
    // Display game state
    println!("Current players:");
    for (name, player) in &game.players {
        println!("  {} - Health: {}, Score: {}", 
                 name, player.health, player.score);
    }
    
    println!("\nLeaderboard:");
    for (i, (name, score)) in game.get_leaderboard().iter().enumerate() {
        println!("  {}. {} - {}", i + 1, name, score);
    }
}
```

## 🔑 Key Takeaways

1. **Structs**: Data + behavior, no inheritance
2. **Enums**: Can hold different types of data
3. **No Null**: Option<T> makes absence explicit
4. **Error Values**: Result<T, E> instead of exceptions
5. **Pattern Matching**: Exhaustive handling of all cases

## 📚 Additional Resources

- [Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust by Example - Custom Types](https://doc.rust-lang.org/rust-by-example/custom_types.html)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Define and instantiate structs
- [ ] Implement methods and associated functions
- [ ] Create enums with data variants
- [ ] Use Option<T> for optional values
- [ ] Handle errors with Result<T, E>
- [ ] Apply pattern matching to custom types

---

Next Module: [02 - Ownership and Borrowing](../02-ownership-and-borrowing/README.md) →
