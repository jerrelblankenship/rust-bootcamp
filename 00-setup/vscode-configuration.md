# VS Code Configuration for Rust Development

This guide will help you configure Visual Studio Code for optimal Rust development, with settings that work across both macOS and Windows.

## 🧩 Required Extensions

Install these essential extensions:

```bash
# Install via command line
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
code --install-extension tamasfe.even-better-toml
code --install-extension usernamehw.errorlens
```

### Extension Details

1. **rust-analyzer** - Official Rust language server
   - IntelliSense, code completion, and refactoring
   - Real-time error checking
   - Inline type hints

2. **CodeLLDB** - Native debugger
   - Breakpoints and stepping
   - Variable inspection
   - Memory viewing

3. **crates** - Cargo.toml dependency management
   - Shows latest versions inline
   - Quick update commands

4. **Even Better TOML** - TOML language support
   - Syntax highlighting for Cargo.toml
   - Schema validation

5. **Error Lens** - Inline error display
   - Shows errors directly in code
   - Improves debugging workflow

## ⚙️ VS Code Settings

Add to your `settings.json` (Cmd/Ctrl + Shift + P → "Preferences: Open Settings (JSON)"):

```json
{
    // Rust Analyzer Settings
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.inlayHints.enabled": "always",
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.chainingHints.enable": true,
    "rust-analyzer.inlayHints.closureReturnTypeHints.enable": "always",
    "rust-analyzer.inlayHints.lifetimeElisionHints.enable": "skip_trivial",
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.lens.enable": true,
    "rust-analyzer.lens.implementations.enable": true,
    "rust-analyzer.lens.references.enable": true,
    
    // Cargo settings
    "rust-analyzer.cargo.autoreload": true,
    "rust-analyzer.cargo.allFeatures": true,
    
    // Formatting
    "rust-analyzer.rustfmt.enableRangeFormatting": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true,
        "editor.formatOnPaste": true,
        "editor.rulers": [100]
    },
    
    // Error Lens
    "errorLens.enabled": true,
    "errorLens.delay": 100,
    
    // File associations
    "files.associations": {
        "*.rs": "rust",
        "Cargo.lock": "toml",
        "Cargo.toml": "toml"
    },
    
    // Terminal settings for cargo commands
    "terminal.integrated.env.osx": {
        "RUST_BACKTRACE": "1"
    },
    "terminal.integrated.env.windows": {
        "RUST_BACKTRACE": "1"
    },
    "terminal.integrated.env.linux": {
        "RUST_BACKTRACE": "1"
    },
    
    // Editor settings for Rust
    "editor.semanticHighlighting.enabled": true,
    "editor.codeLens": true,
    "editor.inlayHints.enabled": "on"
}
```

## 🐛 Debugging Configuration

Create `.vscode/launch.json` in your project root:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable",
            "cargo": {
                "args": [
                    "build",
                    "--bin=${workspaceFolderBasename}",
                    "--package=${workspaceFolderBasename}"
                ],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=${workspaceFolderBasename}",
                    "--package=${workspaceFolderBasename}"
                ],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug with arguments",
            "cargo": {
                "args": ["build", "--bin=${workspaceFolderBasename}"],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": ["${input:commandLineArgs}"],
            "cwd": "${workspaceFolder}"
        }
    ],
    "inputs": [
        {
            "id": "commandLineArgs",
            "type": "promptString",
            "description": "Command line arguments",
            "default": ""
        }
    ]
}
```

## ⌨️ Useful Keyboard Shortcuts

Configure these Rust-specific shortcuts in `keybindings.json`:

```json
[
    {
        "key": "ctrl+shift+b",
        "command": "rust-analyzer.run",
        "when": "editorTextFocus && editorLangId == rust"
    },
    {
        "key": "ctrl+shift+t",
        "command": "rust-analyzer.runTests",
        "when": "editorTextFocus && editorLangId == rust"
    },
    {
        "key": "ctrl+shift+d",
        "command": "rust-analyzer.openDocs",
        "when": "editorTextFocus && editorLangId == rust"
    },
    {
        "key": "ctrl+shift+a",
        "command": "rust-analyzer.expandMacro",
        "when": "editorTextFocus && editorLangId == rust"
    }
]
```

## 📋 Tasks Configuration

Create `.vscode/tasks.json` for common Rust tasks:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo build",
            "type": "cargo",
            "command": "build",
            "problemMatcher": ["$rustc"],
            "group": {
                "kind": "build",
                "isDefault": true
            }
        },
        {
            "label": "cargo build --release",
            "type": "cargo",
            "command": "build",
            "args": ["--release"],
            "problemMatcher": ["$rustc"],
            "group": "build"
        },
        {
            "label": "cargo test",
            "type": "cargo",
            "command": "test",
            "problemMatcher": ["$rustc"],
            "group": {
                "kind": "test",
                "isDefault": true
            }
        },
        {
            "label": "cargo run",
            "type": "cargo",
            "command": "run",
            "problemMatcher": ["$rustc"],
            "group": "none"
        },
        {
            "label": "cargo check",
            "type": "cargo",
            "command": "check",
            "problemMatcher": ["$rustc"],
            "group": "build"
        },
        {
            "label": "cargo clippy",
            "type": "cargo",
            "command": "clippy",
            "problemMatcher": ["$rustc"],
            "group": "build"
        },
        {
            "label": "cargo watch",
            "type": "shell",
            "command": "cargo",
            "args": ["watch", "-x", "check", "-x", "test", "-x", "run"],
            "problemMatcher": ["$rustc"],
            "isBackground": true,
            "group": "build"
        }
    ]
}
```

## 🎨 Recommended Theme Settings

For better Rust syntax highlighting:

```json
{
    "workbench.colorCustomizations": {
        "[Your Theme Name]": {
            "textMateRules": [
                {
                    "scope": "entity.name.type.rust",
                    "settings": {
                        "foreground": "#4EC9B0"
                    }
                },
                {
                    "scope": "storage.type.rust",
                    "settings": {
                        "foreground": "#569CD6"
                    }
                },
                {
                    "scope": "support.constant.core.rust",
                    "settings": {
                        "foreground": "#4FC1FF"
                    }
                }
            ]
        }
    }
}
```

## 🔧 Workspace Settings

Create `.vscode/settings.json` in your project root:

```json
{
    "rust-analyzer.linkedProjects": ["./Cargo.toml"],
    "rust-analyzer.cargo.target": null,
    "rust-analyzer.cargo.features": [],
    "editor.formatOnSave": true,
    "files.watcherExclude": {
        "**/target/**": true
    },
    "files.exclude": {
        "**/*.rs.bk": true,
        "**/*.pdb": true
    }
}
```

## 🚀 Productivity Tips

1. **Quick Actions**
   - `Ctrl+.` - Quick fixes and refactoring
   - `F12` - Go to definition
   - `Shift+F12` - Find all references
   - `Ctrl+Shift+Space` - Trigger parameter hints

2. **Rust Analyzer Commands**
   - "Rust Analyzer: Run" - Run current file
   - "Rust Analyzer: Expand Macro" - See macro expansion
   - "Rust Analyzer: Open Docs" - Open docs.rs for item

3. **Snippets**
   Add custom snippets in `rust.json`:
   ```json
   {
       "Test Function": {
           "prefix": "testfn",
           "body": [
               "#[test]",
               "fn ${1:test_name}() {",
               "    $0",
               "}"
           ]
       },
       "Async Function": {
           "prefix": "asyncfn",
           "body": [
               "async fn ${1:function_name}(${2:params}) -> ${3:ReturnType} {",
               "    $0",
               "}"
           ]
       }
   }
   ```

## 📱 Platform-Specific Settings

### macOS
```json
{
    "terminal.integrated.fontFamily": "SF Mono, Menlo, monospace",
    "terminal.integrated.fontSize": 13
}
```

### Windows
```json
{
    "terminal.integrated.defaultProfile.windows": "PowerShell",
    "terminal.integrated.fontFamily": "Cascadia Code, Consolas, monospace"
}
```

## ✅ Verification

Test your setup:

1. Open a Rust file
2. Check that:
   - Syntax highlighting works
   - IntelliSense provides completions
   - Error squiggles appear for invalid code
   - Ctrl+Click navigates to definitions
   - Debugging breakpoints work

## 🆘 Troubleshooting

1. **rust-analyzer not starting**
   - Check Output panel for errors
   - Run `rustup component add rust-analyzer`

2. **Slow IntelliSense**
   - Reduce `rust-analyzer.cargo.features`
   - Disable `rust-analyzer.procMacro.enable`

3. **Debugging not working**
   - Ensure CodeLLDB is installed
   - Check launch.json configuration

---

Next: [Container Setup (Optional)](container-setup.md) →
