# Web Scraper Project - Level 2 Hints 🟡

## Specific Implementation Patterns

### Channel Architecture Pattern

```rust
use std::sync::mpsc;

// Work distribution channel
let (work_sender, work_receiver) = mpsc::channel::<String>();

// Result collection channel
let (result_sender, result_receiver) = mpsc::channel::<ScrapResult>();

// Shared progress tracking
let stats = Arc::new(Mutex::new(Stats::new()));
```

### Worker Thread Spawning

```rust
use std::thread;

fn spawn_workers(num_workers: usize, work_rx: mpsc::Receiver<String>) -> Vec<thread::JoinHandle<()>> {
    let work_rx = Arc::new(Mutex::new(work_rx));
    
    (0..num_workers)
        .map(|worker_id| {
            let work_rx = work_rx.clone();
            let result_tx = result_sender.clone();
            let stats = stats.clone();
            
            thread::Builder::new()
                .name(format!("worker-{}", worker_id))
                .spawn(move || {
                    worker_loop(worker_id, work_rx, result_tx, stats)
                })
                .unwrap()
        })
        .collect()
}
```

### Worker Loop Implementation

```rust
fn worker_loop(
    worker_id: usize,
    work_rx: Arc<Mutex<mpsc::Receiver<String>>>,
    result_tx: mpsc::Sender<ScrapResult>,
    stats: Arc<Mutex<Stats>>,
) {
    println!("Worker {} started", worker_id);
    
    loop {
        // Get work from the queue
        let url = match work_rx.lock().unwrap().recv() {
            Ok(url) => url,
            Err(_) => {
                println!("Worker {} shutting down", worker_id);
                break;
            }
        };
        
        println!("Worker {} processing: {}", worker_id, url);
        
        // Process the URL
        let result = fetch_url(&url);
        
        // Update stats
        {
            let mut stats = stats.lock().unwrap();
            match result {
                Ok(_) => stats.successful += 1,
                Err(_) => stats.failed += 1,
            }
            stats.completed += 1;
        }
        
        // Send result back
        if result_tx.send(result).is_err() {
            println!("Worker {}: Result receiver dropped", worker_id);
            break;
        }
    }
}
```

### HTTP Fetching with Error Handling

```rust
use std::time::Duration;

#[derive(Debug)]
enum ScrapError {
    NetworkError(String),
    InvalidUrl(String),
    Timeout,
}

fn fetch_url(url: &str) -> Result<String, ScrapError> {
    // Simulate HTTP request with different outcomes
    match url {
        url if url.contains("timeout") => {
            thread::sleep(Duration::from_secs(5));
            Err(ScrapError::Timeout)
        }
        url if url.contains("error") => {
            Err(ScrapError::NetworkError("Connection failed".to_string()))
        }
        url if !url.starts_with("http") => {
            Err(ScrapError::InvalidUrl(url.to_string()))
        }
        _ => {
            // Simulate work
            thread::sleep(Duration::from_millis(100));
            Ok(format!("Content from {}", url))
        }
    }
}
```

### Progress Tracking with Shared State

```rust
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Stats {
    total: usize,
    completed: usize,
    successful: usize,
    failed: usize,
}

impl Stats {
    fn new() -> Self {
        Self {
            total: 0,
            completed: 0,
            successful: 0,
            failed: 0,
        }
    }
    
    fn progress_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.completed as f32 / self.total as f32) * 100.0
        }
    }
}

// Progress monitoring thread
fn spawn_progress_monitor(stats: Arc<Mutex<Stats>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            {
                let stats = stats.lock().unwrap();
                println!(
                    "Progress: {:.1}% ({}/{}) - Success: {}, Failed: {}",
                    stats.progress_percent(),
                    stats.completed,
                    stats.total,
                    stats.successful,
                    stats.failed
                );
                
                if stats.completed >= stats.total && stats.total > 0 {
                    println!("All URLs processed!");
                    break;
                }
            }
            
            thread::sleep(Duration::from_millis(500));
        }
    })
}
```

### Result Collection Pattern

```rust
fn collect_results(
    result_rx: mpsc::Receiver<Result<String, ScrapError>>,
    total_expected: usize,
) -> Vec<Result<String, ScrapError>> {
    let mut results = Vec::new();
    
    for _ in 0..total_expected {
        match result_rx.recv() {
            Ok(result) => results.push(result),
            Err(_) => {
                println!("Result sender dropped early");
                break;
            }
        }
    }
    
    results
}
```

### Putting It All Together

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://httpbin.org/status/200".to_string(),
        "https://httpbin.org/delay/1".to_string(),
        "https://example.com".to_string(),
        "invalid-url".to_string(),  // Will cause error
    ];
    
    let num_workers = 3;
    
    // Initialize shared state
    let stats = Arc::new(Mutex::new(Stats::new()));
    stats.lock().unwrap().total = urls.len();
    
    // Create channels
    let (work_tx, work_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    
    // Spawn workers
    let worker_handles = spawn_workers(num_workers, work_rx, result_tx, stats.clone());
    
    // Spawn progress monitor
    let progress_handle = spawn_progress_monitor(stats.clone());
    
    // Send work
    for url in urls.iter() {
        work_tx.send(url.clone())?;
    }
    drop(work_tx); // Signal no more work
    
    // Collect results
    let results = collect_results(result_rx, urls.len());
    
    // Wait for all threads
    for handle in worker_handles {
        handle.join().unwrap();
    }
    progress_handle.join().unwrap();
    
    // Print final results
    println!("\nFinal Results:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(content) => println!("✅ {}: Success", urls[i]),
            Err(e) => println!("❌ {}: {:?}", urls[i], e),
        }
    }
    
    Ok(())
}
```

### Common Issues and Solutions

1. **Channel Deadlock**
   ```rust
   // PROBLEM: Forgot to drop work_tx
   for url in urls {
       work_tx.send(url)?;
   }
   // Workers wait forever because channel never closes
   
   // SOLUTION: Explicitly drop
   drop(work_tx);
   ```

2. **Progress Race Conditions**
   ```rust
   // PROBLEM: Reading stats without lock
   println!("Progress: {}", stats.completed); // COMPILE ERROR
   
   // SOLUTION: Lock properly
   let current_stats = stats.lock().unwrap();
   println!("Progress: {}", current_stats.completed);
   ```

3. **Worker Thread Panics**
   ```rust
   // PROBLEM: Panic brings down whole program
   let content = response.unwrap(); // Might panic!
   
   // SOLUTION: Handle errors gracefully
   let content = match response {
       Ok(c) => c,
       Err(e) => return Err(ScrapError::NetworkError(e.to_string())),
   };
   ```

Need complete working implementation? Check Level 3 hints!