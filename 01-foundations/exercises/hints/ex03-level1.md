# Exercise 3 Hints - Level 1 (Gentle Nudges)

## 🔍 Getting Started

You're working on functions and control flow in `ex03-functions.rs`. The compilation errors will teach you Rust's approach to functions and pattern matching.

## 💡 General Approach

1. **Read the error message completely** - Rust's function errors are very descriptive
2. **Fix ONE error at a time** - Start with the first compilation error
3. **Compare with C#** - Function concepts are similar but with different syntax
4. **Think about return types** - Rust is expression-based

## 🎯 Hint 1: Function Signatures

**Error**: Missing parameter types or return type

**C# Context**: In C#, you'd write `public int Add(int a, int b) { return a + b; }`

**Rust Difference**: Uses `fn` keyword and different syntax for types.

**Gentle Hint**: Look at the lesson materials. How do you specify parameter types and return types in Rust functions?

## 🎯 Hint 2: Pattern Matching

**Error**: Non-exhaustive patterns or missing match arms

**C# Context**: In C#, you'd use `switch` statements or `if-else` chains.

**Rust Difference**: `match` expressions must handle all possible cases.

**Gentle Hint**: What keyword does Rust use instead of `switch`? What do you need to handle cases that aren't explicitly listed?

## 🎯 Hint 3: Option and Result Types

**Error**: Expected `Option<T>` or `Result<T, E>` handling

**C# Context**: In C#, you'd check for null or use try-catch blocks.

**Rust Difference**: Rust makes null checking and error handling explicit.

**Gentle Hint**: How do you handle values that might be missing (`Option`) or operations that might fail (`Result`)?

## 🎯 Hint 4: Return Values

**Error**: Mismatched return type or missing return

**C# Context**: In C#, you'd use `return` statements explicitly.

**Rust Difference**: Rust functions can return expressions without `return`.

**Gentle Hint**: Look at the lesson about expression-based returns. When do you need `return` and when can you omit it?

## 🚀 Success Criteria

You're on the right track when:
- ✅ Function signatures compile with proper parameter and return types
- ✅ `match` expressions handle all cases
- ✅ `Option` and `Result` types are handled properly
- ✅ Return values match the function signature

## ➡️ Next Level

Still stuck? Try [Level 2 Hints](ex03-level2.md) for more specific guidance.

Remember: Rust's function system prevents many runtime errors! 🦀
