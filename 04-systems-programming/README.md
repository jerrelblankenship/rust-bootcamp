# Module 04: Systems Programming

Dive into low-level programming with Rust, exploring memory management, unsafe code, and foreign function interfaces.

## 🎯 Learning Objectives

After completing this module, you will:
- Understand memory layout and representation in Rust
- Write safe abstractions over unsafe code
- Interface with C libraries using FFI
- Make direct system calls
- Work with raw pointers safely
- Understand when and how to use unsafe Rust

## 📚 Module Overview

Systems programming is where Rust truly shines. You get the control of C/C++ with the safety guarantees of a high-level language.

## 📖 Lessons

1. **[Memory Layout](01-memory-layout.md)** - How Rust represents data in memory
2. **[Unsafe Rust](02-unsafe-rust.md)** - When and how to use unsafe code
3. **[Foreign Function Interface](03-ffi.md)** - Calling C from Rust and vice versa
4. **[Low-Level I/O](04-low-level-io.md)** - Direct system calls and OS interfaces

## 💻 Project: System Resource Monitor

Build a system monitor that:
- Reads CPU and memory usage directly from `/proc` (Linux) or system APIs
- Uses unsafe code for performance-critical sections
- Interfaces with system libraries
- Provides real-time updates

## 🔄 C# to Rust Systems Programming

| C# Concept | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| `unsafe` blocks | `unsafe` blocks | More restricted in Rust |
| P/Invoke | FFI | More explicit, safer |
| `fixed` statement | Pin API | Different memory model |
| `stackalloc` | Arrays, `alloca` | Rust prefers stack allocation |
| Pointers | Raw pointers | Must be in unsafe blocks |
| `Marshal` class | `std::mem` | Manual memory management |

## 🛡️ Safety First

Remember: Unsafe doesn't mean "turn off all safety checks." It means:
- Dereferencing raw pointers
- Calling unsafe functions
- Accessing mutable statics
- Implementing unsafe traits
- Accessing union fields

The goal is to write minimal unsafe code wrapped in safe abstractions.

## 📊 Module Structure

```
04-systems-programming/
├── README.md
├── 01-memory-layout.md
├── 02-unsafe-rust.md
├── 03-ffi.md
├── 04-low-level-io.md
├── exercises/
│   ├── ex01-memory-exploration.rs
│   ├── ex02-unsafe-abstractions.rs
│   ├── ex03-c-interop.rs
│   └── ex04-syscalls.rs
└── project-system-monitor/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── cpu.rs
    │   ├── memory.rs
    │   └── ffi/
    └── README.md
```

---

Ready? Let's explore the depths of systems programming with Rust!
