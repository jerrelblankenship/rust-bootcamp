# Exercise 08 - Coverage Gaps: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Don't know what's not tested"
**Think about:**
- How do you measure test coverage in C#? (Think Visual Studio, dotCover)
- What tools exist in Rust for coverage analysis?
- What's the difference between line coverage and branch coverage?

### Checkpoint 2: "Missing error cases"
**Questions to consider:**
- What can go wrong in your functions?
- Are you testing both happy path and error scenarios?
- What edge cases exist for your input parameters?

### Checkpoint 3: "Not testing edge cases"
**Reflect on:**
- What are the boundary values for your inputs?
- What happens with empty collections, null values, or extreme numbers?
- How does your code behave at the limits?

### Checkpoint 4: "Complex conditions untested"
**Consider:**
- Do you have if/else statements with multiple conditions?
- Are all combinations of boolean logic tested?
- What about nested conditional logic?

### Checkpoint 5: "State transitions missing"
**Think about:**
- If your code has states, are all transitions tested?
- What invalid state changes might occur?
- How does your code handle unexpected state combinations?

### Checkpoint 6: "Integration paths incomplete"
**Ask yourself:**
- Are you testing all code paths through your system?
- What happens when external dependencies fail?
- Do you test the interaction between components?

## 🤔 Still Stuck?

Try these exploration steps:
1. Run `cargo tarpaulin` to see coverage reports
2. Look for untested branches in your if/match statements
3. Think about what could fail in each function
4. Consider using mutation testing to validate test quality

Remember: High coverage doesn't guarantee good tests, but low coverage definitely indicates gaps!