// Exercise 6: Color Output - Fix the Bland Terminal Interface!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 color features to fix)
//
// Your task: Add colors and styling to make the CLI beautiful and functional
// This demonstrates terminal capabilities detection and responsive styling
//
// INSTRUCTIONS:
// 1. Run program to see boring black and white output
// 2. Add colors for different message types (errors, warnings, success)
// 3. Test in different terminals and with NO_COLOR environment variable
// 4. Make colors enhance usability, not just decoration
//
// LEARNING STRATEGY:
// - Start with basic ANSI color codes
// - Add color detection (don't force colors in pipes)
// - Use colors functionally (red=error, green=success, etc.)
// - Respect user preferences (NO_COLOR, color blindness)
//
// COLOR FEATURES TO FIX:
// [] Add ANSI color codes for different message types
// [] Detect terminal color capabilities
// [] Respect NO_COLOR environment variable
// [] Use colors functionally, not decoratively
// [] Handle color-blind friendly alternatives

use std::env;
use std::io::{self, Write};

fn main() {
    // TERRIBLE: Everything looks the same - no visual hierarchy
    println!("Welcome to the File Processor!");
    println!("Starting analysis...");
    
    // Simulate different types of output
    show_file_analysis();
    
    println!("Analysis complete!");
}

// FIXME: All output looks identical - no visual distinction
fn show_file_analysis() {
    // These should have different colors/styles but don't
    println!("INFO: Starting file scan...");
    println!("SUCCESS: Found 42 files");
    println!("WARNING: 3 files have unusual permissions");
    println!("ERROR: 1 file is corrupted");
    println!("DEBUG: Processing took 1.2 seconds");
    
    // Show a summary
    show_summary();
}

// FIXME: Summary is hard to read without visual hierarchy
fn show_summary() {
    println!("=== SUMMARY ===");
    println!("Total files: 42");
    println!("Processed: 41");
    println!("Warnings: 3"); 
    println!("Errors: 1");
    println!("Status: COMPLETED WITH ISSUES");
}

// TODO: Define ANSI color codes
mod colors {
    // TODO: Add ANSI escape codes for colors
    // Basic colors
    // pub const RESET: &str = "\x1b[0m";
    // pub const RED: &str = "\x1b[31m";
    // pub const GREEN: &str = "\x1b[32m";
    // etc.
    
    pub const RESET: &str = "";     // FIXME: No reset code!
    pub const RED: &str = "";       // FIXME: No red color!
    pub const GREEN: &str = "";     // FIXME: No green color!
    pub const YELLOW: &str = "";    // FIXME: No yellow color!
    pub const BLUE: &str = "";      // FIXME: No blue color!
    pub const MAGENTA: &str = "";   // FIXME: No magenta color!
    pub const CYAN: &str = "";      // FIXME: No cyan color!
    
    // TODO: Add styles
    pub const BOLD: &str = "";      // FIXME: No bold style!
    pub const UNDERLINE: &str = ""; // FIXME: No underline!
    pub const DIM: &str = "";       // FIXME: No dim style!
}

// TODO: Create color-aware printing functions
fn print_error(message: &str) {
    // TODO: Print in red with error symbol
    // Should look like: ❌ Error: File not found
    println!("Error: {}", message);  // FIXME: No color or styling!
}

fn print_warning(message: &str) {
    // TODO: Print in yellow with warning symbol  
    // Should look like: ⚠️  Warning: Unusual permissions
    println!("Warning: {}", message);  // FIXME: No color or styling!
}

fn print_success(message: &str) {
    // TODO: Print in green with success symbol
    // Should look like: ✅ Success: Processing complete
    println!("Success: {}", message);  // FIXME: No color or styling!
}

fn print_info(message: &str) {
    // TODO: Print in blue with info symbol
    // Should look like: ℹ️  Info: Starting scan
    println!("Info: {}", message);  // FIXME: No color or styling!
}

fn print_debug(message: &str) {
    // TODO: Print in dim gray for debug messages
    // Should look like: 🔍 Debug: Processing took 1.2s
    println!("Debug: {}", message);  // FIXME: No color or styling!
}

// TODO: Detect terminal color capabilities
fn supports_color() -> bool {
    // TODO: Check if terminal supports colors
    // Factors to consider:
    // 1. TERM environment variable
    // 2. NO_COLOR environment variable (should disable colors)
    // 3. FORCE_COLOR environment variable (should enable colors)
    // 4. Whether output is a TTY (no colors in pipes/redirects)
    // 5. CI environment (usually no colors)
    
    // HINT: Use std::io::IsTerminal or atty crate
    use std::io::IsTerminal;
    
    // Check NO_COLOR first (user preference to disable)
    if env::var("NO_COLOR").is_ok() {
        return false;
    }
    
    // Check FORCE_COLOR (user preference to enable)
    if env::var("FORCE_COLOR").is_ok() {
        return true;
    }
    
    // TODO: Check if output is a terminal
    if !io::stdout().is_terminal() {
        return false;  // Don't colorize pipes/redirects
    }
    
    // TODO: Check TERM environment variable
    // Most terminals set TERM to something indicating color support
    match env::var("TERM") {
        Ok(term) => {
            // TODO: Check if TERM indicates color support
            // Common values: "xterm-256color", "screen", "tmux", etc.
            !term.contains("dumb") && term != "unknown"
        },
        Err(_) => false,  // No TERM set, assume no color
    }
}

// TODO: Create a color formatter that respects capabilities
struct ColorFormatter {
    use_colors: bool,
}

impl ColorFormatter {
    fn new() -> Self {
        Self {
            use_colors: supports_color(),
        }
    }
    
    // TODO: Format text with color if supported
    fn format(&self, text: &str, color: &str, style: &str) -> String {
        if self.use_colors {
            format!("{}{}{}{}", style, color, text, colors::RESET)
        } else {
            text.to_string()
        }
    }
    
    // TODO: Convenience methods for common formatting
    fn error(&self, text: &str) -> String {
        if self.use_colors {
            format!("❌ {}", self.format(text, colors::RED, colors::BOLD))
        } else {
            format!("ERROR: {}", text)
        }
    }
    
    fn warning(&self, text: &str) -> String {
        if self.use_colors {
            format!("⚠️  {}", self.format(text, colors::YELLOW, ""))
        } else {
            format!("WARNING: {}", text)
        }
    }
    
    fn success(&self, text: &str) -> String {
        // TODO: Format success messages
        todo!("Format success message")
    }
    
    fn info(&self, text: &str) -> String {
        // TODO: Format info messages
        todo!("Format info message")
    }
    
    fn debug(&self, text: &str) -> String {
        // TODO: Format debug messages (dimmed)
        todo!("Format debug message")
    }
}

// TODO: Create improved output functions using ColorFormatter
fn show_file_analysis_improved() {
    let formatter = ColorFormatter::new();
    
    println!("{}", formatter.info("Starting file scan..."));
    println!("{}", formatter.success("Found 42 files"));
    println!("{}", formatter.warning("3 files have unusual permissions"));
    println!("{}", formatter.error("1 file is corrupted"));
    println!("{}", formatter.debug("Processing took 1.2 seconds"));
    
    show_summary_improved(&formatter);
}

// TODO: Create improved summary with colors and visual hierarchy
fn show_summary_improved(formatter: &ColorFormatter) {
    // TODO: Create a nicely formatted summary
    // Should use:
    // - Colors for different values (green=good, red=bad, yellow=warnings)
    // - Visual separators
    // - Clear hierarchy
    
    println!("\n{}", formatter.format("=== SUMMARY ===", colors::CYAN, colors::BOLD));
    println!("Total files: {}", formatter.format("42", colors::BLUE, ""));
    println!("Processed: {}", formatter.format("41", colors::GREEN, ""));
    println!("Warnings: {}", formatter.format("3", colors::YELLOW, ""));
    println!("Errors: {}", formatter.format("1", colors::RED, ""));
    
    // TODO: Status should reflect overall health
    let status = if true { // TODO: Check if there were errors
        formatter.error("COMPLETED WITH ISSUES")
    } else {
        formatter.success("COMPLETED SUCCESSFULLY")
    };
    println!("Status: {}", status);
}

// TODO: Add progress bar colors (from previous exercise)
fn colored_progress_bar(progress: f32, width: usize) -> String {
    // TODO: Create colored progress bar
    // [████████░░░░░░░░] with colors:
    // - Green filled blocks for good progress
    // - Red filled blocks if there are errors
    // - Yellow for warnings
    
    let filled = (progress * width as f32) as usize;
    let empty = width - filled;
    
    // FIXME: This creates a boring monochrome bar
    format!("[{}{}]", 
           "█".repeat(filled),
           "░".repeat(empty))
}

// TODO: Create a table formatter with colors
fn format_table() {
    // TODO: Create a colored table like:
    // ┌─────────────┬────────┬────────┐
    // │ File        │ Status │ Issues │
    // ├─────────────┼────────┼────────┤
    // │ file1.txt   │   ✅    │   0    │
    // │ file2.txt   │   ⚠️    │   1    │  
    // │ file3.txt   │   ❌    │   3    │
    // └─────────────┴────────┴────────┘
    
    println!("TODO: Implement colored table formatting");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_detection() {
        // TODO: Test color capability detection
        // Should respect NO_COLOR, FORCE_COLOR, TERM variables
        todo!("Test color detection logic");
    }
    
    #[test]
    fn test_color_formatting() {
        let formatter = ColorFormatter::new();
        // TODO: Test that formatting works correctly
        todo!("Test color formatting");
    }
    
    #[test]
    fn test_no_color_mode() {
        // TODO: Test that NO_COLOR environment variable disables colors
        todo!("Test NO_COLOR compliance");
    }
}

// COLOR USAGE PRINCIPLES:
//
// FUNCTIONAL COLORS (good):
// ❌ Red: Errors, failures, critical issues
// ⚠️  Yellow: Warnings, cautions, potential issues  
// ✅ Green: Success, completed, good status
// ℹ️  Blue: Information, neutral status
// 🔍 Gray/Dim: Debug info, less important details
//
// DECORATIVE COLORS (avoid):
// 🌈 Random colors just for visual appeal
// 💫 Excessive use that becomes distracting
// 🔥 Colors that don't match their semantic meaning
//
// ACCESSIBILITY CONSIDERATIONS:
// - Don't rely on color alone (use symbols too)
// - Test with color-blind simulators
// - Provide high contrast options
// - Respect NO_COLOR environment variable
// - Consider users with different terminal themes
//
// ENVIRONMENT DETECTION:
// NO_COLOR=1        # Disable all colors
// FORCE_COLOR=1     # Force colors even in pipes
// TERM=dumb         # Usually means no color support
// CI=true           # Continuous integration, usually no colors
//
// C# COMPARISON:
// C#: Console.ForegroundColor = ConsoleColor.Red
// Rust: print!("{}text{}", colors::RED, colors::RESET)
//
// C#: if (Console.IsOutputRedirected) { /* no colors */ }
// Rust: if !io::stdout().is_terminal() { /* no colors */ }
//
// 📊 PROGRESS TRACKER:
// Feature 1 (ANSI colors): [ ] Working
// Feature 2 (Terminal detection): [ ] Working
// Feature 3 (NO_COLOR support): [ ] Working
// Feature 4 (Functional usage): [ ] Working
// Feature 5 (Visual hierarchy): [ ] Working
//
// 🎯 SUCCESS CRITERIA:
// ✅ Errors are clearly red with ❌ symbol
// ✅ Successes are green with ✅ symbol
// ✅ Warnings are yellow with ⚠️ symbol
// ✅ Colors disabled in pipes and with NO_COLOR
// ✅ Good visual hierarchy and readability
// ✅ Functional use of color, not just decoration
//
// 🚀 NEXT CHALLENGE:
// Move to ex07-cross-platform.rs for Windows/Unix compatibility!