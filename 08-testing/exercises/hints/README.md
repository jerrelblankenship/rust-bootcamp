# Progressive Hint System 🎯

Struggling with test patterns? Follow this hint progression:

## When to Use Hints

1. **First 15 minutes**: Read test output and error messages
2. **After 15 minutes**: Check Level 1 hints
3. **After 30 minutes**: Check Level 2 hints  
4. **After 45 minutes**: Check Level 3 hints

## Hint Levels

### 🟢 Level 1: Testing Concepts
- Points to testing best practices
- Asks about test isolation
- References familiar C# testing patterns

### 🟡 Level 2: Rust-Specific Patterns
- Shows test organization structures
- Explains assertion macros
- Demonstrates mocking strategies

### 🔴 Level 3: Complete Examples
- Provides working test structure
- Shows advanced patterns
- Leaves some implementation details

## Testing Philosophy

1. **Test Behavior, Not Implementation** - What, not how
2. **Fast and Isolated** - Each test independent
3. **Clear Failure Messages** - Know what broke
4. **Comprehensive Coverage** - Happy path + edge cases
5. **Maintainable Tests** - Easy to update

## Example Hint Progression

**Exercise**: Fix failing mock test
- **Level 1**: "How do you mock interfaces in C#? What's needed in Rust?"
- **Level 2**: "You need a trait to mock. Like C# interfaces but..."
- **Level 3**: "Define trait `Database`, then use `#[automock]`..."

Remember: Good tests are documentation. They show how code should be used!