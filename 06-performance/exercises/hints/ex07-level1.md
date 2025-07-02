# Exercise 7 Hints - Level 1 (Gentle Nudges)

## 🤔 Stuck on blocking performance? Here are some gentle hints...

### Checkpoint 1: Sequential Blocking
- Each `simulate_blocking_work(i)` takes 20ms and blocks the next one
- **Question**: Can these operations run at the same time?
- **Hint**: Think about using threads to overlap the work

### Checkpoint 2: Thread Creation Overhead
- Creating 100 threads for tiny operations
- **Question**: Is the thread creation cost worth it for simple math?
- **Hint**: Compare thread overhead vs actual work time

### Checkpoint 3: Blocking I/O
- Each I/O operation blocks for 25ms sequentially
- **Question**: Can I/O operations happen concurrently?
- **Hint**: I/O is perfect for concurrency - threads can overlap waits

### Checkpoint 4: Producer-Consumer
- Producer creates items, consumer processes them sequentially
- **Question**: Can consumer work on items while producer is still working?
- **Hint**: Think about pipeline processing

## 💡 Concurrency vs Parallelism
- **Concurrency**: Multiple tasks making progress (can be on one core)
- **Parallelism**: Multiple tasks running simultaneously (needs multiple cores)
- **Blocking**: One task waits, all progress stops
- **Non-blocking**: Tasks can make progress while others wait

Still stuck? Try Level 2 hints! 🚀