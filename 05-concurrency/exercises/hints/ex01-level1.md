# Exercise 01 - Level 1 Hints 🟢

## Getting Started

You're seeing compiler errors about ownership and borrowing in threads. This is Rust protecting you from data races that would be runtime bugs in C#!

## Key Questions to Consider

1. **Who owns the data?** When you spawn a thread, who should own the data it uses?

2. **How long does the thread live?** Could the thread outlive the data it's trying to use?

3. **In C#, what happens here?**
   ```csharp
   var data = new List<int> {1, 2, 3};
   var task = Task.Run(() => Console.WriteLine(data.Count));
   ```
   The lambda captures `data` by reference. Rust needs you to be explicit!

## Concepts to Review

- **Move semantics**: The `move` keyword transfers ownership
- **Thread lifetime**: Spawned threads can outlive their parent scope
- **Closure capture**: How closures capture variables from their environment

## Next Steps

1. Read the compiler error messages carefully - they're very helpful!
2. Try adding the `move` keyword to your closures
3. Consider when you need to clone data vs share it

Need more guidance? Check Level 2 hints after trying for 15+ more minutes!