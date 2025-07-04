# Exercise 05 - Level 2 Hints: Cross-Platform Chaos

## 🎯 Cross-Platform Solutions

### 🗋 Path Handling
```rust
use std::path::PathBuf;
use dirs;

// Instead of "/home/user/.config"
let config_dir = dirs::config_dir().unwrap().join("myapp");

// Instead of "/var/log"
let log_dir = dirs::cache_dir().unwrap().join("myapp");
```

### 🗂 File Operations
```rust
use std::fs;

// Instead of shell commands
fs::create_dir_all(&config_dir)?;
fs::File::create(&config_path)?;
```

### 🔄 Process Management
```rust
use std::process;

// Instead of libc::getpid()
let pid = process::id();
```

## 📚 Required Crates

```toml
[dependencies]
dirs = "5.0"  # Cross-platform directories
```

## ⏰ Time Check

Still stuck after 30 minutes? Move to Level 3 for the complete solution.