# Rust vs C# - Comprehensive Comparison

A detailed comparison guide for C# developers learning Rust.

## 🏗️ Language Philosophy

| Aspect | C# | Rust |
|--------|-----|------|
| **Memory Management** | Garbage Collection | Ownership System |
| **Null Safety** | Nullable reference types (C# 8+) | No null, Option<T> |
| **Error Handling** | Exceptions | Result<T, E> |
| **Concurrency** | async/await, Tasks, Threads | async/await, Threads, Channels |
| **Type System** | Nominal with some inference | Strong with inference |
| **Paradigm** | Multi-paradigm (OO-first) | Multi-paradigm (functional-friendly) |

## 📦 Project Structure

### C# Solution Structure
```
MySolution/
├── MySolution.sln
├── MyApp/
│   ├── MyApp.csproj
│   ├── Program.cs
│   ├── Models/
│   ├── Services/
│   └── Controllers/
├── MyApp.Tests/
│   └── MyApp.Tests.csproj
└── packages/
```

### Rust Workspace Structure
```
my_project/
├── Cargo.toml         # Like .sln
├── my_app/
│   ├── Cargo.toml     # Like .csproj
│   ├── src/
│   │   ├── main.rs    # Entry point
│   │   ├── models/
│   │   ├── services/
│   │   └── handlers/
│   └── tests/
└── target/            # Like bin/obj
```

## 🔧 Build Systems

| Feature | C# (MSBuild/dotnet) | Rust (Cargo) |
|---------|---------------------|--------------|
| **Build** | `dotnet build` | `cargo build` |
| **Run** | `dotnet run` | `cargo run` |
| **Test** | `dotnet test` | `cargo test` |
| **Package** | `dotnet pack` | `cargo package` |
| **Publish** | `dotnet publish` | `cargo publish` |
| **Dependencies** | NuGet | crates.io |
| **Config File** | .csproj (XML) | Cargo.toml (TOML) |

## 📝 Syntax Comparison

### Variables and Types

**C#:**
```csharp
// Immutable
const int MaxItems = 100;
readonly string name = "Alice";

// Mutable
var count = 0;
string message = "Hello";

// Nullable
string? nullableString = null;
int? nullableInt = null;
```

**Rust:**
```rust
// Immutable by default
const MAX_ITEMS: i32 = 100;
let name = "Alice";

// Mutable
let mut count = 0;
let mut message = String::from("Hello");

// Option type (no null)
let nullable_string: Option<String> = None;
let nullable_int: Option<i32> = None;
```

### Functions/Methods

**C#:**
```csharp
public class Calculator
{
    private int value;
    
    public Calculator(int initial) 
    {
        value = initial;
    }
    
    public int Add(int x) 
    {
        value += x;
        return value;
    }
    
    public static int StaticAdd(int a, int b) 
    {
        return a + b;
    }
}
```

**Rust:**
```rust
struct Calculator {
    value: i32,
}

impl Calculator {
    // Associated function (like static)
    fn new(initial: i32) -> Self {
        Calculator { value: initial }
    }
    
    // Method (takes &mut self)
    fn add(&mut self, x: i32) -> i32 {
        self.value += x;
        self.value
    }
    
    // Associated function
    fn static_add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

### Generics

**C#:**
```csharp
public class Box<T>
{
    public T Value { get; set; }
    
    public Box(T value)
    {
        Value = value;
    }
}

public interface IProcessor<T>
{
    T Process(T input);
}
```

**Rust:**
```rust
struct Box<T> {
    value: T,
}

impl<T> Box<T> {
    fn new(value: T) -> Self {
        Box { value }
    }
}

trait Processor<T> {
    fn process(&self, input: T) -> T;
}
```

### Collections

**C#:**
```csharp
// List
List<int> numbers = new List<int> { 1, 2, 3 };
numbers.Add(4);

// Dictionary
Dictionary<string, int> scores = new Dictionary<string, int>();
scores["Alice"] = 100;

// Array
int[] array = new int[] { 1, 2, 3, 4, 5 };

// LINQ
var evens = numbers.Where(x => x % 2 == 0).ToList();
```

**Rust:**
```rust
// Vector (like List)
let mut numbers = vec![1, 2, 3];
numbers.push(4);

// HashMap (like Dictionary)
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Alice", 100);

// Array (fixed size)
let array: [i32; 5] = [1, 2, 3, 4, 5];

// Iterator methods (like LINQ)
let evens: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)
    .copied()
    .collect();
```

## 🔄 Control Flow

### Pattern Matching

**C# (Switch Expressions):**
```csharp
string result = value switch
{
    1 => "one",
    2 => "two",
    3 or 4 or 5 => "three to five",
    _ => "other"
};

// Pattern matching
if (obj is string { Length: > 5 } str)
{
    Console.WriteLine($"Long string: {str}");
}
```

**Rust:**
```rust
let result = match value {
    1 => "one",
    2 => "two",
    3..=5 => "three to five",
    _ => "other",
};

// Pattern matching
match obj {
    Some(s) if s.len() > 5 => println!("Long string: {}", s),
    Some(s) => println!("Short string: {}", s),
    None => println!("No string"),
}
```

### Loops

**C#:**
```csharp
// foreach
foreach (var item in collection)
{
    Console.WriteLine(item);
}

// for
for (int i = 0; i < 10; i++)
{
    Console.WriteLine(i);
}

// while
while (condition)
{
    // ...
}
```

**Rust:**
```rust
// for (like foreach)
for item in collection {
    println!("{}", item);
}

// range-based for
for i in 0..10 {
    println!("{}", i);
}

// while
while condition {
    // ...
}

// loop (infinite)
loop {
    if should_break {
        break;
    }
}
```

## ⚡ Async Programming

### C# Async/Await
```csharp
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    var response = await client.GetAsync(url);
    return await response.Content.ReadAsStringAsync();
}

public async Task ProcessMultipleAsync()
{
    var tasks = urls.Select(FetchDataAsync);
    var results = await Task.WhenAll(tasks);
}
```

### Rust Async/Await
```rust
async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn process_multiple() -> Result<Vec<String>, Error> {
    let futures = urls.iter().map(|url| fetch_data(url));
    let results = futures::future::try_join_all(futures).await?;
    Ok(results)
}
```

## 🚨 Error Handling

### C# Exceptions
```csharp
try
{
    var result = DangerousOperation();
    Console.WriteLine($"Success: {result}");
}
catch (FileNotFoundException ex)
{
    Console.WriteLine($"File not found: {ex.Message}");
}
catch (Exception ex)
{
    Console.WriteLine($"Error: {ex.Message}");
}
finally
{
    // Cleanup
}
```

### Rust Result/Option
```rust
match dangerous_operation() {
    Ok(result) => println!("Success: {}", result),
    Err(Error::FileNotFound(msg)) => println!("File not found: {}", msg),
    Err(e) => println!("Error: {}", e),
}

// Using ? operator
fn process() -> Result<String, Error> {
    let result = dangerous_operation()?; // Early return on error
    Ok(result.to_uppercase())
}

// Cleanup with Drop trait (automatic)
```

## 🏛️ Object-Oriented vs Trait-Based

### C# Inheritance
```csharp
public abstract class Animal
{
    public abstract void MakeSound();
}

public class Dog : Animal
{
    public override void MakeSound()
    {
        Console.WriteLine("Woof!");
    }
}

public interface ISwimmable
{
    void Swim();
}

public class Fish : Animal, ISwimmable
{
    public override void MakeSound() { }
    public void Swim() { }
}
```

### Rust Traits
```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

trait Swimmable {
    fn swim(&self);
}

struct Fish;

impl Animal for Fish {
    fn make_sound(&self) {}
}

impl Swimmable for Fish {
    fn swim(&self) {}
}
```

## 🔒 Memory Management

### C# Garbage Collection
```csharp
public class ResourceManager
{
    private FileStream file;
    
    public ResourceManager(string path)
    {
        file = new FileStream(path, FileMode.Open);
    }
    
    // Implementing IDisposable
    public void Dispose()
    {
        file?.Dispose();
        GC.SuppressFinalize(this);
    }
    
    ~ResourceManager()
    {
        Dispose();
    }
}

// Usage
using (var manager = new ResourceManager("file.txt"))
{
    // Use manager
} // Dispose called automatically
```

### Rust Ownership/Drop
```rust
struct ResourceManager {
    file: File,
}

impl ResourceManager {
    fn new(path: &str) -> Result<Self, Error> {
        Ok(ResourceManager {
            file: File::open(path)?,
        })
    }
}

// Drop trait (automatically implemented)
impl Drop for ResourceManager {
    fn drop(&mut self) {
        // Cleanup happens automatically
        println!("Resource cleaned up");
    }
}

// Usage
{
    let manager = ResourceManager::new("file.txt")?;
    // Use manager
} // Drop called automatically
```

## 🎯 Common Patterns

### Builder Pattern

**C#:**
```csharp
public class PersonBuilder
{
    private Person person = new Person();
    
    public PersonBuilder WithName(string name)
    {
        person.Name = name;
        return this;
    }
    
    public PersonBuilder WithAge(int age)
    {
        person.Age = age;
        return this;
    }
    
    public Person Build() => person;
}

var person = new PersonBuilder()
    .WithName("Alice")
    .WithAge(30)
    .Build();
```

**Rust:**
```rust
struct PersonBuilder {
    person: Person,
}

impl PersonBuilder {
    fn new() -> Self {
        PersonBuilder {
            person: Person::default(),
        }
    }
    
    fn name(mut self, name: &str) -> Self {
        self.person.name = name.to_string();
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.person.age = age;
        self
    }
    
    fn build(self) -> Person {
        self.person
    }
}

let person = PersonBuilder::new()
    .name("Alice")
    .age(30)
    .build();
```

## 📊 Performance Considerations

| Aspect | C# | Rust |
|--------|-----|------|
| **Memory Overhead** | GC overhead, object headers | Minimal, predictable |
| **Allocation** | Heap-heavy | Stack-preferred |
| **Runtime** | JIT compilation, runtime | Ahead-of-time, no runtime |
| **Predictability** | GC pauses | Deterministic |
| **Optimization** | JIT optimizations | Compile-time optimizations |

## 🛠️ Tooling Comparison

| Tool | C# | Rust |
|------|-----|------|
| **Package Manager** | NuGet | Cargo |
| **Formatter** | dotnet format | cargo fmt |
| **Linter** | Roslyn analyzers | cargo clippy |
| **Test Runner** | dotnet test/xUnit/NUnit | cargo test |
| **Benchmarking** | BenchmarkDotNet | criterion |
| **Documentation** | XML docs/DocFX | rustdoc |
| **REPL** | C# Interactive | Rust Playground |

## 🔑 Key Takeaways for C# Developers

1. **No Garbage Collection**: Learn ownership, borrowing, and lifetimes
2. **No Inheritance**: Use composition and traits
3. **No Null**: Embrace Option<T> and Result<T, E>
4. **No Exceptions**: Errors are values
5. **Immutable by Default**: Explicit mutability
6. **Zero-Cost Abstractions**: High-level code, low-level performance
7. **Compiler is Strict**: But catches many bugs at compile time

## 📚 Learning Resources

### For C# Developers
- [Rust for C# Developers](https://github.com/microsoft/rust-for-dotnet-devs)
- [From C# to Rust](https://dev.to/sebnilsson/from-c-to-rust-introduction-4650)
- [Rust Book](https://doc.rust-lang.org/book/) - Start here!

### Practice
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Exercism Rust Track](https://exercism.io/tracks/rust) - With mentorship
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing

---

Remember: Rust's learning curve is steep but rewarding. The concepts that seem foreign at first (ownership, lifetimes) will become second nature and make you a better programmer in any language!
