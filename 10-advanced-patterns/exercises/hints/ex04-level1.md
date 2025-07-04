# Exercise 04 - Level 1 Hints: HRTB Headaches

## 🎯 What's Going Wrong?

Your higher-ranked trait bounds (HRTB) are broken! This is one of Rust's most advanced features, used when you need closures or functions to work with any lifetime.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex04-hrtb-headaches.rs
   ```

2. **Read the error messages** - Look for lifetime mismatch errors

3. **Identify the error categories**:
   - Missing `for<'a>` syntax for higher-ranked bounds
   - Incorrect lifetime relationships in function signatures
   - Closures that don't work with arbitrary lifetimes
   - Trait bounds that need lifetime polymorphism

## 🤔 Think About It

- **C# Analogy**: Like generic delegates that need to work with any reference type
- **Key Question**: Why do these functions need to work with any lifetime?
- **Strategy**: Understand when HRTB is needed before fixing syntax

## 🔧 What to Research

1. **Higher-Ranked Trait Bounds** - The `for<'a>` syntax
2. **Lifetime Polymorphism** - Functions that work with any lifetime
3. **Closure Lifetimes** - How closures interact with lifetimes
4. **When HRTB is needed** - Specific use cases and patterns

## 📚 Resources to Use

- **Rust Nomicon** - Higher-Ranked Trait Bounds
- **Rust Reference** - Lifetime bounds
- **Error messages** - The compiler explains lifetime issues

## 🎮 Systematic Approach

1. **Understand HRTB syntax** - Learn `for<'a>` patterns
2. **Identify where HRTB is needed** - Functions accepting closures
3. **Fix closure constraints** - Add proper lifetime bounds
4. **Handle trait object lifetimes** - Ensure they work with any lifetime
5. **Test with different lifetimes** - Verify flexibility

## ⏰ Time Check

Spent 15 minutes? If you're confused about when to use HRTB, move to Level 2 for specific guidance.

**Hint**: Look for functions that take closures with `&str` parameters - they likely need `for<'a> Fn(&'a str)` bounds!