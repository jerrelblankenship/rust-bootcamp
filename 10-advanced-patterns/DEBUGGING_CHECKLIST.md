# Advanced Topics Debugging Checklist

## 🎯 Quick Diagnostic Commands

Before diving into specific issues, run these commands to get a complete picture:

```bash
# Basic compilation check
cargo check

# Full build with all features
cargo build --all-features

# Run tests with output
cargo test -- --nocapture

# Expand macros to see generated code
cargo expand

# Check for common issues
cargo clippy -- -D warnings

# Format code consistently
cargo fmt

# Check unused dependencies
cargo machete
```

## 🔧 Macro System Issues

### Declarative Macro Problems

**Common Symptoms:**
- "no rules expected this token" errors
- Macro expansion failures
- Unexpected token sequences

**Debugging Steps:**
1. **Check pattern matching**:
   ```bash
   # Use cargo expand to see what's generated
   cargo expand ::module::macro_name
   
   # Enable macro debugging in Cargo.toml
   [features]
   macro-debug = []
   ```

2. **Verify fragment specifiers**:
   - `$name:ident` - Identifiers (variable names, function names)
   - `$expr:expr` - Expressions
   - `$block:block` - Block expressions `{ ... }`
   - `$stmt:stmt` - Statements
   - `$type:ty` - Types
   - `$pat:pat` - Patterns

3. **Check repetition syntax**:
   - `$(...)* ` - Zero or more
   - `$(...)+ ` - One or more
   - `$(...)? ` - Zero or one

**C# Analogy**: Like debugging T4 templates or Source Generators

### Procedural Macro Problems

**Common Symptoms:**
- "proc-macro derive panicked" errors
- Missing dependencies for proc macros
- Invalid generated code

**Debugging Steps:**
1. **Check dependencies** in `Cargo.toml`:
   ```toml
   [dependencies]
   syn = { version = "2.0", features = ["full"] }
   quote = "1.0"
   proc-macro2 = "1.0"
   
   [lib]
   proc-macro = true
   ```

2. **Add error handling**:
   ```rust
   use syn::{parse_macro_input, DeriveInput, Error};
   
   #[proc_macro_derive(MyDerive)]
   pub fn derive_my_trait(input: TokenStream) -> TokenStream {
       let input = parse_macro_input!(input as DeriveInput);
       
       match generate_impl(&input) {
           Ok(tokens) => tokens.into(),
           Err(err) => err.to_compile_error().into(),
       }
   }
   ```

3. **Use `cargo expand` to verify output**:
   ```bash
   cargo expand --bin your_binary
   ```

**C# Analogy**: Like debugging Source Generators in Visual Studio

## 🎭 Trait Object Issues

### Object Safety Violations

**Common Symptoms:**
- "the trait cannot be made into an object" errors
- "method has generic type parameters" errors
- "method references the `Self` type" errors

**Debugging Steps:**
1. **Check object safety rules**:
   - ❌ No generic methods: `fn process<T>(&self, item: T)`
   - ❌ No associated types: `type Output;`
   - ❌ No static methods: `fn create() -> Self`
   - ❌ No `Self` return types: `fn clone(&self) -> Self`

2. **Fix strategies**:
   ```rust
   // ❌ Not object-safe
   trait Processor {
       fn process<T>(&self, item: T) -> String;
   }
   
   // ✅ Object-safe - split the trait
   trait ProcessorBase {
       fn process_string(&self, item: &str) -> String;
   }
   
   trait ProcessorAdvanced: ProcessorBase {
       fn process<T: Display>(&self, item: T) -> String;
   }
   ```

3. **Use trait objects correctly**:
   ```rust
   // ✅ Correct trait object usage
   let processors: Vec<Box<dyn ProcessorBase>> = vec![
       Box::new(StringProcessor),
       Box::new(NumberProcessor),
   ];
   ```

**C# Analogy**: Like interface methods that can't be called through interface references

### Downcasting Issues

**Common Symptoms:**
- Can't convert trait objects back to concrete types
- Missing `Any` trait implementations

**Debugging Steps:**
1. **Add `Any` trait**:
   ```rust
   use std::any::Any;
   
   trait Component: Any {
       fn render(&self) -> String;
       
       fn as_any(&self) -> &dyn Any {
           self
       }
   }
   ```

2. **Implement safe downcasting**:
   ```rust
   fn handle_component(component: &dyn Component) {
       if let Some(button) = component.as_any().downcast_ref::<Button>() {
           println!("Button: {}", button.label);
       }
   }
   ```

**C# Analogy**: Like casting interface references back to concrete types

## 🔄 Higher-Ranked Trait Bounds (HRTB)

### HRTB Syntax Issues

**Common Symptoms:**
- "expected lifetime parameter" errors
- "cannot infer an appropriate lifetime" errors
- Complex lifetime error messages

**Debugging Steps:**
1. **Use `for<'a>` syntax correctly**:
   ```rust
   // ✅ Correct HRTB
   fn process_with_closure<F>(f: F) -> String
   where
       F: for<'a> Fn(&'a str) -> String,
   {
       f("test")
   }
   ```

2. **Debug with explicit lifetimes first**:
   ```rust
   // Start with explicit lifetimes
   fn debug_version<'a, F>(f: F) -> String
   where
       F: Fn(&'a str) -> String,
   {
       f("test")
   }
   
   // Then convert to HRTB
   fn final_version<F>(f: F) -> String
   where
       F: for<'a> Fn(&'a str) -> String,
   {
       f("test")
   }
   ```

**C# Analogy**: Like generic constraints that work with any type parameter

## 👻 Phantom Type Issues

### PhantomData Usage Problems

**Common Symptoms:**
- "unused type parameter" warnings
- State machine transitions not working
- Variance issues with generic parameters

**Debugging Steps:**
1. **Add PhantomData correctly**:
   ```rust
   use std::marker::PhantomData;
   
   struct StateMachine<State> {
       data: String,
       _state: PhantomData<State>,  // Not just State
   }
   ```

2. **Implement transitions properly**:
   ```rust
   impl StateMachine<Initial> {
       fn new(data: String) -> Self {
           Self {
               data,
               _state: PhantomData,
           }
       }
       
       fn start(self) -> StateMachine<Processing> {
           StateMachine {
               data: self.data,
               _state: PhantomData,
           }
       }
   }
   ```

**C# Analogy**: Like generic type parameters used only for compile-time checking

## ⚠️ Unsafe Code Issues

### Undefined Behavior Detection

**Common Symptoms:**
- Segmentation faults
- Memory corruption
- Inconsistent behavior across runs

**Debugging Steps:**
1. **Use debugging tools**:
   ```bash
   # Run with address sanitizer
   RUSTFLAGS="-Z sanitizer=address" cargo run
   
   # Run with memory sanitizer
   RUSTFLAGS="-Z sanitizer=memory" cargo run
   
   # Use Miri for UB detection
   cargo +nightly miri run
   
   # Use Valgrind (on Linux)
   cargo build && valgrind ./target/debug/your_binary
   ```

2. **Add debug assertions**:
   ```rust
   unsafe fn get_unchecked(&self, index: usize) -> &T {
       debug_assert!(index < self.len, "Index out of bounds");
       &*self.ptr.add(index)
   }
   ```

3. **Verify safety contracts**:
   ```rust
   // Document safety requirements
   /// # Safety
   /// 
   /// `ptr` must be:
   /// - Non-null
   /// - Properly aligned for type T
   /// - Valid for reads of size_of::<T>() bytes
   unsafe fn read_raw<T>(ptr: *const T) -> T {
       debug_assert!(!ptr.is_null());
       debug_assert!(ptr as usize % std::mem::align_of::<T>() == 0);
       std::ptr::read(ptr)
   }
   ```

**C# Analogy**: Like debugging unsafe pointer operations in C#

### Memory Safety Issues

**Common Symptoms:**
- Use-after-free errors
- Double-free errors
- Buffer overflows

**Debugging Steps:**
1. **Check allocation/deallocation pairs**:
   ```rust
   // ❌ Common mistake
   unsafe {
       let ptr = std::alloc::alloc(layout) as *mut T;
       // ... use ptr ...
       std::alloc::dealloc(ptr as *mut u8, layout);
       std::alloc::dealloc(ptr as *mut u8, layout); // Double free!
   }
   ```

2. **Validate pointer arithmetic**:
   ```rust
   unsafe fn advance_ptr<T>(ptr: *mut T, offset: usize, len: usize) -> *mut T {
       assert!(offset <= len, "Offset beyond buffer");
       ptr.add(offset)
   }
   ```

**C# Analogy**: Like managing unmanaged memory with Marshal.AllocHGlobal

## 📌 Pin and Async Issues

### Pin Safety Violations

**Common Symptoms:**
- "cannot move out of Pin" errors
- Self-referential struct issues
- Async state machine problems

**Debugging Steps:**
1. **Add PhantomPinned**:
   ```rust
   use std::marker::PhantomPinned;
   
   struct SelfReferential {
       data: String,
       self_ref: *const String,
       _phantom: PhantomPinned,  // Required!
   }
   ```

2. **Use Pin correctly**:
   ```rust
   impl SelfReferential {
       fn new(data: String) -> Pin<Box<Self>> {
           let mut boxed = Box::new(SelfReferential {
               data,
               self_ref: std::ptr::null(),
               _phantom: PhantomPinned,
           });
           
           let self_ref = &boxed.data as *const String;
           boxed.self_ref = self_ref;
           
           unsafe { Pin::new_unchecked(boxed) }
       }
   }
   ```

3. **Implement Future correctly**:
   ```rust
   impl Future for MyFuture {
       type Output = String;
       
       fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
           // Use get_unchecked_mut() carefully
           let this = unsafe { self.get_unchecked_mut() };
           // Don't move out of `this`
           Poll::Ready(this.data.clone())
       }
   }
   ```

**C# Analogy**: Like async state machines that can't be moved in memory

## 🚀 Zero-Cost Abstraction Issues

### Performance Regression Detection

**Common Symptoms:**
- Slower than expected performance
- Allocations where none expected
- Function calls not inlined

**Debugging Steps:**
1. **Check assembly output**:
   ```bash
   # Generate assembly
   cargo rustc --release -- --emit asm
   
   # Or use cargo-asm
   cargo install cargo-asm
   cargo asm your_crate::function_name --rust
   ```

2. **Profile with perf**:
   ```bash
   # Build with debug info
   cargo build --release
   
   # Profile with perf (Linux)
   perf record ./target/release/your_binary
   perf report
   
   # Use cargo-flamegraph
   cargo install flamegraph
   cargo flamegraph --bin your_binary
   ```

3. **Check for heap allocations**:
   ```bash
   # Use heaptrack (Linux)
   heaptrack ./target/release/your_binary
   
   # Use dhat (via valgrind)
   valgrind --tool=dhat ./target/release/your_binary
   ```

**C# Analogy**: Like profiling .NET code to find boxing and allocations

## 🧪 Testing Advanced Features

### Macro Testing

```rust
// Test macro expansion
#[cfg(test)]
mod macro_tests {
    #[test]
    fn test_macro_expansion() {
        let t = trybuild::TestCases::new();
        t.pass("tests/01-basic.rs");
        t.compile_fail("tests/02-invalid.rs");
    }
}
```

### Unsafe Code Testing

```rust
#[cfg(test)]
mod unsafe_tests {
    use super::*;
    
    #[test]
    fn test_with_miri() {
        // This test will be run with Miri for UB detection
        let mut buffer = UnsafeBuffer::new(10);
        buffer.push(42).unwrap();
        assert_eq!(buffer.get(0), Some(&42));
    }
}
```

## 💡 C# Developer Tips

1. **Rust errors are more helpful** - Read them carefully, they often contain the solution
2. **Use `cargo expand`** - Like viewing generated C# code from Source Generators
3. **Understand ownership** - Unlike C# references, Rust ownership is explicit
4. **Embrace the type system** - Rust's types prevent entire classes of bugs
5. **Profile early and often** - Zero-cost doesn't mean zero cost in debug builds

## 🔍 Tools and Resources

### Essential Tools
- `cargo expand` - View macro expansions
- `cargo clippy` - Advanced linting
- `cargo miri` - Undefined behavior detection
- `cargo flamegraph` - Performance profiling
- `rust-analyzer` - IDE support

### Debugging Resources
- [Rust Error Index](https://doc.rust-lang.org/error-index.html)
- [Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust guide
- [Async Book](https://rust-lang.github.io/async-book/) - Async programming
- [Miri Documentation](https://github.com/rust-lang/miri) - UB detection

Remember: Advanced Rust features require patience and practice. Each error message is a learning opportunity that makes you a better Rust developer!