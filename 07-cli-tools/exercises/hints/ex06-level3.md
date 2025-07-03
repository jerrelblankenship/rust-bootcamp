# Exercise 6 - Level 3 Hint (Near-Complete Solution)

## Professional Color Output Implementation

Here's a complete solution with advanced color features and robust terminal detection.

```rust
use std::env;
use std::io::{self, IsTerminal, Write};
use colored::{Colorize, control, Color};

// Complete color configuration system
struct ColorTheme {
    enabled: bool,
    error_color: Color,
    success_color: Color,
    warning_color: Color,
    info_color: Color,
    highlight_color: Color,
}

impl ColorTheme {
    fn new() -> Self {
        let enabled = Self::detect_color_support();
        
        if !enabled {
            control::set_override(false);
        }
        
        Self {
            enabled,
            error_color: Color::Red,
            success_color: Color::Green,
            warning_color: Color::Yellow,
            info_color: Color::Cyan,
            highlight_color: Color::BrightBlue,
        }
    }
    
    fn detect_color_support() -> bool {
        // Comprehensive color detection
        let has_term = std::io::stdout().is_terminal() && std::io::stderr().is_terminal();
        let no_color = std::env::var("NO_COLOR").is_err();
        let term_supports = Self::term_supports_color();
        let not_ci = !Self::is_ci_environment();
        
        has_term && no_color && term_supports && not_ci
    }
    
    fn term_supports_color() -> bool {\n        match std::env::var("TERM").as_deref() {\n            Ok("dumb") | Ok("") => false,\n            Ok(term) => {\n                // Check for common color-supporting terminals\n                term.contains("color") ||\n                term.contains("xterm") ||\n                term.contains("screen") ||\n                term.contains("tmux") ||\n                term == "ansi"\n            },\n            Err(_) => {\n                // On Windows, check for newer terminals\n                cfg!(windows) && Self::windows_supports_color()\n            }\n        }\n    }\n    \n    #[cfg(windows)]\n    fn windows_supports_color() -> bool {\n        // Windows 10 version 1511+ supports ANSI colors\n        std::env::var("WT_SESSION\").is_ok() || // Windows Terminal\n        std::env::var("ConEmuPID\").is_ok() ||  // ConEmu\n        std::env::var("TERM_PROGRAM\").is_ok()  // VS Code terminal\n    }\n    \n    #[cfg(not(windows))]\n    fn windows_supports_color() -> bool { false }\n    \n    fn is_ci_environment() -> bool {\n        // Common CI environment variables\n        std::env::var("CI\").is_ok() ||\n        std::env::var("CONTINUOUS_INTEGRATION\").is_ok() ||\n        std::env::var("GITHUB_ACTIONS\").is_ok() ||\n        std::env::var("TRAVIS\").is_ok() ||\n        std::env::var("JENKINS\").is_ok()\n    }\n}\n\n// Enhanced output functions with icons and formatting\nimpl ColorTheme {\n    fn error(&self, message: &str) {\n        let prefix = if self.enabled {\n            \"❌\".red().bold()\n        } else {\n            \"Error:\".normal()\n        };\n        \n        let msg = if self.enabled {\n            message.color(self.error_color)\n        } else {\n            message.normal()\n        };\n        \n        eprintln!(\"{} {}\", prefix, msg);\n    }\n    \n    fn success(&self, message: &str) {\n        let prefix = if self.enabled {\n            \"✅\".green().bold()\n        } else {\n            \"Success:\".normal()\n        };\n        \n        let msg = if self.enabled {\n            message.color(self.success_color)\n        } else {\n            message.normal()\n        };\n        \n        println!(\"{} {}\", prefix, msg);\n    }\n    \n    fn warning(&self, message: &str) {\n        let prefix = if self.enabled {\n            \"⚠️\".yellow().bold()\n        } else {\n            \"Warning:\".normal()\n        };\n        \n        let msg = if self.enabled {\n            message.color(self.warning_color)\n        } else {\n            message.normal()\n        };\n        \n        eprintln!(\"{} {}\", prefix, msg);\n    }\n    \n    fn info(&self, message: &str) {\n        let prefix = if self.enabled {\n            \"ℹ️\".cyan()\n        } else {\n            \"Info:\".normal()\n        };\n        \n        let msg = if self.enabled {\n            message.color(self.info_color)\n        } else {\n            message.normal()\n        };\n        \n        println!(\"{} {}\", prefix, msg);\n    }\n    \n    fn highlight(&self, text: &str) -> String {\n        if self.enabled {\n            text.color(self.highlight_color).bold().to_string()\n        } else {\n            format!(\"'{}'\" , text) // Use quotes for emphasis without color\n        }\n    }\n    \n    fn progress_bar(&self, filled: usize, total: usize, width: usize) -> String {\n        let fill_width = (filled * width) / total.max(1);\n        \n        if self.enabled {\n            let filled_part: String = \"█\".repeat(fill_width).green().to_string();\n            let empty_part: String = \"░\".repeat(width - fill_width).bright_black().to_string();\n            format!(\"[{}{}]\", filled_part, empty_part)\n        } else {\n            let filled_part = \"#\".repeat(fill_width);\n            let empty_part = \"-\".repeat(width - fill_width);\n            format!(\"[{}{}]\", filled_part, empty_part)\n        }\n    }\n}\n\n// Main application with comprehensive color usage\nfn main() {\n    let theme = ColorTheme::new();\n    \n    // Display color status\n    if theme.enabled {\n        theme.info(\"Color output enabled\");\n    } else {\n        println!(\"Color output disabled (terminal doesn't support colors or NO_COLOR set)\");\n    }\n    \n    match run(&theme) {\n        Ok(()) => {\n            theme.success(\"All operations completed successfully!\");\n        },\n        Err(e) => {\n            theme.error(&format!(\"Operation failed: {}\", e));\n            std::process::exit(1);\n        }\n    }\n}\n\nfn run(theme: &ColorTheme) -> Result<(), Box<dyn std::error::Error>> {\n    let filename = \"example.txt\";\n    \n    theme.info(&format!(\"Processing file: {}\", theme.highlight(filename)));\n    \n    // Simulate file processing with progress\n    let total_steps = 50;\n    for step in 1..=total_steps {\n        // Simulate work\n        std::thread::sleep(std::time::Duration::from_millis(100));\n        \n        // Show progress every 10 steps\n        if step % 10 == 0 {\n            let percentage = (step * 100) / total_steps;\n            let bar = theme.progress_bar(step, total_steps, 20);\n            \n            print!(\"\\rProgress: {} {}%\", bar, percentage);\n            io::stdout().flush()?;\n        }\n    }\n    \n    println!(); // New line after progress\n    \n    // Demonstrate different message types\n    theme.info(\"File validation started\");\n    \n    if std::env::args().any(|arg| arg == \"--warn\") {\n        theme.warning(\"Some non-critical issues found\");\n    }\n    \n    // Simulate potential error\n    if std::env::args().any(|arg| arg == \"--error\") {\n        theme.error(\"Critical error encountered\");\n        return Err(\"Simulated error\".into());\n    }\n    \n    theme.success(&format!(\"Processed {} successfully\", theme.highlight(filename)));\n    \n    Ok(())\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n    \n    #[test]\n    fn test_color_detection() {\n        // Test in various environments\n        std::env::set_var(\"NO_COLOR\", \"1\");\n        let theme = ColorTheme::new();\n        assert!(!theme.enabled);\n        \n        std::env::remove_var(\"NO_COLOR\");\n        std::env::set_var(\"TERM\", \"dumb\");\n        let theme = ColorTheme::new();\n        assert!(!theme.enabled);\n        \n        std::env::set_var(\"TERM\", \"xterm-256color\");\n        // Note: This test might still fail if stdout is not a TTY\n    }\n    \n    #[test]\n    fn test_progress_bar() {\n        let theme = ColorTheme::new();\n        let bar = theme.progress_bar(25, 100, 20);\n        // Should contain progress indication (either colored or plain)\n        assert!(bar.contains('[') && bar.contains(']'));\n    }\n}\n```\n\n### Testing Script\n\n```bash\n#!/bin/bash\n# test_colors.sh\n\necho \"=== Testing Color Output ===\"\n\necho \"\\n1. Normal execution (colors if supported):\"\ncargo run --bin ex06-color-output\n\necho \"\\n2. With NO_COLOR environment variable:\"\nNO_COLOR=1 cargo run --bin ex06-color-output\n\necho \"\\n3. With redirected output (no colors):\"\ncargo run --bin ex06-color-output > /tmp/output.txt 2>&1\necho \"Output saved to file (no ANSI codes):\"\ncat /tmp/output.txt\n\necho \"\\n4. Testing warning flag:\"\ncargo run --bin ex06-color-output -- --warn\n\necho \"\\n5. Testing error simulation:\"\ncargo run --bin ex06-color-output -- --error || echo \"Exit code: $?\"\n\necho \"\\n6. In dumb terminal:\"\nTERM=dumb cargo run --bin ex06-color-output\n\nrm -f /tmp/output.txt\n```\n\nThis implementation provides professional-grade color support that works across different terminals and respects user preferences!