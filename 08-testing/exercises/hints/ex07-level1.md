# Exercise 07 - Test Fixtures: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Duplicate test setup"
**Think about:**
- In C#, you might use test class setup methods - what patterns exist in Rust?
- How can you share setup code between tests?
- What's the DRY principle telling you here?

### Checkpoint 2: "Test data inconsistent"
**Questions to consider:**
- Are you hard-coding test data in each test?
- What happens when your data model changes?
- How would a builder pattern help here?

### Checkpoint 3: "Setup getting complex"
**Reflect on:**
- Is your test setup longer than the actual test?
- How would you create a "valid user with all fields" easily?
- What's the Object Mother pattern?

### Checkpoint 4: "Can't reset between tests"
**Consider:**
- What state persists between test runs?
- How do you ensure test isolation?
- What cleanup patterns exist?

### Checkpoint 5: "Fixtures not composable"
**Think about:**
- Can you combine simple fixtures to create complex ones?
- How would method chaining help?
- What makes a good builder API?

### Checkpoint 6: "Different scenarios hard"
**Ask yourself:**
- How do you create variations of test data?
- What if you need a user with specific attributes?
- Can your fixtures be parameterized?

## 🤔 Still Stuck?

Try these exploration steps:
1. Look up the Builder pattern in Rust
2. Consider how game testing creates test scenarios
3. Think about factory methods for common test objects
4. Explore the concept of test data builders

Remember: Good test fixtures make tests readable and maintainable - they're like factories for test data!