# Exercise 05 Hints - Level 1: Gentle Nudges 🟢

## 🤔 Async/Await Compilation Errors?

You're working with Rust's async system - it's similar to C# but with some key differences!

## 💡 Key Concepts to Consider

### 1. **Runtime Dependency** 
Unlike C#, Rust async needs an explicit runtime. You can't just write `async fn main()` without setup.

**C# (built-in runtime)**:
```csharp
public static async Task Main(string[] args) {
    await SomeAsyncWork();
}
```

**Rust (needs runtime)**:
```rust
#[tokio::main]  // This sets up the runtime!
async fn main() {
    some_async_work().await;
}
```

### 2. **Blocking vs Non-blocking Sleep**
This is a common mistake when coming from sync code:

```rust
// ❌ WRONG: This blocks the entire thread!
std::thread::sleep(Duration::from_secs(1));

// ✅ RIGHT: This yields to other tasks
tokio::time::sleep(Duration::from_secs(1)).await;
```

### 3. **Missing .await**
In Rust, futures are lazy - they don't run until awaited:

```rust
// ❌ WRONG: Returns a future, doesn't execute
let result = fetch_data("url");

// ✅ RIGHT: Actually executes the async function
let result = fetch_data("url").await;
```

## 🔍 Questions to Ask Yourself

1. **Look at your imports** - Do you have `tokio::time` for async sleep?
2. **Check your main function** - How do you make `main()` async in Rust?
3. **Review your .await calls** - Are you actually awaiting your async functions?
4. **Think about C# async** - What would this look like with `Task<T>` and `await`?

## 🎯 Common Compilation Errors

- **"cannot await in non-async context"** → Your main function needs to be async
- **"no method named `sleep`"** → You need `tokio::time::sleep`, not `std::thread::sleep`
- **"expected `()`, found opaque type"** → You're missing `.await` on an async call

## 💭 Mental Model

Think of this like C#:
- `async fn` = `async Task<T>`
- `.await` = `await` (same!)
- `tokio::spawn` = `Task.Run`
- `#[tokio::main]` = the async runtime setup C# does for you

---

💪 **Try focusing on one compilation error at a time!** 

The Rust compiler is very helpful with async errors - read the messages carefully!