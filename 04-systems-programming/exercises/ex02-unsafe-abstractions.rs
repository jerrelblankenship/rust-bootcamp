// Exercise 2: Safe Abstractions Over Unsafe Code
//
// This exercise teaches you to build safe, high-level APIs on top of unsafe operations.
// You'll learn to maintain safety invariants while providing zero-cost abstractions.

use std::alloc::{self, Layout};
use std::mem;
use std::ptr;
use std::slice;

fn main() {
    println!("=== Exercise 2: Safe Abstractions Over Unsafe Code ===\n");
    
    exercise_2_1();
    exercise_2_2();
    exercise_2_3();
    exercise_2_4();
    exercise_2_5();
}

// Exercise 2.1: Build a safe dynamic array from scratch
fn exercise_2_1() {
    println!("Exercise 2.1: Safe Dynamic Array Implementation");
    
    // TODO: Complete the SafeVec implementation
    // This should provide memory safety while using raw pointers internally
    
    pub struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SafeVec<T> {
        pub fn new() -> Self {
            // TODO: Initialize empty vector
            Self {
                ptr: ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        }
        
        pub fn with_capacity(capacity: usize) -> Self {
            // TODO: Allocate initial capacity
            if capacity == 0 {
                return Self::new();
            }
            
            let layout = Layout::array::<T>(capacity).unwrap();
            let ptr = unsafe { alloc::alloc(layout) as *mut T };
            
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }
            
            Self { ptr, len: 0, capacity }
        }
        
        pub fn push(&mut self, value: T) {
            // TODO: Add element, growing if necessary
            if self.len == self.capacity {
                self.grow();
            }
            
            unsafe {
                ptr::write(self.ptr.add(self.len), value);
            }
            self.len += 1;
        }
        
        pub fn pop(&mut self) -> Option<T> {
            // TODO: Remove and return last element
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                unsafe {
                    Some(ptr::read(self.ptr.add(self.len)))
                }
            }
        }
        
        pub fn get(&self, index: usize) -> Option<&T> {
            // TODO: Safe indexing
            if index < self.len {
                unsafe {
                    Some(&*self.ptr.add(index))
                }
            } else {
                None
            }
        }
        
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            // TODO: Safe mutable indexing
            if index < self.len {
                unsafe {
                    Some(&mut *self.ptr.add(index))
                }
            } else {
                None
            }
        }
        
        pub fn len(&self) -> usize {
            self.len
        }
        
        pub fn capacity(&self) -> usize {
            self.capacity
        }
        
        pub fn as_slice(&self) -> &[T] {
            // TODO: Return slice view of data
            if self.len == 0 {
                &[]
            } else {
                unsafe {
                    slice::from_raw_parts(self.ptr, self.len)
                }
            }
        }
        
        fn grow(&mut self) {
            // TODO: Double capacity (or set to 1 if empty)
            let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
            
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            let new_ptr = unsafe { alloc::alloc(new_layout) as *mut T };
            
            if new_ptr.is_null() {
                alloc::handle_alloc_error(new_layout);
            }
            
            // Copy existing elements
            if self.len > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                }
            }
            
            // Deallocate old memory
            if !self.ptr.is_null() {
                unsafe {
                    let old_layout = Layout::array::<T>(self.capacity).unwrap();
                    alloc::dealloc(self.ptr as *mut u8, old_layout);
                }
            }
            
            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
    }
    
    impl<T> Drop for SafeVec<T> {
        fn drop(&mut self) {
            // TODO: Proper cleanup - drop elements and deallocate
            // Drop all elements
            while let Some(_) = self.pop() {}
            
            // Deallocate memory
            if !self.ptr.is_null() {
                unsafe {
                    let layout = Layout::array::<T>(self.capacity).unwrap();
                    alloc::dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }
    
    // Test the implementation
    let mut vec = SafeVec::new();
    
    // Test basic operations
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(1), Some(&2));
    
    if let Some(value) = vec.get_mut(0) {
        *value = 10;
    }
    
    assert_eq!(vec.get(0), Some(&10));
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.len(), 2);
    
    println!("SafeVec implementation: ✅");
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Raw pointer utilities with safety guarantees
fn exercise_2_2() {
    println!("Exercise 2.2: Safe Raw Pointer Utilities");
    
    // TODO: Implement safe utilities for working with raw pointers
    
    pub struct RawBuffer {
        ptr: *mut u8,
        len: usize,
        capacity: usize,
    }
    
    impl RawBuffer {
        pub fn new(capacity: usize) -> Self {
            // TODO: Allocate raw byte buffer
            let layout = Layout::array::<u8>(capacity).unwrap();
            let ptr = unsafe { alloc::alloc(layout) };
            
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }
            
            Self { ptr, len: 0, capacity }
        }
        
        pub fn write_bytes(&mut self, data: &[u8]) -> Result<(), &'static str> {
            // TODO: Safe write to buffer
            if self.len + data.len() > self.capacity {
                return Err("Buffer overflow");
            }
            
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    self.ptr.add(self.len),
                    data.len()
                );
            }
            self.len += data.len();
            Ok(())
        }
        
        pub fn read_bytes(&self, offset: usize, len: usize) -> Result<&[u8], &'static str> {
            // TODO: Safe read from buffer
            if offset + len > self.len {
                return Err("Read beyond buffer end");
            }
            
            unsafe {
                Ok(slice::from_raw_parts(self.ptr.add(offset), len))
            }
        }
        
        pub fn as_slice(&self) -> &[u8] {
            // TODO: Return slice view
            if self.len == 0 {
                &[]
            } else {
                unsafe {
                    slice::from_raw_parts(self.ptr, self.len)
                }
            }
        }
        
        pub fn clear(&mut self) {
            // TODO: Reset length (but keep capacity)
            self.len = 0;
        }
        
        pub fn capacity(&self) -> usize {
            self.capacity
        }
        
        pub fn len(&self) -> usize {
            self.len
        }
    }
    
    impl Drop for RawBuffer {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe {
                    let layout = Layout::array::<u8>(self.capacity).unwrap();
                    alloc::dealloc(self.ptr, layout);
                }
            }
        }
    }
    
    // Test the buffer
    let mut buffer = RawBuffer::new(100);
    
    buffer.write_bytes(b"Hello, ").unwrap();
    buffer.write_bytes(b"World!").unwrap();
    
    assert_eq!(buffer.len(), 13);
    
    let data = buffer.read_bytes(0, 5).unwrap();
    assert_eq!(data, b"Hello");
    
    let all_data = buffer.as_slice();
    assert_eq!(all_data, b"Hello, World!");
    
    // Test overflow protection
    let large_data = vec![0u8; 200];
    assert!(buffer.write_bytes(&large_data).is_err());
    
    println!("RawBuffer implementation: ✅");
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: Thread-safe reference counting
fn exercise_2_3() {
    println!("Exercise 2.3: Thread-Safe Reference Counting");
    
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    
    // TODO: Implement a simplified Arc (Atomic Reference Counter)
    
    pub struct SimpleArc<T> {
        ptr: *const ArcInner<T>,
    }
    
    struct ArcInner<T> {
        data: T,
        ref_count: AtomicUsize,
    }
    
    impl<T> SimpleArc<T> {
        pub fn new(data: T) -> Self {
            // TODO: Allocate and initialize
            let inner = Box::new(ArcInner {
                data,
                ref_count: AtomicUsize::new(1),
            });
            
            Self {
                ptr: Box::into_raw(inner),
            }
        }
        
        pub fn clone(&self) -> Self {
            // TODO: Increment reference count
            unsafe {
                (*self.ptr).ref_count.fetch_add(1, Ordering::Relaxed);
            }
            
            Self { ptr: self.ptr }
        }
        
        pub fn strong_count(&self) -> usize {
            // TODO: Return current reference count
            unsafe {
                (*self.ptr).ref_count.load(Ordering::Relaxed)
            }
        }
    }
    
    impl<T> std::ops::Deref for SimpleArc<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            // TODO: Safe dereference
            unsafe {
                &(*self.ptr).data
            }
        }
    }
    
    impl<T> Drop for SimpleArc<T> {
        fn drop(&mut self) {
            // TODO: Decrement count, deallocate if last reference
            unsafe {
                let old_count = (*self.ptr).ref_count.fetch_sub(1, Ordering::Release);
                
                if old_count == 1 {
                    // Last reference - clean up
                    let _ = Box::from_raw(self.ptr as *mut ArcInner<T>);
                }
            }
        }
    }
    
    // Safety: T must be Send + Sync for SimpleArc<T> to be Send
    unsafe impl<T: Send + Sync> Send for SimpleArc<T> {}
    unsafe impl<T: Send + Sync> Sync for SimpleArc<T> {}
    
    // Test the Arc implementation
    let arc1 = SimpleArc::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(arc1.strong_count(), 1);
    
    let arc2 = arc1.clone();
    assert_eq!(arc1.strong_count(), 2);
    assert_eq!(arc2.strong_count(), 2);
    
    // Test data access
    assert_eq!(arc1.len(), 5);
    assert_eq!(arc2[0], 1);
    
    drop(arc2);
    assert_eq!(arc1.strong_count(), 1);
    
    println!("SimpleArc implementation: ✅");
    println!("✅ Exercise 2.3 complete\n");
}

// Exercise 2.4: Memory pool allocator
fn exercise_2_4() {
    println!("Exercise 2.4: Memory Pool Allocator");
    
    // TODO: Implement a memory pool for fixed-size allocations
    
    pub struct MemoryPool {
        block_size: usize,
        blocks: Vec<*mut u8>,
        free_list: Vec<*mut u8>,
        memory: *mut u8,
        capacity: usize,
    }
    
    impl MemoryPool {
        pub fn new(block_size: usize, block_count: usize) -> Self {
            // TODO: Initialize pool with fixed-size blocks
            let total_size = block_size * block_count;
            let layout = Layout::from_size_align(total_size, mem::align_of::<u8>()).unwrap();
            
            let memory = unsafe { alloc::alloc(layout) };
            if memory.is_null() {
                alloc::handle_alloc_error(layout);
            }
            
            let mut blocks = Vec::with_capacity(block_count);
            let mut free_list = Vec::with_capacity(block_count);
            
            // Initialize all blocks as free
            for i in 0..block_count {
                unsafe {
                    let block = memory.add(i * block_size);
                    blocks.push(block);
                    free_list.push(block);
                }
            }
            
            Self {
                block_size,
                blocks,
                free_list,
                memory,
                capacity: block_count,
            }
        }
        
        pub fn allocate(&mut self) -> Option<*mut u8> {
            // TODO: Return next available block
            self.free_list.pop()
        }
        
        pub fn deallocate(&mut self, ptr: *mut u8) -> Result<(), &'static str> {
            // TODO: Return block to free list
            // Verify ptr is valid
            if !self.is_valid_block(ptr) {
                return Err("Invalid pointer");
            }
            
            // Check if already in free list
            if self.free_list.contains(&ptr) {
                return Err("Double free");
            }
            
            self.free_list.push(ptr);
            Ok(())
        }
        
        pub fn available_blocks(&self) -> usize {
            self.free_list.len()
        }
        
        pub fn block_size(&self) -> usize {
            self.block_size
        }
        
        fn is_valid_block(&self, ptr: *mut u8) -> bool {
            // TODO: Check if pointer is within our managed memory
            if ptr.is_null() {
                return false;
            }
            
            unsafe {
                let offset = ptr.offset_from(self.memory);
                if offset < 0 {
                    return false;
                }
                
                let offset = offset as usize;
                offset < (self.capacity * self.block_size) && (offset % self.block_size == 0)
            }
        }
    }
    
    impl Drop for MemoryPool {
        fn drop(&mut self) {
            if !self.memory.is_null() {
                unsafe {
                    let total_size = self.block_size * self.capacity;
                    let layout = Layout::from_size_align(total_size, mem::align_of::<u8>()).unwrap();
                    alloc::dealloc(self.memory, layout);
                }
            }
        }
    }
    
    // Test the memory pool
    let mut pool = MemoryPool::new(64, 10);
    
    assert_eq!(pool.available_blocks(), 10);
    assert_eq!(pool.block_size(), 64);
    
    // Allocate some blocks
    let block1 = pool.allocate().unwrap();
    let block2 = pool.allocate().unwrap();
    
    assert_eq!(pool.available_blocks(), 8);
    
    // Use the blocks
    unsafe {
        *block1 = 42;
        *block2 = 100;
        
        assert_eq!(*block1, 42);
        assert_eq!(*block2, 100);
    }
    
    // Return blocks
    pool.deallocate(block1).unwrap();
    assert_eq!(pool.available_blocks(), 9);
    
    // Test error conditions
    assert!(pool.deallocate(ptr::null_mut()).is_err());
    assert!(pool.deallocate(block1).is_err()); // Double free
    
    println!("MemoryPool implementation: ✅");
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Lock-free data structure
fn exercise_2_5() {
    println!("Exercise 2.5: Lock-Free Stack");
    
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    // TODO: Implement a lock-free stack using atomic operations
    
    pub struct LockFreeStack<T> {
        head: AtomicPtr<Node<T>>,
    }
    
    struct Node<T> {
        data: T,
        next: *mut Node<T>,
    }
    
    impl<T> LockFreeStack<T> {
        pub fn new() -> Self {
            // TODO: Initialize empty stack
            Self {
                head: AtomicPtr::new(ptr::null_mut()),
            }
        }
        
        pub fn push(&self, data: T) {
            // TODO: Atomic push operation
            let new_node = Box::into_raw(Box::new(Node {
                data,
                next: ptr::null_mut(),
            }));
            
            loop {
                let head = self.head.load(Ordering::Acquire);
                unsafe {
                    (*new_node).next = head;
                }
                
                match self.head.compare_exchange_weak(
                    head,
                    new_node,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(_) => continue, // Retry
                }
            }
        }
        
        pub fn pop(&self) -> Option<T> {
            // TODO: Atomic pop operation
            loop {
                let head = self.head.load(Ordering::Acquire);
                if head.is_null() {
                    return None;
                }
                
                let next = unsafe { (*head).next };
                
                match self.head.compare_exchange_weak(
                    head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        let node = unsafe { Box::from_raw(head) };
                        return Some(node.data);
                    }
                    Err(_) => continue, // Retry
                }
            }
        }
        
        pub fn is_empty(&self) -> bool {
            self.head.load(Ordering::Acquire).is_null()
        }
    }
    
    impl<T> Drop for LockFreeStack<T> {
        fn drop(&mut self) {
            // TODO: Clean up remaining nodes
            while let Some(_) = self.pop() {}
        }
    }
    
    // Safety: Stack operations are atomic
    unsafe impl<T: Send> Send for LockFreeStack<T> {}
    unsafe impl<T: Send> Sync for LockFreeStack<T> {}
    
    // Test the lock-free stack
    let stack = LockFreeStack::new();
    
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    assert!(!stack.is_empty());
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
    
    println!("LockFreeStack implementation: ✅");
    
    // Test thread safety (basic test)
    use std::sync::Arc;
    use std::thread;
    
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];
    
    // Spawn threads to push data
    for i in 0..4 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                stack_clone.push(i * 10 + j);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Count items
    let mut count = 0;
    while stack.pop().is_some() {
        count += 1;
    }
    
    assert_eq!(count, 40); // 4 threads * 10 items each
    
    println!("Thread safety test: ✅");
    println!("✅ Exercise 2.5 complete\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_vec_basic() {
        let mut vec = SafeVec::new();
        assert_eq!(vec.len(), 0);
        
        vec.push(42);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.get(0), Some(&42));
        
        assert_eq!(vec.pop(), Some(42));
        assert_eq!(vec.len(), 0);
    }
    
    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(32, 5);
        assert_eq!(pool.available_blocks(), 5);
        
        let block = pool.allocate().unwrap();
        assert_eq!(pool.available_blocks(), 4);
        
        pool.deallocate(block).unwrap();
        assert_eq!(pool.available_blocks(), 5);
    }
    
    #[test]
    fn test_lock_free_stack() {
        let stack = LockFreeStack::new();
        
        stack.push(1);
        stack.push(2);
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
