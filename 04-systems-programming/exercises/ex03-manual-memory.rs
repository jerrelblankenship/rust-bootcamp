// Exercise 03: Manual Memory Management - Fix Broken Code!
//
// BROKEN: This code has 5 memory management compilation errors
// Your mission: Fix each error to understand manual allocation/deallocation
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 checkpoints to fix)
//
// APPROACH:
// - Fix ONE memory error at a time following the FIXME comments
// - Compile after each fix: `rustc ex03-manual-memory.rs`
// - Understand memory ownership and lifetimes
// - Use hints only after trying for 15+ minutes per checkpoint
//
// C# COMPARISON: Like using Marshal.AllocHGlobal/FreeHGlobal but with
// Rust's ownership system preventing memory leaks and double-free!

use std::alloc::{self, Layout};
use std::ptr;
use std::mem;

fn main() {
    println!("=== Exercise 3: Manual Memory Management ===\n");
    
    // Fix these ONE AT A TIME - uncomment as you go
    checkpoint_1_allocation_basics();
    // checkpoint_2_deallocation_safety();
    // checkpoint_3_memory_leaks();
    // checkpoint_4_double_free_prevention();
    // checkpoint_5_raii_patterns();
    
    println!("All checkpoints complete! 🎉");
    println!("Progress: [██████████] 100% Complete");
}

// ✅ CHECKPOINT 1: Fix allocation basics
fn checkpoint_1_allocation_basics() {
    println!("🔧 Checkpoint 1: Fix memory allocation");
    
    // FIXME: Manual allocation is unsafe
    // ERROR: call to unsafe function is unsafe
    let layout = Layout::new::<i32>();
    let ptr = alloc::alloc(layout) as *mut i32;  // COMPILE ERROR: unsafe!
    
    if ptr.is_null() {
        panic!("Allocation failed!");
    }
    
    // FIXME: Writing to allocated memory is unsafe
    // ERROR: dereference of raw pointer is unsafe
    *ptr = 42;  // COMPILE ERROR: unsafe dereference!
    
    println!("Allocated and wrote value: {}", *ptr);  // COMPILE ERROR: unsafe!
    
    // FIXME: Deallocation is unsafe
    // ERROR: call to unsafe function is unsafe
    alloc::dealloc(ptr as *mut u8, layout);  // COMPILE ERROR: unsafe!
    
    println!("✅ Checkpoint 1 complete!");
    println!("Progress: [██░░░░░░░░] 20% Complete");
    println!("Now uncomment checkpoint_2_deallocation_safety() in main()\n");
}

// ✅ CHECKPOINT 2: Fix deallocation safety
fn checkpoint_2_deallocation_safety() {
    println!("🔧 Checkpoint 2: Fix deallocation issues");
    
    unsafe {
        let layout = Layout::new::<u64>();
        let ptr = alloc::alloc(layout) as *mut u64;
        
        if !ptr.is_null() {
            *ptr = 123456;
            println!("Value: {}", *ptr);
            
            // FIXME: Wrong layout for deallocation!
            // ERROR: This will cause undefined behavior
            let wrong_layout = Layout::new::<u32>();  // Wrong size!
            alloc::dealloc(ptr as *mut u8, wrong_layout);  // SAFETY ERROR!
        }
    }
    
    println!("✅ Checkpoint 2 complete!");
    println!("Progress: [████░░░░░░] 40% Complete");
    println!("Now uncomment checkpoint_3_memory_leaks() in main()\n");
}

// ✅ CHECKPOINT 3: Fix memory leaks
fn checkpoint_3_memory_leaks() {
    println!("🔧 Checkpoint 3: Fix memory leak");
    
    unsafe {
        let layout = Layout::array::<i32>(1000).unwrap();
        let ptr = alloc::alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            // Initialize some values
            for i in 0..10 {
                *ptr.add(i) = i as i32;
            }
            
            println!("First few values: {:?}", 
                std::slice::from_raw_parts(ptr, 5));
            
            // FIXME: Memory is never freed! This is a leak!
            // TODO: Add proper deallocation
        }
    }
    
    println!("✅ Checkpoint 3 complete!");
    println!("Progress: [██████░░░░] 60% Complete");
    println!("Now uncomment checkpoint_4_double_free_prevention() in main()\n");
}

// ✅ CHECKPOINT 4: Fix double-free prevention
fn checkpoint_4_double_free_prevention() {
    println!("🔧 Checkpoint 4: Fix double-free issue");
    
    unsafe {
        let layout = Layout::new::<f64>();
        let ptr = alloc::alloc(layout) as *mut f64;
        
        if !ptr.is_null() {
            *ptr = 3.14159;
            println!("Pi: {}", *ptr);
            
            // Free once
            alloc::dealloc(ptr as *mut u8, layout);
            
            // FIXME: Double free! This is undefined behavior!
            alloc::dealloc(ptr as *mut u8, layout);  // SAFETY ERROR: Double free!
        }
    }
    
    println!("✅ Checkpoint 4 complete!");
    println!("Progress: [████████░░] 80% Complete");
    println!("Now uncomment checkpoint_5_raii_patterns() in main()\n");
}

// ✅ CHECKPOINT 5: Fix RAII patterns
fn checkpoint_5_raii_patterns() {
    println!("🔧 Checkpoint 5: Fix RAII wrapper");
    
    // FIXME: This struct doesn't implement Drop!
    // Memory will leak when it goes out of scope
    struct ManagedMemory {
        ptr: *mut u8,
        layout: Layout,
    }
    
    impl ManagedMemory {
        fn new(size: usize) -> Option<Self> {
            unsafe {
                let layout = Layout::from_size_align(size, 1).ok()?;
                let ptr = alloc::alloc(layout);
                
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr, layout })
                }
            }
        }
        
        fn as_slice_mut(&mut self) -> &mut [u8] {
            unsafe {
                std::slice::from_raw_parts_mut(self.ptr, self.layout.size())
            }
        }
    }
    
    // TODO: Implement Drop trait to prevent memory leaks
    // impl Drop for ManagedMemory {
    //     fn drop(&mut self) {
    //         // TODO: Free the memory
    //     }
    // }
    
    {
        let mut memory = ManagedMemory::new(1024).expect("Allocation failed");
        let slice = memory.as_slice_mut();
        slice[0] = 42;
        slice[1] = 24;
        println!("First two bytes: {} {}", slice[0], slice[1]);
        
        // Memory goes out of scope here - will it be freed?
    }
    
    println!("✅ Checkpoint 5 complete!");
    println!("Progress: [██████████] 100% Complete");
    println!("🎉 Exercise 3 completed! Move to ex04-safe-abstractions.rs\n");
}

/*
COMPILATION CHALLENGES:
1. error[E0133]: call to unsafe function `alloc` is unsafe
2. error[E0133]: dereference of raw pointer is unsafe
3. Wrong layout causes undefined behavior
4. Memory leak when allocation isn't freed
5. Double-free causes undefined behavior
6. Missing Drop implementation causes memory leaks

C# COMPARISON:
In C# you might write:
```csharp
// Manual allocation (rare)
IntPtr ptr = Marshal.AllocHGlobal(sizeof(int));
Marshal.WriteInt32(ptr, 42);
int value = Marshal.ReadInt32(ptr);
Marshal.FreeHGlobal(ptr);  // Manual cleanup
```

In Rust, you have precise control but must maintain safety!

SAFETY RULES:
- All allocations must be matched with corresponding deallocations
- Use the same Layout for alloc/dealloc
- Never dereference null pointers
- Never access memory after freeing
- Never free the same pointer twice

HINTS:
- Level 1: What keyword makes unsafe operations legal?
- Level 2: Wrap all unsafe operations in unsafe blocks
- Level 3: Implement Drop trait for automatic cleanup
*/