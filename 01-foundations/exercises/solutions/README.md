# Exercise Help - Module 01: Foundations

🎯 **New Approach**: We now use a **progressive hints system** instead of direct solutions! This helps you learn through guided discovery.

## 🔄 How to Get Help

**Instead of looking for complete solutions, follow this progression:**

1. **Try the exercise first** - Use compiler error messages as your guide
2. **Check Level 1 hints** in `/hints/ex01-level1.md` - Gentle nudges
3. **Check Level 2 hints** in `/hints/ex01-level2.md` - More specific guidance  
4. **Check Level 3 hints** in `/hints/ex01-level3.md` - Nearly complete solutions
5. **Ask for help** - If you're still stuck after all hint levels

**Why this approach?** Productive struggle is where learning happens! The hints guide your discovery process rather than giving away answers.

## 📚 Progressive Learning Approach

1. **Attempt First**: Try to fix compilation errors using the compiler messages
2. **One Error at a Time**: Don't try to fix everything at once
3. **Use Hints Progressively**: Start with Level 1, only move up if needed
4. **Understand the Why**: Focus on concepts, not just making it compile
5. **Relate to C#**: Think about how you'd solve this in C# first

## 🔧 General Problem-Solving Strategies

### **For All Exercises**
- **Read the error message completely** - Rust's compiler is very helpful
- **Fix one error at a time** - Compile after each fix
- **Check TODO/FIXME comments** - They contain important hints
- **Use hints when stuck** - But try first!
- **Compile frequently** - `rustc filename.rs`

## 🎯 Exercise-by-Exercise Guidance

### **Exercise 1: Hello World Variations**
**Key Concepts to Master:**
- `println!` macro syntax (don't forget the `!`)
- Variable declaration with `let`  
- Debug formatting with `{:?}` and pretty formatting with `{:#?}`
- Function definition with `fn` keyword
- Expression-based returns (no `return` needed)

**Learning Focus:**
- Macro syntax with `!` 
- Variable declaration with `let`
- Debug formatting patterns
- Function implementation
- Parameter type annotations

**If you're stuck**: Check [Exercise 1 Hints](../hints/ex01-level1.md)

### **Exercise 2: Variables and Types**
**Key Concepts to Master:**
- Variable declaration with `let` vs `let mut`
- Type annotations: `let x: i32 = 5;`
- Type conversions with `as` keyword
- Tuple access with `.0`, `.1` syntax (not indexing!)
- Array slicing with `&array[start..end]`
- String vs &str differences

**Learning Focus:**
- Mutability with `let` vs `let mut`
- Type annotations and inference
- Type conversions with `as`
- Tuple and array access patterns
- Slicing syntax

**If you're stuck**: Check [Exercise 2 Hints](../hints/ex02-level1.md)

### **Exercise 3: Functions and Control Flow**
**Key Concepts to Master:**
- Function signatures with parameter types and return types
- Pattern matching with `match` expressions
- `if let` for simple pattern matching
- Loop variants: `loop`, `while`, `for`
- Early returns and error handling
- Expression vs statement differences

**Learning Focus:**
- Function signatures and types
- Pattern matching with `match`
- Option and Result handling
- Expression-based returns
- Control flow patterns

**If you're stuck**: Check [Exercise 3 Hints](../hints/ex03-level1.md)

### **Exercise 4: Structs and Enums**
**Key Concepts to Master:**
- Struct definition and instantiation
- Method vs associated function (`&self` vs `Self`)
- Enum variants with and without data
- `impl` blocks for adding methods
- Generic types and trait bounds
- Builder pattern implementation

**Learning Focus:**
- Struct definition and instantiation
- Method vs associated function syntax
- Enum variants and pattern matching
- Generic type parameters
- Implementation blocks

**If you're stuck**: Check [Exercise 4 Hints](../hints/ex04-level1.md)

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

## 🆘 When You Need Help

**Follow this progression:**

1. **Try for 10-15 minutes** using compiler error messages
2. **Check Level 1 hints** for gentle guidance
3. **Check Level 2 hints** for more specific help
4. **Check Level 3 hints** for nearly complete solutions
5. **Ask instructor** if still stuck after all hint levels

**Remember**: The struggle is where learning happens! Use hints to guide discovery, not to skip the thinking process.

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

## 📂 Available Hints

- [Exercise 1: Hello World](../hints/ex01-level1.md) - Basic syntax and macros
- [Exercise 2: Types](../hints/ex02-level1.md) - Type system fundamentals  
- [Exercise 3: Functions](../hints/ex03-level1.md) - Functions and control flow
- [Exercise 4: Structs](../hints/ex04-level1.md) - Custom types and methods
- [Calculator Project](../hints/calculator-level1.md) - Complete CLI application

**🎯 Ready for the next challenge?** Move on to [Module 02: Ownership and Borrowing](../../02-ownership-and-borrowing/README.md) when you've completed all exercises!
