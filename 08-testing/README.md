# Module 08: Testing Strategies

🎯 **Mission**: Fix broken test suites to master Rust testing!

## 🚀 Quick Start

1. **Start coding immediately**:
   ```bash
   cd 08-testing/exercises
   cargo test --bin ex01-basic-tests  # Shows failing tests to fix!
   ```

2. **Fix one failure at a time** - Let the test output guide you
3. **Use hints only when stuck** - Check `hints/` directory
4. **Build the testing framework** - Apply what you've learned

## 📝 What You'll Master

Through **fixing broken tests**, you'll learn:
- ✅ Rust's built-in testing framework
- ✅ Test organization and isolation patterns
- ✅ Async testing strategies
- ✅ Mocking and dependency injection
- ✅ Property-based testing

## 🔧 Learning Path

### **Step 1: Fix the Exercises**
```bash
# Fix failing tests one by one
cargo test --bin ex01-basic-tests  # Basic assertions
cargo test --bin ex02-test-organization  # Module structure
cargo test --bin ex03-integration-tests  # Integration patterns
cargo test --bin ex04-mock-troubles  # Mocking strategies
# ... continue through ex08
```

### **Step 2: Build the Testing Framework**
```bash
cd project-testing-suite
cargo build  # Shows errors to fix
cargo test   # Verify your framework
cargo run --example demo  # Test your testing tools!
```

## 🆘 When You Get Stuck

1. **Read the test output** - Rust's test runner provides excellent feedback
2. **Check [Debugging Guide](DEBUGGING_CHECKLIST.md)** - Common testing pitfalls
3. **Use progressive hints** - `hints/ex01-level1.md` → `level2.md` → `level3.md`
4. **Compare with C#** - xUnit/NUnit patterns often translate directly

## 🏆 Success = Passing Tests

You've mastered this module when:
- ✅ All test exercises compile and pass
- ✅ Testing framework works: `cargo test` in project shows green
- ✅ You can write tests for any Rust code
- ✅ You understand mocking, async testing, and property testing

## 📚 Need More Detail?

- 📖 **[Detailed Concepts](reference/)** - Testing patterns and strategies
- 🔄 **[C# vs Rust Testing](reference/csharp-comparison.md)** - xUnit/NUnit translation guide
- 📋 **[Troubleshooting](DEBUGGING_CHECKLIST.md)** - When tests go wrong

## 🏗️ Project: Testing Framework

**Mission**: Fix a broken testing framework that needs:
- Custom assertions with helpful error messages
- Test fixtures and data builders
- Powerful mocking capabilities
- Property-based testing helpers
- Useful test reports

**Current State**: Tests won't even compile!
**Your Goal**: A testing toolkit that makes TDD enjoyable!

---

**Ready?** Start with: `cd exercises && cargo test --bin ex01-basic-tests` 🧪

**Next Module**: [09 - Ecosystem](../09-ecosystem/README.md) →