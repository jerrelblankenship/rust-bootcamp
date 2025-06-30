# Web Scraper Project 🕷️

Fix this broken multi-threaded web scraper that should fetch multiple URLs concurrently and extract data efficiently.

## 🎯 Project Goals

Build a web scraper that can:
- Fetch multiple URLs concurrently using threads
- Parse HTML content to extract specific data
- Use channels to coordinate between producer and consumer threads
- Share results using Arc<Mutex<T>>
- Handle errors gracefully without crashing
- Show progress indication to the user

## 🚨 Current State: BROKEN!

The scraper currently has these issues:
- [ ] **Thread spawning errors** - Ownership problems with URL data
- [ ] **Channel communication broken** - Sender/receiver issues
- [ ] **Shared state conflicts** - Arc/Mutex usage problems
- [ ] **Error handling missing** - Panics instead of graceful handling
- [ ] **Resource leaks** - Threads not properly joined
- [ ] **Performance issues** - No connection pooling or limits

## 🏃 How to Run

```bash
# Try to run (will fail with errors!)
cargo run --bin scraper

# Fix the errors one by one
cargo check  # See what's broken
cargo run    # Test your fixes
```

## 🎮 Usage (once fixed)

```bash
# Scrape a list of URLs
cargo run -- --urls https://example.com https://rust-lang.org

# With custom thread count
cargo run -- --urls https://example.com --threads 4

# Save results to file
cargo run -- --urls https://example.com --output results.json
```

## 🏆 Victory Conditions

You've completed this project when:
- [ ] Scraper runs without panicking
- [ ] Multiple URLs are fetched concurrently
- [ ] Results are properly shared between threads
- [ ] Progress bar shows during scraping
- [ ] Error handling works for invalid URLs
- [ ] All threads are properly joined
- [ ] Memory usage stays reasonable

## 💡 Hints

1. **Start with compilation errors** - Fix one error at a time
2. **Use Arc for shared ownership** - URLs need to be accessed by multiple threads
3. **Channels for work distribution** - Send URLs to worker threads
4. **Mutex for shared results** - Collect scraped data safely
5. **Join handles** - Don't let threads leak

## 🧪 Testing

```bash
# Run the integration tests
cargo test

# Test with different thread counts
cargo test -- --nocapture
```

Remember: This project combines everything you've learned about concurrency. Take it step by step!