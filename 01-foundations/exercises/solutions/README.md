# Exercise Solutions - Module 01: Foundations

🎯 **Important**: Use this guide only **after** attempting the exercises yourself! The learning happens when you struggle with compilation errors and discover the solutions.

## 📚 How to Approach These Exercises

1. **Try First**: Attempt each exercise completely before looking here
2. **Debug Systematically**: Fix one compilation error at a time
3. **Read Compiler Messages**: Rust's error messages are your best teacher
4. **Learn from Mistakes**: Each error teaches you something about Rust's safety

## 🔧 General Solution Strategies

### **For All Exercises**
- Start by making the code compile (fix syntax errors first)
- Then make the code work correctly (implement the logic)
- Use `rustc filename.rs` to compile and test
- Read every TODO and FIXME comment carefully

## 🎯 Exercise-by-Exercise Guidance

### **Exercise 1: Hello World Variations**
**Key Concepts to Master:**
- `println!` macro syntax (don't forget the `!`)
- Variable declaration with `let`  
- Debug formatting with `{:?}` and pretty formatting with `{:#?}`
- Function definition with `fn` keyword
- Expression-based returns (no `return` needed)

**Common Fixes You'll Make:**
- Add missing `!` to `println("Hello")` → `println!("Hello")`
- Declare variables: `let name = "Your Name";`
- Use correct format specifiers for complex data types
- Implement the `print_initials()` function
- Add parameter types to functions

### **Exercise 2: Variables and Types**
**Key Concepts to Master:**
- Variable declaration with `let` vs `let mut`
- Type annotations: `let x: i32 = 5;`
- Type conversions with `as` keyword
- Tuple access with `.0`, `.1` syntax (not indexing!)
- Array slicing with `&array[start..end]`
- String vs &str differences

**Common Fixes You'll Make:**
- Add `let` before variable declarations
- Add `mut` for variables that change: `let mut counter = 0;`
- Use type annotations when compiler can't infer
- Convert between numeric types: `x as i64`
- Access tuples with `.0` not `[0]`
- Create slices with `&data[1..4]` not `data[1:4]`

### **Exercise 3: Functions and Control Flow**
**Key Concepts to Master:**
- Function signatures with parameter types and return types
- Pattern matching with `match` expressions
- `if let` for simple pattern matching
- Loop variants: `loop`, `while`, `for`
- Early returns and error handling
- Expression vs statement differences

**Common Fixes You'll Make:**
- Add parameter types: `fn greet(name: &str, age: i32)`
- Add return types: `fn calculate() -> i32`
- Use `match` instead of `if/else` chains for enums
- Handle `Option` and `Result` types properly
- Return expressions without semicolons

### **Exercise 4: Structs and Enums**
**Key Concepts to Master:**
- Struct definition and instantiation
- Method vs associated function (`&self` vs `Self`)
- Enum variants with and without data
- `impl` blocks for adding methods
- Generic types and trait bounds
- Builder pattern implementation

**Common Fixes You'll Make:**
- Define struct fields with types
- Implement methods with `&self` parameter
- Create enum variants for different cases
- Use `::` for associated functions, `.` for methods
- Handle generic type parameters properly

## 💡 General Problem-Solving Patterns

### **When You See: "cannot find type 'X' in this scope"**
**Solution**: Define the missing type (enum, struct, etc.)
```rust
// Define missing enum
enum Operation {
    Add,
    Subtract,
    // ... more variants
}
```

### **When You See: "cannot assign to immutable variable"**
**Solution**: Add `mut` keyword
```rust
let mut counter = 0;  // Now can be changed
counter += 1;
```

### **When You See: "mismatched types"**
**Solution**: Convert between types or fix the type annotation
```rust
let x: i32 = 5;
let y: i64 = x as i64;  // Explicit conversion
```

### **When You See: "cannot index into a value of type"**
**Solution**: Use correct access syntax
```rust
let tuple = (1, 2, 3);
let first = tuple.0;  // Not tuple[0]

let array = [1, 2, 3, 4];
let slice = &array[1..3];  // Not array[1:3]
```

## 🔄 Key Differences from C#

### **Variable Declaration**
```rust
// C#: var x = 5; x = 10; (mutable by default)
// Rust: let mut x = 5; x = 10; (must specify mut)
```

### **Function Definitions**
```rust
// C#: public int Add(int a, int b) { return a + b; }
// Rust: fn add(a: i32, b: i32) -> i32 { a + b }
```

### **Pattern Matching**
```rust
// C#: switch (value) { case 1: ...; default: ...; }
// Rust: match value { 1 => ..., _ => ..., }
```

### **Error Handling**
```rust
// C#: try/catch with exceptions
// Rust: Result<T, E> with match or ? operator
```

## 🧪 Testing Your Solutions

Each exercise can be compiled and run:

```bash
# Compile and run
rustc ex01-hello-world.rs && ./ex01-hello-world

# Or use Rust's test framework
rustc --test ex01-hello-world.rs && ./ex01-hello-world
```

For the calculator project:
```bash
cd project-calculator
cargo build    # Should succeed when fixed
cargo test     # Should pass when implemented
cargo run -- 5 + 3  # Should calculate correctly
```

## 🎓 Learning Progression

### **After Exercise 1**: You understand basic Rust syntax
- Macros vs functions (`println!` vs `println`)
- Variable declaration patterns
- Basic string formatting

### **After Exercise 2**: You understand Rust's type system
- Explicit vs implicit typing
- Mutability annotations
- Type conversions and safety

### **After Exercise 3**: You understand control flow
- Pattern matching power
- Expression-based language features
- Error handling approaches

### **After Exercise 4**: You understand data organization
- Custom types with structs and enums
- Methods vs functions
- Generic programming basics

## 🆘 When Completely Stuck

1. **Read the error message** - Rust's compiler is very helpful
2. **Focus on one error** - Don't try to fix everything at once
3. **Look at the hints** - TODO and FIXME comments contain guidance
4. **Think C# first** - Then translate the concept to Rust
5. **Check working solutions** - In the calculator project's solutions/ directory

## 📖 Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- [Rust Playground](https://play.rust-lang.org/) - Try code online
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

## 🏆 Success Metrics

You've mastered Module 01 when:
- ✅ All exercises compile without errors
- ✅ You can explain the difference between `let` and `let mut`
- ✅ You understand when to use `&self` vs `Self` in methods
- ✅ You can handle `Option` and `Result` types
- ✅ The calculator project works completely

---

**Remember**: The goal isn't perfect code on the first try. The goal is understanding Rust's principles through hands-on problem solving. Every error you fix makes you a better Rust developer!

**🎯 Ready for the next challenge?** Move on to [Module 02: Ownership and Borrowing](../../02-ownership-and-borrowing/README.md) when you've completed all exercises!
