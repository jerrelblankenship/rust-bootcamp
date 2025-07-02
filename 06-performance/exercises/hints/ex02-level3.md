# Exercise 2 Hints - Level 3 (Near-Complete Solutions)

## 🎯 Complete solutions for String Builder performance optimization

### Checkpoint 1: String Concatenation Solution

**Replace this**:
```rust
let mut result = String::new();
for i in 0..5_000 {
    result = result + &format!("Item {} | ", i);  // ❌ O(n²) allocations
}
```

**With this**:
```rust
// Option 1: Pre-allocate and use push_str (best)
let mut result = String::with_capacity(5_000 * 15); // Estimate ~15 chars per item
for i in 0..5_000 {
    result.push_str("Item ");
    result.push_str(&i.to_string());
    result.push_str(" | ");
}

// Option 2: Use format! with push_str (good)
// let mut result = String::with_capacity(5_000 * 15);
// for i in 0..5_000 {
//     result.push_str(&format!("Item {} | ", i));
// }
```

### Checkpoint 2: Reduce Format! Usage Solution

**Replace this**:
```rust
let mut result = String::new();
for &num in &numbers {
    for _ in 0..1_000 {
        let formatted = format!("Number: {}", num);  // ❌ Expensive format!
        result.push_str(&formatted);
        result.push_str(" | ");
        let more_format = format!(" [{}] ", num);    // ❌ Another expensive format!
        result.push_str(&more_format);
    }
}
```

**With this**:
```rust
let estimated_size = numbers.len() * 1_000 * 20; // Estimate total size
let mut result = String::with_capacity(estimated_size);
for &num in &numbers {
    for _ in 0..1_000 {
        // ✅ Direct string building (fastest)
        result.push_str("Number: ");
        result.push_str(&num.to_string());
        result.push_str(" | [");
        result.push_str(&num.to_string());
        result.push_str("] ");
        
        // Alternative: Single format! call
        // result.push_str(&format!("Number: {} | [{}] ", num, num));
    }
}
```

### Checkpoint 3: Eliminate Processing Clones Solution

**Replace this**:
```rust
let mut processed: Vec<String> = Vec::new();
for item in &input_data {
    let cloned = item.clone();                    // ❌ Unnecessary clone
    let processed = cloned.to_uppercase();       // ❌ Could work on reference
    let trimmed = processed.trim().to_string();  // ❌ Another allocation
    let final_result = format!("FINAL_{}", trimmed);  // ❌ More allocation
    processed.push(final_result);
}
```

**With this**:
```rust
let mut results = Vec::with_capacity(input_data.len());
for item in &input_data {
    // ✅ Single allocation - work with references until final result
    let final_result = format!("FINAL_{}", item.trim().to_uppercase());
    results.push(final_result);
}

// Or even better - use iterator:
let results: Vec<String> = input_data
    .iter()
    .map(|item| format!("FINAL_{}", item.trim().to_uppercase()))
    .collect();
```

### Checkpoint 4: Multiple Replace Operations Solution

**Replace this**:
```rust
let mut result = text.clone();  // ❌ Unnecessary clone
for _ in 0..100 {
    result = result.replace("the", "THE");  // ❌ New string allocation
    result = result.replace("fox", "FOX");  // ❌ Another allocation
    result = result.replace("dog", "DOG");  // ❌ Yet another allocation
}
```

**With this**:
```rust
// Option 1: Single-pass replacement function
fn replace_multiple(text: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = String::with_capacity(text.len() * 110 / 100); // 10% larger estimate
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        let mut found_replacement = false;
        
        for &(from, to) in replacements {
            if text[chars.as_str().len()..].starts_with(from) {
                result.push_str(to);
                // Skip the matched characters
                for _ in 1..from.len() {
                    chars.next();
                }
                found_replacement = true;
                break;
            }
        }
        
        if !found_replacement {
            result.push(ch);
        }
    }
    result
}

// Use it:
let mut result = text; // No clone needed
for _ in 0..100 {
    result = replace_multiple(&result, &[("the", "THE"), ("fox", "FOX"), ("dog", "DOG")]);
}

// Option 2: Simple chained replace (less optimal but easier)
let mut result = text;
for _ in 0..100 {
    result = result
        .replace("the", "THE")
        .replace("fox", "FOX") 
        .replace("dog", "DOG");
}
```

### Checkpoint 5: String Capacity Solution

**Replace this**:
```rust
let mut large_string = String::new();  // ❌ No initial capacity
for i in 0..20_000 {
    large_string.push_str("This is a reasonably long line of text that will cause reallocations ");
    large_string.push_str(&format!("Line number: {} ", i));  // ❌ Could optimize format too
    large_string.push('\n');
}
```

**With this**:
```rust
// ✅ Calculate estimated size and pre-allocate
let base_line = "This is a reasonably long line of text that will cause reallocations ";
let avg_number_chars = 8; // "Line number: 12345 " ≈ 8 chars for number + text
let line_size = base_line.len() + "Line number:  \n".len() + avg_number_chars;
let estimated_size = line_size * 20_000;

let mut large_string = String::with_capacity(estimated_size);

for i in 0..20_000 {
    large_string.push_str(base_line);
    large_string.push_str("Line number: ");
    large_string.push_str(&i.to_string());  // ✅ Avoid format! for simple case
    large_string.push_str(" \n");
}
```

## 🎯 Performance Expectations

After all fixes, you should see:
- **Checkpoint 1**: ~100ms → ~8ms (12x improvement)
- **Checkpoint 2**: ~50ms → ~15ms (3x improvement)  
- **Checkpoint 3**: ~100ms → ~25ms (4x improvement)
- **Checkpoint 4**: ~200ms → ~35ms (6x improvement)
- **Checkpoint 5**: ~100ms → ~20ms (5x improvement)

## 🧠 Key Principles Applied

### String Performance Optimization:
1. **Pre-allocation**: Use `String::with_capacity()` when you know approximate size
2. **Avoid `+` operator**: Use `push_str()` instead of string concatenation
3. **Minimize `format!`**: Use direct string building for simple cases
4. **Single allocation**: Work with references until final result needed
5. **Batch operations**: Single-pass algorithms when possible

### Capacity Estimation Techniques:
```rust
// Method 1: Sample calculation
let sample_output = format!("Item {} | ", 12345);
let estimated_total = sample_output.len() * iteration_count;

// Method 2: Component-based estimation  
let fixed_part = "Item  | ".len();
let variable_part = 6; // max digits in number
let estimated_per_item = fixed_part + variable_part;
let total_capacity = estimated_per_item * count;
```

### C# Equivalents:
- `String::with_capacity()` ↔ `new StringBuilder(capacity)`
- `push_str()` ↔ `StringBuilder.Append()`
- Avoiding string `+` ↔ Avoiding `string +=` in loops
- Single-pass replacement ↔ Custom string building instead of multiple `.Replace()`

You've now mastered string performance optimization! 🚀