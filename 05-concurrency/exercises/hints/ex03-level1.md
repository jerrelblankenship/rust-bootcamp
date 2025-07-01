# Exercise 03 Hints - Level 1: Gentle Nudges

## 🤔 Channel Communication Issues?

You're learning about message passing between threads - this is like C#'s `Channel<T>` or `BlockingCollection<T>`!

## 💡 Key Concepts to Consider

1. **Single Channel, Multiple Producers**: How do you share a sender between multiple threads?

2. **Producer-Consumer Pattern**: One receiver, multiple senders sending data to it.

3. **Channel Cleanup**: What happens when all senders are dropped?

## 🔍 Questions to Ask Yourself

- Look at your compilation errors about moving `tx` (the sender)
- Can you move the same sender to multiple threads?
- What's the difference between the two channels you created?
- Which receiver should the consumer be using?

## 🎯 What to Focus On

The errors are telling you about ownership of `tx`. In C#, you'd just share the writer, but Rust needs explicit sharing mechanisms.

---

💪 **Keep experimenting! The solution involves sharing the sender properly.**