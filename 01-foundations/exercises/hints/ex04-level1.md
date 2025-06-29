# Exercise 4 Hints - Level 1 (Gentle Nudges)

## 🔍 Getting Started

You're working on structs and enums in `ex04-structs.rs`. The compilation errors will teach you Rust's approach to custom types and methods.

## 💡 General Approach

1. **Read the error message completely** - Rust's struct/enum errors are very descriptive
2. **Fix ONE error at a time** - Start with type definitions, then methods
3. **Compare with C#** - Classes vs structs, enums vs enums
4. **Think about ownership** - Who owns the data in your structs?

## 🎯 Hint 1: Struct Definition

**Error**: Cannot find type `StructName` in scope

**C# Context**: In C#, you'd define `class Person { public string Name; }`

**Rust Difference**: Uses `struct` keyword and different syntax.

**Gentle Hint**: Look at the lesson materials. How do you define a struct with fields in Rust? What keyword defines custom types?

## 🎯 Hint 2: Method Implementation

**Error**: No method named `method_name` found

**C# Context**: In C#, methods are defined inside the class body.

**Rust Difference**: Methods are defined in separate `impl` blocks.

**Gentle Hint**: What keyword do you use to add methods to a struct? How do you reference the struct instance in methods?

## 🎯 Hint 3: Constructor Patterns

**Error**: No associated function or constructor found

**C# Context**: In C#, you'd have constructors like `public Person(string name) { }`

**Rust Difference**: Rust uses associated functions, typically named `new`.

**Gentle Hint**: How do you create "constructor-like" functions in Rust? What's the difference between `&self` and `Self`?

## 🎯 Hint 4: Enum Variants

**Error**: Expected enum variant or cannot find enum

**C# Context**: In C#, enums are simple: `enum Color { Red, Green, Blue }`

**Rust Difference**: Rust enums can hold data and are much more powerful.

**Gentle Hint**: How do you define enum variants that can hold different types of data? How do you match on enum variants?

## 🚀 Success Criteria

You're on the right track when:
- ✅ Structs are defined with proper field types
- ✅ Methods are implemented in `impl` blocks
- ✅ Constructor functions return `Self`
- ✅ Enum variants are properly matched

## ➡️ Next Level

Still stuck? Try [Level 2 Hints](ex04-level2.md) for more specific guidance.

Remember: Rust's type system gives you powerful abstraction tools! 🦀
