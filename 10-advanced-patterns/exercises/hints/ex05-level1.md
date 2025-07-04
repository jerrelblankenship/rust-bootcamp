# Exercise 05 - Level 1 Hints: Phantom Problems

## 🎯 What's Going Wrong?

Your phantom types are broken! Phantom types are used for compile-time type safety without runtime overhead. They're like generic type parameters that exist only to help the compiler.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex05-phantom-problems.rs
   ```

2. **Read the error messages** - Look for PhantomData usage errors

3. **Identify the error categories**:
   - Missing `PhantomData` fields in structs
   - Incorrect phantom data initialization
   - Wrong type parameters in phantom types
   - State transition type mismatches

## 🤔 Think About It

- **C# Analogy**: Like generic type parameters that exist only for compile-time safety
- **Key Question**: Why do these types need phantom parameters?
- **Strategy**: Understand zero-cost abstractions and type-level programming

## 🔧 What to Research

1. **PhantomData** - `std::marker::PhantomData<T>`
2. **Zero-cost abstractions** - How they work in Rust
3. **Type-level programming** - Using types to encode state
4. **Compile-time state tracking** - State machines with phantom types

## 📚 Resources to Use

- **Rust Book** - PhantomData documentation
- **Rust by Example** - Phantom types
- **std::marker documentation** - PhantomData details

## 🎮 Systematic Approach

1. **Understand phantom types** - Learn what they're for
2. **Fix PhantomData usage** - Add missing fields
3. **Fix type parameters** - Ensure correct phantom types
4. **Fix state transitions** - Return correct phantom types
5. **Test zero-cost property** - Verify no runtime overhead

## ⏰ Time Check

Spent 15 minutes? If you're confused about PhantomData usage, move to Level 2 for specific guidance.

**Hint**: Look for structs with generic parameters that aren't used in fields - they need `PhantomData<T>` to tell the compiler about the unused type!