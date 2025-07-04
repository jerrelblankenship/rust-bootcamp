# Exercise 01 - Level 2 Hints: Macro Madness

## 🎯 Specific Issues to Fix

Based on the compilation errors, here are the specific problems:

## 🔧 Checkpoint 1: Basic Macro Syntax

**Problem**: The `make_greeter!` macro is correct, but check the generated code.

**Solution**: The macro itself is fine, but you need to ensure it generates valid Rust code.

## 🔧 Checkpoint 2: Pattern Matching Issues

**Problem**: The `shout` pattern in `greet!` macro is missing braces:
```rust
(shout $name:expr) => 
    println!("HEY {}!!!", $name.to_uppercase())
;
```

**Solution**: Add braces around the expansion:
```rust
(shout $name:expr) => {
    println!("HEY {}!!!", $name.to_uppercase())
};
```

## 🔧 Checkpoint 3: Repetition Syntax

**Problem**: The `declare_vars!` macro is correct.

**Solution**: This should work as-is, but verify the usage in `main()`.

## 🔧 Checkpoint 4: Hygiene Problems

**Problem**: The `create_counter!` macro tries to modify a captured variable:
```rust
let count = 0;
move || {
    count += 1;  // ❌ Can't modify captured variable
    count
}
```

**Solution**: Use `RefCell` or `Cell` for interior mutability:
```rust
macro_rules! create_counter {
    () => {
        {
            use std::cell::Cell;
            let count = Cell::new(0);
            move || {
                let current = count.get();
                count.set(current + 1);
                current + 1
            }
        }
    };
}
```

## 🔧 Checkpoint 5: Fragment Specifier Errors

**Problem**: The second `debug_print!` pattern uses `$block:expr` but should use `$block:block`:
```rust
($block:block) => {
    println!("Executing block...");
    $block
};
```

## 🔧 Checkpoint 6: Macro Recursion

**Problem**: The recursion base case and recursive case have syntax issues.

**Solution**: Fix the recursive call:
```rust
macro_rules! nest_calls {
    ($func:ident, 1) => {
        $func()
    };
    ($func:ident, $n:expr) => {
        $func();
        nest_calls!($func, $n - 1)
    };
}
```

## 🔧 Checkpoint 7: Complex Pattern Matching

**Problem**: The typed `hashmap!` pattern has incorrect syntax.

**Solution**: Use separate patterns or handle type annotation differently.

## 🔧 Checkpoint 8: Struct Creation

**Problem**: The `create_struct!` macro is mostly correct but check the `impl` block.

## ⏰ Time Check

Still struggling after 30 minutes? Move to Level 3 for complete solutions.

**Hint**: Most issues are about missing braces `{}` and incorrect fragment specifiers!