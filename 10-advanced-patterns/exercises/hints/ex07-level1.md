# Exercise 07 - Level 1 Hints: Pin Projection

## 🎯 What's Going Wrong?

Your Pin implementations are broken! Pin is used for self-referential structs and async code - types that can't be moved safely in memory because they contain pointers to themselves.

## 🔍 First Steps

1. **Try to compile** and see what breaks:
   ```bash
   rustc ex07-pin-projection.rs
   ```

2. **Read the error messages** - Look for Pin-related errors

3. **Identify the error categories**:
   - Missing `PhantomPinned` markers
   - Incorrect Pin usage in methods
   - Self-referential struct initialization issues
   - Pin projection problems

## 🤔 Think About It

- **C# Analogy**: Like GCHandle.Pinned preventing object movement during garbage collection
- **Key Question**: Why can't these structs be moved after creation?
- **Strategy**: Understand self-referential types and memory safety

## 🔧 What to Research

1. **Pin and Unpin** - What they do and when to use them
2. **PhantomPinned** - Marking types as not movable
3. **Self-referential structs** - Why they're dangerous
4. **Pin projection** - Accessing fields of pinned types

## 📚 Resources to Use

- **Rust Async Book** - Pin and Unpin
- **std::pin documentation** - Pin details
- **Rust Nomicon** - Self-referential structs

## 🎮 Systematic Approach

1. **Understand Pin concept** - Learn why it exists
2. **Add PhantomPinned markers** - Mark non-movable types
3. **Fix Pin method signatures** - Use Pin<&mut Self>
4. **Fix self-reference initialization** - Do it safely
5. **Test with pin projection** - Verify field access

## ⏰ Time Check

Spent 15 minutes? If you're confused about Pin vs Unpin, move to Level 2 for specific guidance.

**Hint**: Look for structs that have pointer fields pointing to other fields in the same struct - these need `PhantomPinned`!