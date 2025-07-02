# Exercise 5 Hints - Level 1 (Gentle Nudges)

## 🤔 Stuck on bounds checking? Here are some gentle hints...

### Checkpoint 1: Array Indexing Performance
- Look at line with `data[i]` - what happens on every access?
- Think about C# - is `foreach` faster than `for(int i=0...)`?
- **Question**: Do you really need to access by index?

### Checkpoint 2: Slice Operations
- Line with `&data[i..i+3]` - that's a lot of bounds checking
- **Question**: Is there an iterator method for sliding windows?
- **Hint**: Look up `windows()` method on slices

### Checkpoint 3: Matrix Access
- Manual index calculation: `row * SIZE + col` plus bounds check
- **Question**: Can you iterate over the flat array directly?

### Checkpoint 4: Hot Path Optimization
- Three array accesses per iteration: `data[i-1], data[i], data[i+1]`
- **Question**: In performance-critical code, is safety worth the cost?
- **Note**: Only consider `unsafe` after measuring it's actually a bottleneck

## 💡 General Strategy
1. **Try iterator methods first** - they often eliminate bounds checking
2. **Measure the impact** - bounds checking overhead varies
3. **Profile before optimizing** - don't assume where the bottleneck is
4. **Safety first** - only use `unsafe` when necessary and proven safe

Still stuck? Try Level 2 hints! 🚀