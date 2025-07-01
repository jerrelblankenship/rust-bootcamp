# Exercise 03 Hints - Level 2: Specific Guidance

## 🔧 Specific Problems and Solutions

### Problem 1: Two Separate Channels
You created `(tx, rx)` and `(tx2, rx2)` - but you need ONE channel with multiple senders!

```rust
// ❌ Wrong - separate channels:
let (tx, rx) = mpsc::channel();
let (tx2, rx2) = mpsc::channel();

// ✅ Right - one channel, clone sender:
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone(); // Clone the sender!
```

### Problem 2: Moving the Same Sender
You can't move `tx` to multiple threads. You need to clone it first.

### Problem 3: Wrong Receiver
Your consumer is using `rx2` but should use `rx` (the receiver from the shared channel).

### Problem 4: Sender Cleanup
The consumer will run forever because you're not dropping the senders properly.

## 💡 C# vs Rust Pattern

**C# Channel:**
```csharp
var channel = Channel.CreateUnbounded<int>();
var writer = channel.Writer; // Can share this reference
// Multiple tasks can use the same writer
```

**Rust MPSC:**
```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone(); // Must explicitly clone for sharing
// Now tx and tx2 can be moved to different threads
```

## 🎯 Fix Order

1. Remove the second channel creation
2. Clone `tx` before moving into threads
3. Use the right receiver in consumer
4. Drop original senders after spawning threads

---

🚀 **Try these fixes step by step!**