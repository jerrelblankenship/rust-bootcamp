# Module 01 Debugging Checklist

## 🔍 Systematic Debugging Approach

When you encounter compilation errors in Module 01, follow this checklist:

### **Step 1: Read the Complete Error Message**
```bash
# Always run this to see all errors:
rustc filename.rs
# Don't skip reading the full message - Rust gives excellent guidance!
```

**Checklist:**
- [ ] Read the error message completely (don't skim)
- [ ] Note the line number and column position
- [ ] Look for error code (e.g., E0412) for detailed explanations
- [ ] Check if there are multiple errors - fix one at a time

### **Step 2: Identify the Rust Concept**
Common Module 01 concepts and their error patterns:

**Macro Syntax Errors:**
- [ ] Missing `!` in `println` → Add `println!`
- [ ] Wrong format specifiers → Use `{}`, `{:?}`, or `{:#?}`
- [ ] Wrong number of arguments → Match placeholders with variables

**Variable Declaration Errors:**
- [ ] Undefined variable → Add `let variable_name = value;`
- [ ] Wrong mutability → Use `let mut` if changing the variable
- [ ] Type annotation needed → Add `: type` after variable name

**Function Definition Errors:**
- [ ] Missing parameter types → Add `name: Type` for each parameter
- [ ] Missing return type → Add `-> ReturnType` before `{`
- [ ] Using `todo!()` → Replace with actual implementation

### **Step 3: Apply C# Knowledge**
Think about how you'd solve this in C#, then translate:

**C# → Rust Translation Guide:**
```csharp
// C#
Console.WriteLine($"Hello {name}");
string Greet(string name, int age) { return $"Hello {name}"; }
var numbers = new[] { 1, 2, 3 };
```

```rust
// Rust
println!("Hello {}", name);
fn greet(name: &str, age: i32) -> String { format!("Hello {}", name) }
let numbers = vec![1, 2, 3];
```

### **Step 4: Fix Incrementally**
- [ ] Fix only ONE error per compilation cycle
- [ ] Compile after each fix: `rustc filename.rs`
- [ ] Celebrate small wins - each successful compilation is progress!
- [ ] Don't try to fix multiple errors simultaneously

### **Step 5: Use Available Resources**
**When stuck for more than 15 minutes:**
- [ ] Check the hints: `hints/ex01-level1.md`
- [ ] Read the TODO/FIXME comments carefully
- [ ] Review the relevant lesson material
- [ ] Ask for help rather than getting frustrated

## 🚨 Common Error Patterns

### **"cannot find macro 'println' in this scope"**
**Problem:** Missing `!` in macro call
**Solution:** Change `println(...)` to `println!(...)`

### **"cannot find value 'variable_name' in this scope"**
**Problem:** Variable not declared
**Solution:** Add `let variable_name = value;` before use

### **"mismatched types"**
**Problem:** Type mismatch, often in format strings
**Solution:** Check format specifiers and variable types

### **"cannot format ... with the given format arguments"**
**Problem:** Wrong format specifier for data type
**Solution:** Use `{}` for simple types, `{:?}` for debug, `{:#?}` for pretty

### **"expected fn item, found ()"**
**Problem:** Function implementation missing or incomplete
**Solution:** Replace `todo!()` with actual code, add return statements

## 🎯 Success Indicators

**You're debugging effectively when:**
- [ ] Error messages guide your next action
- [ ] Each compilation shows fewer errors
- [ ] You understand WHY each fix works
- [ ] You can predict what the next error might be
- [ ] You feel progress rather than frustration

## 🔧 Tools for Effective Debugging

```bash
# Quick syntax check (faster than full compile)
cargo check

# Show all errors at once
rustc filename.rs 2>&1 | less

# Get detailed explanation for error codes
rustc --explain E0412

# Format your code properly
rustfmt filename.rs
```

## 📚 Learning Mindset

**Remember:**
- **Errors are teachers** - Each one shows you something about Rust's safety
- **Struggle is learning** - If it was easy, you wouldn't be growing
- **Progress over perfection** - Small fixes accumulate into understanding
- **C# knowledge helps** - Use familiar concepts as stepping stones

**Questions to ask yourself:**
1. "How would I do this in C#?"
2. "What is Rust trying to prevent here?"
3. "What does this error message specifically say?"
4. "Have I seen a similar pattern in the lessons?"

---

**Ready to debug?** Start with Exercise 1 and work through each checkpoint systematically!
