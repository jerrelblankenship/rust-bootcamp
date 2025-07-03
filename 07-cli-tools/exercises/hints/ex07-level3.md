# Exercise 7 - Level 3 Hint (Near-Complete Solution)

## Complete Cross-Platform CLI Implementation

Here's a comprehensive solution that works seamlessly across Windows, macOS, and Linux.

```rust
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader, Write};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

// Cross-platform application structure
struct CrossPlatformApp {
    name: String,
    paths: AppPaths,
    config: AppConfig,
}

#[derive(Debug)]
struct AppPaths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    log_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    editor: String,
    auto_save: bool,
    theme: String,
    max_files: usize,
}

impl CrossPlatformApp {
    fn new(app_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let paths = AppPaths::new(app_name)
            .ok_or("Failed to determine application directories")?;
        
        // Ensure all directories exist
        paths.ensure_directories()?;
        
        // Load or create default configuration
        let config = Self::load_or_create_config(&paths)?;
        
        // Platform-specific initialization
        platform_specific_init()?;
        
        Ok(Self {
            name: app_name.to_string(),
            paths,
            config,
        })
    }
    
    fn load_or_create_config(paths: &AppPaths) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let config_file = paths.config_dir.join("config.toml");
        
        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            Ok(toml::from_str(&content)?)
        } else {
            // Create default config
            let default_config = AppConfig {
                editor: Self::detect_default_editor(),
                auto_save: true,
                theme: "auto".to_string(),
                max_files: 100,
            };
            
            // Save default config
            let content = toml::to_string_pretty(&default_config)?;
            fs::write(&config_file, content)?;
            
            Ok(default_config)
        }
    }
    
    fn detect_default_editor() -> String {
        // Try common environment variables
        if let Ok(editor) = env::var("EDITOR") {
            return editor;
        }
        
        if let Ok(editor) = env::var("VISUAL") {
            return editor;
        }
        
        // Platform-specific defaults
        if cfg!(windows) {
            if Self::command_exists("code") {
                "code".to_string()
            } else if Self::command_exists("notepad++") {
                "notepad++".to_string()
            } else {
                "notepad".to_string()
            }
        } else {
            if Self::command_exists("code") {
                "code".to_string()
            } else if Self::command_exists("vim") {
                "vim".to_string()
            } else if Self::command_exists("nano") {
                "nano".to_string()
            } else {
                "vi".to_string()
            }
        }
    }
    
    fn command_exists(command: &str) -> bool {
        let exe_name = if cfg!(windows) {
            format!("{}.exe", command)
        } else {
            command.to_string()
        };
        
        env::var_os("PATH")
            .and_then(|paths| {
                env::split_paths(&paths)
                    .map(|dir| dir.join(&exe_name))
                    .find(|path| path.is_file())
            })
            .is_some()
    }
    
    fn open_file_with_editor(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.config.editor);
        cmd.arg(file_path);
        
        // Platform-specific handling
        #[cfg(windows)]
        {
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        
        let status = cmd.status()?;
        
        if !status.success() {
            return Err(format!("Editor '{}' failed", self.config.editor).into());
        }
        
        Ok(())
    }
    
    fn list_files(&self, directory: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                files.push(path);
            }
        }
        
        // Sort files in a platform-aware manner
        files.sort_by(|a, b| {
            // Case-insensitive on Windows, case-sensitive on Unix
            if cfg!(windows) {
                a.to_string_lossy().to_lowercase()
                    .cmp(&b.to_string_lossy().to_lowercase())
            } else {
                a.cmp(b)
            }
        });
        
        Ok(files)
    }
    
    fn create_backup(&self, file_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let backup_dir = self.paths.data_dir.join("backups");
        fs::create_dir_all(&backup_dir)?;
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let file_name = file_path.file_name()
            .ok_or("Invalid file name")?;
        
        let backup_name = format!("{}.{}.bak", 
            file_name.to_string_lossy(), 
            timestamp
        );
        
        let backup_path = backup_dir.join(backup_name);
        fs::copy(file_path, &backup_path)?;
        
        Ok(backup_path)
    }
}

impl AppPaths {
    fn new(app_name: &str) -> Option<Self> {
        ProjectDirs::from("com", "example", app_name)
            .map(|proj_dirs| {
                let log_dir = if cfg!(windows) {
                    proj_dirs.data_dir().join("logs")
                } else {
                    proj_dirs.cache_dir().join("logs")
                };
                
                Self {
                    config_dir: proj_dirs.config_dir().to_path_buf(),
                    data_dir: proj_dirs.data_dir().to_path_buf(),
                    cache_dir: proj_dirs.cache_dir().to_path_buf(),
                    log_dir,
                }
            })
    }
    
    fn ensure_directories(&self) -> io::Result<()> {
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(&self.data_dir)?;
        fs::create_dir_all(&self.cache_dir)?;
        fs::create_dir_all(&self.log_dir)?;
        Ok(())
    }
}

// Platform-specific initialization
#[cfg(windows)]
fn platform_specific_init() -> Result<(), Box<dyn std::error::Error>> {
    // Enable ANSI color support in Windows Console
    use winapi_util::console::Console;
    
    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    
    Ok(())
}

#[cfg(unix)]
fn platform_specific_init() -> Result<(), Box<dyn std::error::Error>> {
    // Unix systems generally work out of the box
    Ok(())
}

// Main application entry point
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = CrossPlatformApp::new("CrossPlatformDemo")?;
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <command> [args...]", args[0]);
        println!("Commands:");
        println!("  list <dir>     - List files in directory");
        println!("  edit <file>    - Open file in editor");
        println!("  backup <file>  - Create backup of file");
        println!("  config         - Show configuration");
        return Ok(());
    }
    
    match args[1].as_str() {
        "list" => {
            let dir = if args.len() > 2 {
                PathBuf::from(&args[2])
            } else {
                env::current_dir()?
            };
            
            let files = app.list_files(&dir)?;
            
            println!("Files in {}:", dir.display());
            for file in files {
                println!("  {}", file.display());
            }
        },
        
        "edit" => {
            if args.len() < 3 {
                return Err("Please specify a file to edit".into());
            }
            
            let file_path = PathBuf::from(&args[2]);
            app.open_file_with_editor(&file_path)?;
        },
        
        "backup" => {
            if args.len() < 3 {
                return Err("Please specify a file to backup".into());
            }
            
            let file_path = PathBuf::from(&args[2]);
            let backup_path = app.create_backup(&file_path)?;
            
            println!("Backup created: {}", backup_path.display());
        },
        
        "config" => {
            println!("Configuration:");
            println!("  Config dir: {}", app.paths.config_dir.display());
            println!("  Data dir: {}", app.paths.data_dir.display());
            println!("  Cache dir: {}", app.paths.cache_dir.display());
            println!("  Log dir: {}", app.paths.log_dir.display());
            println!("  Editor: {}", app.config.editor);
            println!("  Auto-save: {}", app.config.auto_save);
            println!("  Theme: {}", app.config.theme);
        },
        
        _ => {
            return Err(format!("Unknown command: {}", args[1]).into());
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_cross_platform_paths() {
        // Test that paths work correctly on all platforms
        let path = PathBuf::from("base").join("subdir").join("file.txt");
        
        // Should always contain the file name
        assert!(path.to_string_lossy().contains("file.txt"));
        
        // Should use platform-appropriate separators
        let path_str = path.to_string_lossy();
        
        #[cfg(windows)]
        assert!(path_str.contains('\\') || path_str.contains('/'));
        
        #[cfg(unix)]
        assert!(path_str.contains('/'));
    }
    
    #[test]
    fn test_command_detection() {
        // Test that we can detect common commands
        // This test might vary by platform and installed software
        let has_common_cmd = if cfg!(windows) {
            CrossPlatformApp::command_exists("cmd")
        } else {
            CrossPlatformApp::command_exists("sh")
        };
        
        assert!(has_common_cmd);
    }
}
```

### Testing Script for All Platforms

```bash
#!/bin/bash
# test_cross_platform.sh

echo "=== Cross-Platform Testing ==="

# Test basic functionality
echo "\n1. Show configuration:"
cargo run --bin ex07-cross-platform config

# Test file listing
echo "\n2. List current directory:"
cargo run --bin ex07-cross-platform list

# Test with different path formats
echo "\n3. Test path handling:"
echo "test content" > test_file.txt
cargo run --bin ex07-cross-platform backup test_file.txt
rm test_file.txt

echo "\n4. Platform detection:"
echo "Running on: $(rustc --print target-triple)"

echo "\nAll tests completed!"
```

This implementation handles all major cross-platform concerns and provides a solid foundation for building robust CLI tools that work everywhere!