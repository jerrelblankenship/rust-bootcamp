# Exercise 1 Hints - Level 1 (Gentle Nudges)

## 🔍 Getting Started

You're seeing compilation errors in `ex01-hello-world.rs`. This is perfect! Let's work through them systematically.

## 💡 General Approach

1. **Read the error message completely** - Rust's compiler is very helpful
2. **Fix ONE error at a time** - Don't try to fix everything at once
3. **Compile after each fix** - `rustc ex01-hello-world.rs`
4. **Compare with C#** - Many concepts are similar

## 🎯 Hint 1: The Missing Exclamation Mark

**Error**: `println` function not found

**C# Context**: In C#, you'd write `Console.WriteLine("Hello")`. 

**Rust Difference**: `println` in Rust is actually a *macro*, not a function.

**Gentle Hint**: Macros in Rust end with an exclamation mark. Look at working Rust examples - what's different about the `println` syntax?

## 🎯 Hint 2: Variable Declaration  

**Error**: Cannot find value `name` in this scope

**C# Context**: In C#, you'd declare: `string name = "Alice";`

**Rust Difference**: Variable declaration uses a different keyword.

**Gentle Hint**: Look at the lesson material about variables. What keyword does Rust use instead of `string name`?

## 🎯 Hint 3: Format Specifiers

**Error**: Wrong number of format arguments

**C# Context**: In C#, you'd use `$"Hello {name}"` or `"Hello {0}", name`

**Rust Difference**: Rust has its own formatting syntax.

**Gentle Hint**: Look at working examples in the lesson. How do you put variables inside printed strings in Rust?

## 🚀 Success Criteria

You're on the right track when:
- ✅ `rustc ex01-hello-world.rs` compiles without errors
- ✅ `./ex01-hello-world` runs and prints your name
- ✅ Debug printing shows your data structures

## ➡️ Next Level

Still stuck? Try [Level 2 Hints](ex01-level2.md) for more specific guidance.

Remember: The goal is understanding, not just making it compile! 🦀
