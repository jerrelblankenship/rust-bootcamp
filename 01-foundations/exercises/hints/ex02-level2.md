# Exercise 2 Hints - Level 2 (More Specific Guidance)

## 🎯 Specific Solutions

You've tried Level 1 hints but need more guidance. Here are more specific solutions for common type system issues:

## 🔧 Fix 1: Variable Declaration

**Problem**: Using undefined variables
**Solution**: Add `let` before variable names:
```rust
let x = 42;
let name = "Alice";
let is_rust_fun = true;
```

**Explanation**: 
- `let` declares variables in Rust
- Variables are immutable by default
- Type inference often works automatically

## 🔧 Fix 2: Making Variables Mutable

**Problem**: `cannot assign to immutable variable`
**Solution**: Add `mut` keyword:
```rust
let mut counter = 0;
counter = counter + 1;  // Now this works
counter += 5;           // This also works
```

**Explanation**:
- `let mut` makes variables changeable
- Immutability by default prevents accidental changes
- You must be explicit about mutation

## 🔧 Fix 3: Type Annotations

**Problem**: `cannot infer type`
**Solution**: Add type annotations with colon:
```rust
let age: i32 = 25;
let height: f64 = 5.9;
let initial: char = 'R';
```

**Explanation**:
- Syntax: `variable_name: type = value`
- Common types: `i32`, `f64`, `bool`, `char`, `String`
- Use when compiler can't infer the type

## 🔧 Fix 4: Type Conversions

**Problem**: `mismatched types`
**Solution**: Use `as` keyword for conversions:
```rust
let x: i32 = 10;
let y: i64 = x as i64;  // Convert i32 to i64
```

**Explanation**:
- Rust doesn't do implicit type conversions
- Use `as` for safe numeric conversions
- Some conversions require methods like `.to_string()`

## 🔧 Fix 5: Tuple and Array Access

**Problem**: Wrong indexing syntax
**Solution**: Use dot notation for tuples, brackets for arrays:
```rust
let tuple = (1, 2, 3);
let first = tuple.0;  // Not tuple[0]

let array = [1, 2, 3, 4];
let slice = &array[1..3];  // Slice syntax
```

**Explanation**:
- Tuples: `.0`, `.1`, `.2` etc.
- Arrays: `[index]` or `[start..end]` for slices
- Slices need `&` prefix

## 📋 Compilation Checklist

After each fix, run:
```bash
rustc ex02-types.rs
```

You should see fewer errors each time.

## ➡️ Next Level

Still having trouble? Try [Level 3 Hints](ex02-level3.md) for nearly complete solutions.

## 🎓 Key Learning Points

- Variables declared with `let`, mutable with `let mut`
- Type annotations use colon syntax: `name: type`
- Explicit conversions with `as` keyword
- Tuples use dot notation, arrays use bracket notation

Keep going - you're learning Rust's safety-first approach! 🦀
