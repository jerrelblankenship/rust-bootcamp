# Exercise 04 Hints - Level 2: Specific Guidance 🟡

## 🎯 Deadlock Analysis & Solutions

You've identified the deadlock pattern! Now let's fix it with specific techniques.

## 🔍 The Classic Deadlock Pattern

```rust
// Thread 1: Locks resource1, then resource2
let _guard1 = resource1.lock().unwrap();
thread::sleep(Duration::from_millis(100));
let _guard2 = resource2.lock().unwrap(); // DEADLOCK!

// Thread 2: Locks resource2, then resource1  
let _guard2 = resource2.lock().unwrap();
thread::sleep(Duration::from_millis(100));
let _guard1 = resource1.lock().unwrap(); // DEADLOCK!
```

**Why this deadlocks**: Circular wait condition - each thread holds one resource and waits for the other.

## 🔧 Solution 1: Lock Ordering

**Rule**: Always acquire locks in the same order across all threads.

```rust
// FIXED: Both threads lock resource1 first, then resource2
// Thread 1:
let _guard1 = resource1.lock().unwrap();  // Lock resource1 first
let _guard2 = resource2.lock().unwrap();  // Then resource2

// Thread 2: 
let _guard1 = resource1.lock().unwrap();  // Same order: resource1 first
let _guard2 = resource2.lock().unwrap();  // Then resource2
```

**Result**: No circular dependency, no deadlock!

## 🔧 Solution 2: try_lock() with Timeout

```rust
// Non-blocking lock attempts
match resource1.try_lock() {
    Ok(guard1) => {
        match resource2.try_lock() {
            Ok(guard2) => {
                // Got both locks! Do work here
                println!("Success: got both resources");
            },
            Err(_) => {
                println!("Could not get resource2, will retry");
                // guard1 automatically released here
            }
        }
    },
    Err(_) => {
        println!("Could not get resource1, will retry");
    }
}
```

## 🔧 Solution 3: Hierarchical Locking

Assign levels to resources and always lock lower levels first:

```rust
// Resource hierarchy: resource1 (level 1) < resource2 (level 2)
fn acquire_resources_safely() {
    let _low_level = resource1.lock().unwrap();   // Level 1 first
    let _high_level = resource2.lock().unwrap();  // Level 2 second
    // Safe: consistent ordering based on hierarchy
}
```

## 🔧 Solution 4: Scoped Locking

Minimize lock scope to reduce deadlock window:

```rust
// BAD: Holding locks too long
let _guard1 = resource1.lock().unwrap();
expensive_computation(); // Still holding lock!
let _guard2 = resource2.lock().unwrap();

// GOOD: Minimal lock scope
{
    let _guard1 = resource1.lock().unwrap();
    quick_access_to_resource1();
} // Lock released immediately

{
    let _guard2 = resource2.lock().unwrap();
    quick_access_to_resource2();
} // Lock released immediately
```

## 💡 C# Comparison

```csharp
// C# lock ordering (same concept)
private static readonly object lock1 = new object();
private static readonly object lock2 = new object();

// Thread 1:
lock (lock1) {
    lock (lock2) {
        // Work with both resources
    }
}

// Thread 2: SAME ORDER!
lock (lock1) {  // Same order prevents deadlock
    lock (lock2) {
        // Work with both resources  
    }
}
```

## 🎯 Apply to Your Exercise

1. **Identify the lock order** in both threads
2. **Make them consistent** - both should lock resource1 before resource2
3. **Test your fix** - the program should complete without hanging

Ready for the complete solution? Check Level 3 hints!