# Advanced Patterns Debugging Checklist 🧙‍♂️

When working with advanced Rust patterns, watch for these complex issues:

## 1. Macro System Issues

### ❌ Macro Hygiene Problems
```rust
macro_rules! bad_macro {
    ($name:ident) => {
        let x = 42; // Captures local variable name!
        let $name = x + 1;
    };
}
```
**Fix:** Use unique names or proper hygiene with `$crate::`

### ❌ Recursive Macro Limits
```rust
macro_rules! infinite_macro {
    () => { infinite_macro!(); }; // Stack overflow!
}
```
**Fix:** Add base cases and use `#![recursion_limit = "256"]`

## 2. Trait Object Issues

### ❌ Object Safety Violations
```rust
trait NotObjectSafe {
    fn generic_method<T>(&self) -> T; // Can't be trait object!
}
```
**Fix:** Use associated types or separate traits

### ❌ Lifetime Issues with Trait Objects
```rust
fn bad_trait_object(s: &str) -> Box<dyn Display> {
    Box::new(s) // Lifetime error!
}
```
**Fix:** Use owned types or lifetime parameters

## 3. Higher-Ranked Trait Bounds (HRTB)

### ❌ Lifetime Confusion
```rust
fn apply<F>(f: F) 
where 
    F: Fn(&str) -> String, // Too restrictive!
{
    // ...
}
```
**Fix:** Use HRTB: `F: for<'a> Fn(&'a str) -> String`

## 4. Phantom Types

### ❌ Unused Type Parameters
```rust
struct Container<T> {
    data: Vec<i32>, // T is unused - compile error!
}
```
**Fix:** Use `PhantomData<T>` to mark phantom types

## 5. Zero-Sized Types (ZST)

### ❌ Surprising Memory Layout
```rust
struct Empty; // ZST
let vec = vec![Empty; 1000]; // Still allocates!
```
**Fix:** Understand ZST behavior for collections

## 6. Unsafe Code Issues

### ❌ Memory Safety Violations
```rust
unsafe fn dangerous() {
    let mut x = 42;
    let ptr = &mut x as *mut i32;
    let slice = std::slice::from_raw_parts_mut(ptr, 1000); // Out of bounds!
}
```
**Fix:** Ensure all unsafe invariants are maintained

### ❌ Aliasing Mutable References
```rust
unsafe fn aliasing_violation() {
    let mut x = 42;
    let ptr1 = &mut x as *mut i32;
    let ptr2 = &mut x as *mut i32; // Undefined behavior!
    *ptr1 = 10;
    *ptr2 = 20;
}
```
**Fix:** Never create aliasing mutable pointers

## 7. Pin and Unpin

### ❌ Moving Pinned Data
```rust
let mut future = async { /* self-referential */ };
let pinned = Pin::new(&mut future);
let moved = pinned.get_mut(); // Might break self-references!
```
**Fix:** Use `Pin::as_mut()` and understand pin guarantees

## C# to Rust Advanced Patterns

| C# Concept | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| Generics with constraints | Trait bounds | More flexible |
| Expression trees | Macros | Compile-time code gen |
| Reflection | No runtime reflection | Compile-time only |
| `dynamic` | Trait objects | Type-safe |
| LINQ | Iterator adapters | Zero-cost |
| Attributes | Derive macros | More powerful |

## Complex Pattern Checklist

- [ ] Are macro expansions correct? Use `cargo expand`
- [ ] Are trait objects object-safe?
- [ ] Do lifetimes work with HRTB?
- [ ] Are unsafe invariants maintained?
- [ ] Is Pin used correctly for self-referential structs?
- [ ] Are phantom types properly marked?

## Debugging Advanced Code

1. **Use `cargo expand`** - See macro expansions
2. **Enable `-Z macro-backtrace`** - Track macro errors
3. **Use MIRI** - Detect undefined behavior
4. **Read the nomicon** - Understand unsafe patterns
5. **Test with sanitizers** - Catch memory errors

## Pro Tips

- Start simple before adding advanced patterns
- Prefer safe abstractions over unsafe when possible
- Use `cargo +nightly rustc -- -Z help` for unstable features
- The Rust Reference documents all advanced features
- Join #black-magic in Rust Discord for pattern help
- Consider if the complexity is worth it - simple is often better