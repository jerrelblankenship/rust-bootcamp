# Exercise 2 Hints - Level 1 (Gentle Nudges)

## 🔍 Getting Started with Borrowing

You're working with `ex02-borrowing.rs` and seeing borrowing-related compilation errors. This is the natural progression from ownership! Let's work through borrowing systematically.

## 💡 Core Borrowing Concepts to Remember

### The Golden Rule of Borrowing:
**You can have either ONE mutable reference OR multiple immutable references, but not both at the same time.**

### Two Types of References:
1. **Immutable references (`&T`)**: Multiple allowed, read-only access
2. **Mutable references (`&mut T`)**: Only one allowed, read-write access

### C# vs Rust Reference Model:
- **C#**: References are the default, copying creates new references to same object
- **Rust**: Ownership is default, references must be explicit and follow strict rules

## 🎯 Gentle Hints for Common Borrowing Errors

### Hint 1: "Move vs Borrow" Confusion

**C# Context**: In C#, assignment usually creates another reference to the same object.

**Rust Difference**: Assignment moves ownership unless you explicitly borrow with `&`.

**Gentle Guidance**:
- Look for assignments like `let reference = data;`
- Ask: "Do I want to transfer ownership or just access the data?"
- If just accessing: use `let reference = &data;`
- The `&` symbol means "borrow, don't move"

**Questions to ask yourself:**
- Am I trying to use both the original and the "reference" afterward?
- Do I just need to read the data without owning it?
- Should I be using `&` to create a reference?

### Hint 2: "Cannot borrow as mutable"

**C# Context**: Objects can usually be modified through any reference.

**Rust Difference**: You need explicit `mut` in multiple places for mutation.

**Gentle Guidance**:
- Look for two different `mut` keywords needed:
  1. `let mut data` - makes the variable mutable
  2. `&mut data` - creates a mutable reference
- Both are required for mutation through references
- Think: "Am I trying to modify data through a reference?"

### Hint 3: "Cannot have multiple mutable references"

**C# Context**: Multiple variables can refer to the same object and all can modify it.

**Rust Difference**: Only ONE mutable reference allowed at a time.

**Gentle Guidance**:
- Rust prevents data races at compile time
- Look for overlapping mutable borrows
- Consider: "Do these references exist at the same time?"
- Solution: Use non-overlapping scopes or different timing

### Hint 4: "Reference doesn't live long enough"

**C# Context**: GC keeps objects alive as long as any reference exists.

**Rust Difference**: References can't outlive the data they point to.

**Gentle Guidance**:
- Find where the referenced data goes out of scope
- Find where you're trying to use the reference
- Ask: "Does the original data live longer than my reference?"
- Solution: Move data to outer scope or use owned data instead

## 🔧 General Debugging Approach

### Step 1: Identify the Borrowing Pattern
```rust
// Moving (ownership transfer)
let x = data;

// Borrowing immutably (shared access)
let x = &data;

// Borrowing mutably (exclusive access)  
let x = &mut data;
```

### Step 2: Check Reference Timing
- When is each reference created?
- When is each reference last used?
- Do any overlap when they shouldn't?

### Step 3: Verify Mutability
- Is the original data declared `mut`?
- Are you creating `&mut` references correctly?
- Do you have multiple mutable references at once?

### Step 4: Choose Your Strategy
- **Use immutable references**: When you only need to read
- **Use one mutable reference**: When you need to modify
- **Use scopes**: To end references before creating new ones
- **Clone if necessary**: When borrowing rules are too restrictive

## 🎓 Learning Questions

Before moving to Level 2 hints, try to answer:

1. **What's the difference between `&T` and `&mut T`?**
2. **Why can't you have multiple mutable references?**
3. **When do references "end" and stop being active?**
4. **How do you modify data through a reference?**

## 💭 Think About It

**In C#**: Multiple references to same object are normal and all can modify it.
```csharp
var list = new List<int>();
var ref1 = list;
var ref2 = list;
ref1.Add(1);  // Both ref1 and ref2 see the change
ref2.Add(2);  // This is fine in C#
```

**In Rust**: Borrowing rules prevent data races at compile time.
```rust
let mut list = vec![];
let ref1 = &mut list;
let ref2 = &mut list;  // ERROR: Can't have two mutable references!
```

This prevents bugs where one part of code modifies data while another part is reading it!

## 🔄 From Ownership to Borrowing

You've learned:
- **Ownership**: Who is responsible for cleaning up data
- **Borrowing**: How to access data without taking responsibility

Borrowing is like "checking out a library book" - you can read it (immutable borrow) or write in it (mutable borrow), but you have to return it!

## ➡️ Next Level

Still struggling with specific borrowing patterns? Try [Level 2 Hints](ex02-level2.md) for more concrete guidance.

Remember: Borrowing rules feel restrictive at first, but they prevent entire classes of bugs that are common in other languages! 🦀