# Lesson 01: Threads and Message Passing

Welcome to Rust's fearless concurrency! Coming from C#, you know about tasks, threads, and locks. Rust takes a different approach: the type system prevents data races at compile time, making concurrent programming safer and more predictable.

## 🎯 Learning Objectives

- Master thread creation and management in Rust
- Understand message passing with channels
- Learn how ownership prevents data races
- Compare Rust's threading model with C# Tasks
- Build scalable concurrent applications

## 🧵 Threading in Rust vs C#

### C# Task-Based Concurrency
```csharp
// C# - Tasks and async/await
public async Task<string> ProcessDataAsync(string input)
{
    var task1 = Task.Run(() => ProcessPart1(input));
    var task2 = Task.Run(() => ProcessPart2(input));
    
    await Task.WhenAll(task1, task2);
    
    return task1.Result + task2.Result;
}

// C# - Thread creation
public void StartBackgroundWork()
{
    var thread = new Thread(() => {
        // Background work - can access any shared state
        SharedCounter++;  // Potential race condition!
    });
    thread.Start();
}
```

### Rust Thread-Based Concurrency
```rust
use std::thread;
use std::time::Duration;

// Rust - Thread spawning with ownership
fn process_data_concurrent(input: String) -> String {
    let input1 = input.clone();
    let input2 = input.clone();
    
    let handle1 = thread::spawn(move || {
        process_part1(input1)  // input1 is moved into this thread
    });
    
    let handle2 = thread::spawn(move || {
        process_part2(input2)  // input2 is moved into this thread
    });
    
    // Wait for both threads to complete
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    
    format!("{}{}", result1, result2)
}

fn process_part1(input: String) -> String {
    // Simulate work
    thread::sleep(Duration::from_millis(100));
    format!("Part1: {}", input)
}

fn process_part2(input: String) -> String {
    // Simulate work
    thread::sleep(Duration::from_millis(150));
    format!("Part2: {}", input)
}

// Rust prevents data races at compile time
fn safe_threading_demo() {
    let data = vec![1, 2, 3, 4, 5];
    
    // This won't compile - can't share mutable reference between threads
    /*
    let handle = thread::spawn(|| {
        data.push(6);  // ❌ Error: can't capture mutable reference
    });
    */
    
    // Solution: Move ownership or use synchronization primitives
    let handle = thread::spawn(move || {
        println!("Data in thread: {:?}", data);  // ✅ data is moved
    });
    
    handle.join().unwrap();
    // println!("{:?}", data);  // ❌ Error: data was moved
}
```

## 📨 Message Passing with Channels

Rust emphasizes message passing over shared memory, following the principle: "Don't communicate by sharing memory; share memory by communicating."

### Basic Channel Usage

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn basic_channel_demo() {
    // Create a channel (multiple producer, single consumer)
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread that sends messages
    thread::spawn(move || {
        let messages = vec![
            "Hello",
            "from", 
            "the",
            "thread"
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages in main thread
    for received in rx {
        println!("Received: {}", received);
    }
}

// Multiple producers
fn multiple_producers_demo() {
    let (tx, rx) = mpsc::channel();
    
    // Clone the transmitter for multiple producers
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for j in 0..5 {
                let message = format!("Producer {} - Message {}", i, j);
                tx_clone.send(message).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
    
    // Drop the original transmitter
    drop(tx);
    
    // Receive all messages
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Worker Pool Pattern

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Job trait for work items
trait Job: Send + 'static {
    fn execute(self: Box<Self>);
}

// Simple job implementation
struct SimpleJob<F>
where
    F: FnOnce() + Send + 'static,
{
    func: Option<F>,
}

impl<F> SimpleJob<F>
where
    F: FnOnce() + Send + 'static,
{
    fn new(func: F) -> Self {
        Self { func: Some(func) }
    }
}

impl<F> Job for SimpleJob<F>
where
    F: FnOnce() + Send + 'static,
{
    fn execute(mut self: Box<Self>) {
        if let Some(func) = self.func.take() {
            func();
        }
    }
}

// Thread pool implementation
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Box<dyn Job>>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(SimpleJob::new(f));
        
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Close the sending side
        drop(self.sender.take());
        
        // Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn Job>>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            
            match job {
                Ok(job) => {
                    println!("Worker {} executing job", id);
                    job.execute();
                }
                Err(_) => {
                    println!("Worker {} shutting down", id);
                    break;
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// Usage example
fn thread_pool_demo() {
    let pool = ThreadPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("Task {} is running on thread {:?}", i, thread::current().id());
            thread::sleep(Duration::from_millis(500));
            println!("Task {} completed", i);
        });
    }
    
    // Pool will be dropped here, waiting for all tasks to complete
}
```

### Producer-Consumer Pattern

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// Bounded buffer for producer-consumer
pub struct BoundedBuffer<T> {
    buffer: Arc<(Mutex<VecDeque<T>>, Condvar, usize)>,
}

impl<T> BoundedBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new((Mutex::new(VecDeque::new()), Condvar::new(), capacity)),
        }
    }
    
    pub fn produce(&self, item: T) {
        let (lock, cvar, capacity) = &*self.buffer;
        let mut queue = lock.lock().unwrap();
        
        // Wait while buffer is full
        while queue.len() >= *capacity {
            queue = cvar.wait(queue).unwrap();
        }
        
        queue.push_back(item);
        cvar.notify_all();
    }
    
    pub fn consume(&self) -> T {
        let (lock, cvar, _) = &*self.buffer;
        let mut queue = lock.lock().unwrap();
        
        // Wait while buffer is empty
        while queue.is_empty() {
            queue = cvar.wait(queue).unwrap();
        }
        
        let item = queue.pop_front().unwrap();
        cvar.notify_all();
        item
    }
    
    pub fn len(&self) -> usize {
        let (lock, _, _) = &*self.buffer;
        lock.lock().unwrap().len()
    }
}

impl<T> Clone for BoundedBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            buffer: Arc::clone(&self.buffer),
        }
    }
}

// Producer-consumer example
fn producer_consumer_demo() {
    let buffer = BoundedBuffer::new(5);
    let producer_buffer = buffer.clone();
    let consumer_buffer = buffer.clone();
    
    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..10 {
            println!("Producing item {}", i);
            producer_buffer.produce(i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let item = consumer_buffer.consume();
            println!("Consumed item {}", item);
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

## 🔄 Cross-Beam Channels (Advanced)

For more advanced use cases, the crossbeam crate provides enhanced channels:

```rust
// This would require adding crossbeam to Cargo.toml
// crossbeam = "0.8"

/*
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn crossbeam_channels_demo() {
    // Unbounded channel
    let (tx, rx) = channel::unbounded();
    
    // Bounded channel
    let (bounded_tx, bounded_rx) = channel::bounded(10);
    
    // Select operation (like Go's select)
    let (tx1, rx1) = channel::unbounded();
    let (tx2, rx2) = channel::unbounded();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx1.send("Message from channel 1").unwrap();
    });
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx2.send("Message from channel 2").unwrap();
    });
    
    // Select the first message to arrive
    crossbeam::select! {
        msg = rx1.recv() => {
            println!("Received from channel 1: {:?}", msg);
        }
        msg = rx2.recv() => {
            println!("Received from channel 2: {:?}", msg);
        }
    }
}
*/
```

## 🚀 Real-World Example: Parallel File Processor

```rust
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
struct FileProcessingResult {
    filename: String,
    line_count: usize,
    word_count: usize,
    error: Option<String>,
}

fn parallel_file_processor(file_paths: Vec<String>) -> Vec<FileProcessingResult> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn worker threads
    for path in file_paths {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let result = process_file(&path);
            tx_clone.send(result).unwrap();
        });
        handles.push(handle);
    }
    
    // Drop the original transmitter so receiver knows when to stop
    drop(tx);
    
    // Collect results
    let mut results = vec![];
    for result in rx {
        results.push(result);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    results
}

fn process_file(path: &str) -> FileProcessingResult {
    let filename = Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    match fs::File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut line_count = 0;
            let mut word_count = 0;
            
            for line in reader.lines() {
                match line {
                    Ok(line_content) => {
                        line_count += 1;
                        word_count += line_content.split_whitespace().count();
                    }
                    Err(e) => {
                        return FileProcessingResult {
                            filename,
                            line_count: 0,
                            word_count: 0,
                            error: Some(format!("Error reading line: {}", e)),
                        };
                    }
                }
            }
            
            FileProcessingResult {
                filename,
                line_count,
                word_count,
                error: None,
            }
        }
        Err(e) => FileProcessingResult {
            filename,
            line_count: 0,
            word_count: 0,
            error: Some(format!("Error opening file: {}", e)),
        },
    }
}

// Create test files and process them
fn file_processor_demo() {
    // Create some test files
    let test_files = vec!["test1.txt", "test2.txt", "test3.txt"];
    
    for (i, filename) in test_files.iter().enumerate() {
        let content = format!("This is test file number {}.\nIt has multiple lines.\nFor testing parallel processing.", i + 1);
        fs::write(filename, content).unwrap();
    }
    
    // Process files in parallel
    let file_paths: Vec<String> = test_files.iter().map(|s| s.to_string()).collect();
    let results = parallel_file_processor(file_paths);
    
    // Display results
    println!("File processing results:");
    for result in results {
        match result.error {
            None => {
                println!("✅ {}: {} lines, {} words", 
                         result.filename, result.line_count, result.word_count);
            }
            Some(error) => {
                println!("❌ {}: {}", result.filename, error);
            }
        }
    }
    
    // Clean up test files
    for filename in test_files {
        let _ = fs::remove_file(filename);
    }
}
```

## 🛡️ Thread Safety Guarantees

### Send and Sync Traits

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn send_sync_examples() {
    // Send: Can be transferred across thread boundaries
    let sendable_data = vec![1, 2, 3];
    thread::spawn(move || {
        println!("Sendable data: {:?}", sendable_data);
    });
    
    // Sync: Can be shared between threads (behind a reference)
    let sync_data = 42;
    let handle = thread::spawn(move || {
        println!("Sync data: {}", sync_data);
    });
    handle.join().unwrap();
    
    // Rc is NOT Send or Sync (not thread-safe)
    let rc_data = Rc::new(42);
    // thread::spawn(move || {
    //     println!("{}", *rc_data);  // ❌ Won't compile
    // });
    
    // RefCell is Send but NOT Sync
    let ref_cell = RefCell::new(42);
    // let ref_to_refcell = &ref_cell;
    // thread::spawn(move || {
    //     println!("{}", ref_to_refcell.borrow());  // ❌ Won't compile
    // });
    
    // But you can move RefCell to another thread
    thread::spawn(move || {
        println!("{}", ref_cell.borrow());  // ✅ Works
    });
}

// Custom type that implements Send + Sync
#[derive(Debug)]
struct ThreadSafeCounter {
    value: std::sync::atomic::AtomicUsize,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        Self {
            value: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    
    fn increment(&self) {
        self.value.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    
    fn get(&self) -> usize {
        self.value.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// ThreadSafeCounter is automatically Send + Sync because AtomicUsize is Send + Sync
```

## 🎯 Key Takeaways

1. **Ownership**: Rust's ownership system prevents data races at compile time
2. **Message Passing**: Preferred over shared memory for communication
3. **Channels**: Safe, efficient way to communicate between threads
4. **Thread Safety**: Send and Sync traits ensure thread safety
5. **Performance**: Zero-cost abstractions for concurrent programming

## 💻 Exercises

### Exercise 1: Parallel Number Cruncher
```rust
// TODO: Implement a parallel number processor
// Process a large vector of numbers using multiple threads
// Calculate sum, product, min, max in parallel

fn parallel_process_numbers(numbers: Vec<i32>) -> (i32, i64, i32, i32) {
    // TODO: Split work across threads using channels
    // Return (sum, product, min, max)
    todo!()
}
```

### Exercise 2: Message Router
```rust
// TODO: Implement a message routing system
// Route messages from multiple producers to multiple consumers
// Based on message type or routing key

enum Message {
    Text(String),
    Number(i32),
    Command(String),
}

struct MessageRouter {
    // TODO: Add fields for routing logic
}

impl MessageRouter {
    fn new() -> Self {
        todo!()
    }
    
    fn route_message(&self, message: Message) {
        todo!()
    }
    
    fn add_consumer(&mut self, message_type: &str, consumer_id: usize) {
        todo!()
    }
}
```

### Exercise 3: Rate-Limited Task Executor
```rust
// TODO: Implement a task executor with rate limiting
// Limit the number of tasks executed per second

struct RateLimitedExecutor {
    // TODO: Add fields
}

impl RateLimitedExecutor {
    fn new(max_tasks_per_second: u32) -> Self {
        todo!()
    }
    
    fn execute<F>(&self, task: F) 
    where 
        F: FnOnce() + Send + 'static 
    {
        todo!()
    }
}
```

## 📚 Additional Resources

- [Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Crossbeam crate](https://docs.rs/crossbeam/) - Advanced concurrency utilities
- [Rayon crate](https://docs.rs/rayon/) - Data parallelism library

---

Next: [Shared State Concurrency](02-shared-state.md) →
