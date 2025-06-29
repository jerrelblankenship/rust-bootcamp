# C# to Rust Translation Guide

Quick reference for C# developers learning Rust fundamentals.

## 🔄 Core Syntax Translations

### Variable Declaration

| C# | Rust | Notes |
|----|------|-------|
| `var x = 5;` | `let x = 5;` | Immutable by default |
| `int x = 5;` | `let x: i32 = 5;` | Explicit typing |
| `const int X = 5;` | `const X: i32 = 5;` | Compile-time constants |
| `int x = 5; x = 10;` | `let mut x = 5; x = 10;` | Must specify `mut` |

### Functions

| C# | Rust | Notes |
|----|------|-------|
| `public void Method() { }` | `fn method() { }` | No access modifiers needed |
| `public int Add(int a, int b) { return a + b; }` | `fn add(a: i32, b: i32) -> i32 { a + b }` | Expression-based return |
| `public static void Main() { }` | `fn main() { }` | Entry point |

### Control Flow

| C# | Rust | Notes |
|----|------|-------|
| `if (condition) { }` | `if condition { }` | No parentheses needed |
| `for (int i = 0; i < 10; i++)` | `for i in 0..10` | Range syntax |
| `foreach (var item in list)` | `for item in list` | Iterator syntax |
| `while (condition) { }` | `while condition { }` | Same concept |

### Types

| C# | Rust | Notes |
|----|------|-------|
| `int` | `i32` | 32-bit signed |
| `uint` | `u32` | 32-bit unsigned |
| `long` | `i64` | 64-bit signed |
| `float` | `f32` | 32-bit float |
| `double` | `f64` | 64-bit float |
| `bool` | `bool` | Same |
| `char` | `char` | Unicode scalar value |
| `string` | `String` or `&str` | Owned vs borrowed |

### Console Output

| C# | Rust | Notes |
|----|------|-------|
| `Console.WriteLine("Hello");` | `println!("Hello");` | Macro, not method |
| `Console.Write("Hello");` | `print!("Hello");` | No newline |
| `Console.WriteLine($"Hello {name}");` | `println!("Hello {}", name);` | Different formatting |
| `Console.WriteLine("{0} {1}", a, b);` | `println!("{} {}", a, b);` | Positional arguments |

## 🏗️ Object-Oriented to Rust Patterns

### Classes vs Structs

```csharp
// C#
public class Person {
    public string Name { get; set; }
    public int Age { get; set; }
    
    public void SayHello() {
        Console.WriteLine($"Hello, I'm {Name}");
    }
}
```

```rust
// Rust
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, I'm {}", self.name);
    }
}
```

### Constructors

```csharp
// C#
public Person(string name, int age) {
    Name = name;
    Age = age;
}
```

```rust
// Rust
impl Person {
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}
```

### Enums

```csharp
// C#
public enum Status {
    Active,
    Inactive,
    Pending
}
```

```rust
// Rust - More powerful!
enum Status {
    Active,
    Inactive,
    Pending,
}

// Can also hold data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

## 🔍 Error Handling

### Null vs Option

```csharp
// C# - Nullable reference
string? name = GetName();
if (name != null) {
    Console.WriteLine(name);
}
```

```rust
// Rust - Option<T>
let name: Option<String> = get_name();
match name {
    Some(n) => println!("{}", n),
    None => println!("No name"),
}
```

### Exceptions vs Result

```csharp
// C#
try {
    int result = ParseNumber("42");
    Console.WriteLine(result);
} catch (Exception e) {
    Console.WriteLine($"Error: {e.Message}");
}
```

```rust
// Rust
match parse_number("42") {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Error: {}", e),
}
```

## 🎯 Memory Management Mindset

### C# (Garbage Collected)
```csharp
var data = new List<int> { 1, 2, 3 };
var copy1 = data;  // Both refer to same object
var copy2 = data;  // Multiple references OK
// GC handles cleanup
```

### Rust (Ownership)
```rust
let data = vec![1, 2, 3];
let copy1 = data;        // data MOVED to copy1
// let copy2 = data;     // ERROR: data already moved

// Solutions:
let data = vec![1, 2, 3];
let copy1 = data.clone();    // Explicit copy
let copy2 = &data;           // Borrow instead
```

## 🚀 Common Patterns

### String Operations

```csharp
// C#
string name = "Alice";
string greeting = "Hello " + name;
string formatted = $"Hello {name}!";
```

```rust
// Rust
let name = "Alice";
let greeting = format!("Hello {}", name);
let greeting2 = "Hello ".to_string() + name;
```

### Collections

```csharp
// C#
var numbers = new List<int> { 1, 2, 3 };
numbers.Add(4);
foreach (var num in numbers) {
    Console.WriteLine(num);
}
```

```rust
// Rust
let mut numbers = vec![1, 2, 3];
numbers.push(4);
for num in &numbers {
    println!("{}", num);
}
```

## 💡 Key Mental Shifts

1. **Mutability**: Immutable by default (opposite of C#)
2. **Memory**: Explicit ownership vs garbage collection
3. **Errors**: Values, not exceptions
4. **Null**: Impossible unless explicitly allowed with `Option<T>`
5. **Methods**: Free functions + `impl` blocks vs class methods

## 🎯 Learning Strategy

1. **Start with similarities** - Basic syntax is familiar
2. **Embrace the compiler** - It's your friend, not enemy
3. **Think in ownership** - Who owns this data?
4. **Use pattern matching** - More powerful than C# switch
5. **Trust the type system** - It prevents runtime crashes

---

**Remember**: Rust isn't just "C# with different syntax" - it's a different paradigm that provides compile-time safety! 🦀
