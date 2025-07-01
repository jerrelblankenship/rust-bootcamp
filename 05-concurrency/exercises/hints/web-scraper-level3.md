# Web Scraper Project - Level 3 Hints 🔴

## Complete Working Implementation

### Complete Project Structure

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::fmt;

// Error types
#[derive(Debug, Clone)]
enum ScrapError {
    NetworkError(String),
    InvalidUrl(String),
    Timeout,
    ParseError(String),
}

impl fmt::Display for ScrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScrapError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ScrapError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            ScrapError::Timeout => write!(f, "Request timeout"),
            ScrapError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for ScrapError {}

// Result type
type ScrapResult = Result<ScrapedContent, ScrapError>;

#[derive(Debug, Clone)]
struct ScrapedContent {
    url: String,
    content_length: usize,
    title: Option<String>,
    status: u16,
}

// Shared statistics
#[derive(Debug, Clone)]
struct Stats {
    total: usize,
    completed: usize,
    successful: usize,
    failed: usize,
    start_time: std::time::Instant,
}

impl Stats {
    fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            successful: 0,
            failed: 0,
            start_time: std::time::Instant::now(),
        }
    }
    
    fn progress_percent(&self) -> f32 {
        if self.total == 0 {
            100.0
        } else {
            (self.completed as f32 / self.total as f32) * 100.0
        }
    }
    
    fn elapsed_seconds(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
    
    fn urls_per_second(&self) -> f32 {
        let elapsed = self.elapsed_seconds();
        if elapsed > 0.0 {
            self.completed as f32 / elapsed
        } else {
            0.0
        }
    }
}

// Main scraper configuration
struct WebScraperConfig {
    num_workers: usize,
    timeout_seconds: u64,
    progress_interval_ms: u64,
}

impl Default for WebScraperConfig {
    fn default() -> Self {
        Self {
            num_workers: 4,
            timeout_seconds: 10,
            progress_interval_ms: 500,
        }
    }
}

struct WebScraper {
    config: WebScraperConfig,
}

impl WebScraper {
    fn new(config: WebScraperConfig) -> Self {
        Self { config }
    }
    
    fn scrape_urls(&self, urls: Vec<String>) -> Result<Vec<ScrapResult>, Box<dyn Error>> {
        if urls.is_empty() {
            return Ok(Vec::new());
        }
        
        println!("🚀 Starting web scraper with {} workers for {} URLs", 
                 self.config.num_workers, urls.len());
        
        // Initialize shared state
        let stats = Arc::new(Mutex::new(Stats::new(urls.len())));
        
        // Create channels
        let (work_tx, work_rx) = mpsc::channel::<String>();
        let (result_tx, result_rx) = mpsc::channel::<ScrapResult>();
        
        // Spawn worker threads
        let worker_handles = self.spawn_workers(work_rx, result_tx.clone(), stats.clone());
        
        // Spawn progress monitor
        let progress_handle = self.spawn_progress_monitor(stats.clone());
        
        // Send all work
        for url in urls.iter() {
            if let Err(e) = work_tx.send(url.clone()) {
                eprintln!("Failed to send work: {}", e);
                break;
            }
        }
        drop(work_tx); // Signal no more work
        drop(result_tx); // Drop main result sender
        
        // Collect all results
        let results = self.collect_results(result_rx, urls.len());
        
        // Wait for all threads to complete
        for handle in worker_handles {
            if let Err(e) = handle.join() {
                eprintln!("Worker thread panicked: {:?}", e);
            }
        }
        
        if let Err(e) = progress_handle.join() {
            eprintln!("Progress monitor thread panicked: {:?}", e);
        }
        
        // Print final summary
        self.print_final_summary(&stats, &results);
        
        Ok(results)
    }
    
    fn spawn_workers(
        &self,
        work_rx: mpsc::Receiver<String>,
        result_tx: mpsc::Sender<ScrapResult>,
        stats: Arc<Mutex<Stats>>,
    ) -> Vec<thread::JoinHandle<()>> {
        let work_rx = Arc::new(Mutex::new(work_rx));
        
        (0..self.config.num_workers)
            .map(|worker_id| {
                let work_rx = work_rx.clone();
                let result_tx = result_tx.clone();
                let stats = stats.clone();
                let timeout = Duration::from_secs(self.config.timeout_seconds);
                
                thread::Builder::new()
                    .name(format!("scraper-worker-{}", worker_id))
                    .spawn(move || {
                        Self::worker_loop(worker_id, work_rx, result_tx, stats, timeout)
                    })
                    .expect("Failed to spawn worker thread")
            })
            .collect()
    }
    
    fn worker_loop(
        worker_id: usize,
        work_rx: Arc<Mutex<mpsc::Receiver<String>>>,
        result_tx: mpsc::Sender<ScrapResult>,
        stats: Arc<Mutex<Stats>>,
        timeout: Duration,
    ) {
        println!("Worker {} started", worker_id);
        
        loop {
            // Get work from the queue
            let url = match work_rx.lock() {
                Ok(rx) => match rx.recv() {
                    Ok(url) => url,
                    Err(_) => {
                        println!("Worker {}: No more work, shutting down", worker_id);
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Worker {}: Failed to lock work receiver: {}", worker_id, e);
                    break;
                }
            };
            
            // Process the URL
            let result = Self::fetch_url(&url, timeout);
            
            // Update stats
            if let Ok(mut stats) = stats.lock() {
                match &result {
                    Ok(_) => stats.successful += 1,
                    Err(_) => stats.failed += 1,
                }
                stats.completed += 1;
            }
            
            // Send result back
            if result_tx.send(result).is_err() {
                println!("Worker {}: Result receiver dropped, stopping", worker_id);
                break;
            }
        }
        
        println!("Worker {} finished", worker_id);
    }
    
    fn fetch_url(url: &str, timeout: Duration) -> ScrapResult {
        // Validate URL format
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ScrapError::InvalidUrl(url.to_string()));
        }
        
        // Simulate different types of responses based on URL content
        if url.contains("timeout") {
            thread::sleep(timeout + Duration::from_millis(100));
            return Err(ScrapError::Timeout);
        }
        
        if url.contains("error") || url.contains("500") {
            return Err(ScrapError::NetworkError("Server returned 500".to_string()));
        }
        
        if url.contains("notfound") || url.contains("404") {
            return Err(ScrapError::NetworkError("404 Not Found".to_string()));
        }
        
        // Simulate variable response times
        let delay_ms = if url.contains("slow") {
            1000
        } else if url.contains("fast") {
            50
        } else {
            (url.len() % 300) + 100  // 100-400ms based on URL
        };
        
        thread::sleep(Duration::from_millis(delay_ms as u64));
        
        // Simulate successful response
        let content_length = (url.len() * 100) + 1000; // Fake content length
        let title = if url.contains("example") {
            Some("Example Domain".to_string())
        } else if url.contains("httpbin") {
            Some("HTTPBin Test".to_string())
        } else {
            Some(format!("Page from {}", url))
        };
        
        Ok(ScrapedContent {
            url: url.to_string(),
            content_length,
            title,
            status: 200,
        })
    }
    
    fn spawn_progress_monitor(&self, stats: Arc<Mutex<Stats>>) -> thread::JoinHandle<()> {
        let interval = Duration::from_millis(self.config.progress_interval_ms);
        
        thread::Builder::new()
            .name("progress-monitor".to_string())
            .spawn(move || {
                loop {
                    let (should_exit, progress_info) = {
                        match stats.lock() {
                            Ok(stats) => {
                                let info = format!(
                                    "📊 Progress: {:.1}% ({}/{}) | ✅ Success: {} | ❌ Failed: {} | ⚡ {:.1} URLs/sec",
                                    stats.progress_percent(),
                                    stats.completed,
                                    stats.total,
                                    stats.successful,
                                    stats.failed,
                                    stats.urls_per_second()
                                );
                                
                                let should_exit = stats.completed >= stats.total && stats.total > 0;
                                (should_exit, info)
                            }
                            Err(_) => {
                                (true, "Stats lock poisoned, exiting monitor".to_string())
                            }
                        }
                    };
                    
                    println!("{}", progress_info);
                    
                    if should_exit {
                        println!("🎉 Progress monitoring complete!");
                        break;
                    }
                    
                    thread::sleep(interval);
                }
            })
            .expect("Failed to spawn progress monitor")
    }
    
    fn collect_results(
        &self,
        result_rx: mpsc::Receiver<ScrapResult>,
        expected_count: usize,
    ) -> Vec<ScrapResult> {
        let mut results = Vec::with_capacity(expected_count);
        
        for _ in 0..expected_count {
            match result_rx.recv() {
                Ok(result) => results.push(result),
                Err(_) => {
                    eprintln!("Result receiver disconnected early");
                    break;
                }
            }
        }
        
        results
    }
    
    fn print_final_summary(&self, stats: &Arc<Mutex<Stats>>, results: &[ScrapResult]) {
        println!("\n" + "=".repeat(60).as_str());
        println!("🎯 SCRAPING COMPLETE");
        println!("=".repeat(60));
        
        if let Ok(stats) = stats.lock() {
            println!("📈 Final Statistics:");
            println!("   Total URLs:    {}", stats.total);
            println!("   Successful:    {} ({:.1}%)", stats.successful, 
                     (stats.successful as f32 / stats.total as f32) * 100.0);
            println!("   Failed:        {} ({:.1}%)", stats.failed,
                     (stats.failed as f32 / stats.total as f32) * 100.0);
            println!("   Total Time:    {:.2} seconds", stats.elapsed_seconds());
            println!("   Average Speed: {:.1} URLs/second", stats.urls_per_second());
        }
        
        println!("\n📋 Detailed Results:");
        for (i, result) in results.iter().enumerate() {
            match result {
                Ok(content) => {
                    println!("  ✅ [{}] {} - {} bytes - {}", 
                             i + 1, 
                             content.url, 
                             content.content_length,
                             content.title.as_deref().unwrap_or("No title"));
                }
                Err(error) => {
                    println!("  ❌ [{}] Error: {}", i + 1, error);
                }
            }
        }
        
        println!("=".repeat(60));
    }
}

// Main function with example usage
fn main() -> Result<(), Box<dyn Error>> {
    let urls = vec![
        "https://httpbin.org/status/200".to_string(),
        "https://httpbin.org/delay/1".to_string(),
        "https://example.com".to_string(),
        "https://httpbin.org/status/404".to_string(),  // Will fail
        "https://slow-site.example.com/slow".to_string(),
        "https://fast-site.example.com/fast".to_string(),
        "invalid-url-format".to_string(),  // Will fail
        "https://httpbin.org/status/500".to_string(),  // Will fail
        "https://timeout-site.example.com/timeout".to_string(),  // Will timeout
        "https://example.org".to_string(),
    ];
    
    let config = WebScraperConfig {
        num_workers: 3,
        timeout_seconds: 5,
        progress_interval_ms: 750,
    };
    
    let scraper = WebScraper::new(config);
    let results = scraper.scrape_urls(urls)?;
    
    println!("\n🚀 Scraping job completed with {} results", results.len());
    
    Ok(())
}

// Additional utility functions for testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stats_progress() {
        let mut stats = Stats::new(10);
        assert_eq!(stats.progress_percent(), 0.0);
        
        stats.completed = 5;
        assert_eq!(stats.progress_percent(), 50.0);
        
        stats.completed = 10;
        assert_eq!(stats.progress_percent(), 100.0);
    }
    
    #[test]
    fn test_url_validation() {
        let timeout = Duration::from_secs(1);
        
        // Valid URLs should not fail validation
        assert!(WebScraper::fetch_url("https://example.com", timeout).is_ok());
        assert!(WebScraper::fetch_url("http://example.com", timeout).is_ok());
        
        // Invalid URLs should fail
        match WebScraper::fetch_url("invalid-url", timeout) {
            Err(ScrapError::InvalidUrl(_)) => (),
            _ => panic!("Expected InvalidUrl error"),
        }
    }
}
```

### Key Architecture Benefits

1. **Type Safety** - Comprehensive error handling with custom error types
2. **Resource Management** - Proper cleanup with RAII patterns
3. **Scalability** - Configurable worker count and timeouts
4. **Observability** - Real-time progress monitoring and detailed statistics
5. **Robustness** - Graceful handling of thread panics and channel errors
6. **Testing** - Unit tests for core functionality

### Performance Optimizations

1. **Bounded Channels** - For memory-constrained environments
2. **Worker Pool Reuse** - Avoid constant thread creation/destruction
3. **Connection Pooling** - For real HTTP client integration
4. **Async I/O** - Use tokio for better I/O concurrency

This implementation demonstrates all the concurrency concepts from exercises 1-5 in a real-world application!

**Next Steps:**
1. Replace the simulated HTTP with real HTTP client (reqwest)
2. Add command-line argument parsing
3. Implement result output to files (JSON, CSV)
4. Add retry logic for failed requests
5. Implement rate limiting to respect server limits