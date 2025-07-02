// Memory Pool Allocator - Performance Disaster Zone!
//
// This module provides memory management with terrible allocation patterns.

use std::collections::VecDeque;
use std::sync::Mutex;

pub struct MemoryPool {
    // PERFORMANCE DISASTER: Locks on every allocation
    buffers: Mutex<VecDeque<Vec<u8>>>,  // FIXME: Global lock contention
    allocation_count: Mutex<usize>,     // FIXME: Another lock
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            buffers: Mutex::new(VecDeque::new()),
            allocation_count: Mutex::new(0),
        }
    }

    // PERFORMANCE DISASTER: Lock contention for buffer allocation
    pub fn get_buffer(&self, capacity: usize) -> Vec<u8> {
        // PERFORMANCE DISASTER: Lock for every allocation
        let mut buffers = self.buffers.lock().unwrap();
        
        // PERFORMANCE DISASTER: Linear search through buffers
        if let Some(mut buffer) = buffers.pop_front() {  // FIXME: Always takes first, not best fit
            if buffer.capacity() >= capacity {
                buffer.clear();
                return buffer;
            } else {
                // PERFORMANCE DISASTER: Return undersized buffer and allocate new one
                buffers.push_back(buffer);  // FIXME: Put it back, but still allocate new
            }
        }
        
        // PERFORMANCE DISASTER: Track allocations with another lock
        {
            let mut count = self.allocation_count.lock().unwrap();
            *count += 1;
        }
        
        Vec::with_capacity(capacity)
    }

    // PERFORMANCE DISASTER: Return buffer with lock
    pub fn return_buffer(&self, buffer: Vec<u8>) {
        if buffer.capacity() > 0 {
            let mut buffers = self.buffers.lock().unwrap();  // FIXME: Lock again
            buffers.push_back(buffer);
            
            // PERFORMANCE DISASTER: No limit on pool size (memory leak potential)
            // FIXME: Pool can grow unbounded
        }
    }

    // PERFORMANCE DISASTER: Statistics with locks
    pub fn get_allocation_count(&self) -> usize {
        *self.allocation_count.lock().unwrap()  // FIXME: Lock just for reading
    }

    // PERFORMANCE DISASTER: Clear pool with lock
    pub fn clear_pool(&self) {
        let mut buffers = self.buffers.lock().unwrap();
        buffers.clear();  // FIXME: Doesn't actually free capacity
        
        let mut count = self.allocation_count.lock().unwrap();
        *count = 0;
    }
}

// PERFORMANCE DISASTER: Thread-unsafe version that could be faster
pub struct UnsafeMemoryPool {
    buffers: VecDeque<Vec<u8>>,
    allocation_count: usize,
}

impl UnsafeMemoryPool {
    pub fn new() -> Self {
        Self {
            buffers: VecDeque::new(),
            allocation_count: 0,
        }
    }

    // Still has performance issues even without locks
    pub fn get_buffer(&mut self, capacity: usize) -> Vec<u8> {
        // PERFORMANCE DISASTER: Still linear search
        for (i, buffer) in self.buffers.iter().enumerate() {
            if buffer.capacity() >= capacity {
                let mut buffer = self.buffers.remove(i).unwrap();  // FIXME: O(n) removal
                buffer.clear();
                return buffer;
            }
        }
        
        self.allocation_count += 1;
        Vec::with_capacity(capacity)
    }

    pub fn return_buffer(&mut self, buffer: Vec<u8>) {
        // PERFORMANCE DISASTER: No size-based organization
        self.buffers.push_back(buffer);  // FIXME: Just dumps at end
    }
}

/*
PERFORMANCE DISASTERS TO FIX:
1. Lock contention on every allocation/deallocation
2. Linear search through buffer pool
3. No size-based bucket organization
4. No pool size limits (potential memory leak)
5. O(n) operations for buffer management
6. Multiple locks for simple operations
7. No thread-local pools to avoid contention

BETTER APPROACHES:
- Thread-local pools to avoid locks
- Size-based buckets for O(1) lookup
- Lock-free data structures
- Pre-allocated pool with fixed size
- RAII wrapper for automatic return
*/