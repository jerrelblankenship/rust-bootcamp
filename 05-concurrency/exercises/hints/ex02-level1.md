# Exercise 02 Hints - Level 1: Gentle Nudges 🟢

## 🤔 Stuck on Channel Communication?

You're working with Rust's message-passing concurrency - think of it like C#'s `Channel<T>` or `BlockingCollection<T>`!

## 💡 Key Concepts to Consider

### 1. **MPSC = Multiple Producer, Single Consumer**
- **Multiple threads** can send messages (producers)
- **One thread** receives all messages (consumer)
- Like having multiple workers feeding into one processing queue

### 2. **Channel Ownership**
When you create a channel, you get two parts:
```rust
let (tx, rx) = mpsc::channel();
// tx = transmitter/sender (can be cloned)
// rx = receiver (only one allowed)
```

### 3. **Sharing Senders**
Multiple threads need to send? You need to clone the sender:
```rust
// ❌ Can't move tx to multiple threads
// ✅ Clone tx for each thread that needs to send
```

## 🔍 Questions to Ask Yourself

1. **Look at your channel creation** - Are you creating multiple channels when you need just one?

2. **Check your thread spawning** - How many threads are trying to use the same sender?

3. **Think about C# patterns**:
   ```csharp
   // C# Channel pattern
   var writer = channel.Writer;
   Task.Run(() => writer.WriteAsync("message"));
   ```
   In Rust, each thread needs its own clone of the sender.

4. **Consumer logic** - Which receiver should your consumer thread use?

## 💭 Mental Model: Restaurant Kitchen

- **Cooks (producer threads)**: Multiple cooks prepare orders
- **Order window (channel)**: Single place to put completed orders  
- **Server (consumer thread)**: One server takes orders from the window

Each cook needs their own "order ticket pad" (cloned sender), but there's only one order window (receiver).

## 🎯 What to Focus On

- **Channel creation**: Do you need one channel or multiple?
- **Sender cloning**: How do multiple threads each get a sender?
- **Receiver usage**: Which thread should receive messages?
- **Thread cleanup**: How do senders signal "no more messages"?

---

💪 **Try working through the compiler errors systematically!** 

Focus on one checkpoint at a time - channel creation first, then producer threads, then the consumer.