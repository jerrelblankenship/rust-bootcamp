# Exercise 03 - Level 1 Hints: Trait Object Trouble

## 🎯 What's Going Wrong?

Your trait objects are failing to compile! This is common when learning Rust's object safety rules. The compiler is protecting you from runtime errors that would be hard to debug.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex03-trait-object-trouble.rs
   ```

2. **Read the error messages** - Look for "object safety" violations

3. **Identify the error categories**:
   - Traits with generic methods (can't be object-safe)
   - Traits with associated types (can't be trait objects)
   - Traits with static methods (no `self` parameter)
   - Missing `dyn` keyword in trait object syntax

## 🤔 Think About It

- **C# Analogy**: Like interfaces that can't be used as object references due to generic methods
- **Key Question**: Why can't these traits be converted to trait objects?
- **Strategy**: Understand object safety rules before fixing syntax

## 🔧 What to Research

1. **Object Safety Rules** - What makes a trait object-safe?
2. **Dynamic vs Static Dispatch** - When to use each
3. **Trait Object Syntax** - `Box<dyn Trait>` vs `Box<Trait>`
4. **Alternative Patterns** - Enums, generics, or trait redesign

## 📚 Resources to Use

- **Rust Book Chapter 17** - Object-Oriented Programming Features
- **Rust Reference** - Object Safety
- **Error messages** - The compiler explains object safety violations

## 🎮 Systematic Approach

1. **Understand object safety** - Learn the rules first
2. **Fix syntax issues** - Add `dyn` keyword where needed
3. **Handle generic methods** - Remove or redesign them
4. **Fix static methods** - Remove or move to separate traits
5. **Test trait object creation** - Ensure boxes and references work

## ⏰ Time Check

Spent 15 minutes? If you're confused about object safety rules, move to Level 2 for specific guidance.

**Hint**: Start by adding `dyn` to all trait object types, then tackle the object safety violations one by one!