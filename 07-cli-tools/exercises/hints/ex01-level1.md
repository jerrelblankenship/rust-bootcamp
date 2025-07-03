# Exercise 1 Hints - Level 1: Gentle Guidance 🟢

## Getting Started with Argument Parsing

**Stuck on clap imports or basic setup?** Let's think through this step by step.

### 🤔 First Questions to Ask Yourself

1. **What's the first compilation error you see?**
   - Missing dependencies are common - check your Cargo.toml
   - Import errors usually mean you need to add crates

2. **How would you handle command line arguments in C#?**
   - In C#: `CommandLineParser.Default.ParseArguments<Options>(args)`
   - In Rust: We use clap with derive macros for similar functionality

3. **What's different about Rust CLI parsing?**
   - Rust emphasizes compile-time correctness
   - clap derive macros generate the parsing code for you
   - Similar to how DataAnnotations work in C# but more powerful

### 🔍 General Approach

1. **Start with dependencies**: What does the code import that might be missing?
2. **Fix one error at a time**: Don't try to fix everything at once
3. **Read the compile errors**: Rust's compiler is incredibly helpful
4. **Think incremental**: Get basic parsing working, then add features

### 💡 Key Concepts

**Derive Macros**: Think of these like C#'s attributes, but they generate code:
```rust
// This generates argument parsing code automatically
#[derive(Parser)]
struct Args { /* fields */ }
```

**Global Options**: Some options apply to all subcommands:
```rust
#[clap(short, long, global = true)]
verbose: bool,  // Available everywhere
```

### 🎯 Focus Areas

- **Dependencies**: What crates do you need in Cargo.toml?
- **Basic Structure**: How do you define a CLI structure?
- **Subcommands**: How do you organize complex command hierarchies?

### ⏱️ Time Check

If you've been working for **15+ minutes** and still stuck on basic imports or dependencies, check the Level 2 hints.

### 🚀 Next Steps

1. Look at the imports - what's missing from Cargo.toml?
2. Try to get the basic structure compiling first
3. Add subcommands once the foundation works

Remember: Every expert was once a beginner. The goal is learning through fixing!