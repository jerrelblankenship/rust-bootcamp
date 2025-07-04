# Exercise 07 - Level 2 Hints: Publishing Problems

## 🎯 Required Metadata

```toml
[package]
name = "unique-text-utils-yourname"  # Must be unique
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A utility library for text processing operations"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/unique-text-utils"
homepage = "https://github.com/yourusername/unique-text-utils"
keywords = ["text", "string", "processing", "utilities"]
categories = ["text-processing"]
```

## 🔧 Structure Changes

1. **Remove `main()` function** - this is a library
2. **Add `lib.rs`** if using binary structure
3. **Add `examples/` directory** with usage examples
4. **Add `README.md`** with documentation

## 📚 Required Files

- `README.md` - User-facing documentation
- `LICENSE` - License file
- `CHANGELOG.md` - Version history
- `examples/usage.rs` - Working examples

## ⏰ Time Check

Still stuck after 30 minutes? Move to Level 3 for the complete setup.