# Lesson 02: Shared State Concurrency

While message passing is preferred in Rust, sometimes you need shared state. Rust provides powerful synchronization primitives that make shared state safe through the type system. Coming from C#, you'll find similarities but with stronger compile-time guarantees.

## 🎯 Learning Objectives

- Master Mutex and RwLock for shared state
- Understand Arc for shared ownership
- Learn atomic operations for lock-free programming
- Compare with C# synchronization primitives
- Build high-performance concurrent data structures

## 🔐 Mutex: Mutual Exclusion

### C# vs Rust Mutex Patterns

#### C# Lock Statement
```csharp
private readonly object _lockObject = new object();
private int _sharedCounter = 0;

public void IncrementCounter()
{
    lock (_lockObject)
    {
        _sharedCounter++;  // Critical section
    }
}

public int GetCounter()
{
    lock (_lockObject)
    {
        return _sharedCounter;
    }
}
```

#### Rust Mutex with Type Safety
```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Mutex wraps the data it protects
struct SharedState {
    counter: Mutex<i32>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            counter: Mutex::new(0),
        }
    }
    
    fn increment(&self) {
        let mut count = self.counter.lock().unwrap();
        *count += 1;
        // Lock is automatically released when `count` goes out of scope
    }
    
    fn get_count(&self) -> i32 {
        let count = self.counter.lock().unwrap();
        *count  // Value is copied, lock is released
    }
}

fn basic_mutex_demo() {
    let state = Arc::new(SharedState::new());
    let mut handles = vec![];
    
    // Spawn multiple threads that increment the counter
    for _ in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                state_clone.increment();
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", state.get_count());
    // Should print 10,000 (10 threads × 1,000 increments)
}
```

### Advanced Mutex Patterns

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

// Condition variable pattern (like C#'s Monitor.Wait/Pulse)
struct NotificationQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> NotificationQueue<T> {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }
    
    fn send(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        self.condvar.notify_one();
    }
    
    fn recv(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        
        // Wait until queue is not empty
        while queue.is_empty() {
            queue = self.condvar.wait(queue).unwrap();
        }
        
        queue.pop_front().unwrap()
    }
    
    fn try_recv(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
    
    fn recv_timeout(&self, timeout: Duration) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        
        if queue.is_empty() {
            let (_queue, timeout_result) = self.condvar.wait_timeout(queue, timeout).unwrap();
            queue = _queue;
            
            if timeout_result.timed_out() {
                return None;
            }
        }
        
        queue.pop_front()
    }
}

fn condvar_demo() {
    let queue = Arc::new(NotificationQueue::new());
    let producer_queue = Arc::clone(&queue);
    let consumer_queue = Arc::clone(&queue);
    
    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..5 {
            thread::sleep(Duration::from_millis(500));
            producer_queue.send(format!("Message {}", i));
            println!("Sent message {}", i);
        }
    });
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            let message = consumer_queue.recv();
            println!("Received: {}", message);
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

## 📖 RwLock: Reader-Writer Lock

RwLock allows multiple readers OR one writer, similar to C#'s ReaderWriterLockSlim:

```rust
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// Thread-safe cache with reader-writer lock
struct ThreadSafeCache<K, V> {
    data: RwLock<HashMap<K, V>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
    
    fn get(&self, key: &K) -> Option<V> {
        let read_guard = self.data.read().unwrap();
        read_guard.get(key).cloned()
    }
    
    fn insert(&self, key: K, value: V) -> Option<V> {
        let mut write_guard = self.data.write().unwrap();
        write_guard.insert(key, value)
    }
    
    fn remove(&self, key: &K) -> Option<V> {
        let mut write_guard = self.data.write().unwrap();
        write_guard.remove(key)
    }
    
    fn len(&self) -> usize {
        let read_guard = self.data.read().unwrap();
        read_guard.len()
    }
    
    // Demonstrate multiple readers
    fn contains_key(&self, key: &K) -> bool {
        let read_guard = self.data.read().unwrap();
        read_guard.contains_key(key)
    }
}

fn rwlock_demo() {
    let cache = Arc::new(ThreadSafeCache::new());
    let mut handles = vec![];
    
    // Writer thread - adds data periodically
    let writer_cache = Arc::clone(&cache);
    let writer = thread::spawn(move || {
        for i in 0..10 {
            writer_cache.insert(format!("key_{}", i), i * 10);
            println!("Writer: Added key_{} = {}", i, i * 10);
            thread::sleep(Duration::from_millis(100));
        }
    });
    handles.push(writer);
    
    // Multiple reader threads
    for reader_id in 0..3 {
        let reader_cache = Arc::clone(&cache);
        let reader = thread::spawn(move || {
            for i in 0..15 {
                let key = format!("key_{}", i % 10);
                
                if let Some(value) = reader_cache.get(&key) {
                    println!("Reader {}: {} = {}", reader_id, key, value);
                } else {
                    println!("Reader {}: {} not found", reader_id, key);
                }
                
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(reader);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final cache size: {}", cache.len());
}
```

## ⚛️ Atomic Operations: Lock-Free Programming

For simple operations, atomics provide lock-free concurrency:

```rust
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Performance counters using atomics
struct PerformanceCounters {
    requests_processed: AtomicUsize,
    errors_encountered: AtomicUsize,
    active_connections: AtomicUsize,
    shutdown_requested: AtomicBool,
}

impl PerformanceCounters {
    fn new() -> Self {
        Self {
            requests_processed: AtomicUsize::new(0),
            errors_encountered: AtomicUsize::new(0),
            active_connections: AtomicUsize::new(0),
            shutdown_requested: AtomicBool::new(false),
        }
    }
    
    fn increment_requests(&self) {
        self.requests_processed.fetch_add(1, Ordering::SeqCst);
    }
    
    fn increment_errors(&self) {
        self.errors_encountered.fetch_add(1, Ordering::SeqCst);
    }
    
    fn increment_connections(&self) -> usize {
        self.active_connections.fetch_add(1, Ordering::SeqCst) + 1
    }
    
    fn decrement_connections(&self) -> usize {
        self.active_connections.fetch_sub(1, Ordering::SeqCst) - 1
    }
    
    fn request_shutdown(&self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
    }
    
    fn should_shutdown(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }
    
    fn get_stats(&self) -> (usize, usize, usize) {
        (
            self.requests_processed.load(Ordering::SeqCst),
            self.errors_encountered.load(Ordering::SeqCst),
            self.active_connections.load(Ordering::SeqCst),
        )
    }
}

fn atomic_counters_demo() {
    let counters = Arc::new(PerformanceCounters::new());
    let mut handles = vec![];
    
    // Simulate multiple worker threads
    for worker_id in 0..5 {
        let counters_clone = Arc::clone(&counters);
        let handle = thread::spawn(move || {
            // Simulate connection
            let connection_count = counters_clone.increment_connections();
            println!("Worker {} connected (total: {})", worker_id, connection_count);
            
            // Process requests until shutdown
            while !counters_clone.should_shutdown() {
                // Simulate processing
                thread::sleep(Duration::from_millis(10));
                
                // Simulate occasional errors
                if worker_id == 0 && rand::random::<f32>() < 0.1 {
                    counters_clone.increment_errors();
                } else {
                    counters_clone.increment_requests();
                }
            }
            
            // Disconnect
            let connection_count = counters_clone.decrement_connections();
            println!("Worker {} disconnected (remaining: {})", worker_id, connection_count);
        });
        handles.push(handle);
    }
    
    // Stats reporter thread
    let stats_counters = Arc::clone(&counters);
    let stats_handle = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(100));
            let (requests, errors, connections) = stats_counters.get_stats();
            println!("Stats: {} requests, {} errors, {} connections", 
                     requests, errors, connections);
        }
    });
    
    // Let it run for a while, then request shutdown
    thread::sleep(Duration::from_millis(500));
    counters.request_shutdown();
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    stats_handle.join().unwrap();
    
    let (final_requests, final_errors, final_connections) = counters.get_stats();
    println!("Final stats: {} requests, {} errors, {} connections", 
             final_requests, final_errors, final_connections);
}

// Simulate random numbers without external crate
mod rand {
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static SEED: AtomicU64 = AtomicU64::new(1);
    
    pub fn random<T>() -> T 
    where 
        T: From<u32>
    {
        let old_seed = SEED.load(Ordering::SeqCst);
        let new_seed = old_seed.wrapping_mul(1103515245).wrapping_add(12345);
        SEED.store(new_seed, Ordering::SeqCst);
        T::from((new_seed >> 16) as u32)
    }
}
```

## 🔀 Compare-and-Swap Operations

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

// Lock-free stack using compare-and-swap
struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));
        
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = current_head;
            }
            
            // Try to update head pointer atomically
            match self.head.compare_exchange_weak(
                current_head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,  // Success!
                Err(_) => {
                    // Another thread modified head, retry
                    continue;
                }
            }
        }
    }
    
    fn pop(&self) -> Option<T> {
        loop {
            let current_head = self.head.load(Ordering::Acquire);
            
            if current_head.is_null() {
                return None;  // Stack is empty
            }
            
            let next = unsafe { (*current_head).next };
            
            // Try to update head to next node
            match self.head.compare_exchange_weak(
                current_head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    // Successfully updated head
                    let old_head = unsafe { Box::from_raw(current_head) };
                    return Some(old_head.data);
                }
                Err(_) => {
                    // Another thread modified head, retry
                    continue;
                }
            }
        }
    }
    
    fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

// Safe to send and share between threads
unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

fn lock_free_stack_demo() {
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];
    
    // Producer threads
    for producer_id in 0..3 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for i in 0..10 {
                let value = producer_id * 10 + i;
                stack_clone.push(value);
                println!("Producer {} pushed {}", producer_id, value);
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    // Consumer threads
    for consumer_id in 0..2 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            let mut consumed = 0;
            while consumed < 15 {  // Each consumer gets 15 items
                if let Some(value) = stack_clone.pop() {
                    println!("Consumer {} popped {}", consumer_id, value);
                    consumed += 1;
                } else {
                    thread::sleep(Duration::from_millis(5));
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Stack empty: {}", stack.is_empty());
}
```

## 📊 Memory Ordering

Understanding memory ordering is crucial for correct atomic operations:

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// Different memory orderings and their effects
fn memory_ordering_demo() {
    let flag = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicUsize::new(0));
    
    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);
    
    // Producer thread
    let producer = thread::spawn(move || {
        // Write data first
        data_clone.store(42, Ordering::Relaxed);
        
        // Then set flag (with stronger ordering to ensure visibility)
        flag_clone.store(true, Ordering::Release);
    });
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        // Wait for flag with acquire ordering
        while !flag.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        
        // Now data should be visible
        let value = data.load(Ordering::Relaxed);
        println!("Consumer read: {}", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

// Compare different orderings
fn ordering_comparison() {
    println!("Memory Ordering Comparison:");
    println!("- Relaxed: No ordering guarantees except atomicity");
    println!("- Acquire: No reads/writes can be reordered before this operation");
    println!("- Release: No reads/writes can be reordered after this operation");
    println!("- AcqRel: Both Acquire and Release");
    println!("- SeqCst: Sequential consistency - strongest guarantee");
}
```

## 🎯 Real-World Example: Thread-Safe Event Log

```rust
use std::sync::{Arc, RwLock, Mutex};
use std::collections::VecDeque;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: u64,
    level: LogLevel,
    message: String,
    thread_id: String,
}

#[derive(Debug, Clone)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

struct ThreadSafeEventLog {
    entries: RwLock<VecDeque<LogEntry>>,
    max_entries: usize,
    stats: Mutex<LogStats>,
}

#[derive(Debug)]
struct LogStats {
    info_count: usize,
    warning_count: usize,
    error_count: usize,
}

impl ThreadSafeEventLog {
    fn new(max_entries: usize) -> Self {
        Self {
            entries: RwLock::new(VecDeque::with_capacity(max_entries)),
            max_entries,
            stats: Mutex::new(LogStats {
                info_count: 0,
                warning_count: 0,
                error_count: 0,
            }),
        }
    }
    
    fn log(&self, level: LogLevel, message: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        let thread_id = format!("{:?}", thread::current().id());
        
        let entry = LogEntry {
            timestamp,
            level: level.clone(),
            message,
            thread_id,
        };
        
        // Update entries (write lock)
        {
            let mut entries = self.entries.write().unwrap();
            
            // Remove old entries if at capacity
            while entries.len() >= self.max_entries {
                entries.pop_front();
            }
            
            entries.push_back(entry);
        }
        
        // Update stats (separate lock to minimize contention)
        {
            let mut stats = self.stats.lock().unwrap();
            match level {
                LogLevel::Info => stats.info_count += 1,
                LogLevel::Warning => stats.warning_count += 1,
                LogLevel::Error => stats.error_count += 1,
            }
        }
    }
    
    fn get_recent_entries(&self, count: usize) -> Vec<LogEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .rev()  // Most recent first
            .take(count)
            .cloned()
            .collect()
    }
    
    fn get_stats(&self) -> LogStats {
        let stats = self.stats.lock().unwrap();
        LogStats {
            info_count: stats.info_count,
            warning_count: stats.warning_count,
            error_count: stats.error_count,
        }
    }
    
    fn search_entries(&self, pattern: &str) -> Vec<LogEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|entry| entry.message.contains(pattern))
            .cloned()
            .collect()
    }
}

fn event_log_demo() {
    let log = Arc::new(ThreadSafeEventLog::new(100));
    let mut handles = vec![];
    
    // Multiple worker threads logging events
    for worker_id in 0..5 {
        let log_clone = Arc::clone(&log);
        let handle = thread::spawn(move || {
            for i in 0..20 {
                let level = match i % 4 {
                    0 => LogLevel::Info,
                    1 => LogLevel::Info,
                    2 => LogLevel::Warning,
                    3 => LogLevel::Error,
                    _ => unreachable!(),
                };
                
                let message = format!("Worker {} - Event {}", worker_id, i);
                log_clone.log(level, message);
                
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    // Reader thread that periodically displays stats
    let stats_log = Arc::clone(&log);
    let stats_handle = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(50));
            let stats = stats_log.get_stats();
            println!("Log stats: {:?}", stats);
        }
    });
    
    // Wait for all workers
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Display final results
    stats_handle.join().unwrap();
    
    let recent_entries = log.get_recent_entries(10);
    println!("\nRecent log entries:");
    for entry in recent_entries {
        println!("{} [{}] {}: {}", 
                 entry.timestamp, entry.level, entry.thread_id, entry.message);
    }
    
    let error_entries = log.search_entries("Error");
    println!("\nFound {} error entries", error_entries.len());
}
```

## 🎯 Key Takeaways

1. **Type Safety**: Rust's type system prevents data races at compile time
2. **RAII**: Locks are automatically released when guards go out of scope
3. **Granular Control**: Choose the right synchronization primitive for your use case
4. **Performance**: Atomic operations provide lock-free alternatives
5. **Memory Ordering**: Understanding ordering is crucial for correctness

## 💻 Exercises

### Exercise 1: Thread-Safe Cache
```rust
// TODO: Implement a thread-safe LRU cache
use std::collections::HashMap;

struct LruCache<K, V> {
    // TODO: Add fields for HashMap and LRU ordering
    // Use appropriate synchronization primitives
}

impl<K, V> LruCache<K, V> 
where 
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        todo!()
    }
    
    fn get(&self, key: &K) -> Option<V> {
        // TODO: Update LRU order on access
        todo!()
    }
    
    fn put(&self, key: K, value: V) {
        // TODO: Evict least recently used if at capacity
        todo!()
    }
}
```

### Exercise 2: Atomic Reference Counter
```rust
// TODO: Implement a custom atomic reference counter
// Similar to Arc but with custom behavior

struct AtomicRc<T> {
    // TODO: Add fields
}

impl<T> AtomicRc<T> {
    fn new(data: T) -> Self {
        todo!()
    }
    
    fn clone(&self) -> Self {
        todo!()
    }
    
    fn strong_count(&self) -> usize {
        todo!()
    }
}
```

### Exercise 3: Work-Stealing Queue
```rust
// TODO: Implement a work-stealing queue for load balancing
// Workers can steal work from each other's queues

struct WorkStealingQueue<T> {
    // TODO: Add fields
}

impl<T> WorkStealingQueue<T> {
    fn new() -> Self {
        todo!()
    }
    
    fn push_local(&self, item: T) {
        // Push to local end
        todo!()
    }
    
    fn pop_local(&self) -> Option<T> {
        // Pop from local end
        todo!()
    }
    
    fn steal(&self) -> Option<T> {
        // Steal from remote end
        todo!()
    }
}
```

## 📚 Additional Resources

- [Rust Atomics and Locks](https://marabos.nl/atomics/) - Comprehensive guide to concurrency
- [crossbeam-utils](https://docs.rs/crossbeam-utils/) - Additional synchronization utilities
- [parking_lot](https://docs.rs/parking_lot/) - More efficient synchronization primitives

---

Next: [Async Programming](03-async-await.md) →
