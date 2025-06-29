# Rust Bootcamp for C# Developers

Welcome to an intensive 3-week Rust bootcamp designed specifically for C# developers! This curriculum leverages your existing programming expertise while introducing you to Rust's unique approach to systems programming, memory safety, and performance.

## 🎯 Learning Objectives

By the end of this bootcamp, you will:

- Master Rust's ownership model and understand how it guarantees memory safety without garbage collection
- Build high-performance, concurrent applications that rival C/C++ in speed
- Create robust command-line tools with excellent error handling
- Profile and optimize Rust applications for maximum performance
- Understand systems programming concepts and low-level operations
- Leverage Rust's powerful type system for compile-time correctness
- Apply Rust idioms and best practices in real-world projects

## 📚 Prerequisites

- **Professional C# and .NET experience** (any level - from junior to senior) ✓
- Understanding of object-oriented programming concepts
- Familiarity with command-line interfaces
- Git knowledge for version control
- 2-3 hours daily commitment for 3 weeks

**Note**: This bootcamp is designed to leverage your existing C# knowledge regardless of your years of experience. Whether you're a recent bootcamp graduate or a seasoned architect, your understanding of C# concepts will accelerate your Rust learning journey.

## 🚀 Quick Start

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/rust-bootcamp.git
   cd rust-bootcamp
   ```

2. Follow the setup guide for your platform:
   - [macOS M4 Setup](00-setup/macos-m4-setup.md)
   - [Windows 11 Setup](00-setup/windows-11-setup.md)

3. Configure your development environment:
   - [VS Code Configuration](00-setup/vscode-configuration.md)
   - [Container Setup (Optional)](00-setup/container-setup.md)

## 📅 Bootcamp Schedule

### Week 1: Foundations (Days 1-7) ✅ COMPLETE
**Focus**: Core Rust concepts and the ownership model

- **Days 1-2**: Environment setup and Rust fundamentals ✅
  - Hello Rust, variables, data types, functions
  - Pattern matching and error handling basics
  - **Project**: CLI Calculator (Complete implementation)

- **Days 3-4**: Ownership and Borrowing ✅ **COMPLETE**
  - Understanding ownership, moves, and copies
  - Borrowing rules and lifetimes
  - Smart pointers (Box, Rc, Arc, RefCell)
  - **Project**: Memory Visualizer (**Complete Interactive Implementation**)

- **Days 5-7**: Error Handling ✅ FULLY IMPLEMENTED
  - Result<T, E> and Option<T> types
  - Error propagation and custom errors
  - Unit and integration testing
  - **Project**: Robust File Processor (Complete with CLI)

### Week 2: Systems Programming (Days 8-14) ✅ CORE CONTENT COMPLETE
**Focus**: Low-level programming, concurrency, and performance

- **Days 8-9**: Systems Programming ✅ IMPLEMENTED
  - Memory layout and unsafe Rust
  - Foreign Function Interface (FFI)
  - Direct system calls and safety patterns
  - **Project**: System Resource Monitor (Outlined)

- **Days 10-12**: Concurrency and Parallelism ✅ IMPLEMENTED
  - Threading model comparison with C#
  - Message passing with channels
  - Shared state concurrency (Mutex, RwLock, Atomics)
  - Lock-free programming patterns
  - **Project**: Parallel Data Processor (Thread pools, examples)

- **Days 13-14**: Performance Optimization 📋 PLANNED
  - Profiling with perf and flamegraph
  - Rust-specific optimizations
  - Benchmarking with Criterion
  - **Project**: Performance Benchmark Suite

### Week 3: Advanced Topics (Days 15-21)
**Focus**: Real-world applications and advanced patterns

- **Days 15-16**: CLI Tools Development
  - Building with Clap
  - Configuration management
  - Terminal UI design
  - **Project**: Task Runner (Make alternative)

- **Days 17-18**: Rust Ecosystem
  - Advanced Cargo usage
  - Popular crates overview
  - Publishing your own crates
  - **Project**: Reusable Crate Template

- **Days 19-20**: Advanced Patterns
  - Trait patterns and type state
  - Builder pattern and DSLs
  - Interior mutability patterns
  - **Project**: Domain-Specific Language

- **Day 21**: Final Project
  - **High-Performance Web Server**
  - Combining all learned concepts
  - Performance comparison with C# implementation

## 🔄 Learning Approach

This bootcamp uses a unique approach tailored for developers with C# experience:

1. **Comparative Learning**: Each concept is introduced with C# comparisons to leverage your existing knowledge
2. **Hands-On Focus**: Every module includes practical exercises and projects
3. **Performance-Oriented**: Benchmark and profile your code against C# equivalents
4. **Systems Thinking**: Understand what happens "under the hood"
5. **Real-World Applications**: Build tools you'll actually use

## 🛠️ Development Environment

- **Primary IDE**: VS Code with rust-analyzer
- **Container Support**: Podman configurations included
- **Platform Support**: Specific instructions for M4 MacBook Pro and Windows 11
- **Debugging**: Integrated debugging setup for both platforms

## 📊 Current Status

**Phase 1 Complete** ✅ (Essential Content)
- **Module 01**: Foundations - **COMPLETE** with calculator project ✅
- **Module 02**: Ownership & Borrowing - **COMPLETE** with memory visualizer ✅
- **Module 03**: Error Handling - **FULLY IMPLEMENTED** with file processor ✅
- **Module 04**: Systems Programming - Core lessons on memory & unsafe Rust
- **Module 05**: Concurrency - Threads, channels, shared state, atomics

**Ready for Students**: The bootcamp now contains substantial, high-quality content covering Rust's core differentiating features for C# developers.

### ✅ **Fully Complete Modules:**

#### **Module 01: Foundations** 
- ✅ 4 comprehensive lessons with C# comparisons
- ✅ 5 complete exercises (50+ individual problems)
- ✅ Complete CLI calculator project with advanced features
- ✅ 20+ integration tests and comprehensive documentation

#### **Module 02: Ownership and Borrowing**
- ✅ 4 detailed lessons on ownership, borrowing, lifetimes, smart pointers
- ✅ 5 complete exercises (40+ ownership scenarios)
- ✅ **Interactive Memory Visualizer project** with:
  - Real-time ownership tracking
  - Borrow checker simulation
  - Visual memory state representation
  - Educational demonstrations
  - Cross-platform runner scripts
- ✅ Comprehensive test coverage and documentation

## 🤝 Community and Support

- Join the discussion in [Issues](https://github.com/yourusername/rust-bootcamp/issues)
- Share your projects and get feedback
- Find study partners for pair programming
- Access the [Troubleshooting Guide](resources/troubleshooting.md)

## 📖 Recommended Resources

- **The Rust Programming Language** (included in resources)
- **Rust for Rustaceans** by Jon Gjengset
- **Programming Rust** by Blandy, Orendorff, and Tindall
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)

## 🎯 Final Project: High-Performance Web Server

The culmination of your learning journey is building a production-ready web server that demonstrates:

- Concurrent request handling with async/await
- Zero-copy networking operations
- Custom protocol implementation
- Performance metrics and monitoring
- Graceful shutdown and error recovery

This server will be benchmarked against a C# implementation to showcase Rust's performance advantages in systems programming.

## 🚦 Getting Started

Ready to begin your Rust journey? Start with [Module 00: Setup](00-setup/README.md) to configure your development environment, then dive into [Module 01: Foundations](01-foundations/README.md).

**New learners**: The first two modules are **100% complete** and ready for intensive learning:
1. **[Module 01: Foundations](01-foundations/README.md)** - Master Rust basics with hands-on exercises
2. **[Module 02: Ownership](02-ownership-and-borrowing/README.md)** - Understand Rust's unique memory model

Remember: Rust has a learning curve, but your C# experience gives you a significant advantage. Whether you're coming from years in enterprise development or fresh from learning C#, you already understand many programming concepts that will accelerate your Rust journey. Embrace the compiler as your ally—it's there to help you write correct, performant code!

---

*"In Rust, the compiler is your friend, not your enemy. It catches at compile-time what other languages catch at runtime (or not at all)."*
