# Exercise 2 Hints - Level 1 (Gentle Nudges)

## 🔍 Getting Started

You're working on type system concepts in `ex02-types.rs`. The compilation errors will teach you Rust's approach to types and mutability.

## 💡 General Approach

1. **Read the error message completely** - Rust's type errors are very descriptive
2. **Fix ONE error at a time** - Start with the first compilation error
3. **Compare with C#** - Many type concepts translate directly
4. **Think about mutability** - Rust is immutable by default

## 🎯 Hint 1: Variable Declaration

**Error**: Missing variable declaration

**C# Context**: In C#, you'd write `var x = 42;` or `int x = 42;`

**Rust Difference**: Uses `let` keyword and is immutable by default.

**Gentle Hint**: What keyword does Rust use to declare variables? Check the lesson materials for the basic syntax.

## 🎯 Hint 2: Mutability

**Error**: Cannot assign to immutable variable

**C# Context**: In C#, variables are mutable by default: `int x = 5; x = 10;`

**Rust Difference**: Variables are immutable by default in Rust.

**Gentle Hint**: Look for a keyword that makes variables changeable. What do you add to `let` to make a variable mutable?

## 🎯 Hint 3: Type Annotations

**Error**: Missing type annotation

**C# Context**: In C#, you might write `int age = 25;` or use `var` for inference.

**Rust Difference**: Sometimes you need to be explicit about types.

**Gentle Hint**: Type annotations in Rust use a colon. How do you tell Rust what type a variable should be?

## 🚀 Success Criteria

You're on the right track when:
- ✅ Variables are declared with `let`
- ✅ Mutable variables use `let mut`
- ✅ Type annotations work with colon syntax
- ✅ All compilation errors are resolved

## ➡️ Next Level

Still stuck? Try [Level 2 Hints](ex02-level2.md) for more specific guidance.

Remember: Rust's type system prevents runtime errors! 🦀
