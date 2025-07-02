# Exercise 6 Hints - Level 1 (Gentle Nudges)

## 🤔 Stuck on threading performance? Here are some gentle hints...

### Checkpoint 1: Thread Creation Overhead
- Creating 4 threads for 1000 items - is the work per thread worth it?
- **Question**: When does thread overhead exceed the benefit?
- **Hint**: Compare serial vs threaded performance

### Checkpoint 2: Lock Contention
- Line with `counter.lock().unwrap()` - what happens when all threads want this?
- **Question**: Can you aggregate locally first, then combine?
- **Hint**: Think C# ThreadLocal<T> pattern

### Checkpoint 3: Work Distribution
- Some chunks have expensive items (% 100 == 0), others don't
- **Question**: Are all threads finishing at the same time?
- **Hint**: Can you distribute work more evenly?

### Checkpoint 4: Allocations in Threads
- Creating `Vec::new()` in each thread iteration
- **Question**: Can you reuse allocations?
- **Hint**: Think object pooling patterns

## 💡 Threading Performance Principles
- **Measure overhead**: Thread creation costs time
- **Avoid contention**: Shared locks kill performance
- **Balance work**: Idle threads waste resources
- **Minimize allocation**: Memory allocation in threads is expensive

Still stuck? Try Level 2 hints! 🚀