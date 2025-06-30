# Module 03: Error Handling

🎯 **Mission**: Eliminate runtime crashes by mastering Rust's revolutionary error handling!

## 🚀 Fix Errors Now (2 minutes)

```bash
cd 03-error-handling/exercises  
rustc ex01-option-basics.rs  # Fix null reference equivalents!
```

**The Revolution**: Errors are values, not exceptions. No more surprise crashes!

## 💡 The Game Changer

**C# has exceptions** - errors crash your program unless caught  
**Rust has `Result<T, E>` and `Option<T>`** - errors are impossible to ignore

**Result**: Bulletproof applications that handle every error case at compile time!

## 🔧 Your Learning Path

### **Step 1: Master Option<T>** (30 minutes)  
```rust
// C#: string name = GetName(); // Might be null!
// Rust: let name: Option<String> = get_name(); // Explicit!

match name {
    Some(n) => println!("Hello {}", n),  // Handle success
    None => println!("No name found"),  // Handle missing value
}
```

### **Step 2: Conquer Result<T, E>** (30 minutes)
```rust
// C#: int result = Parse(input); // Throws exception on failure  
// Rust: let result: Result<i32, _> = input.parse(); // Explicit!

match result {
    Ok(number) => println!("Got: {}", number),    // Handle success
    Err(e) => println!("Parse error: {}", e),     // Handle failure  
}
```

### **Step 3: Build File Processor** (90 minutes)
```bash
cd project-file-processor
cargo build  # Fix error handling throughout real project
cargo test   # Verify robust error handling
```

## 🔍 Your "Never Again!" Moments

### **No More Null Reference Exceptions**
```rust
// This is IMPOSSIBLE in Rust:
// let name: String = null; // ❌ Compile error!

// This is explicit and safe:
let name: Option<String> = None;  // ✅ Compiler enforces handling
```

### **No More Unhandled Exceptions**
```rust
// The ? operator makes error propagation explicit:
fn process_file() -> Result<String, Error> {
    let content = read_file("data.txt")?;  // Auto-propagate errors
    let processed = transform(content)?;   // Chain operations safely  
    Ok(processed)
}
```

## 🔄 C# vs Rust Error Philosophy

| Scenario | C# (Exceptions) | Rust (Values) |
|----------|-----------------|---------------|
| **Nullable** | `string? name` (can be null) | `Option<String>` (explicit handling) |
| **Try/Catch** | `try { Parse(x) } catch { }` | `match x.parse() { Ok(n) => ..., Err(e) => ... }` |
| **Propagation** | Exception bubbles up | `?` operator explicitly propagates |
| **Ignore errors** | Easy to forget try/catch | Impossible - compiler enforces handling |

## 🏆 Success = Crash-Proof Code

You've mastered this module when:
- ✅ You can't write code that crashes from null references
- ✅ Every error case is explicitly handled
- ✅ Your file processor gracefully handles all file operations
- ✅ You use `?` operator for elegant error propagation

## 🛠️ The Essential Tools

### **Option<T> for "Maybe" Values**
```rust
let maybe_name: Option<String> = get_name();
// Compiler forces you to handle both Some(value) and None!
```

### **Result<T, E> for Operations That Can Fail**  
```rust
let result: Result<i32, ParseIntError> = "42".parse();
// Compiler forces you to handle both Ok(value) and Err(error)!
```

### **The ? Operator for Error Propagation**
```rust
fn chain_operations() -> Result<String, Error> {
    let step1 = operation1()?;  // Return early if error
    let step2 = operation2(step1)?;  // Chain safely
    Ok(step2)
}
```

## 🆘 When Error Handling Gets Tricky

1. **Read the compiler** - It points to unhandled cases
2. **Use [Debugging Guide](DEBUGGING_CHECKLIST.md)** - Error handling patterns
3. **Think "What can go wrong?"** - Every operation that might fail
4. **Embrace the safety** - Compiler prevents runtime crashes

## 📚 Go Deeper When Ready

- 📖 **[Error Handling Deep Dive](reference/)** - Advanced patterns
- 🔧 **[Progressive Hints](exercises/hints/)** - Guided discovery
- 💡 **[Custom Error Types](reference/custom-errors-detailed.md)** - Professional patterns

---

**Start now**: `cd exercises && rustc ex01-option-basics.rs` 🦀

**Next Module**: [04 - Systems Programming](../04-systems-programming/README.md) →
