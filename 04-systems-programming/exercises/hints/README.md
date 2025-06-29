# Exercise Hints - Module 04: Systems Programming

This directory contains progressive hints for systems programming exercises. These concepts require careful attention to memory safety and low-level details.

## 🎯 How to Use These Hints

1. **Attempt the exercise first** - Try to understand unsafe code and memory layout errors
2. **Level 1 hints** - Gentle guidance on systems programming concepts
3. **Level 2 hints** - More specific solutions with safety explanations
4. **Level 3 hints** - Nearly complete implementations with detailed safety comments
5. **Research safety patterns** - Look up established unsafe code patterns when stuck

## 📚 Available Hints

- [Exercise 1: Memory Layout](ex01-level1.md) - Struct padding and memory optimization
- [Exercise 2: Unsafe Debugging](ex02-level1.md) - Safe abstractions over unsafe code
- [Exercise 3: C Interop](ex03-level1.md) - FFI and C string handling
- [System Monitor Project](system-monitor-level1.md) - Production systems programming

## 🔒 Systems Programming Philosophy

**Systems programming is about precise control** - these hints help you build intuition for:

### Core Systems Questions:
1. **"Is this operation safe?"** - Every unsafe block needs justification
2. **"What are the safety invariants?"** - Document assumptions clearly
3. **"How can I minimize unsafe surface area?"** - Safe wrappers over unsafe internals
4. **"What could go wrong if my assumptions are violated?"** - Think about edge cases

### When to Use Hints:
- ✅ After trying for 15+ minutes with compiler messages
- ✅ When you understand the safety issue but need guidance on solutions
- ✅ When choosing between different systems programming approaches
- ✅ When unsafe code or FFI concepts feel overwhelming

### When NOT to Use Hints:
- ❌ Immediately when seeing an unsafe or FFI error
- ❌ As a substitute for understanding memory safety concepts
- ❌ When you haven't read the lesson material
- ❌ Before trying to understand what makes the operation unsafe

## 🔄 From C# Managed to Rust Systems

The mental model shift is significant:

**C# Managed Environment:**
- Garbage collector handles memory
- P/Invoke for unsafe operations
- Runtime safety checks
- Hidden memory layout

**Rust Systems Programming:**
- Explicit memory management
- Unsafe blocks for precise control
- Compile-time safety verification
- Direct control over memory layout

## 🚀 Building Systems Programming Intuition

**Key Differences from C# unsafe:**
- **More restrictive**: Only 4 specific unsafe superpowers
- **Better documented**: Safety contracts are explicit
- **Easier to audit**: Unsafe surface area is minimized
- **Still memory safe**: Other safety checks remain active

**Remember:** Every unsafe block you write correctly makes you a better systems programmer. Rust's approach prevents entire classes of vulnerabilities while giving you the control you need.

The challenge is learning to think about safety invariants explicitly - embrace it! 🦀
