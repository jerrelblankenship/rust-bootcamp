# Exercise 08 - Level 2 Hints: Zero-Cost Abstractions

## 🎯 Focus Areas

You should now understand that zero-cost abstractions require eliminating unnecessary overhead. Let's fix the specific performance issues.

## 🔧 Specific Issues to Fix

### Issue 1: Iterator Chain Optimization
```rust
// ❌ Wrong - inefficient intermediate collection
numbers
    .iter()
    .map(|&x| x)  // Unnecessary copy
    .filter(|&x| x > 0)
    .map(|x| x * 2)
    .collect()

// ✅ Correct - direct iterator chain
numbers
    .into_iter()
    .filter(|&x| x > 0)
    .map(|x| x * 2)
    .collect()
```

### Issue 2: String Allocation Issues
```rust
// ❌ Wrong - many temporary allocations
let mut result = String::new();
for msg in messages {
    result = result + &format!("[{}]", msg);
}

// ✅ Correct - pre-allocate and use efficient operations
let mut result = String::with_capacity(estimated_size);
for msg in messages {
    result.push('[');
    result.push_str(&msg);
    result.push(']');
}
```

### Issue 3: Trait Objects vs Generics
```rust
// ❌ Wrong - dynamic dispatch prevents optimization
fn process_with_trait_object(processors: Vec<Box<dyn Processor>>, value: i32) -> Vec<i32>

// ✅ Correct - use generics for static dispatch
fn process_with_generics<P: Processor>(processors: &[P], value: i32) -> Vec<i32>
```

## 🔍 C# Comparison

```csharp
// C# LINQ that might not optimize well
var result = numbers
    .Select(x => x)        // Unnecessary select
    .Where(x => x > 0)
    .Select(x => x * 2)
    .ToList();

// Better C# - more direct
var result = new List<int>();
foreach (var x in numbers)
{
    if (x > 0)
        result.Add(x * 2);
}
```

## 🎮 Implementation Strategy

1. **Eliminate unnecessary operations** - Remove redundant transformations
2. **Use efficient collections** - Pre-allocate when possible
3. **Choose static dispatch** - Use generics instead of trait objects
4. **Optimize string operations** - Use capacity and efficient methods
5. **Profile and verify** - Measure performance improvements

## 🔧 Code Patterns to Apply

### Pattern 1: Efficient Iterator Chain
```rust
fn process_numbers_optimized(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .collect()
}
```

### Pattern 2: Generic Processing
```rust
fn process_with_generics<P: Processor>(processors: &[P], value: i32) -> Vec<i32> {
    processors
        .iter()
        .map(|p| p.process(value))
        .collect()
}
```

### Pattern 3: Efficient String Building
```rust
fn format_messages_optimized(messages: Vec<String>) -> String {
    let capacity = messages.iter().map(|m| m.len() + 2).sum();
    let mut result = String::with_capacity(capacity);
    
    for msg in messages {
        result.push('[');
        result.push_str(&msg);
        result.push(']');
    }
    
    result
}
```

### Pattern 4: HashMap Optimization
```rust
fn count_occurrences_optimized(items: Vec<String>) -> HashMap<String, usize> {
    let mut counts = HashMap::with_capacity(items.len());
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}
```

## ⏰ Time Check

Spent 30 minutes total? If you're still struggling with specific optimization patterns, move to Level 3 for complete solutions.

**Hint**: Focus on the `apply_operations_slow` function - it should use static dispatch and avoid unnecessary closure overhead!