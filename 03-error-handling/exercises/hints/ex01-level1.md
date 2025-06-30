# Exercise 1 Hints - Level 1 (Gentle Guidance)

## 🔍 Understanding Option<T> for C# Developers

You're working on Option<T> basics, which replaces nullable types from C#. This is about making "missing values" explicit and safe at compile time.

## 💡 Core Concept: No More Null Reference Exceptions

### What is Option<T>?
Option<T> is Rust's way of representing values that might be present (`Some(value)`) or absent (`None`). It's like C#'s nullable types, but the compiler forces you to handle both cases.

**C# Comparison:**
```csharp
// C# - Can cause NullReferenceException
string name = GetUserName(); // Might be null!
Console.WriteLine(name.Length); // 💥 Crash if null!

// C# with nullable reference types
string? name = GetUserName(); // Explicit nullable
if (name != null) {
    Console.WriteLine(name.Length); // ✅ Safe
}
```

**Rust Equivalent:**
```rust
// Rust - Compiler forces handling
let name: Option<String> = get_user_name(); // Explicit Option
match name {
    Some(n) => println!("{}", n.len()), // ✅ Safe - we know it exists
    None => println!("No name found"),  // ✅ Safe - explicitly handled
}
```

## 🎯 Exercise 1.1 Gentle Hints

### Pattern Goal: Basic Option Handling
**What you're trying to achieve:** Return a user's name if found, or "Unknown" if not found.

**Key Insight:** The `find_user()` function returns `Option<User>`, not `User`. You need to handle both the `Some(user)` and `None` cases.

**Gentle Questions to Ask Yourself:**
- What does `find_user(user_id)` return? (Hint: Look at the function signature)
- If the user is found, how do you get their name from `Some(user)`?
- If the user is not found (`None`), what should you return?
- How can you handle both cases in one expression?

**Pattern Approach:**
```rust
// Think about this pattern:
match optional_value {
    Some(value) => /* do something with value */,
    None => /* provide a default */,
}
```

## 🎯 Exercise 1.2 Gentle Hints

### Pattern Goal: Option Chaining
**What you're trying to achieve:** Chain operations on optional values without nested if-statements.

**Key Insight:** You want to go from `user` → `email` → `domain`, but any step might fail.

**Gentle Questions:**
- How do you get the email from `Option<User>`?
- How do you extract the domain from an email string?
- What if the user doesn't exist?
- What if the email doesn't have an '@' symbol?

**Method Hints:**
- `.map()` - Transform the value inside Some, leave None unchanged
- `.and_then()` - Chain operations that return Option
- `.split_once('@')` - Split string at first occurrence

## 🎯 Exercise 1.3 Gentle Hints

### Pattern Goal: Working with Collections of Options
**What you're trying to achieve:** Calculate average of only the valid scores, ignoring None values.

**Key Insight:** You have `Vec<Option<i32>>` and want to work only with the `Some(score)` values.

**Gentle Questions:**
- How do you filter out the None values?
- How do you extract the actual numbers from Some(number)?
- What should you return if there are no valid scores?
- How do you calculate an average?

**Iterator Method Hints:**
- `.filter_map()` - Filter and transform in one step
- `.collect::<Vec<_>>()` - Collect iterator into Vec
- `.len()` and `.sum()` - For calculating average

## 🔄 General Learning Approach

### Mental Model Shift
**C# Thinking:** "I hope this value isn't null"
**Rust Thinking:** "The compiler ensures I handle both cases"

### Common Patterns
1. **Basic Handling:** Use `match` to handle both Some and None
2. **Default Values:** Use `.unwrap_or()` for simple defaults
3. **Chaining:** Use `.map()` and `.and_then()` for transformations
4. **Collection Processing:** Use `.filter_map()` to work with valid values

### When You See Compilation Errors
- "expected `String`, found `Option<String>`" → You need to extract the value
- "cannot call method on Option" → You need to handle the Option first
- "mismatched types" → You might be mixing Some/None handling

## 💭 Think About It

**Why is Option<T> better than null?**
- **Explicit**: Function signatures tell you when values might be missing
- **Safe**: Compiler prevents null reference exceptions at compile time
- **Composable**: Chain operations safely with .map() and .and_then()
- **Performance**: No runtime null checks needed

## ➡️ Next Level

Still struggling with specific implementations? Try [Level 2 Hints](ex01-level2.md) for concrete code solutions.

Remember: Option<T> makes "maybe missing" explicit and safe. The compiler is your friend! 🦀