# Module 01: Foundations

Welcome to your Rust learning journey! This module introduces core Rust concepts by building on your existing C# knowledge. You'll learn through hands-on problem solving, fixing broken code, and building real projects.

## 🎯 What You'll Learn

By the end of this module, you'll be able to:

- Write and run Rust programs with confidence
- Understand Rust's type system and how it differs from C#
- Work with variables, functions, and control flow in Rust
- Create custom data types using structs and enums
- Handle errors gracefully with Result and Option types
- Build a complete CLI calculator application

## 📚 Learning Materials

### **Concept Lessons**
Start with these to understand the fundamentals:

1. **[Hello Rust](01-hello-rust.md)** - Your first Rust program with C# comparisons
2. **[Variables and Types](02-variables-and-types.md)** - Understanding Rust's type system
3. **[Functions and Control Flow](03-functions-and-flow.md)** - Functions, loops, and conditions
4. **[Structs and Enums](04-structs-and-enums.md)** - Creating custom data types

### **Hands-On Practice**
Apply what you've learned by fixing broken code:

- **ex01-hello-world.rs** - Basic syntax and printing (broken code to fix)
- **ex02-types.rs** - Type system exploration (broken code to fix)
- **ex03-functions.rs** - Functions and control flow (broken code to fix)
- **ex04-structs.rs** - Custom types and methods (broken code to fix)
- **ex05-advanced-text-analysis.rs** - Advanced example (working code to study)

### **Major Project**
- **Calculator CLI** - Build a complete command-line calculator by fixing broken starter code

## 🔄 For C# Developers

Here's how familiar C# concepts map to Rust:

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| `class` | `struct` + `impl` | No inheritance, use composition |
| `interface` | `trait` | More powerful, includes default implementations |
| `null` | `Option<T>` | Explicit null handling, no crashes |
| `Exception` | `Result<T, E>` | Errors are values, not exceptions |
| `using` | `use` | Module system instead of namespaces |
| `var` | `let` | Immutable by default |

## 🚀 Getting Started

### **Step 1: Read the First Lesson**
Start with [Hello Rust](01-hello-rust.md) to understand the basics.

### **Step 2: Try the Exercises**
The exercises contain broken code that you need to fix:

```bash
cd 01-foundations/exercises
rustc ex01-hello-world.rs   # This will show compilation errors
```

Your job is to fix the errors by reading the error messages and implementing the missing code.

### **Step 3: Build the Calculator**
```bash
cd project-calculator
cargo build  # This will also show compilation errors to fix
```

Start with simple fixes and gradually implement the full calculator.

## 💡 Learning Approach

### **How the Exercises Work**
- Each exercise file contains broken code with TODO comments
- Compilation errors guide you to what needs to be fixed
- Start with the first error and fix them one by one
- Use the Rust compiler's helpful error messages

### **How to Succeed**
1. **Read error messages carefully** - Rust's compiler is very helpful
2. **Fix one error at a time** - Don't try to fix everything at once
3. **Use your C# knowledge** - Many concepts translate directly
4. **Don't skip the lessons** - They explain the concepts you'll need
5. **Ask for help when stuck** - Check the solutions/ directory

## 📈 Your Learning Path

### **Day 1: Get Comfortable with Rust Syntax**
- Complete the "Hello Rust" lesson
- Fix compilation errors in exercise 1
- Understand basic Rust syntax differences from C#

### **Day 2: Master Types and Functions**
- Study "Variables and Types" and "Functions and Control Flow" lessons
- Fix exercises 2 and 3
- Start the calculator project

### **Day 3: Build Custom Types and Complete Your Calculator**
- Learn "Structs and Enums"
- Fix exercise 4
- Complete the calculator project
- Study the advanced text analysis example

## 🏆 Success Criteria

You've mastered this module when:
- ✅ All exercises compile and run without errors
- ✅ Your calculator project works for basic arithmetic
- ✅ You can explain the key differences between C# and Rust
- ✅ You're comfortable reading and fixing Rust compilation errors

## 📁 Module Structure

```
01-foundations/
├── README.md                    # This guide
├── 01-hello-rust.md            # First lesson
├── 02-variables-and-types.md   # Type system lesson
├── 03-functions-and-flow.md    # Functions lesson
├── 04-structs-and-enums.md     # Custom types lesson
├── exercises/
│   ├── ex01-hello-world.rs     # Exercise 1 (broken code)
│   ├── ex02-types.rs           # Exercise 2 (broken code)
│   ├── ex03-functions.rs       # Exercise 3 (broken code)
│   ├── ex04-structs.rs         # Exercise 4 (broken code)
│   ├── ex05-advanced-text-analysis.rs  # Advanced example
│   └── solutions/
│       └── README.md           # Help when you're stuck
└── project-calculator/         # Major project
    ├── src/main.rs             # Broken starter code to fix
    ├── tests/                  # Tests to verify your work
    ├── solutions/              # Complete working solution
    └── README.md               # Project guide
```

## 🆘 When You Get Stuck

**Follow this systematic approach:**

1. **Read the error message carefully** - Rust's compiler is exceptionally helpful
2. **Check the [Debugging Checklist](DEBUGGING_CHECKLIST.md)** - Systematic troubleshooting guide
3. **Use the progressive hints system** - Check `exercises/hints/` directory
   - Level 1: Gentle nudges in the right direction
   - Level 2: More specific guidance with examples  
   - Level 3: Nearly complete solutions with explanations
4. **Review the lesson material** - Make sure you understand the concepts
5. **Ask for help** - After working through the hints system

**Remember:** The struggle is where learning happens! Use resources to guide discovery, not skip thinking.

## 🔗 Additional Resources

### **Built-in Learning Support:**
- [Debugging Checklist](DEBUGGING_CHECKLIST.md) - Systematic error troubleshooting
- [Progressive Hints](exercises/hints/README.md) - Guided discovery system
- [Exercise Solutions](exercises/instructor-only/README.md) - For instructors only

### **External Resources:**
- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- [Rust Playground](https://play.rust-lang.org/) - Try code online

## ➡️ What's Next?

After completing this module, you'll be ready for [Module 02: Ownership and Borrowing](../02-ownership-and-borrowing/README.md), where you'll learn Rust's most unique and powerful feature!

---

**Ready to start?** Begin with [Hello Rust](01-hello-rust.md) and let's write some Rust code! 🦀
