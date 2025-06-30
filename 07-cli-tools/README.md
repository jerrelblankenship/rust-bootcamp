# Module 07: Command Line Tools

Build professional CLI tools by fixing broken command-line applications. Master argument parsing, error handling, and user experience!

## 🎯 Learning Objectives

By fixing broken CLI tools, you will:
- Master `clap` for argument parsing through compiler errors
- Fix error handling to provide helpful user messages
- Debug cross-platform path and terminal issues
- Repair broken configuration management
- Convert C# console apps to idiomatic Rust CLIs
- Build a powerful developer toolchain

## 🚀 Quick Start

```bash
# Start with the first exercise
cd exercises
cargo run --bin ex01-arg-parser -- --help

# When it crashes or shows poor UX, that's your learning opportunity!
# Fix the issues one by one to create delightful CLI tools
```

## 📚 Module Overview

**Your C# Experience**: You've built console applications, used `CommandLineParser`, and handled `Console` I/O.
**What's Different**: Rust CLIs compile to tiny, fast binaries with zero dependencies. Ship one file!

## 💪 Exercises - Fix These Broken CLI Tools!

Each exercise contains a broken CLI tool. Your mission: make it user-friendly and robust!

1. **ex01-arg-parser.rs** - Fix clap argument parsing errors
2. **ex02-error-handling.rs** - Improve terrible error messages
3. **ex03-config-files.rs** - Fix configuration loading issues
4. **ex04-pipe-friendly.rs** - Make tool work in Unix pipes
5. **ex05-progress-bars.rs** - Add missing progress indication
6. **ex06-color-output.rs** - Fix terminal color detection
7. **ex07-cross-platform.rs** - Resolve Windows/Unix incompatibilities
8. **ex08-subcommands.rs** - Repair git-style subcommand structure

## 🏗️ Project: Developer Tool Suite

Fix a broken developer tool suite that should:
- Parse complex command structures (like `git` or `cargo`)
- Provide excellent error messages with suggestions
- Support configuration files and environment variables
- Show progress for long operations
- Work perfectly on Windows, macOS, and Linux

**Starting State**: Crashes on basic usage!
**Your Goal**: A tool so good you'll use it daily!

## 🧰 Debugging Toolkit

- **[DEBUGGING_CHECKLIST.md](DEBUGGING_CHECKLIST.md)** - Common CLI pitfalls and fixes
- **Hint System** - Progressive CLI hints in `exercises/hints/`
- **Reference Docs** - Best practices in `reference/`

## 🎮 Learning Path

1. **Start here**: `exercises/ex01-arg-parser.rs`
2. **Try to use it**: Run with various arguments
3. **Stuck?** Wait 15 minutes, then check `hints/ex01-level1.md`
4. **Test thoroughly**: Try edge cases and bad input
5. **All exercises done?** Build the dev tool suite!

## 🏆 Victory Conditions

You've mastered this module when you can:
- [ ] Fix all 8 exercises creating polished CLIs
- [ ] Complete the developer tool suite
- [ ] Build CLIs with excellent user experience
- [ ] Handle all edge cases gracefully
- [ ] Create cross-platform tools confidently

## 📂 Module Structure

```
07-cli-tools/
├── README.md                          # You are here!
├── DEBUGGING_CHECKLIST.md             # CLI debugging guide
├── exercises/
│   ├── ex01-arg-parser.rs             # Argument parsing issues
│   ├── ex02-error-handling.rs         # Poor error messages
│   ├── ex03-config-files.rs           # Config management
│   ├── ex04-pipe-friendly.rs          # Unix pipe compatibility
│   ├── ex05-progress-bars.rs          # Missing feedback
│   ├── ex06-color-output.rs           # Terminal detection
│   ├── ex07-cross-platform.rs         # Platform issues
│   ├── ex08-subcommands.rs            # Complex commands
│   └── hints/
│       ├── README.md                  # How to use hints
│       ├── ex01-level1.md             # Gentle guidance
│       ├── ex01-level2.md             # Specific patterns
│       ├── ex01-level3.md             # Near-complete solution
│       └── ... (3 levels per exercise)
├── project-dev-tools/
│   ├── Cargo.toml                     # Dependencies ready
│   ├── README.md                      # Project instructions
│   ├── src/
│   │   ├── main.rs                    # Broken entry point
│   │   ├── commands/                  # Subcommand modules
│   │   └── config.rs                  # Config management
│   └── tests/
│       └── cli.rs                     # Integration tests
└── reference/
    ├── README.md                      # Additional resources
    ├── clap-patterns.md               # Argument parsing patterns
    ├── error-design.md                # Error message UX
    ├── config-hierarchy.md            # Config best practices
    └── csharp-console.md              # Console app comparison
```

---

Ready to build CLI tools people love to use? Start with `exercises/ex01-arg-parser.rs`! 🚀