# Exercise 02 - Level 3 Hints 🔴

## Complete SafeVec Implementation

### Full Implementation Structure
```rust
use std::alloc::{self, Layout};
use std::mem;
use std::ptr;

pub struct SafeVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SafeVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }
        
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };
        
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }
        
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }
    
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.add(self.len))) }
        }
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
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
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let new_ptr = unsafe { alloc::alloc(new_layout) as *mut T };
        
        if new_ptr.is_null() {
            alloc::handle_alloc_error(new_layout);
        }
        
        // Copy existing elements
        if !self.ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                
                // Deallocate old memory
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.ptr as *mut u8, old_layout);
            }
        }
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

// Critical: Proper cleanup
impl<T> Drop for SafeVec<T> {
    fn drop(&mut self) {
        // Drop all elements
        while let Some(_) = self.pop() {}
        
        // Deallocate memory
        if !self.ptr.is_null() {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

// Make it usable in for loops
impl<T> IntoIterator for SafeVec<T> {
    type Item = T;
    type IntoIter = SafeVecIntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        SafeVecIntoIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct SafeVecIntoIter<T> {
    vec: SafeVec<T>,
    index: usize,
}

impl<T> Iterator for SafeVecIntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let value = unsafe { ptr::read(self.vec.ptr.add(self.index)) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
```

### Key Safety Insights

1. **RAII Pattern**: Constructor allocates, destructor deallocates
2. **Bounds Checking**: Always validate indices before unsafe operations
3. **Null Pointer Checks**: Handle allocation failures gracefully
4. **Proper Element Handling**: Use `ptr::read`/`ptr::write` for non-Copy types
5. **Iterator Safety**: Ensure proper cleanup even during iteration

You've now implemented a production-quality safe abstraction over unsafe code!