# Baby Calculator Project - Fix and Learn!

🎯 **Your Mission**: Fix broken Rust functions to build a working calculator!

## 🏢 Why This Matters

**Enterprise applications** use the same patterns:
- **Financial systems**: Risk calculations, trading algorithms
- **Analytics tools**: Data processing pipelines  
- **DevOps utilities**: Resource calculation scripts
- **APIs**: Mathematical endpoints

**This teaches**: Functions, types, pattern matching, testing - core Rust skills for any domain.

## 🚨 Current Status: BROKEN! 

```bash
cd project-calculator
cargo build  # ❌ Shows compilation errors to fix
```

**Your job**: Make it work by fixing the broken code!

## 🎯 What You'll Build (Step by Step)

### **Phase 1: Basic Functions** (20 minutes)
Fix broken arithmetic functions:
```rust
fn add(a: f64, b: f64) -> f64 {
    todo!("Implement addition")  // ← Fix this!
}
```

### **Phase 2: Calculator Logic** (15 minutes)  
Fix the main calculator function:
```rust
fn calculate(left: f64, op: &str, right: f64) -> f64 {
    // ← Fix the pattern matching
}
```

### **Phase 3: User Interface** (15 minutes)
Fix the CLI interface:
```rust
fn main() {
    // ← Make it parse "5 + 3" and show "8"
}
```

## 🔧 How to Approach This

### **Step 1: Fix One Function at a Time**
```bash
cargo build  # Read the first error
# Fix that error
cargo build  # Read the next error
# Repeat until it compiles
```

### **Step 2: Run Tests to Verify**
```bash
cargo test  # See which tests pass
# Fix broken functions until all tests pass
```

### **Step 3: Test Your Calculator**
```bash
cargo run 5 + 3      # Should output: 8
cargo run 10 - 4     # Should output: 6
cargo run 6 * 7      # Should output: 42
cargo run 15 / 3     # Should output: 5
```

## 🎓 What You'll Learn

By fixing this broken code, you'll master:

1. **Function Syntax**: `fn name(params) -> return_type`
2. **Pattern Matching**: `match` statements for different operations
3. **Error Handling**: Basic `expect()` usage  
4. **Command Line**: Parsing `std::env::args()`
5. **Testing**: Writing and running `#[test]` functions

## 🔄 C# Developer Notes

| Concept | C# | Rust |
|---------|-----|------|
| **Function** | `public double Add(double a, double b)` | `fn add(a: f64, b: f64) -> f64` |
| **Switch** | `switch (op) { case "+" => a + b; }` | `match op { "+" => a + b, }` |
| **Console** | `Console.WriteLine(result)` | `println!("{}", result)` |
| **Args** | `string[] args` | `Vec<String>` from `env::args()` |

## 🏆 Success Criteria

You've mastered this project when:
- ✅ `cargo build` succeeds (no compilation errors)
- ✅ `cargo test` passes (all tests green)
- ✅ `cargo run 5 + 3` outputs `5 + 3 = 8`
- ✅ You understand every line of code you fixed

## 🆘 When You Get Stuck

1. **Read the error message** - Rust's compiler is incredibly helpful
2. **Look at the tests** - They show expected behavior
3. **Think in C# first** - Then translate to Rust syntax
4. **Use hints** - Check the `solutions/` directory only as last resort

## 📁 Project Structure

```
project-calculator/
├── src/
│   └── main.rs          # All code in one file (simple!)
├── tests/
│   └── basic_tests.rs   # Tests to verify your fixes
└── Cargo.toml           # Project configuration
```

## 🚀 Ready to Start?

```bash
cd project-calculator
cargo build  # See the errors
# Fix them one by one!
```

---

**Expected time**: 45-60 minutes  
**Difficulty**: Perfect for Rust beginners with C# experience  
**Next**: [Module 02 - Ownership](../../02-ownership-and-borrowing/README.md) 🦀
