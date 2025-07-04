# Exercise 07 - Level 2 Hints: Pin Projection

## 🎯 Focus Areas

You should now understand that Pin prevents moving self-referential structs. Let's fix the specific Pin and projection issues.

## 🔧 Specific Issues to Fix

### Issue 1: Missing PhantomPinned Markers
```rust
// ❌ Wrong - self-referential struct without marker
struct SelfReferential {
    data: String,
    self_ref: *const String,
}

// ✅ Correct - add PhantomPinned
struct SelfReferential {
    data: String,
    self_ref: *const String,
    _pin: PhantomPinned,
}
```

### Issue 2: Pin Method Signatures
```rust
// ❌ Wrong - should work with Pin<&mut Self>
impl SelfReferential {
    fn init_self_ref(&mut self) {
        self.self_ref = &self.data;
    }
}

// ✅ Correct - use Pin<&mut Self>
impl SelfReferential {
    fn init_self_ref(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        this.self_ref = &this.data;
    }
}
```

### Issue 3: Pin Projection
```rust
// ❌ Wrong - direct field access on pinned type
fn project_pinned_field(&mut self) -> &mut SelfReferential {
    &mut self.pinned_field
}

// ✅ Correct - proper pin projection
fn project_pinned_field(self: Pin<&mut Self>) -> Pin<&mut SelfReferential> {
    unsafe { self.map_unchecked_mut(|s| &mut s.pinned_field) }
}
```

## 🔍 C# Comparison

```csharp
// C# pinned memory with GCHandle
GCHandle handle = GCHandle.Alloc(data, GCHandleType.Pinned);
try {
    // Data can't be moved by GC while pinned
    IntPtr ptr = handle.AddrOfPinnedObject();
    // Use ptr...
} finally {
    handle.Free();
}
```

## 🎮 Implementation Strategy

1. **Add PhantomPinned markers** - Mark unmovable types
2. **Fix method signatures** - Use Pin<&mut Self>
3. **Implement pin projection** - Safe field access
4. **Fix Future implementations** - Handle Pin in poll
5. **Test pinned construction** - Verify safe initialization

## 🔧 Code Patterns to Apply

### Pattern 1: PhantomPinned Struct
```rust
use std::marker::PhantomPinned;

struct SelfReferential {
    data: String,
    self_ref: *const String,
    _pin: PhantomPinned,
}
```

### Pattern 2: Pin Constructor
```rust
impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(Self {
            data,
            self_ref: std::ptr::null(),
            _pin: PhantomPinned,
        });
        
        // Initialize self-reference after pinning
        let data_ptr = &boxed.data as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).self_ref = data_ptr;
        }
        
        boxed
    }
}
```

### Pattern 3: Pin Projection
```rust
impl Container {
    fn project_pinned_field(self: Pin<&mut Self>) -> Pin<&mut SelfReferential> {
        unsafe { self.map_unchecked_mut(|s| &mut s.pinned_field) }
    }
    
    fn project_normal_field(self: Pin<&mut Self>) -> &mut i32 {
        unsafe { &mut self.get_unchecked_mut().normal_field }
    }
}
```

## ⏰ Time Check

Spent 30 minutes total? If you're still struggling with Pin projection and Future implementations, move to Level 3 for complete solutions.

**Hint**: The `CustomFuture` needs to handle Pin properly in its `poll` method - use `unsafe` pin projection carefully!