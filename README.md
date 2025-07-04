# 🦀 Rust Bootcamp for C# Developers

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![C#](https://img.shields.io/badge/c%23-%23239120.svg?style=flat&logo=c-sharp&logoColor=white)](https://docs.microsoft.com/en-us/dotnet/csharp/)

> **A comprehensive curriculum designed specifically for C# developers transitioning to Rust**

**🤖 Built with Claude Desktop** - This bootcamp demonstrates AI-assisted curriculum development, creating a structured learning path that strategically leverages your existing C# knowledge to accelerate Rust mastery.

## 🎯 Learning Objectives

By the end of this bootcamp, you will:
**Learning Philosophy**: **70% Doing, 30% Teaching** - Master Rust through guided discovery and real compiler errors.

- ✅ Master Rust's ownership model and understand how it guarantees memory safety without garbage collection
- ✅ Build high-performance, concurrent applications that rival C/C++ in speed
- ✅ Create robust command-line tools with excellent error handling
- ✅ Profile and optimize Rust applications for maximum performance
- ✅ Understand systems programming concepts and low-level operations
- ✅ Leverage Rust's powerful type system for compile-time correctness
- ✅ Apply Rust idioms and best practices in real-world projects

### ✅ **Built for C# Developers**
- Leverages your existing programming knowledge
- Direct C# ↔ Rust comparisons throughout
- Enterprise-focused examples and use cases

### ✅ **Learn by Fixing Broken Code**
- Every exercise starts with compilation errors
- Rust's excellent compiler messages guide your learning
- Build intuition through debugging, not reading

### ✅ **Progressive Skill Building**
- Each module builds on the previous one
- Real projects apply concepts immediately
- Professional debugging and troubleshooting support

## 📋 **Quick Start Paths**

### **New to Rust? Start Here** (Recommended)
```bash
cd 01-foundations/exercises
rustc ex01-hello-world.rs  # Fix basic syntax errors
# Follow the compilation errors to learn!
```

### **Want the Big Picture First?**
```bash
# Skim the concepts, then jump to coding
ls */reference/  # Detailed explanations
# Then: cd 01-foundations/exercises
```

### **Experienced Developer? Jump In**
```bash
cd 02-ownership-and-borrowing/exercises
rustc ex01-ownership.rs  # Challenge yourself!
```

## 🗺️ **Learning Journey**

| Module | Focus | What You'll Build |
|--------|-------|-------------------|
| **[01 - Foundations](01-foundations/)** | Syntax & Types | Working Calculator CLI |
| **[02 - Ownership](02-ownership-and-borrowing/)** | Memory Model | Memory Visualizer Tool |
| **[03 - Error Handling](03-error-handling/)** | Robust Code | File Processing System |
| **[04 - Systems Programming](04-systems-programming/)** | Low-Level Control | System Resource Monitor |
| **[05 - Concurrency](05-concurrency/)** | Fearless Concurrency | Multi-threaded Web Scraper |
| **[06 - Performance](06-performance/)** | Profiling & Optimization | High-Performance Image Processor |
| **[07 - CLI Tools](07-cli-tools/)** | Building Developer Tools | Professional Dev Toolkit |
| **[08 - Testing](08-testing/)** | Comprehensive Testing | Custom Testing Framework |
| **[09 - Ecosystem](09-ecosystem/)** | Crates & Libraries | Published Rust Library |
| **[10 - Advanced Patterns](10-advanced-patterns/)** | Macros & Meta-programming | Advanced Macro System |

### **Capstone Project**
| Project | Focus | Challenge Level |
|---------|-------|-----------------|
| **[Final Project](final-project/)** | Concurrent Web Server | Integrates All Modules |

## 💡 **Learning Approach**

### **The "Broken Code" Method**
1. **Encounter real compilation errors** - Not artificial examples
2. **Read Rust's helpful error messages** - Learn to collaborate with the compiler
3. **Fix one error at a time** - Build understanding incrementally
4. **Apply concepts immediately** - Reinforce through practice

### **C# Knowledge Leverage**
```rust
// You know this in C#:
public int Add(int a, int b) { return a + b; }

// Learn this in Rust:
fn add(a: i32, b: i32) -> i32 { a + b }
```

### **Progressive Hints System**
- **Level 1**: Gentle nudges in the right direction
- **Level 2**: Specific guidance with examples
- **Level 3**: Nearly complete solutions with explanations

## 🏆 **Success Metrics**

You're succeeding when:
- ✅ **Compilation errors become helpful** - Not frustrating obstacles
- ✅ **You think in ownership** - Natural mental model for memory safety
- ✅ **Error handling feels natural** - No more null reference fears
- ✅ **Systems concepts click** - Understanding low-level control

## 🔧 **Prerequisites & Setup**

### **Required Knowledge**
- Professional C# experience (any level)
- Basic command-line familiarity
- Understanding of programming fundamentals

### **Setup (5 minutes)**
1. **Install Rust**: [rustup.rs](https://rustup.rs/)
2. **VS Code + rust-analyzer** (recommended)
3. **Clone this repo** and start coding!

**Platform Support**: ✅ macOS • ✅ Windows 11 • ✅ Linux • ✅ [Containers](containers/)

## 🎯 **Real-World Applications**

### **Enterprise Use Cases You'll Master**
- **High-performance APIs** - Zero-cost abstractions
- **System utilities** - CLI tools and automation
- **Data processing** - ETL pipelines and analytics
- **Legacy integration** - C library interoperability
- **Concurrent services** - Thread-safe shared state

## 🆘 **When You Get Stuck**

### **Built-in Support System**
1. **Compiler messages first** - Rust's error messages are exceptionally helpful
2. **Module debugging guides** - Systematic troubleshooting for each topic
3. **Progressive hints** - Guided discovery without giving away answers
4. **Reference materials** - Deep dives when you're ready

### **Community Resources**
- 💬 [Rust Official Discord](https://discord.gg/rust-lang)
- 📚 [The Rust Book](https://doc.rust-lang.org/book/)
- 🤝 [Rust Users Forum](https://users.rust-lang.org/)

### **Additional Learning Resources**
- 📖 [Books & Courses](resources/) - Curated learning materials
- 🔧 [Troubleshooting Guide](resources/troubleshooting.md) - Common issues and solutions
- 🎯 [Project Ideas](resources/README.md#-project-ideas) - Continue learning after the bootcamp

## 🔍 **What Makes This Different**

### **vs. Traditional Tutorials**
- ❌ **Traditional**: Read → Example → Exercise
- ✅ **This Bootcamp**: Fix → Understand → Apply

### **vs. Generic Rust Resources**
- ❌ **Generic**: Learn Rust syntax and concepts
- ✅ **This Bootcamp**: Leverage C# knowledge for accelerated learning

### **vs. Theoretical Learning**
- ❌ **Theoretical**: Study memory management concepts
- ✅ **This Bootcamp**: Debug real ownership violations

## 📁 **Repository Structure**

```
rust-bootcamp/
├── 00-setup/                # Platform-specific setup guides
├── 01-foundations/          # Syntax, types, functions
│   ├── exercises/          # Broken code to fix
│   ├── project-calculator/ # Working calculator CLI
│   └── reference/         # Detailed explanations
├── 02-ownership-and-borrowing/
│   ├── exercises/          # Ownership violation debugging
│   ├── project-memory-visualizer/
│   └── reference/
├── 03-error-handling/      # Option<T> and Result<T,E>
├── 04-systems-programming/ # Unsafe, FFI, memory layout
├── 05-concurrency/         # Threads, async/await, channels
├── 06-performance/         # Profiling and optimization
├── 07-cli-tools/           # Building command-line tools
├── 08-testing/             # Testing strategies
├── 09-ecosystem/           # Crates and libraries
├── 10-advanced-patterns/   # Macros and meta-programming
├── final-project/          # Capstone web server project
├── containers/             # Docker/Podman setup
├── reference/              # Performance comparisons
└── resources/              # Additional learning materials
```

## 🚀 **Ready to Start?**

### **🎯 Beginner Path**
```bash
cd 01-foundations/
# Read the 2-minute README, then start fixing code!
```

### **🏃 Accelerated Path**
```bash
cd 02-ownership-and-borrowing/exercises/
rustc ex01-ownership.rs  # Jump into ownership debugging
```

### **📚 Reference-First Path**
```bash
ls */reference/  # Skim concepts first
# Then dive into exercises when ready
```

---

<div align="center">

### **🦀 Transform Your C# Skills into Rust Mastery**

**[🚀 Start Learning Now](01-foundations/) • [📖 Browse Concepts](01-foundations/reference/) • [⚡ Performance vs C#](reference/performance-comparisons.md) • [🔧 Setup Guide](00-setup/)**

**⭐ Star this repo if it helps you learn Rust!**

</div>

---

*"The best way to learn Rust is to write Rust. The best way to write Rust is to fix broken Rust code and let the compiler teach you."*
