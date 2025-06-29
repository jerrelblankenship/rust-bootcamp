# Lesson 03: Functions and Control Flow

Functions in Rust are more restrictive than C# methods but this constraint leads to more predictable and safer code. Let's explore how to define functions and control program flow.

## 🎯 Learning Objectives

- Define and call functions with various signatures
- Understand expressions vs statements
- Master control flow constructs
- Work with pattern matching
- Learn about closures (anonymous functions)

## 📝 Function Basics

### Function Declaration

```rust
// C# method
// public int Add(int x, int y) { return x + y; }

// Rust function
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon - this is an expression
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}
```

Key differences from C#:
- `fn` keyword instead of access modifiers
- Parameters require type annotations
- Return type after `->`
- No `return` keyword needed for final expression

### Statements vs Expressions

```rust
fn main() {
    // Statements (perform actions, don't return values)
    let x = 5;      // Statement
    let y = 6;      // Statement
    
    // Expressions (evaluate to values)
    let z = {
        let inner = x + y;
        inner * 2       // No semicolon - returns this value
    };  // z = 22
    
    // Adding semicolon turns expression into statement
    let w = {
        x + y;          // Statement - returns ()
    };  // w = ()
}
```

### Function Parameters and Return Values

```rust
// No parameters, no return
fn print_header() {
    println!("=== Application Started ===");
}

// Multiple parameters
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

// Early return with explicit return keyword
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        return None;    // Early return
    }
    Some(dividend / divisor)
}

// Multiple return values using tuples
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    (min, max)
}

// Named return values (using struct)
struct Dimensions {
    width: f64,
    height: f64,
}

fn get_dimensions() -> Dimensions {
    Dimensions {
        width: 10.0,
        height: 20.0,
    }
}
```

## 🔀 Control Flow

### If Expressions

```rust
fn main() {
    let number = 6;
    
    // Basic if
    if number < 5 {
        println!("Less than 5");
    } else if number == 5 {
        println!("Equal to 5");
    } else {
        println!("Greater than 5");
    }
    
    // If is an expression (like C#'s ternary)
    let message = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    
    // Must return same type from all branches
    // let value = if condition { 5 } else { "five" }; // ERROR!
}
```

### Loops

```rust
fn main() {
    // Infinite loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            continue;   // Skip rest of iteration
        }
        if count == 5 {
            break;      // Exit loop
        }
        println!("count: {}", count);
    }
    
    // Loop with return value
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;  // Return value from loop
        }
    };
    println!("Result: {}", result);  // 20
    
    // While loop
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    
    // For loop (foreach in C#)
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("Value: {}", element);
    }
    
    // Range-based for loop
    for i in 1..5 {      // 1, 2, 3, 4
        println!("{}", i);
    }
    
    for i in 1..=5 {     // 1, 2, 3, 4, 5 (inclusive)
        println!("{}", i);
    }
    
    // Reverse range
    for i in (1..5).rev() {
        println!("{}", i);  // 4, 3, 2, 1
    }
}
```

## 🎯 Pattern Matching

### Match Expressions (Enhanced Switch)

```rust
fn main() {
    let number = 13;
    
    // Basic match
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3..=5 => println!("Three through five"),
        _ => println!("Something else"),  // Default case
    }
    
    // Match with return value
    let word = match number {
        1 => "one",
        2 => "two",
        3..=12 => "three to twelve",
        _ => "thirteen or more",
    };
    
    // Destructuring in match
    let point = (0, 5);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("At ({}, {})", x, y),
    }
    
    // Match with guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => println!("No value"),
    }
}
```

### If Let (Simpler Pattern Matching)

```rust
fn main() {
    // Instead of match for single pattern
    let some_value = Some(3);
    
    // Verbose match
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    // Cleaner if let
    if let Some(3) = some_value {
        println!("three");
    }
    
    // With else
    let number = Some(7);
    if let Some(i) = number {
        println!("Matched: {}", i);
    } else {
        println!("Didn't match");
    }
}
```

### While Let

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    
    // Pop values while Some
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

## 🔧 Advanced Functions

### Closures (Lambda Expressions)

```rust
fn main() {
    // C#: Func<int, int> square = x => x * x;
    // Rust:
    let square = |x| x * x;
    println!("5 squared: {}", square(5));
    
    // With type annotations
    let add: fn(i32, i32) -> i32 = |x, y| x + y;
    
    // Capturing environment
    let multiplier = 3;
    let multiply = |x| x * multiplier;  // Captures multiplier
    println!("5 * 3 = {}", multiply(5));
    
    // Multi-line closure
    let complex = |x: i32| {
        let y = x + 1;
        let z = y * 2;
        z - 1
    };
    
    // Closures as parameters
    fn apply_operation<F>(x: i32, operation: F) -> i32 
    where 
        F: Fn(i32) -> i32 
    {
        operation(x)
    }
    
    let result = apply_operation(5, |x| x * 2);
    println!("Result: {}", result);  // 10
}
```

### Function Pointers

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("Answer: {}", answer);  // 12
}
```

## 💻 Practice Exercises

### Exercise 1: Temperature Converter

```rust
// Create functions to convert between Celsius and Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Your code here
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Your code here
}

fn main() {
    // Test your functions
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C = {}°F", celsius, fahrenheit);
}
```

### Exercise 2: Pattern Matching Practice

```rust
enum Command {
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn process_command(cmd: Command) {
    // Use match to handle each command variant
    // Move: print coordinates
    // Write: print the message
    // ChangeColor: print RGB values
    // Quit: print goodbye message
}

fn main() {
    let commands = vec![
        Command::Move { x: 10, y: 20 },
        Command::Write(String::from("Hello")),
        Command::ChangeColor(255, 0, 0),
        Command::Quit,
    ];
    
    for cmd in commands {
        process_command(cmd);
    }
}
```

### Exercise 3: Iterator Methods

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Use iterator methods to:
    // 1. Filter even numbers
    // 2. Square each number
    // 3. Sum the results
    
    // Hint: numbers.iter().filter(...).map(...).sum()
    
    // Bonus: Do it with a for loop for comparison
}
```

## 🚀 Mini-Project: Calculator Functions

```rust
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(op: Operation, a: f64, b: f64) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(String::from("Division by zero"))
            } else {
                Ok(a / b)
            }
        }
    }
}

fn parse_operation(op_str: &str) -> Option<Operation> {
    match op_str {
        "+" | "add" => Some(Operation::Add),
        "-" | "sub" => Some(Operation::Subtract),
        "*" | "mul" => Some(Operation::Multiply),
        "/" | "div" => Some(Operation::Divide),
        _ => None,
    }
}

fn main() {
    // Test the calculator
    let operations = vec![
        ("+", 10.0, 5.0),
        ("-", 10.0, 5.0),
        ("*", 10.0, 5.0),
        ("/", 10.0, 5.0),
        ("/", 10.0, 0.0),
    ];
    
    for (op_str, a, b) in operations {
        if let Some(op) = parse_operation(op_str) {
            match calculate(op, a, b) {
                Ok(result) => println!("{} {} {} = {}", a, op_str, b, result),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
```

## 🔑 Key Takeaways

1. **Expressions Everywhere**: Almost everything returns a value
2. **Pattern Matching**: More powerful than switch statements
3. **No Null/Void**: Functions return `()` instead of void
4. **Exhaustive Matching**: Compiler ensures all cases handled
5. **Functional Features**: Closures and iterator methods

## 📚 Additional Resources

- [Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Write functions with parameters and return values
- [ ] Understand expressions vs statements
- [ ] Use if, loop, while, and for constructs
- [ ] Apply pattern matching with match
- [ ] Create and use closures
- [ ] Handle all cases in match expressions

---

Next: [Structs and Enums](04-structs-and-enums.md) →
