# Exercise 02 Hints - Level 2: Specific Guidance 🟡

## 🎯 Specific Channel Communication Fixes

Here are the exact issues and solutions for each checkpoint:

## ✅ Fix 1: Channel Creation (Checkpoint 1)

```rust
// ❌ PROBLEM: Creating multiple channels
let (tx, rx) = mpsc::channel();
let (tx2, rx2) = mpsc::channel(); // Wrong! Only need one channel

// ✅ SOLUTION: Use just one channel
let (tx, rx) = mpsc::channel::<i32>();
// Remove the second channel creation entirely
```

## ✅ Fix 2: Producer 1 Ownership (Checkpoint 2)

```rust
// ❌ PROBLEM: tx moved into first thread
let producer1 = thread::spawn(|| {
    tx.send(i).unwrap(); // tx moved here, can't use again
});

// ✅ SOLUTION: Clone tx for the first producer
let tx1 = tx.clone();
let producer1 = thread::spawn(move || {
    for i in 1..=5 {
        tx1.send(i).unwrap(); // Use the clone
        thread::sleep(Duration::from_millis(100));
    }
});
```

## ✅ Fix 3: Producer 2 Cloning (Checkpoint 3)

```rust
// ❌ PROBLEM: tx already moved to first thread
let producer2 = thread::spawn(|| {
    tx.send(i).unwrap(); // ERROR: tx already moved
});

// ✅ SOLUTION: Use original tx for second producer  
let producer2 = thread::spawn(move || {
    for i in 6..=10 {
        tx.send(i).unwrap(); // Move original tx here
        thread::sleep(Duration::from_millis(150));
    }
});
```

## ✅ Fix 4: Consumer Receiver (Checkpoint 4)

```rust
// ❌ PROBLEM: Using wrong receiver
let consumer = thread::spawn(|| {
    match rx2.recv() { // Wrong receiver!
        Ok(value) => println!("Received: {}", value),
        // ...
    }
});

// ✅ SOLUTION: Use the correct receiver
let consumer = thread::spawn(move || {
    loop {
        match rx.recv() { // Use rx, not rx2
            Ok(value) => {
                println!("Consumer received: {}", value);
                count += 1;
            },
            Err(_) => {
                println!("All senders disconnected, received {} messages", count);
                break;
            }
        }
    }
});
```

## ✅ Fix 5: Bounded Channel (Checkpoint 6)

```rust
// ❌ PROBLEM: Using unbounded channel
let (bounded_tx, bounded_rx) = mpsc::channel();

// ✅ SOLUTION: Create bounded channel
let (bounded_tx, bounded_rx) = mpsc::sync_channel(3); // Buffer size 3
```

## ✅ Fix 6: Error Handling (Checkpoint 7)

```rust
// ❌ PROBLEM: Panics when receiver drops
error_tx.send(i).unwrap(); // Will panic if receiver is gone

// ✅ SOLUTION: Handle send errors gracefully
match error_tx.send(i) {
    Ok(_) => println!("Sent: {}", i),
    Err(_) => {
        println!("Receiver disconnected, stopping producer");
        break; // Exit the loop instead of panicking
    }
}
```

## 🎯 C# Channel Comparison

| **Operation** | **C#** | **Rust** |
|---------------|---------|----------|
| **Create** | `Channel.CreateUnbounded<int>()` | `mpsc::channel::<i32>()` |
| **Multiple senders** | Share `channel.Writer` | Clone `tx` for each thread |
| **Send** | `await writer.WriteAsync(value)` | `tx.send(value).unwrap()` |
| **Receive** | `await reader.ReadAsync()` | `rx.recv().unwrap()` |
| **Bounded** | `Channel.CreateBounded<int>(3)` | `mpsc::sync_channel(3)` |
| **Close** | `writer.Complete()` | Drop all `tx` clones |

## 🔄 Pattern Summary

### Producer Pattern:
```rust
let tx_clone = tx.clone();
thread::spawn(move || {
    for item in work_items {
        tx_clone.send(item).unwrap();
    }
    // tx_clone dropped here
});
```

### Consumer Pattern:
```rust
thread::spawn(move || {
    for received_item in rx {
        process(received_item);
    }
    // rx dropped here
});
```

### Graceful Shutdown:
When all `tx` clones are dropped, `rx.recv()` returns `Err` and the consumer loop exits.

## 🎯 Apply These Patterns

1. **Clone senders** for multiple producers
2. **Move receivers** into consumer threads  
3. **Handle send errors** when receivers disconnect
4. **Use sync_channel** for bounded channels
5. **Drop senders** to signal completion

Need complete working code? Check Level 3 hints!