# Exercise 04 - Mock Troubles: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Can't mock concrete type"
**Think about:**
- In C#, you might use interfaces like `IEmailService` - what's the Rust equivalent?
- What makes a type "mockable" in any language?
- How does dependency injection work without interfaces?

### Checkpoint 2: "Mock setup is too manual"
**Questions to consider:**
- What mocking libraries exist for Rust? (Hint: one rhymes with "sockall")
- How do C# mocking frameworks like Moq work under the hood?
- What code generation capabilities does Rust have?

### Checkpoint 3: "Can't verify calls"
**Reflect on:**
- What information do you need to track to verify method calls?
- How would you implement call counting manually?
- What patterns help with test assertions?

### Checkpoint 4: "Async mocking is broken"
**Consider:**
- What's different about mocking async functions?
- How do async traits work in Rust?
- What additional complexity does async add to mocking?

### Checkpoint 5: "Can't mock external crates"
**Think about:**
- Can you mock types you don't own in C#?
- What's the "newtype" pattern?
- How can wrapper types help with testing?

### Checkpoint 6: "Mock state isn't isolated"
**Ask yourself:**
- What happens when mocks are shared between tests?
- How do you ensure each test gets a fresh mock?
- What's the difference between mock instances and mock types?

## 🤔 Still Stuck?

Try these exploration steps:
1. Look up the `mockall` crate documentation
2. Consider creating a trait for your dependencies
3. Think about how you'd manually implement a test double
4. Explore the Adapter pattern for external dependencies

Remember: Rust's trait system is like C# interfaces but more powerful for mocking!