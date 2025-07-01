# Exercise 04 Hints - Level 3: Near-Complete Solutions 🔴

## 🎯 Complete Deadlock Fixes

Here are the working solutions for each deadlock prevention technique:

## ✅ Solution 1: Lock Ordering (Most Common)

```rust
fn lock_ordering_solution() {
    println!("\n=== Solution 1: Lock Ordering ===");
    
    let resource1 = Arc::new(Mutex::new("Database Connection"));
    let resource2 = Arc::new(Mutex::new("File Handle"));

    // FIXED: Both threads acquire locks in the SAME order
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Locking resources in order: DB first, then File");
        let _guard1 = r1_clone.lock().unwrap();   // Resource1 FIRST
        println!("Thread 1: ✅ Got Database Connection");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = r2_clone.lock().unwrap();   // Resource2 SECOND
        println!("Thread 1: ✅ Got File Handle - success!");
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Locking resources in order: DB first, then File");
        let _guard1 = r1_clone2.lock().unwrap();  // Resource1 FIRST (same order!)
        println!("Thread 2: ✅ Got Database Connection");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = r2_clone2.lock().unwrap();  // Resource2 SECOND (same order!)
        println!("Thread 2: ✅ Got File Handle - success!");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Lock ordering solution completed!");
}
```

## ✅ Solution 2: try_lock() with Retry

```rust
fn timeout_solution() {
    println!("\n=== Solution 2: Timeout-based Locking ===");
    
    let resource1 = Arc::new(Mutex::new("Database Connection"));
    let resource2 = Arc::new(Mutex::new("File Handle"));

    // Helper function with retry logic
    fn acquire_both_with_retry(
        name: &str,
        res1: &Arc<Mutex<&str>>,
        res2: &Arc<Mutex<&str>>,
        max_retries: u32
    ) -> Result<(), String> {
        for attempt in 1..=max_retries {
            println!("{}: Attempt {} to acquire both resources", name, attempt);
            
            // Try to get first lock
            if let Ok(_guard1) = res1.try_lock() {
                println!("{}: ✅ Got first resource", name);
                
                // Try to get second lock
                if let Ok(_guard2) = res2.try_lock() {
                    println!("{}: ✅ Got both resources successfully!", name);
                    thread::sleep(Duration::from_millis(50)); // Do work
                    return Ok(());
                } else {
                    println!("{}: ❌ Could not get second resource, will retry", name);
                }
            } else {
                println!("{}: ❌ Could not get first resource, will retry", name);
            }
            
            // Wait before retry
            thread::sleep(Duration::from_millis(10));
        }
        
        Err(format!("{}: Failed after {} attempts", name, max_retries))
    }

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        match acquire_both_with_retry("Thread 1", &r1_clone, &r2_clone, 10) {
            Ok(_) => println!("Thread 1: Success!"),
            Err(e) => println!("Thread 1: {}", e),
        }
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        match acquire_both_with_retry("Thread 2", &r2_clone2, &r1_clone2, 10) {
            Ok(_) => println!("Thread 2: Success!"),
            Err(e) => println!("Thread 2: {}", e),
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ try_lock solution completed!");
}
```

## ✅ Solution 3: Hierarchical Locking

```rust
fn hierarchical_locking() {
    println!("\n=== Solution 3: Hierarchical Locking ===");
    
    // Resources with hierarchy levels (lower numbers locked first)
    struct Resource {
        name: String,
        level: u32,
        data: Mutex<String>,
    }
    
    let resource1 = Arc::new(Resource {
        name: "Database".to_string(),
        level: 1, // Lower level = acquired first
        data: Mutex::new("DB Data".to_string()),
    });
    
    let resource2 = Arc::new(Resource {
        name: "FileSystem".to_string(),  
        level: 2, // Higher level = acquired second
        data: Mutex::new("File Data".to_string()),
    });

    // Function that respects hierarchy
    fn lock_resources(
        thread_name: &str,
        mut resources: Vec<&Arc<Resource>>
    ) {
        println!("{}: Locking resources in hierarchy order", thread_name);
        
        // Sort by level before locking
        resources.sort_by_key(|r| r.level);
        
        let mut guards = Vec::new();
        for resource in resources {
            println!("{}: Locking {} (level {})", thread_name, resource.name, resource.level);
            let guard = resource.data.lock().unwrap();
            guards.push(guard);
        }
        
        println!("{}: ✅ Got all resources in hierarchy order!", thread_name);
        thread::sleep(Duration::from_millis(50));
    }

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let thread1 = thread::spawn(move || {
        lock_resources("Thread 1", vec![&r1_clone, &r2_clone]);
    });

    let r1_clone2 = Arc::clone(&resource1);
    let r2_clone2 = Arc::clone(&resource2);
    
    let thread2 = thread::spawn(move || {
        // Different order, but hierarchy will sort them correctly
        lock_resources("Thread 2", vec![&r2_clone2, &r1_clone2]); 
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("✅ Hierarchical locking completed!");
}
```

## 🎯 Key Implementation Points

### 1. **Fix deadlock_demonstration()** 
**Don't change this function** - it's meant to show the deadlock problem.

### 2. **Implement lock_ordering_solution()**
- Both threads lock resource1 first, then resource2
- Same order = no deadlock

### 3. **Implement timeout_solution()**
- Use `try_lock()` instead of `lock()`
- Add retry logic with delays

### 4. **Implement hierarchical_locking()**
- Sort resources by level before locking
- Always lock lower levels first

### 5. **Uncomment the function calls in main()**
```rust
fn main() {
    deadlock_demonstration(); // Leave this as-is (shows the problem)
    
    // Uncomment these as you implement the solutions:
    lock_ordering_solution();
    timeout_solution(); 
    hierarchical_locking();
}
```

## 🏆 Testing Your Solutions

Run your program:
```bash
cargo run --bin ex04-deadlock
```

**Expected result**: All solutions complete without hanging, demonstrating different deadlock prevention strategies.

**If it still hangs**: Check that you're using the same lock order in both threads for the lock ordering solution.