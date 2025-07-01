# Channel Communication Patterns in Rust

## 🎯 Supporting Exercise: ex02-channels.rs

This reference guide provides detailed explanations for the channel patterns you'll encounter in exercise 02.

## 📡 Understanding MPSC (Multi-Producer, Single-Consumer)

### Basic Channel Creation
```rust
use std::sync::mpsc;

// Unbounded channel - can hold unlimited messages
let (tx, rx) = mpsc::channel();

// Bounded channel - has a capacity limit
let (tx, rx) = mpsc::sync_channel(10);
```

### Key Differences from C# Channels

| Rust MPSC | C# Channel/BlockingCollection |
|-----------|-------------------------------|
| Single consumer only | Multiple consumers allowed |
| Sender can be cloned | Need ConcurrentQueue for multiple producers |
| Receiver moves ownership | Values typically copied |
| No async by default | Async channels available |

## 🔄 Producer Patterns

### Cloning Senders for Multiple Producers
```rust
let (tx, rx) = mpsc::channel();

// Clone sender for each producer thread
let tx1 = tx.clone();
let tx2 = tx.clone();
drop(tx); // Drop original to signal when all producers done

thread::spawn(move || {
    tx1.send("from thread 1").unwrap();
});

thread::spawn(move || {
    tx2.send("from thread 2").unwrap();
});
```

### Bounded vs Unbounded Channels

**Unbounded (mpsc::channel)**
- No capacity limit
- `send()` never blocks
- Can cause memory issues if consumer is slow
- Good for: Low-frequency messages, control signals

**Bounded (mpsc::sync_channel)**
- Fixed capacity
- `send()` blocks when full
- Provides backpressure
- Good for: High-throughput data, memory-constrained systems

## 🎭 Consumer Patterns

### Receiving Messages
```rust
// Blocking receive - waits for message
match rx.recv() {
    Ok(msg) => println!("Got: {}", msg),
    Err(_) => println!("All senders dropped"),
}

// Try receive - non-blocking
match rx.try_recv() {
    Ok(msg) => println!("Got: {}", msg),
    Err(TryRecvError::Empty) => println!("No messages yet"),
    Err(TryRecvError::Disconnected) => println!("Channel closed"),
}

// Iterator pattern - processes all messages
for msg in rx {
    println!("Processing: {}", msg);
}
```

## ⚠️ Common Pitfalls

### 1. Forgetting to Drop Original Sender
```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();
// tx still exists! rx.recv() won't know when done
// FIX: drop(tx) after cloning
```

### 2. Deadlock with Sync Channels
```rust
let (tx, rx) = mpsc::sync_channel(1);
tx.send(1).unwrap();
tx.send(2).unwrap(); // BLOCKS! Buffer full
// Consumer must run to unblock
```

### 3. Sending Non-Send Types
```rust
// Rc is not Send - can't cross thread boundaries
let data = Rc::new(vec![1, 2, 3]);
tx.send(data).unwrap(); // COMPILE ERROR!
// FIX: Use Arc instead of Rc
```

## 📊 Performance Considerations

### Channel Selection Strategy
- **Few messages, low frequency**: Use unbounded `channel()`
- **Many messages, memory constrained**: Use `sync_channel(n)`
- **Need multiple consumers**: Use `crossbeam::channel`
- **Need async**: Use `tokio::sync::mpsc`

### Capacity Tuning for Sync Channels
```rust
// Too small: Producers block frequently
let (tx, rx) = mpsc::sync_channel(1);

// Too large: Wastes memory
let (tx, rx) = mpsc::sync_channel(10000);

// Just right: Based on producer/consumer rates
let buffer_size = producer_rate / consumer_rate * 2;
let (tx, rx) = mpsc::sync_channel(buffer_size);
```

## 🔧 Advanced Patterns

### Shutdown Signaling
```rust
enum Message {
    Work(String),
    Shutdown,
}

// Producer
tx.send(Message::Work("data".to_string())).unwrap();
tx.send(Message::Shutdown).unwrap();

// Consumer
for msg in rx {
    match msg {
        Message::Work(data) => process(data),
        Message::Shutdown => break,
    }
}
```

### Fan-out/Fan-in Pattern
```rust
// Fan-out: One producer, multiple workers
let (tx, rx) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));

for i in 0..4 {
    let rx = rx.clone();
    thread::spawn(move || {
        loop {
            let msg = rx.lock().unwrap().recv().unwrap();
            // Process message
        }
    });
}
```

## 🐛 Debugging Channel Issues

### Detecting Deadlocks
1. Use `try_send()` with timeout for debugging
2. Add logging before/after send/recv
3. Use thread names for clarity
4. Consider `crossbeam::channel::select!` for timeout

### Memory Leak Detection
```rust
// Monitor channel depth
let (tx, rx) = mpsc::sync_channel(100);
// Periodically check if buffer is always full
// Indicates slow consumer
```

## 💡 Tips for C# Developers

1. **No `await` on channels** - Use `recv()` which blocks
2. **Ownership transfers** - Unlike C# where values are copied
3. **Single consumer** - Design accordingly or use crossbeam
4. **Explicit shutdown** - No automatic disposal like C#

## 📚 Related Topics
- See `01-threads.md` for thread spawning patterns
- See `02-shared-state.md` for when to use channels vs shared state
- Exercise ex02-channels.rs applies these concepts