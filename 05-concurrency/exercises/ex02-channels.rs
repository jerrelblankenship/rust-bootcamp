// Exercise 2: Channel Communication - Fix the Broken Code!
// 
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (8 checkpoints to fix)
// 
// Your task: Make this channel communication code work by fixing
// all the compilation errors and logical issues.
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - don't try to fix everything at once!
// 2. Compile after each fix: `rustc ex02-channels.rs`
// 3. Watch your progress bar above fill up as you complete checkpoints
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (channel creation)
// - Think about C# BlockingCollection<T> patterns
// - Each checkpoint builds communication understanding
//
// COMPLETED CONCEPTS:
// [] MPSC channel creation and usage
// [] Sender cloning for multiple producers
// [] Receiver ownership and message handling
// [] Proper thread communication patterns
// [] Graceful shutdown when senders drop

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exercise 2: Channel Communication (Fix the Code!) ===\n");

    // Exercise 2.1: Fix channel creation
    // FIXME: This will fail - multiple channel creation issues
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel(); // COMPILE ERROR: Should share the same channel!
    
    // ✅ CHECKPOINT 1: Fix channel creation - only one channel needed
    // Progress: [█░░░░░░░░░] 12.5% Complete
    
    // Exercise 2.2: Fix producer thread 1
    // FIXME: Ownership issues with moving tx into closures
    let producer1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Producer 1 sending: {}", i);
            tx.send(i).unwrap(); // COMPILE ERROR: tx moved here
            thread::sleep(Duration::from_millis(100));
        }
        println!("Producer 1 finished!");
    });
    
    // ✅ CHECKPOINT 2: Producer 1 should compile and move tx properly
    // Progress: [██░░░░░░░░] 25% Complete
    
    // Exercise 2.3: Fix producer thread 2
    // FIXME: Can't move tx again - it's already moved!
    let producer2 = thread::spawn(|| {
        for i in 6..=10 {
            println!("Producer 2 sending: {}", i);
            tx.send(i).unwrap(); // COMPILE ERROR: tx already moved!
            thread::sleep(Duration::from_millis(150));
        }
        println!("Producer 2 finished!");
    });
    
    // ✅ CHECKPOINT 3: Producer 2 should work with cloned sender
    // Progress: [███░░░░░░░] 37.5% Complete
    
    // Exercise 2.4: Fix consumer thread
    // FIXME: Wrong receiver and infinite loop potential
    let consumer = thread::spawn(|| {
        println!("Consumer starting...");
        let mut count = 0;
        loop {
            match rx2.recv() { // COMPILE ERROR: Wrong receiver!
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
    
    // ✅ CHECKPOINT 4: Consumer should use correct receiver
    // Progress: [████░░░░░░] 50% Complete
    
    // Exercise 2.5: Fix thread joining order
    // FIXME: Consumer might block forever if we don't handle sender drops
    producer1.join().unwrap();
    producer2.join().unwrap();
    // At this point, all senders should be dropped so consumer can exit
    
    consumer.join().unwrap();
    
    // ✅ CHECKPOINT 5: All threads should join cleanly
    // Progress: [█████░░░░░] 62.5% Complete
    
    println!("✅ Basic channel communication working!");
    
    // Exercise 2.6: Bounded channel challenge
    // TODO: Create a bounded channel with capacity 3
    // This should demonstrate backpressure
    println!("\n--- Bounded Channel Challenge ---");
    
    // FIXME: Create a bounded channel instead of unbounded
    let (bounded_tx, bounded_rx) = mpsc::channel(); // COMPILE ERROR: Should be bounded!
    
    let fast_producer = thread::spawn(|| {
        for i in 1..=10 {
            println!("Fast producer sending: {}", i);
            // This should block when buffer is full
            bounded_tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50)); // Faster than consumer
        }
    });
    
    let slow_consumer = thread::spawn(|| {
        for value in bounded_rx {
            println!("Slow consumer processing: {}", value);
            thread::sleep(Duration::from_millis(200)); // Slower than producer
        }
    });
    
    fast_producer.join().unwrap();
    slow_consumer.join().unwrap();
    
    // ✅ CHECKPOINT 6: Bounded channel should demonstrate backpressure
    // Progress: [██████░░░░] 75% Complete
    
    // Exercise 2.7: Error handling challenge
    // TODO: Handle the case where receiver drops first
    println!("\n--- Error Handling Challenge ---");
    
    let (error_tx, error_rx) = mpsc::channel();
    
    // Consumer that drops early
    let early_drop_consumer = thread::spawn(|| {
        let first_message = error_rx.recv().unwrap();
        println!("Received first message: {}", first_message);
        // Consumer exits here, dropping receiver
    });
    
    // Producer that tries to send multiple messages
    let persistent_producer = thread::spawn(|| {
        for i in 1..=5 {
            // FIXME: This will panic on send error - handle it gracefully
            error_tx.send(i).unwrap(); // COMPILE ERROR: Will panic when receiver drops
            println!("Sent: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    early_drop_consumer.join().unwrap();
    persistent_producer.join().unwrap(); // This will panic without proper error handling
    
    // ✅ CHECKPOINT 7: Error handling should prevent panics
    // Progress: [███████░░░] 87.5% Complete
    
    // Exercise 2.8: Performance measurement
    // TODO: Compare channel vs shared state performance
    println!("\n--- Performance Comparison ---");
    
    let start = std::time::Instant::now();
    
    // Send 1000 messages through channel
    let (perf_tx, perf_rx) = mpsc::channel();
    
    let sender_thread = thread::spawn(|| {
        for i in 0..1000 {
            perf_tx.send(i).unwrap();
        }
    });
    
    let receiver_thread = thread::spawn(|| {
        let mut sum = 0;
        for value in perf_rx {
            sum += value;
        }
        sum
    });
    
    sender_thread.join().unwrap();
    let sum = receiver_thread.join().unwrap();
    
    let duration = start.elapsed();
    println!("Channel communication: sent 1000 messages in {:?}", duration);
    println!("Sum: {} (should be 499500)", sum);
    
    // ✅ CHECKPOINT 8: Performance test should complete successfully
    // Progress: [██████████] 100% Complete
    
    println!("\n🎉 Exercise 2 completed! You've mastered channel communication!");
}

// COMPILATION CHALLENGES:
// 1. Use single channel for multiple producers and consumers
// 2. Clone senders for multiple producer threads
// 3. Use correct receiver in consumer thread
// 4. Handle sender drops for graceful consumer shutdown
// 5. Create bounded channels for backpressure
// 6. Handle send errors when receiver drops early
// 7. Implement proper error handling without panics
// 8. Measure and understand channel performance

// LEARNING OBJECTIVES:
// - Understanding MPSC (Multiple Producer, Single Consumer) patterns
// - Sender cloning for multiple producers
// - Receiver ownership and message iteration
// - Bounded vs unbounded channels
// - Error handling in channel communication
// - Performance characteristics of channels

// C# COMPARISON:
// C#: var channel = Channel.CreateUnbounded<int>();
// Rust: let (tx, rx) = mpsc::channel();
//
// C#: await writer.WriteAsync(value);
// Rust: tx.send(value).unwrap();
//
// C#: await foreach (var item in reader.ReadAllAsync())
// Rust: for item in rx { /* process */ }
//
// C#: writer.Complete(); // Signal completion
// Rust: drop(tx); // Drop all senders to signal completion

// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Channel creation): [ ] Complete
// Checkpoint 2 (Producer 1 ownership): [ ] Complete  
// Checkpoint 3 (Producer 2 cloning): [ ] Complete
// Checkpoint 4 (Consumer receiver): [ ] Complete
// Checkpoint 5 (Thread joining): [ ] Complete
// Checkpoint 6 (Bounded channels): [ ] Complete
// Checkpoint 7 (Error handling): [ ] Complete
// Checkpoint 8 (Performance test): [ ] Complete

// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ MPSC channel patterns and ownership
// ✅ Multiple producer coordination
// ✅ Graceful shutdown patterns
// ✅ Error handling in concurrent communication
// ✅ Performance characteristics of channels

// 🚀 Ready for the next challenge?
// Move on to ex03-shared-state.rs to explore Arc and Mutex!
// Or check your work with: `rustc ex02-channels.rs && ./ex02-channels`
