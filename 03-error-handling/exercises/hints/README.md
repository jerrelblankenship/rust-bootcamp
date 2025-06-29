# Exercise Hints - Module 03: Error Handling

This directory contains progressive hints for error handling exercises. Rust's error handling is fundamentally different from C#'s exception model - these hints help bridge that gap.

## 🎯 How to Use These Hints

1. **Attempt the exercise first** - Try to understand what error handling pattern is needed
2. **Level 1 hints** - Gentle guidance on Result/Option patterns  
3. **Level 2 hints** - More specific solutions with examples
4. **Level 3 hints** - Nearly complete implementations with explanations
5. **Ask instructor** - If you're still stuck after Level 3 hints

## 📚 Available Hints

- [Exercise 1: Option Basics](ex01-level1.md) - Handling null-like values safely
- [Exercise 2: Result Chains](ex02-level1.md) - Error propagation and chaining
- [Exercise 3: Error Types](ex03-level1.md) - Custom error design patterns
- [Exercise 4: Conversions](ex04-level1.md) - Error type conversions and From traits
- [File Processor Project](file-processor-level1.md) - Production error handling

## 🤝 Error Handling Learning Philosophy

**Error handling in Rust is revolutionary** - errors are values, not exceptions. These hints help you build intuition for:

### Core Error Handling Questions:
1. **"Can this operation fail?"** - Use Result<T, E> if yes, Option<T> if maybe-missing
2. **"Should errors propagate up?"** - Use ? operator for automatic propagation
3. **"What should happen on error?"** - Recover, retry, or abort gracefully
4. **"How specific should my errors be?"** - Balance detail with usability

### When to Use Hints:
- ✅ After trying for 15+ minutes with compiler messages
- ✅ When you understand the error but need guidance on patterns
- ✅ When choosing between Option, Result, and panic strategies
- ✅ When error handling feels overwhelming

### When NOT to Use Hints:
- ❌ Immediately when seeing an Option/Result error
- ❌ As a substitute for understanding the concepts
- ❌ When you haven't read the lesson material
- ❌ Before trying to understand what the operation should return

## 🔄 From C# Exceptions to Rust Values

The mental model shift is significant:

**C# Exception Model:**
```csharp
try {
    var result = RiskyOperation();
    Console.WriteLine(result);
} catch (SpecificException ex) {
    Console.WriteLine($"Error: {ex.Message}");
} catch (Exception ex) {
    Console.WriteLine($"Unexpected: {ex.Message}");
}
```

**Rust Result Model:**
```rust
match risky_operation() {
    Ok(result) => println!("{}", result),
    Err(e) => match e {
        SpecificError::KnownProblem(msg) => println!("Error: {}", msg),
        SpecificError::UnexpectedProblem(msg) => println!("Unexpected: {}", msg),
    }
}
```

## 🚀 Building Error Handling Intuition

**Key Differences:**
- **Explicit in function signatures**: `fn might_fail() -> Result<T, E>`
- **Cannot be ignored**: Compiler forces you to handle errors
- **Errors are data**: Pattern match, transform, and propagate as values
- **No hidden control flow**: No exceptions jumping up the call stack
- **Performance**: Zero overhead when no errors occur

**Remember:** Every error handling pattern you master makes your code more robust and predictable. Rust prevents entire classes of runtime failures!

The struggle is learning to think of errors as data rather than exceptions - embrace it! 🦀
