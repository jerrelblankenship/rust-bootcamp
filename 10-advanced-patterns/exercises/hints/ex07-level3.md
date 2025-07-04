# Exercise 07 - Level 3 Hints: Pin Projection

## 🎯 Complete Solutions

Here are the fixes for each checkpoint:

## 🔧 Checkpoint 1: Self-Referential Struct Fix

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfReferential {
    data: String,
    self_ref: *const String,
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Self {
        Self {
            data,
            self_ref: std::ptr::null(),
            _pin: PhantomPinned,
        }
    }
    
    fn init_self_ref(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        this.self_ref = &this.data;
    }
    
    fn get_self_ref(&self) -> &str {
        unsafe {
            if self.self_ref.is_null() {
                ""
            } else {
                &*self.self_ref
            }
        }
    }
}
```

## 🔧 Checkpoint 2: Container with Pin Projection

```rust
struct Container {
    pinned_field: SelfReferential,
    normal_field: i32,
    _pin: PhantomPinned,
}

impl Container {
    fn new(data: String) -> Self {
        Self {
            pinned_field: SelfReferential::new(data),
            normal_field: 42,
            _pin: PhantomPinned,
        }
    }
    
    fn project_pinned_field(self: Pin<&mut Self>) -> Pin<&mut SelfReferential> {
        unsafe { self.map_unchecked_mut(|s| &mut s.pinned_field) }
    }
    
    fn project_normal_field(self: Pin<&mut Self>) -> &mut i32 {
        unsafe { &mut self.get_unchecked_mut().normal_field }
    }
}
```

## 🔧 Checkpoint 3: Custom Future Implementation

```rust
use std::future::Future;
use std::task::{Context, Poll};

struct CustomFuture {
    state: State,
    _pin: PhantomPinned,
}

impl Future for CustomFuture {
    type Output = String;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
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
```

## 🔧 Checkpoint 4: Conditional Unpin Fix

```rust
struct ConditionallyUnpin<T> {
    data: T,
    _phantom: PhantomPinned,
}

// Only implement Unpin if T is Unpin
impl<T: Unpin> Unpin for ConditionallyUnpin<T> {}

impl<T> ConditionallyUnpin<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            _phantom: PhantomPinned,
        }
    }
    
    fn get_data(self: Pin<&mut Self>) -> Pin<&mut T> {
        unsafe { self.map_unchecked_mut(|s| &mut s.data) }
    }
    
    // Only if T: Unpin
    fn get_data_unpinned(self: Pin<&mut Self>) -> &mut T 
    where
        T: Unpin,
    {
        unsafe { &mut self.get_unchecked_mut().data }
    }
}
```

## 🔧 Checkpoint 5: Safe Pin Construction

```rust
fn create_pinned_self_ref(data: String) -> Pin<Box<SelfReferential>> {
    let mut boxed = Box::pin(SelfReferential::new(data));
    
    // Safe initialization after pinning
    let pinned_ref = boxed.as_mut();
    pinned_ref.init_self_ref();
    
    boxed
}

fn create_pinned_container(data: String) -> Pin<Box<Container>> {
    let mut boxed = Box::pin(Container::new(data));
    
    // Initialize the pinned field
    let pinned_field = boxed.as_mut().project_pinned_field();
    pinned_field.init_self_ref();
    
    boxed
}
```

## 🔧 Checkpoint 6: Async Block Fix

```rust
async fn async_with_self_ref() -> String {
    // Create pinned self-referential struct
    let self_ref = SelfReferential::new("local".to_string());
    let mut pinned_ref = Box::pin(self_ref);
    
    // Initialize after pinning
    pinned_ref.as_mut().init_self_ref();
    
    // Await operations
    some_async_operation().await;
    
    // Access is safe because pinned_ref can't move
    pinned_ref.get_self_ref().to_string()
}
```

## 🔧 Checkpoint 7: Pin Projection Macro Fix

```rust
// Manual pin projection implementation
impl SelfReferential {
    fn project_data(self: Pin<&mut Self>) -> &mut String {
        unsafe { &mut self.get_unchecked_mut().data }
    }
    
    fn project_self_ref(self: Pin<&mut Self>) -> &mut *const String {
        unsafe { &mut self.get_unchecked_mut().self_ref }
    }
}

// Better: Use the pin-project crate
// [dependencies]
// pin-project = "1.0"

/*
use pin_project::pin_project;

#[pin_project]
struct SelfReferential {
    #[pin]
    data: String,
    self_ref: *const String,
}
*/
```

## 🔧 Checkpoint 8: Safe Drop Implementation

```rust
impl Drop for SelfReferential {
    fn drop(&mut self) {
        // Safe to access in drop - we're being destroyed anyway
        println!("Dropping self-referential struct");
        
        // Don't dereference self_ref during drop - it might be invalid
        // Just clear the pointer
        self.self_ref = std::ptr::null();
    }
}
```

## 🎮 Helper Functions for Testing

```rust
fn test_pin_basics() {
    println!("Testing basic pin usage...");
    let pinned = create_pinned_self_ref("test data".to_string());
    println!("Self-ref: {}", pinned.get_self_ref());
}

fn test_pin_projection() {
    println!("Testing pin projection...");
    let mut container = Box::pin(Container::new("container data".to_string()));
    let pinned_field = container.as_mut().project_pinned_field();
    pinned_field.init_self_ref();
    println!("Projected field initialized");
}

fn test_custom_future() {
    println!("Testing custom future...");
    let future = CustomFuture {
        state: State::Init,
        _pin: PhantomPinned,
    };
    
    let pinned_future = Box::pin(future);
    println!("Future created and pinned");
}
```

## 🎮 Complete Main Function

```rust
fn main() {
    println!("=== Pin Projection Exercise ===");
    
    // Test basic Pin usage
    test_pin_basics();
    
    // Test pin projection
    test_pin_projection();
    
    // Test custom future
    test_custom_future();
    
    // Test conditional Unpin
    let conditionally_unpin = ConditionallyUnpin::new("test".to_string());
    let mut pinned_conditional = Box::pin(conditionally_unpin);
    // This works because String: Unpin
    let _data = pinned_conditional.as_mut().get_data_unpinned();
    
    println!("🎉 Pin concepts demonstrated!");
}
```

## 🔍 Key Takeaways

1. **PhantomPinned marks types as unmovable** - required for self-referential structs
2. **Pin<&mut Self> in methods** - for types that can't be moved
3. **Pin projection** - safe access to fields of pinned types
4. **Box::pin() for heap allocation** - easiest way to create pinned values
5. **Conditional Unpin** - implement Unpin only when safe
6. **Drop is always safe** - objects being destroyed can't be moved

## 🎯 Verification

Your code should now handle self-referential structs safely without undefined behavior. The Pin system should prevent accidental moves that would break internal pointers!