# Module 08: Testing Strategies

Master Rust testing by fixing broken test suites. Learn unit testing, integration testing, mocking, and property-based testing!

## 🎯 Learning Objectives

By fixing broken tests, you will:
- Master Rust's built-in testing framework
- Fix test organization and isolation issues
- Debug async test problems
- Repair mocking and dependency injection
- Convert xUnit/NUnit patterns to Rust idioms
- Build a comprehensive testing framework

## 🚀 Quick Start

```bash
# Start with the first exercise
cd exercises
cargo test --bin ex01-basic-tests

# When tests fail or won't compile, that's your learning moment!
# Fix the test issues to understand testing best practices
```

## 📚 Module Overview

**Your C# Experience**: You've used xUnit, NUnit, or MSTest with mocking frameworks like Moq.
**What's Different**: Rust's testing is built into the language with zero-cost test organization!

## 💪 Exercises - Fix These Broken Test Suites!

Each exercise contains broken tests. Your mission: make them compile, pass, and follow best practices!

1. **ex01-basic-tests.rs** - Fix basic test setup and assertions
2. **ex02-test-organization.rs** - Repair module structure issues
3. **ex03-integration-tests.rs** - Fix integration test problems
4. **ex04-mock-troubles.rs** - Debug mocking and trait issues
5. **ex05-async-tests.rs** - Fix async test runtime problems
6. **ex06-property-tests.rs** - Repair property-based tests
7. **ex07-test-fixtures.rs** - Implement test data builders
8. **ex08-coverage-gaps.rs** - Add missing test coverage

## 🏗️ Project: Testing Framework

Fix a broken testing framework that should:
- Support custom assertions with good error messages
- Provide test fixtures and data builders
- Enable powerful mocking capabilities
- Include property-based testing helpers
- Generate useful test reports

**Starting State**: Tests don't even compile!
**Your Goal**: A testing toolkit that makes TDD a joy!

## 🧰 Debugging Toolkit

- **[DEBUGGING_CHECKLIST.md](DEBUGGING_CHECKLIST.md)** - Common test pitfalls and fixes
- **Hint System** - Progressive testing hints in `exercises/hints/`
- **Reference Docs** - Testing patterns in `reference/`

## 🎮 Learning Path

1. **Start here**: `exercises/ex01-basic-tests.rs`
2. **Run tests**: Use `cargo test` with various flags
3. **Stuck?** Wait 15 minutes, then check `hints/ex01-level1.md`
4. **Check coverage**: Ensure comprehensive testing
5. **All exercises done?** Build the testing framework!

## 🏆 Victory Conditions

You've mastered this module when you can:
- [ ] Fix all 8 exercises with proper test patterns
- [ ] Complete the testing framework project
- [ ] Write fast, isolated, maintainable tests
- [ ] Mock external dependencies effectively
- [ ] Test async code confidently

## 📂 Module Structure

```
08-testing/
├── README.md                          # You are here!
├── DEBUGGING_CHECKLIST.md             # Testing debugging guide
├── exercises/
│   ├── ex01-basic-tests.rs            # Test basics
│   ├── ex02-test-organization.rs      # Module structure
│   ├── ex03-integration-tests.rs      # Integration patterns
│   ├── ex04-mock-troubles.rs          # Mocking issues
│   ├── ex05-async-tests.rs            # Async testing
│   ├── ex06-property-tests.rs         # Property-based tests
│   ├── ex07-test-fixtures.rs          # Test data patterns
│   ├── ex08-coverage-gaps.rs          # Missing coverage
│   └── hints/
│       ├── README.md                  # How to use hints
│       ├── ex01-level1.md             # Testing concepts
│       ├── ex01-level2.md             # Rust patterns
│       ├── ex01-level3.md             # Solutions
│       └── ... (3 levels per exercise)
├── project-testing-suite/
│   ├── Cargo.toml                     # Test dependencies
│   ├── README.md                      # Project instructions
│   ├── src/
│   │   ├── lib.rs                     # Testing utilities
│   │   ├── assertions.rs              # Custom assertions
│   │   ├── fixtures.rs                # Test builders
│   │   └── mocks.rs                   # Mocking helpers
│   └── tests/
│       └── framework_tests.rs         # Test the framework
└── reference/
    ├── README.md                      # Additional resources
    ├── test-organization.md           # Structuring tests
    ├── mocking-patterns.md            # Mock strategies
    ├── async-testing.md               # Async test patterns
    └── csharp-comparison.md           # xUnit vs Rust
```

---

Ready to write bulletproof tests? Start with `exercises/ex01-basic-tests.rs`! 🧪