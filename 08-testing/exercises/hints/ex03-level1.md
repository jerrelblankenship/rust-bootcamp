# Exercise 03 - Integration Tests: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Test doesn't compile"
**Think about:**
- Where do integration tests live in a Rust project?
- In C#, you might have separate test projects - where does Rust put them?
- What's the difference between unit tests and integration tests in terms of file location?

### Checkpoint 2: "Can't import modules"
**Questions to consider:**
- How do integration tests see your library code?
- What's the difference between `crate::` and `use my_crate::`?
- Think about what integration tests are testing - the public API!

### Checkpoint 3: "Setup not working"
**Reflect on:**
- What happens if multiple tests try to set up the same resources?
- How would you handle test fixtures in C#?
- When do you need to clean up after tests?

### Checkpoint 4: "Test file ignored"
**Consider:**
- How does Rust know which files in `tests/` to run?
- What makes a valid integration test file?
- Are there naming conventions to follow?

### Checkpoint 5: "Can't share code"
**Think about:**
- If you have helper functions for tests, where do they go?
- How do multiple integration test files share code?
- What's special about subdirectories in `tests/`?

### Checkpoint 6: "Database tests flaky"
**Ask yourself:**
- What makes integration tests different from unit tests?
- How do you ensure test isolation with external resources?
- What cleanup strategies exist for integration tests?

## 🤔 Still Stuck?

Try these exploration steps:
1. Look at the `tests/` directory structure in popular Rust crates
2. Run `cargo test --test specific_test_name` to run one integration test
3. Check if your test files are being discovered with `cargo test -- --list`
4. Consider what "integration" really means - testing the public interface!

Remember: Integration tests in Rust test your library as an external user would use it!