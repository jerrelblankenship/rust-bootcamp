# Exercise 02 - Test Organization: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Test won't run - what's missing?"
**Think about:**
- In C#, test methods need `[Test]` or `[Fact]` attributes
- What's the Rust equivalent for marking test functions?
- Look at the function name - is it just missing something?

### Checkpoint 2: "Tests module not recognized"
**Questions to consider:**
- How does Rust know which modules contain tests vs production code?
- In C#, test projects are separate - how does Rust handle this?
- What attribute tells Rust to only compile this module for testing?

### Checkpoint 3: "Testing private functions"
**Reflect on:**
- Can tests in C# access private methods directly?
- Where do Rust tests live relative to the code they test?
- How does module visibility work in Rust?

### Checkpoint 4: "Import errors in tests"
**Consider:**
- What does `super::*` mean in Rust?
- How do you bring parent module items into scope?
- Think about the module hierarchy

### Checkpoint 5: "Can't mock system time"
**Think about:**
- What makes code hard to test?
- How would you test time-dependent code in C#?
- What pattern allows dependency injection?

### Checkpoint 6: "Parallel tests interfering"
**Ask yourself:**
- What happens when tests share mutable state?
- How does Rust run tests by default?
- What might tests be sharing that causes conflicts?

## 🤔 Still Stuck?

Try these exploration steps:
1. Run `cargo test -- --nocapture` to see all output
2. Look at how other Rust projects organize their tests
3. Try moving the test to different locations in the file
4. Consider what makes tests independent vs dependent

Remember: Test organization in Rust is different from C# but serves similar purposes!