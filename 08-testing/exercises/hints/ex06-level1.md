# Exercise 06 - Property Tests: Level 1 Hints 🌱

## Gentle Nudges

### Checkpoint 1: "Don't know what to test"
**Think about:**
- In traditional testing, you test specific examples - what's different here?
- What properties should always be true regardless of input?
- Think of mathematical properties like commutativity or associativity

### Checkpoint 2: "Can't import proptest"
**Questions to consider:**
- Have you added proptest to your Cargo.toml?
- What section should test-only dependencies go in?
- Are you using the right proptest version?

### Checkpoint 3: "Strategy syntax confusing"
**Reflect on:**
- What types of random data do you need to generate?
- How would you describe the range of valid inputs?
- What's the Rust equivalent of C#'s test data generators?

### Checkpoint 4: "Tests too slow"
**Consider:**
- How many test cases is proptest running by default?
- Can you configure the number of test iterations?
- What's the trade-off between thoroughness and speed?

### Checkpoint 5: "Shrinking not working"
**Think about:**
- What is shrinking in property-based testing?
- Why would you want minimal failing examples?
- How does proptest know how to shrink your custom types?

### Checkpoint 6: "Properties too weak"
**Ask yourself:**
- Are your properties actually catching bugs?
- What invariants should your code maintain?
- Could a buggy implementation still pass your properties?

## 🤔 Still Stuck?

Try these exploration steps:
1. Read about QuickCheck (the original property testing tool)
2. Think about round-trip properties (encode/decode, serialize/deserialize)
3. Consider properties from the problem domain
4. Look for mathematical properties in your algorithms

Remember: Property testing finds edge cases you didn't think of - it's like fuzz testing with structure!