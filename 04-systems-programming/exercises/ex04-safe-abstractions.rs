// Exercise 04: Safe Abstractions - Fix Broken Code!
//
// BROKEN: This SafeVec implementation has 6 critical safety issues
// Your mission: Fix each issue to create a production-quality safe API
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// APPROACH:
// - Fix ONE safety issue at a time following the FIXME comments
// - Test each fix with the provided test cases
// - Understand the safety invariants being maintained
// - Use hints only after trying for 15+ minutes per checkpoint
//
// C# COMPARISON: Like implementing a custom List<T> but with manual
// memory management and compile-time safety guarantees!

use std::alloc::{self, Layout};
use std::ptr;
use std::mem;

fn main() {
    println!("=== Exercise 2B: Safe Abstractions Over Unsafe Code ===\n");
    
    // Fix these ONE AT A TIME - uncomment as you go
    checkpoint_1_memory_allocation();
    // checkpoint_2_growth_strategy();
    // checkpoint_3_drop_implementation();
    // checkpoint_4_iterator_safety();
    // checkpoint_5_bounds_checking();
    // checkpoint_6_memory_leaks();
    
    println!("All checkpoints complete! 🎉");
    println!("Progress: [██████████] 100% Complete");
}

// ✅ CHECKPOINT 1: Fix memory allocation safety
fn checkpoint_1_memory_allocation() {
    println!("🔧 Checkpoint 1: Fix allocation issues");
    
    struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SafeVec<T> {
        fn with_capacity(capacity: usize) -> Self {
            if capacity == 0 {
                return Self {
                    ptr: ptr::null_mut(),
                    len: 0,
                    capacity: 0,
                };
            }
            
            // FIXME: This allocation is unsafe and doesn't handle errors!
            let layout = Layout::array::<T>(capacity).unwrap();
            let ptr = alloc::alloc(layout) as *mut T;  // SAFETY ISSUE: No unsafe block!
            
            // FIXME: No null check for allocation failure!
            
            Self { ptr, len: 0, capacity }
        }
        
        fn push(&mut self, value: T) {
            // FIXME: Writing to potentially invalid memory!
            ptr::write(self.ptr.add(self.len), value);  // SAFETY ISSUE: No unsafe block!
            self.len += 1;
        }
    }
    
    // Test your fix
    let mut vec = SafeVec::with_capacity(10);
    vec.push(42);
    println!("Created SafeVec with capacity 10");
    
    println!("✅ Checkpoint 1 complete!");
    println!("Progress: [█░░░░░░░░░] 17% Complete");
    println!("Now uncomment checkpoint_2_growth_strategy() in main()\n");
}

// ✅ CHECKPOINT 2: Fix growth strategy
fn checkpoint_2_growth_strategy() {
    println!("🔧 Checkpoint 2: Fix growth and capacity issues");
    
    struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SafeVec<T> {
        fn push(&mut self, value: T) {
            // FIXME: No capacity checking! Will write out of bounds!
            if self.len == self.capacity {
                // self.grow();  // Not implemented yet!
            }
            
            unsafe {
                ptr::write(self.ptr.add(self.len), value);
            }
            self.len += 1;
        }
        
        // TODO: Implement safe growth strategy
        fn grow(&mut self) {
            let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
            
            // FIXME: Unsafe reallocation without proper copying!
            let layout = Layout::array::<T>(new_capacity).unwrap();
            let new_ptr = alloc::alloc(layout) as *mut T;  // SAFETY ISSUE: No unsafe!
            
            // FIXME: No copying of existing elements!
            // FIXME: No deallocation of old memory!
            
            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
    }
    
    println!("✅ Checkpoint 2 complete!");
    println!("Progress: [███░░░░░░░] 33% Complete");
    println!("Now uncomment checkpoint_3_drop_implementation() in main()\n");
}

// ✅ CHECKPOINT 3: Fix Drop implementation
fn checkpoint_3_drop_implementation() {
    println!("🔧 Checkpoint 3: Fix memory cleanup");
    
    struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    // FIXME: No Drop implementation! Memory will leak!
    // impl<T> Drop for SafeVec<T> {
    //     fn drop(&mut self) {
    //         // TODO: Implement proper cleanup
    //     }
    // }
    
    {
        let mut vec = SafeVec {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        };
        // vec goes out of scope here - memory leak!
    }
    
    println!("✅ Checkpoint 3 complete!");
    println!("Progress: [█████░░░░░] 50% Complete");
    println!("Now uncomment checkpoint_4_iterator_safety() in main()\n");
}

// ✅ CHECKPOINT 4: Fix iterator implementation
fn checkpoint_4_iterator_safety() {
    println!("🔧 Checkpoint 4: Fix iterator safety");
    
    struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SafeVec<T> {
        fn iter(&self) -> SafeVecIter<T> {
            SafeVecIter {
                ptr: self.ptr,
                end: self.ptr.add(self.len),  // SAFETY ISSUE: No unsafe block!
                _marker: std::marker::PhantomData,
            }
        }
    }
    
    struct SafeVecIter<T> {
        ptr: *mut T,
        end: *mut T,
        _marker: std::marker::PhantomData<T>,
    }
    
    impl<T> Iterator for SafeVecIter<T> {
        type Item = T;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr == self.end {
                None
            } else {
                // FIXME: Reading from potentially invalid memory!
                let result = ptr::read(self.ptr);  // SAFETY ISSUE: No unsafe block!
                self.ptr = self.ptr.add(1);        // SAFETY ISSUE: No unsafe block!
                Some(result)
            }
        }
    }
    
    println!("✅ Checkpoint 4 complete!");
    println!("Progress: [███████░░░] 67% Complete");
    println!("Now uncomment checkpoint_5_bounds_checking() in main()\n");
}

// ✅ CHECKPOINT 5: Fix bounds checking
fn checkpoint_5_bounds_checking() {
    println!("🔧 Checkpoint 5: Fix indexing safety");
    
    struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SafeVec<T> {
        fn get(&self, index: usize) -> Option<&T> {
            // FIXME: No bounds checking! Can access invalid memory!
            unsafe {
                Some(&*self.ptr.add(index))  // SAFETY ISSUE: No bounds check!
            }
        }
        
        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            // FIXME: No bounds checking for mutable access!
            unsafe {
                Some(&mut *self.ptr.add(index))  // SAFETY ISSUE: No bounds check!
            }
        }
        
        // TODO: Add safe indexing with panic
        fn index(&self, index: usize) -> &T {
            // FIXME: Should panic with helpful message on out of bounds
            self.get(index).unwrap()
        }
    }
    
    println!("✅ Checkpoint 5 complete!");
    println!("Progress: [█████████░] 83% Complete");
    println!("Now uncomment checkpoint_6_memory_leaks() in main()\n");
}

// ✅ CHECKPOINT 6: Fix memory leaks and double-free issues
fn checkpoint_6_memory_leaks() {
    println!("🔧 Checkpoint 6: Fix memory leak issues");
    
    struct SafeVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
    
    impl<T> SafeVec<T> {
        fn clear(&mut self) {
            // FIXME: Not dropping elements before clearing!
            // This leaks memory for types that need Drop!
            self.len = 0;  // SAFETY ISSUE: Elements not properly dropped!
        }
        
        fn truncate(&mut self, new_len: usize) {
            if new_len < self.len {
                // FIXME: Not dropping truncated elements!
                self.len = new_len;  // SAFETY ISSUE: Elements not properly dropped!
            }
        }
        
        fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                // FIXME: Using wrong read operation!
                unsafe {
                    Some(ptr::read_volatile(self.ptr.add(self.len)))  // SAFETY ISSUE: Wrong read!
                }
            }
        }
    }
    
    // Test case that would leak memory
    {
        let mut vec = SafeVec {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        };
        // Add elements of type that needs Drop (like String)
        // vec.clear(); // Should properly drop all elements
    }
    
    println!("✅ Checkpoint 6 complete!");
    println!("Progress: [██████████] 100% Complete");
    println!("🎉 Exercise 2B completed! Move to ex02-unsafe-debugging.rs\n");
}

/*
COMPILATION CHALLENGES:
1. error[E0133]: call to unsafe function is unsafe
2. error[E0133]: dereference of raw pointer is unsafe
3. Missing Drop implementation causes memory leaks
4. Iterator safety violations with raw pointers
5. Missing bounds checking allows out-of-bounds access
6. Improper element dropping causes memory leaks

SAFETY INVARIANTS TO MAINTAIN:
- ptr is either null or points to valid allocated memory
- len <= capacity always
- capacity represents actual allocated space
- All elements [0..len) are initialized
- Proper cleanup in Drop implementation

C# COMPARISON:
```csharp
// C# List<T> handles all this automatically
var list = new List<int>();
list.Add(42);           // Automatic growth
list.Clear();           // Automatic cleanup
// GC handles memory     // Automatic deallocation
```

In Rust, you control every aspect of memory management!

HINTS:
- Level 1: What makes unsafe operations safe to call?
- Level 2: Use unsafe blocks and validate all invariants
- Level 3: Check bounds, handle allocation failures, implement Drop
*/