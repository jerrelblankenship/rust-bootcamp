# Exercise 03 Hints - Level 3: Near-Solution

## 🎯 Exact Code Fixes

### Fix 1: Single Channel Creation
```rust
// ✅ Create only one channel:
let (tx, rx) = mpsc::channel();
// Remove this line: let (tx2, rx2) = mpsc::channel();
```

### Fix 2: Clone Sender for Multiple Producers
```rust
// ✅ Clone the sender before moving:
let tx1 = tx.clone(); // For first producer
let tx2 = tx.clone(); // For second producer

let producer1 = thread::spawn(move || {
    for i in 1..=5 {
        println!("Producer 1 sending: {}", i);
        tx1.send(i).unwrap(); // Use cloned sender
        thread::sleep(Duration::from_millis(100));
    }
});

let producer2 = thread::spawn(move || {
    for i in 6..=10 {
        println!("Producer 2 sending: {}", i);
        tx2.send(i).unwrap(); // Use cloned sender
        thread::sleep(Duration::from_millis(150));
    }
});
```

### Fix 3: Consumer Using Correct Receiver
```rust
// ✅ Use the correct receiver:
let consumer = thread::spawn(move || {
    for value in rx { // This automatically stops when all senders dropped
        println!("Consumer received: {}", value);
    }
    // Alternative explicit loop:
    // loop {
    //     match rx.recv() {
    //         Ok(value) => println!("Consumer received: {}", value),
    //         Err(_) => break, // All senders dropped
    //     }
    // }
});
```

### Fix 4: Proper Cleanup
```rust
// ✅ Drop the original sender after cloning:
drop(tx); // Important! This ensures consumer stops when producers finish

producer1.join().unwrap();
producer2.join().unwrap();
consumer.join().unwrap(); // Will not block forever now
```

## 🔑 Complete Working Pattern
```rust
let (tx, rx) = mpsc::channel(); // One channel
let tx1 = tx.clone();           // Clone for producer 1
let tx2 = tx.clone();           // Clone for producer 2
drop(tx);                       // Drop original sender

// Spawn producers with cloned senders...
// Spawn consumer with receiver that stops when all senders dropped...
```

---

🎉 **This pattern gives you clean producer-consumer communication!**