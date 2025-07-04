# Exercise 05 - Level 2 Hints: Phantom Problems

## 🎯 Focus Areas

You should now understand that phantom types are about compile-time type safety. Let's fix the specific PhantomData issues.

## 🔧 Specific Issues to Fix

### Issue 1: Missing PhantomData Fields
```rust
// ❌ Wrong - generic parameter not used
struct StateMachine<State> {
    data: String,
    _state: State,  // Can't store zero-sized type directly
}

// ✅ Correct - use PhantomData
struct StateMachine<State> {
    data: String,
    _state: PhantomData<State>,
}
```

### Issue 2: Incorrect PhantomData Initialization
```rust
// ❌ Wrong - trying to create instance
_state: Initial,

// ✅ Correct - use PhantomData
_state: PhantomData,
```

### Issue 3: Wrong Return Types in State Transitions
```rust
// ❌ Wrong - returning wrong state type
fn start_processing(self) -> StateMachine<Initial> {
    StateMachine {
        data: self.data,
        _state: PhantomData,  // But this is still Initial!
    }
}

// ✅ Correct - return correct state type
fn start_processing(self) -> StateMachine<Processing> {
    StateMachine {
        data: self.data,
        _state: PhantomData,
    }
}
```

## 🔍 C# Comparison

```csharp
// C# example of phantom-like types
public class StateMachine<TState> where TState : class, new()
{
    private string data;
    // TState is used only for compile-time safety
    // Similar to Rust's PhantomData<TState>
}
```

## 🎮 Implementation Strategy

1. **Add PhantomData fields** - Replace direct type usage
2. **Fix initialization** - Use `PhantomData` constructor
3. **Fix return types** - Ensure state transitions return correct types
4. **Handle measurements** - Fix unit conversion phantom types
5. **Test compile-time safety** - Verify type checking works

## 🔧 Code Patterns to Apply

### Pattern 1: Basic PhantomData
```rust
struct Wrapper<T> {
    data: String,
    _phantom: PhantomData<T>,
}

impl<T> Wrapper<T> {
    fn new(data: String) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}
```

### Pattern 2: State Machine Transitions
```rust
impl StateMachine<Initial> {
    fn start_processing(self) -> StateMachine<Processing> {
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}
```

### Pattern 3: Measurement Units
```rust
impl<T> Measurement<T, Meters> {
    fn to_feet(self) -> Measurement<T, Feet> {
        Measurement {
            value: self.value * 3.28084,
            _unit: PhantomData,
        }
    }
}
```

## ⏰ Time Check

Spent 30 minutes total? If you're still struggling with phantom type patterns, move to Level 3 for complete solutions.

**Hint**: Focus on the StateMachine - each state transition method should return a different `StateMachine<State>` type!