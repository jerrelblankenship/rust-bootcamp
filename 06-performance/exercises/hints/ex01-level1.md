# Exercise 1 Hints - Level 1 (Gentle Nudges)

## 🤔 Stuck on Allocation Storm? Here are some gentle hints...

### Checkpoint 1: Vec Creation in Loop
- Look at line 50: `let mut temp = Vec::new();`
- Think about what happens when you create 1000 new Vec objects
- In C#, this would be like `new List<int>()` inside a `foreach` loop
- **Question**: Do you really need a new Vec for each chunk?

### Checkpoint 2: String Concatenation
- Line 86: `result = result + &format!(...)`
- This creates a new String each time (like C# `string +=`)
- **Question**: What's the Rust equivalent of C# `StringBuilder.Append()`?

### Checkpoint 3: Boxing Primitives
- Line 111: `Vec<Box<i32>>`
- **Question**: Why would you put a 4-byte integer on the heap?
- Think about memory overhead: each `Box<i32>` needs heap allocation

### Checkpoint 4: Excessive Cloning
- Lines 143-145: Multiple `.clone()` calls
- **Question**: Can you work with references instead of owned values?

### Checkpoint 5: HashMap Capacity
- Line 173: `HashMap::new()`
- **Question**: What happens when a HashMap grows beyond its initial capacity?

### Checkpoint 6: Buffer Allocation in Hot Path
- Line 206: `Vec::new()` inside the loop
- **Question**: Can you reuse the same buffer across iterations?

## 💡 General Strategy
1. **Profile first**: Run the code and see which checkpoint is slowest
2. **One at a time**: Fix the worst bottleneck first
3. **Measure impact**: Each fix should show dramatic improvement
4. **Think C#**: Apply similar optimizations you'd use in C#

Still stuck? Try Level 2 hints! 🚀