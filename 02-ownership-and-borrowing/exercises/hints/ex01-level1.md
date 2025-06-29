# Exercise 1 Hints - Level 1 (Ownership Basics)

## 🔍 Getting Started with Ownership

You're working with `ex01-ownership.rs` and seeing ownership-related compilation errors. This is exactly what we want! Let's work through them systematically.

## 💡 Core Ownership Concepts to Remember

### The Three Rules of Ownership:
1. **Each value has exactly one owner**
2. **Only one owner at a time**  
3. **When the owner goes out of scope, the value is dropped**

### Move vs Copy:
- **Copy types** (integers, booleans, chars): Assignment copies the value
- **Move types** (String, Vec, custom structs): Assignment transfers ownership

## 🎯 Gentle Hints for Common Errors

### Hint 1: "value borrowed after move"

**C# Context**: In C#, this would work fine because multiple variables can reference the same object.

**Rust Difference**: When you assign a `String` or `Vec` to another variable, ownership moves.

**Gentle Guidance**: 
- Look at the line where the error says the move occurred
- The value is no longer available to the original variable
- Think: "Do I need a copy or just want to use the value?"

**Questions to ask yourself:**
- Can I use the moved variable instead of the original?
- Do I need both variables to have their own copy of the data?
- Am I just trying to read the data without changing it?

### Hint 2: Copy vs Clone

**C# Context**: Objects are reference types, value types are copied automatically.

**Rust Difference**: Most types require explicit cloning if you want a copy.

**Gentle Guidance**:
- Simple types (i32, bool, char) are copied automatically
- Complex types (String, Vec) need explicit `.clone()` calls
- Think: "Is this expensive data to copy?"

### Hint 3: Function Ownership

**C# Context**: Passing objects to methods doesn't usually affect the original variable.

**Rust Difference**: Passing move types to functions transfers ownership to the function.

**Gentle Guidance**:
- After calling a function with a String/Vec, that variable is no longer usable
- Think: "Should this function own the data or just borrow it?"
- Consider if the function needs to keep the data or just read it

## 🔧 General Debugging Approach

### Step 1: Read the Error Location
- Find the line where ownership was transferred (moved)
- Find the line where you're trying to use the moved value
- Understand the flow: created → moved → attempted use

### Step 2: Choose Your Strategy
- **Use after move**: Use the new owner instead of the old variable
- **Need both**: Use `.clone()` to create an independent copy
- **Just reading**: Consider borrowing (we'll learn this in the next exercise)

### Step 3: Test Your Understanding
After fixing each error:
- Can you explain why the original code didn't work?
- Do you understand where ownership was transferred?
- Does your solution make logical sense?

## 🎓 Learning Questions

Before moving to Level 2 hints, try to answer:

1. **What is ownership transfer?** Why does Rust have this concept?
2. **When does ownership move?** What operations transfer ownership?
3. **Why can't I use a moved value?** What could go wrong if Rust allowed it?
4. **What's the difference between move and copy?** Why do some types copy and others move?

## 💭 Think About It

**In C#**: `var list2 = list1;` creates two variables referencing the same list.
**In Rust**: `let list2 = list1;` transfers ownership - only `list2` is valid afterward.

This prevents bugs where modifying one variable unexpectedly affects another!

## ➡️ Next Level

Still struggling? Try [Level 2 Hints](ex01-level2.md) for more specific guidance.

Remember: This is a fundamental shift from garbage collection to explicit ownership. The mental model takes time to build! 🦀
