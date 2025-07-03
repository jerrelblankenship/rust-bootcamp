# Developer Tools CLI - Final Project

🎯 **Your Mission**: Transform this broken CLI into a professional developer toolchain!

## 🚨 Current Status: BROKEN!

This project currently **doesn't compile** and has **no working features**. Your job is to fix it!

```bash
cd project-dev-tools
cargo build  # ❌ Shows compilation errors to fix
```

## 🎯 What You're Building

A comprehensive CLI tool that developers actually want to use:

```bash
# File operations
dev-tools file process *.txt --output results/
dev-tools file validate src/ --strict  
dev-tools file convert data.csv --to json

# Git integration  
dev-tools git status --format table
dev-tools git branches --show-merged

# Server management
dev-tools server start --port 8080 --watch
dev-tools server logs --follow

# Configuration
dev-tools config set editor.command "code"
dev-tools config show --format yaml
```

## 🔧 How to Approach This

### **Phase 1: Get It Compiling (30 minutes)**
1. **Fix Cargo.toml**: Uncomment the dependencies you need
2. **Create missing modules**: The code imports modules that don't exist yet
3. **Define missing types**: Enums and structs referenced but not defined
4. **Stub out functions**: Add `todo!()` implementations to get it compiling

### **Phase 2: Basic Functionality (2 hours)**
5. **Start with one command**: Pick `file process` and make it work
6. **Add proper error handling**: Apply lessons from ex02-error-handling
7. **Test as you go**: `cargo run -- file process test.txt`
8. **Add colors and progress**: Apply lessons from ex05 and ex06

### **Phase 3: Full Implementation (3-4 hours)**  
9. **Implement all commands**: File, Git, Server, Config
10. **Add configuration support**: Apply lessons from ex03-config-files
11. **Make it cross-platform**: Apply lessons from ex07-cross-platform
12. **Polish the UX**: Great help messages, error handling, colors

### **Phase 4: Polish (1 hour)**
13. **Add comprehensive tests**: Integration tests for all commands
14. **Add shell completions**: Make it feel professional
15. **Performance optimization**: Handle large files efficiently
16. **Documentation**: Comments and examples

## 🛠️ Implementation Guide

### **Start Here: Fix Compilation**
```bash
# 1. Edit Cargo.toml - uncomment dependencies you need
# 2. Create missing module files:
touch src/commands/git.rs src/commands/server.rs src/commands/config.rs
touch src/config.rs src/utils.rs

# 3. Add basic module declarations in each file:
echo "// TODO: Implement git commands
pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}" > src/commands/git.rs

# 4. Try to compile and fix one error at a time:
cargo build
```

### **Essential Dependencies to Uncomment**
- `clap` - Command line parsing (you learned this in ex01)
- `serde` + `toml` - Configuration (you learned this in ex03)  
- `colored` - Terminal colors (you learned this in ex06)
- `indicatif` - Progress bars (you learned this in ex05)

### **Key Design Patterns to Apply**

**Error Handling** (from ex02):
```rust
// Bad (current)
.unwrap()

// Good (what you should build)
.map_err(|e| CliError::FileNotFound(path.clone()))?
```

**Configuration** (from ex03):
```rust
// Support multiple config sources
// CLI args > env vars > config file > defaults
```

**Cross-platform** (from ex07):
```rust
// Use PathBuf, not string concatenation
let path = config_dir.join("dev-tools").join("config.toml");
```

## 🎯 Success Criteria

You've succeeded when:

- ✅ `cargo build` succeeds without errors
- ✅ `cargo test` passes all tests  
- ✅ `dev-tools --help` shows beautiful help
- ✅ All major commands work: `file`, `git`, `server`, `config`
- ✅ Error messages are helpful with suggestions
- ✅ Colors and progress bars enhance the experience
- ✅ Tool works on Windows, macOS, and Linux
- ✅ You'd actually want to use this tool daily

## 🆘 When You Get Stuck

1. **15 minutes struggling**: Check `hints/project-level1.md`
2. **30 minutes struggling**: Check `hints/project-level2.md`  
3. **45 minutes struggling**: Check `hints/project-level3.md`

## 🏆 Going Beyond

Want to make this really special?

- **Shell completions**: `dev-tools completion bash > /etc/bash_completion.d/dev-tools`
- **Plugin system**: Allow custom commands
- **Watch mode**: Auto-run commands when files change
- **Configuration templates**: Pre-configured setups for common workflows
- **Integration**: GitHub, GitLab, Docker, Kubernetes APIs
- **Performance**: Parallel processing, async operations

## 📚 Learning Outcomes

By completing this project, you'll have:

- ✅ Built a production-quality CLI tool
- ✅ Applied all concepts from exercises 1-8
- ✅ Learned CLI design patterns and best practices
- ✅ Created something you can add to your portfolio
- ✅ Gained confidence building complex Rust applications

---

**Ready to build something amazing?** Start with `cargo build` and fix the first error! 🚀