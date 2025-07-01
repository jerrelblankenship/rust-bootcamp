# Exercise 02 - Level 2 Hints 🟡

## Specific Solutions for Unsafe Abstractions

### SafeVec Implementation Strategy

#### 1. Memory Allocation
```rust
use std::alloc::{self, Layout};

// Allocate memory for T
let layout = Layout::array::<T>(capacity).unwrap();
let ptr = unsafe { alloc::alloc(layout) as *mut T };

// Check for allocation failure
if ptr.is_null() {
    alloc::handle_alloc_error(layout);
}
```

#### 2. Safe Public Interface
```rust
impl<T> SafeVec<T> {
    pub fn push(&mut self, value: T) {
        // Check capacity first
        if self.len == self.capacity {
            self.grow();  // Internal unsafe operation
        }
        
        // Safe to write because we ensured capacity
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None  // Bounds checking prevents undefined behavior
        }
    }
}
```

#### 3. Memory Growth Strategy
```rust
fn grow(&mut self) {
    let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
    
    let new_layout = Layout::array::<T>(new_capacity).unwrap();
    let new_ptr = unsafe { alloc::alloc(new_layout) as *mut T };
    
    if !new_ptr.is_null() {
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
```

### Safety Invariants to Maintain

1. **Valid pointer**: `ptr` is either null or points to valid allocated memory
2. **Capacity bounds**: Never write beyond allocated capacity
3. **Length tracking**: `len` always ≤ `capacity`
4. **Proper cleanup**: Drop implementation deallocates memory

### C# Comparison
```csharp
// C# List<T> hides this complexity
var list = new List<int>();
list.Add(42);  // Automatic growth, GC cleanup
```

```rust
// Rust SafeVec makes it explicit
let mut vec = SafeVec::new();
vec.push(42);  // Manual growth, RAII cleanup
```

Need complete implementation? Check Level 3 hints!