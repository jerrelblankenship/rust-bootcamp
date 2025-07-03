// Exercise 7: Cross-Platform CLI - Fix Windows/Unix Incompatibilities!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 platform issues to fix)
//
// Your task: Make this CLI tool work identically on Windows, macOS, and Linux
// This demonstrates cross-platform development and platform-specific handling
//
// INSTRUCTIONS:
// 1. Test on multiple platforms (or use #[cfg] to simulate)
// 2. Fix path separators, line endings, and terminal differences
// 3. Handle platform-specific features gracefully
// 4. Ensure consistent behavior across all platforms
//
// LEARNING STRATEGY:
// - Use std::path for proper path handling
// - Handle different line endings (\r\n vs \n)
// - Detect platform capabilities (colors, Unicode, etc.)
// - Provide platform-specific optimizations where beneficial
//
// CROSS-PLATFORM ISSUES TO FIX:
// [] Fix hardcoded Unix path separators
// [] Handle Windows vs Unix line endings properly
// [] Detect terminal capabilities per platform
// [] Handle Unicode support differences
// [] Fix file permission checking
// [] Adapt keyboard interrupt handling

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

fn main() {
    println!("Cross-Platform File Processor");
    println!("Platform: {}", env::consts::OS);
    
    // TERRIBLE: Hardcoded Unix paths that break on Windows
    let config_path = "~/.config/myapp/config.toml";  // FIXME: Unix-specific!
    let temp_dir = "/tmp/processing";                 // FIXME: Unix-specific!
    let log_file = "logs/app.log";                    // FIXME: Wrong path separator!
    
    println!("Config: {}", config_path);
    println!("Temp: {}", temp_dir);  
    println!("Log: {}", log_file);
    
    // Test various cross-platform issues
    test_path_handling();
    test_line_endings();
    test_terminal_features();
    test_file_permissions();
}

// FIXME: This breaks on Windows due to hardcoded separators
fn test_path_handling() {
    println!("\n=== Path Handling Test ===");
    
    // TERRIBLE: Hardcoded Unix separators
    let file_path = "data/files/input.txt";           // FIXME: Should use PathBuf!
    let nested_path = "output/results/final.csv";     // FIXME: Not cross-platform!
    
    // TERRIBLE: Manual string concatenation
    let combined = format!("{}/{}", file_path, "backup.txt");  // FIXME: Wrong on Windows!
    
    println!("File path: {}", file_path);
    println!("Nested: {}", nested_path);
    println!("Combined: {}", combined);
    
    // TODO: Show how to do it correctly
    demonstrate_correct_paths();
}

// TODO: Show proper cross-platform path handling
fn demonstrate_correct_paths() {
    // TODO: Use PathBuf and proper path methods
    // let config_dir = get_config_dir();
    // let data_file = config_dir.join("data").join("input.txt");
    // println!("Correct path: {}", data_file.display());
    
    println!("TODO: Implement proper path handling");
}

// TODO: Get platform-appropriate config directory
fn get_config_dir() -> PathBuf {
    // TODO: Return correct config directory for each platform:
    // Windows: %APPDATA%\myapp
    // macOS: ~/Library/Application Support/myapp  
    // Linux: ~/.config/myapp (or $XDG_CONFIG_HOME/myapp)
    
    #[cfg(windows)]
    {
        // TODO: Use Windows APPDATA
        todo!("Implement Windows config dir")
    }
    
    #[cfg(target_os = "macos")]
    {
        // TODO: Use macOS Application Support
        todo!("Implement macOS config dir")
    }
    
    #[cfg(unix)]
    {
        // TODO: Use XDG Base Directory Specification
        todo!("Implement Unix config dir")
    }
}

// TODO: Get platform-appropriate temp directory
fn get_temp_dir() -> PathBuf {
    // TODO: Use std::env::temp_dir() which works on all platforms
    // Windows: %TEMP%
    // Unix: /tmp or $TMPDIR
    env::temp_dir()
}

// FIXME: Line ending handling breaks cross-platform compatibility
fn test_line_endings() {
    println!("\n=== Line Endings Test ===");
    
    let sample_text = "Line 1\nLine 2\nLine 3\n";  // FIXME: Unix line endings only!
    
    // TERRIBLE: Manual line ending handling
    let windows_text = sample_text.replace("\n", "\r\n");  // FIXME: Crude conversion!
    
    println!("Original: {:?}", sample_text);
    println!("Windows: {:?}", windows_text);
    
    // TODO: Show proper line ending handling
    demonstrate_line_endings();
}

// TODO: Demonstrate proper line ending handling
fn demonstrate_line_endings() {
    // TODO: Use platform-appropriate line endings
    // let line_ending = if cfg!(windows) { "\r\n" } else { "\n" };
    // or use writeln! which handles it automatically
    
    println!("TODO: Implement proper line ending handling");
}

// FIXME: Terminal features detection is not cross-platform
fn test_terminal_features() {
    println!("\n=== Terminal Features Test ===");
    
    // TERRIBLE: Assumes Unix terminal capabilities
    let supports_color = true;        // FIXME: Should detect properly!
    let supports_unicode = true;      // FIXME: Windows cmd.exe might not!
    let terminal_width = 80;          // FIXME: Should detect actual width!
    
    println!("Colors: {}", supports_color);
    println!("Unicode: {}", supports_unicode); 
    println!("Width: {}", terminal_width);
    
    // TODO: Show proper capability detection
    demonstrate_terminal_detection();
}

// TODO: Properly detect terminal capabilities per platform
fn demonstrate_terminal_detection() {
    // TODO: Detect color support properly on each platform
    let supports_color = detect_color_support();
    
    // TODO: Detect Unicode support (especially important for Windows)
    let supports_unicode = detect_unicode_support();
    
    // TODO: Get actual terminal dimensions
    let (width, height) = get_terminal_size();
    
    println!("Detected - Colors: {}, Unicode: {}, Size: {}x{}", 
             supports_color, supports_unicode, width, height);
}

// TODO: Detect color support per platform
fn detect_color_support() -> bool {
    // TODO: Platform-specific color detection
    // Windows: Check for Windows Terminal, cmd.exe vs PowerShell
    // Unix: Check TERM, COLORTERM environment variables
    // Consider NO_COLOR environment variable on all platforms
    
    #[cfg(windows)]
    {
        // TODO: Windows-specific color detection
        // Check for Windows Terminal, ConEmu, etc.
        todo!("Implement Windows color detection")
    }
    
    #[cfg(unix)]
    {
        // TODO: Unix color detection
        // Check TERM, COLORTERM variables
        env::var("TERM").map_or(false, |term| term != "dumb")
    }
}

// TODO: Detect Unicode support (especially important for Windows)
fn detect_unicode_support() -> bool {
    // TODO: Check if terminal supports Unicode properly
    // Windows cmd.exe: Often problematic
    // Windows PowerShell: Usually good  
    // Windows Terminal: Excellent
    // Unix terminals: Usually good
    
    #[cfg(windows)]
    {
        // TODO: Check Windows terminal type and Unicode support
        todo!("Check Windows Unicode support")
    }
    
    #[cfg(unix)]
    {
        // Most Unix terminals support Unicode well
        true
    }
}

// TODO: Get terminal size cross-platform
fn get_terminal_size() -> (u16, u16) {
    // TODO: Use terminal_size crate or platform-specific calls
    // Windows: GetConsoleScreenBufferInfo
    // Unix: ioctl TIOCGWINSZ
    (80, 24)  // Default fallback
}

// FIXME: File permissions checking is Unix-specific
fn test_file_permissions() {
    println!("\n=== File Permissions Test ===");
    
    let test_file = "test.txt";
    
    // TERRIBLE: Unix-specific permission checking
    match fs::metadata(test_file) {
        Ok(metadata) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                println!("Unix permissions: {:o}", mode);  // FIXME: Breaks on Windows!
            }
            
            #[cfg(windows)]
            {
                // TODO: Windows doesn't have Unix-style permissions
                println!("Windows file attributes: {:?}", metadata.permissions());
            }
        },
        Err(e) => println!("Could not read metadata: {}", e),
    }
    
    // TODO: Show cross-platform permission checking
    demonstrate_permissions();
}

// TODO: Cross-platform permission and attribute checking
fn demonstrate_permissions() {
    // TODO: Check permissions/attributes in a cross-platform way
    // Unix: Use mode bits (rwx for owner/group/other)
    // Windows: Use file attributes (readonly, hidden, system, etc.)
    
    println!("TODO: Implement cross-platform permission checking");
}

// TODO: Handle keyboard interrupts cross-platform
fn setup_interrupt_handler() {
    // TODO: Handle Ctrl+C gracefully on all platforms
    // Unix: SIGINT signal
    // Windows: SetConsoleCtrlHandler
    
    // HINT: Use ctrlc crate for cross-platform interrupt handling
    println!("TODO: Setup cross-platform interrupt handling");
}

// TODO: Platform-specific optimizations
fn platform_optimizations() {
    // TODO: Use platform-specific features when available
    // Windows: Use Windows API for better performance
    // macOS: Use Foundation framework features
    // Linux: Use Linux-specific syscalls
    
    #[cfg(windows)]
    {
        // TODO: Windows-specific optimizations
        println!("Using Windows optimizations");
    }
    
    #[cfg(target_os = "macos")]
    {
        // TODO: macOS-specific features
        println!("Using macOS optimizations");
    }
    
    #[cfg(target_os = "linux")]
    {
        // TODO: Linux-specific features
        println!("Using Linux optimizations");
    }
}

// TODO: Create cross-platform file operations
mod file_ops {
    use super::*;
    
    // TODO: Cross-platform file creation with proper permissions
    pub fn create_file_with_permissions(path: &Path) -> io::Result<fs::File> {
        let file = fs::File::create(path)?;
        
        #[cfg(unix)]
        {
            // Set Unix permissions (e.g., 644)
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o644);
            file.set_permissions(perms)?;
        }
        
        #[cfg(windows)]
        {
            // Windows doesn't use Unix-style permissions
            // Could set file attributes if needed
        }
        
        Ok(file)
    }
    
    // TODO: Cross-platform directory creation
    pub fn create_config_dir() -> io::Result<PathBuf> {
        let config_dir = get_config_dir();
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_path_construction() {
        // TODO: Test that paths work on all platforms
        let path = PathBuf::from("data").join("files").join("test.txt");
        
        #[cfg(windows)]
        assert!(path.to_string_lossy().contains("\\"));
        
        #[cfg(unix)]
        assert!(path.to_string_lossy().contains("/"));
    }
    
    #[test]
    fn test_config_directory() {
        // TODO: Test config directory is platform-appropriate
        let config_dir = get_config_dir();
        
        #[cfg(windows)]
        assert!(config_dir.to_string_lossy().contains("AppData"));
        
        #[cfg(unix)]
        assert!(config_dir.to_string_lossy().contains(".config") || 
                config_dir.to_string_lossy().contains("Library"));
    }
    
    #[test]
    fn test_line_endings() {
        // TODO: Test proper line ending handling
        todo!("Test line endings per platform");
    }
}

// CROSS-PLATFORM COMPATIBILITY CHECKLIST:
//
// PATHS:
// ✅ Use PathBuf and Path instead of string concatenation
// ✅ Use join() instead of manual separator insertion  
// ✅ Use platform-appropriate config/data directories
// ✅ Handle case sensitivity differences (Windows vs Unix)
//
// LINE ENDINGS:
// ✅ Use writeln! which handles platform line endings
// ✅ When reading, handle both \n and \r\n  
// ✅ When writing text files, use platform-appropriate endings
//
// TERMINAL FEATURES:
// ✅ Detect color support per platform and terminal
// ✅ Check Unicode support (especially Windows cmd.exe)
// ✅ Get actual terminal dimensions
// ✅ Respect NO_COLOR and other environment variables
//
// FILE OPERATIONS:
// ✅ Use std::fs for cross-platform file operations
// ✅ Handle permission differences (Unix mode vs Windows attributes)
// ✅ Use appropriate temp directories (std::env::temp_dir())
//
// SIGNALS/INTERRUPTS:
// ✅ Handle Ctrl+C gracefully on all platforms
// ✅ Use ctrlc crate for cross-platform signal handling
//
// C# COMPARISON:
// C#: Path.Combine(), Environment.SpecialFolder
// Rust: PathBuf::join(), platform-specific config dirs
//
// C#: Environment.NewLine for platform line endings
// Rust: writeln! handles platform line endings automatically
//
// C#: Console.WindowWidth, Console.WindowHeight
// Rust: terminal_size crate or platform-specific calls
//
// 📊 PROGRESS TRACKER:
// Issue 1 (Path separators): [ ] Fixed
// Issue 2 (Line endings): [ ] Fixed
// Issue 3 (Terminal capabilities): [ ] Fixed  
// Issue 4 (Unicode support): [ ] Fixed
// Issue 5 (File permissions): [ ] Fixed
// Issue 6 (Interrupt handling): [ ] Fixed
//
// 🎯 SUCCESS CRITERIA:
// ✅ Works identically on Windows, macOS, and Linux
// ✅ Uses appropriate config directories per platform
// ✅ Handles file paths correctly with proper separators
// ✅ Detects terminal capabilities accurately
// ✅ Respects platform conventions and user preferences
// ✅ Graceful fallbacks when features aren't supported
//
// 🚀 NEXT CHALLENGE:
// Move to ex08-subcommands.rs for complex command structures!