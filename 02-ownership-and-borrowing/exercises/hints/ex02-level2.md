# Exercise 2 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Borrowing Issues

You've tried Level 1 hints but need more concrete guidance. Here are specific solutions for each borrowing challenge in Exercise 2.

## 🔧 Exercise 2.1: Basic Immutable Borrowing

**Problem**: `let reference = data;` moves instead of borrowing.

**Specific Solution**:
```rust
fn exercise_2_1() {
    println!("Exercise 2.1: Basic immutable borrowing");
    
    let data = String::from("borrowed data");
    
    // SOLUTION: Use & to create immutable reference
    let reference = &data;  // Borrow, don't move
    
    // Both work now
    println!("Original: {}", data);
    println!("Reference: {}", reference);
    
    let length = reference.len();
    println!("Length through reference: {}", length);
    
    // ANSWER: Moving transfers ownership, borrowing shares access
    
    println!("✅ Exercise 2.1 complete\n");
}
```

**Key Learning**: `&` creates a reference that borrows without taking ownership.

## 🔧 Exercise 2.2: Mutable Borrowing

**Problem**: Need both `mut` on variable and `&mut` for mutable reference.

**Specific Solution**:
```rust
fn exercise_2_2() {
    println!("Exercise 2.2: Mutable borrowing");
    
    let mut data = String::from("mutable data");  // SOLUTION: Add mut here
    
    // SOLUTION: Create mutable reference with &mut
    let mut_ref = &mut data;
    
    // Now modification works
    mut_ref.push_str(" - modified");
    
    println!("Modified data: {}", data);
    
    // ANSWERS:
    // 1. `let mut data` makes the variable itself mutable
    // 2. `&mut data` creates a mutable reference to mutable data
    
    println!("✅ Exercise 2.2 complete\n");
}
```

**Key Learning**: Need `mut` in two places - on the variable and in the reference type.

## 🔧 Exercise 2.3: Borrowing Rules Conflicts

**Problem**: Immutable and mutable references can't coexist.

**Specific Solution**:
```rust
fn exercise_2_3() {
    println!("Exercise 2.3: Borrowing rules");
    
    let mut data = String::from("rules demo");
    
    // Try 1: Multiple immutable references (this works)
    let r1 = &data;
    let r2 = &data;
    println!("Multiple immutable refs: {} and {}", r1, r2);
    
    // SOLUTION: End immutable references before creating mutable ones
    // r1 and r2 are no longer used after the println above
    
    // Try 2: Now we can create mutable references
    let mut_ref1 = &mut data;
    mut_ref1.push_str(" modified");
    
    // SOLUTION: Use mut_ref1, then create mut_ref2 in separate scope
    println!("Modified: {}", mut_ref1);
    
    // Alternative: Use separate scopes
    {
        let mut_ref2 = &mut data;
        mut_ref2.push_str(" again");
        println!("Modified again: {}", mut_ref2);
    } // mut_ref2 ends here
    
    println!("✅ Exercise 2.3 complete\n");
}
```

**Key Learning**: References have scopes - they "end" when last used or scope closes.

## 🔧 Exercise 2.4: Reference Lifetime Issues

**Problem**: Reference tries to outlive the data it points to.

**Specific Solution**:
```rust
fn exercise_2_4() {
    println!("Exercise 2.4: Reference scope and lifetime");
    
    // SOLUTION: Move data to outer scope so it lives longer
    let temp_data = String::from("temporary");
    let reference = &temp_data;  // Now temp_data lives long enough
    
    println!("Inside scope: {}", reference);
    // Both temp_data and reference are in same scope
    
    println!("Outside scope: {}", reference);  // Works now!
    
    // Alternative solution: Don't store the reference
    {
        let inner_data = String::from("inner");
        println!("Use immediately: {}", &inner_data);  // Use without storing
    }
    
    println!("✅ Exercise 2.4 complete\n");
}
```

**Key Learning**: References can't outlive the data they reference.

## 🔧 Exercise 2.5: Functions with Borrowing

**Problem**: Function takes ownership when it should borrow.

**Specific Solution**:
```rust
fn exercise_2_5() {
    println!("Exercise 2.5: Functions with borrowing");
    
    let mut text = String::from("hello");
    
    // SOLUTION: Use corrected function that borrows
    let length = calculate_length_correctly(&text);
    println!("Length: {}", length);
    println!("Original text: {}", text);  // Works because we borrowed
    
    println!("✅ Exercise 2.5 complete\n");
}

// SOLUTION: Function that borrows instead of taking ownership
fn calculate_length_correctly(s: &String) -> usize {
    s.len()  // Can access through reference
}
```

**Key Learning**: Functions should borrow when they don't need to own the data.

## 🔧 Exercise 2.6: Modifying Through Mutable References

**Problem**: Function needs to modify data without taking ownership.

**Specific Solution**:
```rust
fn exercise_2_6() {
    println!("Exercise 2.6: Modifying through mutable references");
    
    let mut message = String::from("Hello");
    
    // SOLUTION: Use function that takes mutable reference
    append_exclamation_correctly(&mut message);
    println!("After function: {}", message);  // Works - we still own message
    
    println!("✅ Exercise 2.6 complete\n");
}

// SOLUTION: Function that takes mutable reference
fn append_exclamation_correctly(s: &mut String) {
    s.push_str("!");  // Modify through mutable reference
}
```

**Key Learning**: Use `&mut T` parameters when function needs to modify but not own data.

## 🔧 Exercise 2.7: Slices and Borrowing

**Problem**: Taking entire vector instead of creating slice.

**Specific Solution**:
```rust
fn exercise_2_7() {
    println!("Exercise 2.7: Slices and borrowing");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // SOLUTION: Create slice instead of moving entire vector
    let first_three = &numbers[0..3];  // Slice syntax
    
    println!("First three: {:?}", first_three);
    println!("Original numbers: {:?}", numbers);  // Still works!
    
    // Alternative slice syntaxes:
    let first_two = &numbers[0..2];      // First 2 elements
    let last_two = &numbers[3..5];       // Last 2 elements  
    let middle = &numbers[2..3];         // Middle element
    let all = &numbers[..];              // Entire slice
    
    println!("First 2: {:?}", first_two);
    println!("Last 2: {:?}", last_two);
    println!("Middle: {:?}", middle);
    
    println!("✅ Exercise 2.7 complete\n");
}
```

**Key Learning**: Slices borrow parts of collections without taking ownership.

## 🔧 Exercise 2.8: String Slices

**Problem**: Functions take ownership of entire String when they only need to read part.

**Specific Solution**:
```rust
fn exercise_2_8() {
    println!("Exercise 2.8: String slices");
    
    let sentence = String::from("The quick brown fox");
    
    // SOLUTION: Use functions that take string slices
    let first_word = get_first_word_correctly(&sentence);
    println!("First word: {}", first_word);
    
    let second_word = get_second_word_correctly(&sentence);
    println!("Second word: {}", second_word);
    
    // sentence is still usable because we borrowed, didn't move
    println!("Original sentence: {}", sentence);
    
    println!("✅ Exercise 2.8 complete\n");
}

// SOLUTION: Functions that return string slices
fn get_first_word_correctly(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[0]  // Return slice, not owned String
}

fn get_second_word_correctly(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[1]  // Return slice, not owned String
}
```

**Key Learning**: `&str` is more flexible than `String` - works with owned strings and literals.

## 🔧 Advanced Borrowing Patterns

### Working with Multiple References
```rust
fn process_data_safely() {
    let mut data = vec![1, 2, 3];
    
    // This pattern works - separate borrows in time
    {
        let read_ref = &data;
        println!("Reading: {:?}", read_ref);
    } // read_ref ends here
    
    {
        let write_ref = &mut data;
        write_ref.push(4);
        println!("After writing: {:?}", write_ref);
    } // write_ref ends here
}
```

### Interior Mutability Pattern (Advanced)
```rust
use std::cell::RefCell;

fn interior_mutability_example() {
    // RefCell allows runtime borrowing rules
    let data = RefCell::new(vec![1, 2, 3]);
    
    // Multiple immutable borrows OK
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("Both: {:?} {:?}", borrow1, borrow2);
    }
    
    // Mutable borrow when no other borrows exist
    {
        let mut mut_borrow = data.borrow_mut();
        mut_borrow.push(4);
    }
}
```

## 💡 Key Learning Points

### Reference Types and Usage
```rust
// Immutable reference - for reading
fn read_data(data: &Vec<i32>) -> usize {
    data.len()  // Can read through reference
}

// Mutable reference - for modifying
fn modify_data(data: &mut Vec<i32>) {
    data.push(42);  // Can modify through mutable reference
}

// String slices - most flexible for string data
fn process_text(text: &str) -> usize {
    text.len()  // Works with String, &String, and string literals
}
```

### Borrowing Rules Summary
1. **Multiple immutable references**: ✅ Allowed
2. **One mutable reference**: ✅ Allowed (but no others)
3. **Mix immutable + mutable**: ❌ Not allowed
4. **References can't outlive data**: ❌ Lifetime violation

### Common Borrowing Patterns
```rust
// Reading data without ownership
let length = calculate_length(&my_string);

// Modifying data without ownership  
append_text(&mut my_string);

// Working with slices
let part = &my_vec[1..4];
let word = &my_string[0..5];

// Ending borrows early with scopes
{
    let temp_ref = &mut data;
    temp_ref.modify();
} // temp_ref ends here, can create new borrows
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex02-level3.md) for full solutions.

## 🎓 Understanding Check

You should now understand:
1. **When to use `&` vs `&mut`**: Reading vs modifying access
2. **Why borrowing rules exist**: Prevent data races and use-after-free
3. **How reference scopes work**: When references start and end
4. **String slices advantages**: More flexible than owned strings

Ready for lifetimes in Exercise 3! 🦀