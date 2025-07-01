# Deadlock Prevention and Resolution

## 🎯 Supporting Exercise: ex04-deadlock.rs

This reference guide helps you understand and fix deadlocks you'll encounter in exercise 04.

## 🔒 Understanding Deadlocks

### What is a Deadlock?
A deadlock occurs when two or more threads are blocked forever, each waiting for resources held by the others.

### Classic Deadlock Example (Dining Philosophers)
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let fork1 = Arc::new(Mutex::new(()));
let fork2 = Arc::new(Mutex::new(()));

let fork1_clone = fork1.clone();
let fork2_clone = fork2.clone();

// Thread 1: Takes fork1 then fork2
thread::spawn(move || {
    let _f1 = fork1_clone.lock().unwrap();
    thread::sleep(Duration::from_millis(1)); // Simulate work
    let _f2 = fork2_clone.lock().unwrap(); // DEADLOCK!
});

// Thread 2: Takes fork2 then fork1
thread::spawn(move || {
    let _f2 = fork2.lock().unwrap();
    thread::sleep(Duration::from_millis(1)); // Simulate work
    let _f1 = fork1.lock().unwrap(); // DEADLOCK!
});
```

## 🚫 Deadlock Prevention Strategies

### 1. Lock Ordering (Most Important)
Always acquire locks in the same order across all threads:

```rust
// BAD: Different lock ordering
// Thread 1: lock A, then B
// Thread 2: lock B, then A

// GOOD: Same lock ordering everywhere
fn process_data(data1: &Mutex<i32>, data2: &Mutex<i32>) {
    // Always lock in memory address order
    let (first, second) = if data1.as_ptr() < data2.as_ptr() {
        (data1, data2)
    } else {
        (data2, data1)
    };
    
    let _guard1 = first.lock().unwrap();
    let _guard2 = second.lock().unwrap();
    // Safe to proceed
}
```

### 2. Lock Timeout with try_lock
```rust
use std::time::Duration;

fn safe_double_lock(m1: &Mutex<i32>, m2: &Mutex<i32>) -> Result<(), &'static str> {
    let _guard1 = m1.lock().unwrap();
    
    match m2.try_lock() {
        Ok(_guard2) => {
            // Got both locks, proceed
            Ok(())
        }
        Err(_) => {
            // Couldn't get second lock, release first and retry
            drop(_guard1);
            thread::sleep(Duration::from_millis(1));
            Err("Retry needed")
        }
    }
}
```

### 3. Hierarchical Locking
Assign levels to locks and always acquire in increasing order:

```rust
struct HierarchicalMutex<T> {
    level: usize,
    mutex: Mutex<T>,
}

impl<T> HierarchicalMutex<T> {
    fn new(data: T, level: usize) -> Self {
        Self {
            level,
            mutex: Mutex::new(data),
        }
    }
    
    fn lock(&self, current_level: &mut usize) -> Result<MutexGuard<T>, &'static str> {
        if self.level >= *current_level {
            return Err("Lock order violation");
        }
        *current_level = self.level;
        Ok(self.mutex.lock().unwrap())
    }
}
```

## 🔍 Deadlock Detection Techniques

### 1. Timeout Detection
```rust
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn detect_deadlock<T>(mutex: &Mutex<T>, timeout: Duration) -> Option<MutexGuard<T>> {
    let start = Instant::now();
    
    loop {
        match mutex.try_lock() {
            Ok(guard) => return Some(guard),
            Err(_) if start.elapsed() > timeout => {
                eprintln!("Potential deadlock detected!");
                return None;
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
```

### 2. Thread Monitoring
```rust
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn monitor_threads() {
    let mut thread_states = HashMap::new();
    
    loop {
        let active_threads = THREAD_COUNTER.load(Ordering::Relaxed);
        
        if active_threads == 0 {
            println!("All threads completed");
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
        println!("Active threads: {}", active_threads);
    }
}
```

## 🛠️ Deadlock Resolution Patterns

### 1. Backoff and Retry
```rust
fn backoff_retry<F, T>(mut operation: F, max_retries: usize) -> Option<T>
where
    F: FnMut() -> Option<T>,
{
    for attempt in 0..max_retries {
        if let Some(result) = operation() {
            return Some(result);
        }
        
        // Exponential backoff
        let backoff = Duration::from_millis(1 << attempt);
        thread::sleep(backoff);
    }
    None
}
```

### 2. Resource Ordering
```rust
use std::sync::Arc;

struct ResourceManager {
    resources: Vec<Arc<Mutex<String>>>,
}

impl ResourceManager {
    fn acquire_multiple(&self, indices: &mut [usize]) -> Vec<MutexGuard<String>> {
        // Sort indices to ensure consistent ordering
        indices.sort();
        
        indices.iter()
            .map(|&i| self.resources[i].lock().unwrap())
            .collect()
    }
}
```

## 🔧 Advanced Deadlock Prevention

### 1. Lock-Free Data Structures
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

struct LockFreeCounter {
    value: AtomicUsize,
}

impl LockFreeCounter {
    fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::Relaxed)
    }
    
    fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }
}
```

### 2. Read-Write Locks for Shared Data
```rust
use std::sync::RwLock;

// Multiple readers, single writer - reduces contention
let data = Arc::new(RwLock::new(vec![1, 2, 3]));

// Multiple readers can access simultaneously
let reader1 = data.read().unwrap();
let reader2 = data.read().unwrap(); // OK - multiple readers

// Writer needs exclusive access
let mut writer = data.write().unwrap(); // Blocks until all readers done
```

## 📊 Debugging Deadlocks

### 1. Logging Lock Acquisition
```rust
use std::sync::Mutex;

struct DebugMutex<T> {
    inner: Mutex<T>,
    name: String,
}

impl<T> DebugMutex<T> {
    fn new(data: T, name: &str) -> Self {
        Self {
            inner: Mutex::new(data),
            name: name.to_string(),
        }
    }
    
    fn lock(&self) -> MutexGuard<T> {
        println!("Thread {:?} acquiring lock: {}", thread::current().id(), self.name);
        let guard = self.inner.lock().unwrap();
        println!("Thread {:?} acquired lock: {}", thread::current().id(), self.name);
        guard
    }
}
```

### 2. Deadlock Detection with Parking Lot
```rust
// Using parking_lot crate for better debugging
use parking_lot::{Mutex, MutexGuard};
use std::time::Duration;

fn safe_lock_with_timeout<T>(mutex: &Mutex<T>) -> Option<MutexGuard<T>> {
    mutex.try_lock_for(Duration::from_secs(5))
}
```

## 🎯 Tips for Exercise ex04-deadlock.rs

1. **Identify the lock ordering problem** - Look for threads acquiring locks in different orders
2. **Use consistent ordering** - Always acquire locks in the same sequence
3. **Add timeout mechanisms** - Detect when locks take too long
4. **Consider alternatives** - Sometimes channels are better than shared state

## 💡 Tips for C# Developers

### C# vs Rust Lock Differences
| C# | Rust |
|-----|------|
| `lock(obj)` | `mutex.lock().unwrap()` |
| Automatic unlock | Explicit RAII with guards |
| `Monitor.TryEnter()` | `mutex.try_lock()` |
| `ReaderWriterLockSlim` | `RwLock` |

### Common Mistakes
1. **Forgetting to handle lock poisoning** - Use `unwrap()` carefully
2. **Not dropping guards early** - Guards live until end of scope
3. **Mixing sync and async** - Use `tokio::sync::Mutex` for async

## 📚 Related Topics
- See `02-shared-state.md` for Arc and Mutex patterns
- See `01-threads.md` for thread management
- Exercise ex04-deadlock.rs applies these prevention techniques