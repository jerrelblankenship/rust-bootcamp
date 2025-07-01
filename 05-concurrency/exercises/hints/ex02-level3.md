# Exercise 02 Hints - Level 3: Near-Complete Solutions 🔴

## 🎯 Complete Channel Communication Solutions

Here are the working implementations for each checkpoint:

## ✅ Complete Basic Channel Implementation

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 2: Channel Communication (Fix the Code!) ===\n");

    // ✅ FIXED: Exercise 2.1 - Single channel creation
    let (tx, rx) = mpsc::channel::<i32>();
    // Remove: let (tx2, rx2) = mpsc::channel(); // Only need one channel!
    
    // ✅ FIXED: Exercise 2.2 - Producer 1 with cloned sender
    let tx1 = tx.clone(); // Clone sender for first producer
    let producer1 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Producer 1 sending: {}", i);
            tx1.send(i).unwrap(); // Use cloned sender
            thread::sleep(Duration::from_millis(100));
        }
        println!("Producer 1 finished!");
    });
    
    // ✅ FIXED: Exercise 2.3 - Producer 2 with original sender
    let producer2 = thread::spawn(move || {
        for i in 6..=10 {
            println!("Producer 2 sending: {}", i);
            tx.send(i).unwrap(); // Use original sender (moved here)
            thread::sleep(Duration::from_millis(150));
        }
        println!("Producer 2 finished!");
    });
    
    // ✅ FIXED: Exercise 2.4 - Consumer using correct receiver
    let consumer = thread::spawn(move || {
        println!("Consumer starting...");
        let mut count = 0;
        loop {
            match rx.recv() { // Use rx, not rx2!
                Ok(value) => {
                    println!("Consumer received: {}", value);
                    count += 1;
                },
                Err(_) => {
                    println!("Consumer: All senders disconnected, received {} messages", count);
                    break;
                }
            }
        }
    });
    
    // ✅ FIXED: Exercise 2.5 - Proper thread joining
    producer1.join().unwrap();
    producer2.join().unwrap();
    // At this point, all senders are dropped, so consumer will exit
    
    consumer.join().unwrap();
    
    println!("✅ Basic channel communication working!");
}
```

## ✅ Complete Bounded Channel Implementation

```rust
// ✅ FIXED: Exercise 2.6 - Bounded channel with backpressure
println!("\n--- Bounded Channel Challenge ---");

// Use sync_channel for bounded capacity
let (bounded_tx, bounded_rx) = mpsc::sync_channel(3); // Capacity of 3

let fast_producer = thread::spawn(move || {
    for i in 1..=10 {
        println!("Fast producer sending: {}", i);
        // This will block when buffer is full (backpressure)
        bounded_tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(50)); // Faster than consumer
    }
});

let slow_consumer = thread::spawn(move || {
    for value in bounded_rx {
        println!("Slow consumer processing: {}", value);
        thread::sleep(Duration::from_millis(200)); // Slower than producer
    }
});

fast_producer.join().unwrap();
slow_consumer.join().unwrap();
```

## ✅ Complete Error Handling Implementation

```rust
// ✅ FIXED: Exercise 2.7 - Graceful error handling
println!("\n--- Error Handling Challenge ---");

let (error_tx, error_rx) = mpsc::channel();

// Consumer that drops early
let early_drop_consumer = thread::spawn(move || {
    let first_message = error_rx.recv().unwrap();
    println!("Received first message: {}", first_message);
    // Consumer exits here, dropping receiver
});

// Producer that handles send errors gracefully
let persistent_producer = thread::spawn(move || {
    for i in 1..=5 {
        // FIXED: Handle send errors instead of panicking
        match error_tx.send(i) {
            Ok(_) => println!("Sent: {}", i),
            Err(_) => {
                println!("Receiver disconnected, stopping producer at message {}", i);
                break; // Exit gracefully instead of panicking
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
});

early_drop_consumer.join().unwrap();
persistent_producer.join().unwrap(); // No panic now!
```

## ✅ Complete Performance Test Implementation

```rust
// ✅ FIXED: Exercise 2.8 - Performance measurement
println!("\n--- Performance Comparison ---");

let start = std::time::Instant::now();

// Send 1000 messages through channel
let (perf_tx, perf_rx) = mpsc::channel();

let sender_thread = thread::spawn(move || {
    for i in 0..1000 {
        perf_tx.send(i).unwrap();
    }
    // perf_tx dropped here, signaling completion
});

let receiver_thread = thread::spawn(move || {
    let mut sum = 0;
    for value in perf_rx { // Iterate until sender drops
        sum += value;
    }
    sum
});

sender_thread.join().unwrap();
let sum = receiver_thread.join().unwrap();

let duration = start.elapsed();
println!("Channel communication: sent 1000 messages in {:?}", duration);
println!("Sum: {} (should be 499500)", sum);
```

## ✅ Advanced: Multiple Producers Pattern

```rust
// Bonus: Pattern for many producers, one consumer
fn multiple_producers_example() {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    
    // Spawn 5 producer threads
    for producer_id in 0..5 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for message_id in 0..3 {
                let message = format!("Producer {} - Message {}", producer_id, message_id);
                tx_clone.send(message).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    // Drop the original sender
    drop(tx);
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        let mut count = 0;
        for message in rx {
            println!("Received: {}", message);
            count += 1;
        }
        println!("Total messages received: {}", count);
    });
    
    // Wait for all producers to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Wait for consumer to finish
    consumer.join().unwrap();
}
```

## 🎯 Key Implementation Patterns

### 1. **Sender Cloning Pattern**
```rust
let tx1 = tx.clone(); // For thread 1
let tx2 = tx.clone(); // For thread 2
// Original tx can be used in thread 3 or dropped
```

### 2. **Consumer Loop Pattern**
```rust
for message in rx {
    process(message);
}
// Loop exits when all senders are dropped
```

### 3. **Error Handling Pattern**
```rust
match sender.send(data) {
    Ok(_) => continue_processing(),
    Err(_) => handle_disconnection(),
}
```

### 4. **Graceful Shutdown Pattern**
```rust
// Producers finish and drop their senders
drop(tx1);
drop(tx2);
// Consumer's rx.recv() returns Err and loop exits
```

## 🏆 Testing Your Solution

When working correctly, your program should:
1. **Complete without panicking** - All error cases handled
2. **Show proper message ordering** - Consumer receives all messages  
3. **Demonstrate backpressure** - Fast producer blocked by slow consumer
4. **Handle early disconnection** - Producer stops gracefully when receiver drops

**Expected Output Pattern**:
```
Producer 1 sending: 1
Producer 2 sending: 6  
Consumer received: 1
Producer 1 sending: 2
Consumer received: 6
...
All producers finished, consumer exits cleanly
```

Run with: `rustc ex02-channels.rs && ./ex02-channels`