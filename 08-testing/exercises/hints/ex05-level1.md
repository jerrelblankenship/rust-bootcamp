# Exercise 05 - Async Tests: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Test doesn't run"
**Think about:**
- In C#, async tests return `Task` - what about Rust?
- What attribute do you need besides `#[test]` for async?
- Which async runtime should tests use?

### Checkpoint 2: "Can't await in test"
**Questions to consider:**
- What makes a function async in Rust?
- How do test frameworks know to handle async tests?
- What crate provides async test support?

### Checkpoint 3: "Blocking in async"
**Reflect on:**
- What's the difference between blocking and async I/O?
- In C#, what happens when you use `.Result` on a Task?
- Why is blocking bad in async contexts?

### Checkpoint 4: "Tests hanging"
**Consider:**
- What could cause an async function to never complete?
- How do you add timeouts to async operations?
- What's the async equivalent of a deadlock?

### Checkpoint 5: "Can't test timeouts"
**Think about:**
- How would you test timeout behavior in C#?
- Can you control time in tests?
- What patterns exist for testing time-dependent async code?

### Checkpoint 6: "Parallel async tests failing"
**Ask yourself:**
- Do async tests run concurrently by default?
- What resources might async tests share?
- How do you coordinate async test execution?

## 🤔 Still Stuck?

Try these exploration steps:
1. Look up `tokio::test` macro documentation
2. Consider what `.await` actually does
3. Think about async vs sync I/O operations
4. Explore `tokio::time` utilities for testing

Remember: Async testing in Rust is like async/await in C# but with explicit runtime management!