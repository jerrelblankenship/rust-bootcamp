# Exercise 04 Hints - Level 1: Gentle Nudges

## 🤔 Shared State Between Threads?

You're tackling the classic shared memory problem - this is like using `lock` statements in C# but with compile-time safety!

## 💡 Key Concepts to Consider

1. **Shared Ownership**: Multiple threads need access to the same data. How do you share ownership safely?

2. **Mutual Exclusion**: Only one thread should modify the data at a time.

3. **Reference Counting**: Rust needs to know how many threads are using the data.

## 🔍 Questions to Ask Yourself

- The compiler is complaining about moving `Mutex<i32>` directly. What wrapper do you need?
- How do you share the same `Mutex` between multiple threads?
- What does `Arc` stand for? (Hint: Atomically Reference Counted)

## 🎯 What to Focus On

Your `Mutex<i32>` needs to be wrapped in something that allows multiple owners. The pattern is `Arc<Mutex<T>>`.

---

💪 **Think about how C# handles shared objects with multiple references!**