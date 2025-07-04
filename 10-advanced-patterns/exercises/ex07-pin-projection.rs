// Exercise 07: Pin Projection - Pin, Unpin, and Self-Referential Structs
//
// Learning Objectives:
// - Understand Pin and Unpin traits
// - Debug self-referential struct issues
// - Learn async/await implementation details
// - Compare with C# async state machines and GC handles
//
// C# Analogy: Like async state machines that can't be moved in memory,
// similar to GCHandle.Pinned for preventing garbage collection movement.
//
// Your Mission: Fix the broken Pin implementations to enable safe
// self-referential structs and async state machines.

use std::pin::Pin;
use std::marker::PhantomPinned;
use std::ptr::NonNull;

// ❌ CHECKPOINT 1: Basic Pin Usage Issues
// This self-referential struct can't be moved safely
// C# equivalent: Struct with internal pointer that breaks when moved
struct SelfReferential {
    data: String,
    // ❌ This should be a pointer to data, but the setup is wrong
    self_ref: *const String,
    // ❌ Missing PhantomPinned marker
}

impl SelfReferential {
    fn new(data: String) -> Self {
        Self {
            data,
            // ❌ This self-reference is invalid
            self_ref: std::ptr::null(),
        }
    }
    
    // ❌ This method should work with Pin<&mut Self>
    fn init_self_ref(&mut self) {
        self.self_ref = &self.data;
    }
    
    fn get_self_ref(&self) -> &str {
        unsafe {
            // ❌ This dereference is potentially unsafe
            &*self.self_ref
        }
    }
}

// ❌ CHECKPOINT 2: Pin Projection Issues
// This struct should support pin projection for its fields
// C# equivalent: Pinning individual fields in a larger pinned structure
struct Container {
    pinned_field: SelfReferential,
    normal_field: i32,
    // ❌ Missing PhantomPinned marker
}

impl Container {
    fn new(data: String) -> Self {
        Self {
            pinned_field: SelfReferential::new(data),
            normal_field: 42,
        }
    }
    
    // ❌ This pin projection method is incorrect
    fn project_pinned_field(&mut self) -> &mut SelfReferential {
        &mut self.pinned_field
    }
    
    // ❌ This should return Pin<&mut i32>
    fn project_normal_field(&mut self) -> &mut i32 {
        &mut self.normal_field
    }
}

// ❌ CHECKPOINT 3: Custom Future Implementation Issues
// This custom future has incorrect Pin usage
// C# equivalent: Custom async state machine with incorrect state management
use std::future::Future;
use std::task::{Context, Poll};

struct CustomFuture {
    state: State,
    // ❌ Missing PhantomPinned marker for self-reference
}

enum State {
    Init,
    Waiting(String),
    Complete(String),
}

impl Future for CustomFuture {
    type Output = String;
    
    // ❌ This poll method doesn't handle Pin correctly
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // ❌ This should not move self out of Pin
        let this = self.get_mut();
        
        match &this.state {
            State::Init => {
                this.state = State::Waiting("processing".to_string());
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Waiting(msg) => {
                this.state = State::Complete(format!("Done: {}", msg));
                Poll::Ready("completed".to_string())
            }
            State::Complete(result) => Poll::Ready(result.clone()),
        }
    }
}

// ❌ CHECKPOINT 4: Unpin Trait Implementation Issues
// This type should implement Unpin conditionally
// C# equivalent: Types that can be safely moved vs those that can't
struct ConditionallyUnpin<T> {
    data: T,
    // ❌ This should be conditionally pinned based on T
    _phantom: PhantomPinned,
}

// ❌ This Unpin implementation is wrong
impl<T> Unpin for ConditionallyUnpin<T> {}

impl<T> ConditionallyUnpin<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            _phantom: PhantomPinned,
        }
    }
    
    // ❌ This should use proper pin projection
    fn get_data(&mut self) -> &mut T {
        &mut self.data
    }
}

// ❌ CHECKPOINT 5: Pin Construction Issues
// These pin construction methods are unsafe
// C# equivalent: Incorrect pinning of objects in memory
fn create_pinned_self_ref(data: String) -> Pin<Box<SelfReferential>> {
    let mut instance = Box::new(SelfReferential::new(data));
    // ❌ This initialization is unsafe
    instance.init_self_ref();
    // ❌ This pin creation is potentially unsafe
    unsafe { Pin::new_unchecked(instance) }
}

fn create_pinned_container(data: String) -> Pin<Box<Container>> {
    let instance = Box::new(Container::new(data));
    // ❌ This pin creation doesn't handle initialization properly
    unsafe { Pin::new_unchecked(instance) }
}

// ❌ CHECKPOINT 6: Async Block Pin Issues
// This async block has pin-related problems
// C# equivalent: Async method with incorrect state machine handling
async fn async_with_self_ref() -> String {
    let mut container = Container::new("async data".to_string());
    
    // ❌ This creates a self-reference that breaks when moved
    let self_ref = SelfReferential::new("local".to_string());
    let mut pinned_ref = Box::pin(self_ref);
    pinned_ref.as_mut().init_self_ref();
    
    // ❌ This await might cause the future to be moved
    some_async_operation().await;
    
    // ❌ This access might be invalid after the move
    pinned_ref.get_self_ref().to_string()
}

async fn some_async_operation() {
    // Simulate some async work
    std::thread::sleep(std::time::Duration::from_millis(1));
}

// ❌ CHECKPOINT 7: Pin Projection Macros Issues
// This manual pin projection is incorrect
// C# equivalent: Manual field projection in pinned structures
macro_rules! pin_project {
    ($($field:ident),*) => {
        // ❌ This macro doesn't generate correct pin projection
        impl SelfReferential {
            $(
                fn $field(&mut self) -> &mut _ {
                    &mut self.$field
                }
            )*
        }
    };
}

pin_project!(data, self_ref);

// ❌ CHECKPOINT 8: Pin Drop Issues
// This drop implementation is incorrect for pinned types
// C# equivalent: Finalizer that doesn't handle pinned memory correctly
impl Drop for SelfReferential {
    fn drop(&mut self) {
        // ❌ This drop implementation doesn't consider pinning
        println!("Dropping self-referential struct");
        // The self-reference might be invalid here
        if !self.self_ref.is_null() {
            unsafe {
                // ❌ This might access invalid memory
                println!("Self-ref was: {}", &*self.self_ref);
            }
        }
    }
}

// Helper functions for testing
fn test_pin_basics() {
    println!("Testing basic pin usage...");
    let pinned = create_pinned_self_ref("test data".to_string());
    println!("Self-ref: {}", pinned.get_self_ref());
}

fn test_pin_projection() {
    println!("Testing pin projection...");
    let mut container = Container::new("container data".to_string());
    let pinned_field = container.project_pinned_field();
    println!("Projected field data: {}", pinned_field.data);
}

fn test_custom_future() {
    println!("Testing custom future...");
    let future = CustomFuture {
        state: State::Init,
    };
    
    // This would normally be used with an executor
    let pinned_future = Box::pin(future);
    println!("Future created and pinned");
}

fn main() {
    println!("=== Pin Projection Exercise ===");
    
    // Test Checkpoint 1: Basic Pin usage
    test_pin_basics();
    
    // Test Checkpoint 2: Pin projection
    test_pin_projection();
    
    // Test Checkpoint 3: Custom future
    test_custom_future();
    
    // Test Checkpoint 4: Conditional Unpin
    let conditionally_unpin = ConditionallyUnpin::new("test".to_string());
    println!("Conditionally unpin created");
    
    // Test Checkpoint 5: Pin construction
    let pinned_container = create_pinned_container("pin test".to_string());
    println!("Pinned container created");
    
    // Test Checkpoint 6: Async with self-ref
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let result = async_with_self_ref().await;
        println!("Async result: {}", result);
    });
    
    // Test Checkpoint 7: Pin projection macros
    let mut self_ref = SelfReferential::new("macro test".to_string());
    self_ref.init_self_ref();
    println!("Macro projection test completed");
    
    // Test Checkpoint 8: Drop behavior
    {
        let _temp = SelfReferential::new("drop test".to_string());
        // Drop will be called here
    }
    
    println!("🎉 Pin concepts demonstrated!");
}

// C# Comparison Notes:
//
// 1. Pin is like GCHandle.Pinned preventing object movement
// 2. PhantomPinned is like marking objects as non-movable
// 3. Self-referential structs are like objects with internal pointers
// 4. Pin projection is like accessing fields in pinned objects
// 5. Custom futures are like async state machines with pinning
// 6. Unpin is like marking objects as safe to move
// 7. Pin construction is like creating pinned object handles
// 8. Drop with Pin is like finalizers for pinned objects

// Key Differences from C#:
// - No garbage collector to handle object movement
// - More explicit control over object pinning
// - Better compile-time guarantees for memory safety
// - More complex but safer than GC pinning
// - Zero-cost abstractions for async without GC overhead