# Exercise 6 - Level 2 Hint (Specific Solutions)

## Implementing Color Output Step by Step

Let's add professional color support with proper terminal detection.

### 1. Color Detection and Environment Setup

```rust
use std::io::IsTerminal;
use colored::{Colorize, control};

struct ColorConfig {
    enabled: bool,
}

impl ColorConfig {
    fn new() -> Self {
        let enabled = Self::should_use_colors();
        
        // Configure the colored crate
        if !enabled {
            control::set_override(false);
        }
        
        Self { enabled }
    }
    
    fn should_use_colors() -> bool {
        // Check multiple conditions
        std::io::stdout().is_terminal() &&
        std::env::var("NO_COLOR").is_err() &&
        std::env::var("TERM").unwrap_or_default() != "dumb" &&
        !std::env::var("CI").is_ok() // Many CI systems don't handle colors well
    }
}
```

### 2. Colored Output Functions

```rust
fn print_error(message: &str, colors: &ColorConfig) {
    if colors.enabled {
        eprintln!("{} {}", "Error:".red().bold(), message);
    } else {
        eprintln!("Error: {}", message);
    }
}

fn print_success(message: &str, colors: &ColorConfig) {
    if colors.enabled {
        println!("{} {}", "✓".green().bold(), message.green());
    } else {
        println!("✓ {}", message);
    }
}

fn print_warning(message: &str, colors: &ColorConfig) {
    if colors.enabled {
        eprintln!("{} {}", "Warning:".yellow().bold(), message.yellow());
    } else {
        eprintln!("Warning: {}", message);
    }
}

fn print_info(message: &str, colors: &ColorConfig) {
    if colors.enabled {
        println!("{} {}", "Info:".cyan().bold(), message);
    } else {
        println!("Info: {}", message);
    }
}
```

### 3. Data Highlighting

```rust
fn format_file_path(path: &str, colors: &ColorConfig) -> String {
    if colors.enabled {
        path.bright_blue().to_string()
    } else {
        path.to_string()
    }
}

fn format_number(num: u64, colors: &ColorConfig) -> String {
    if colors.enabled {
        num.to_string().bright_magenta().bold().to_string()
    } else {
        num.to_string()
    }
}

fn format_percentage(percent: f64, colors: &ColorConfig) -> String {
    let text = format!("{:.1}%", percent);
    
    if !colors.enabled {
        return text;
    }
    
    // Color based on percentage
    if percent >= 90.0 {
        text.green().bold().to_string()
    } else if percent >= 70.0 {
        text.yellow().to_string()
    } else {
        text.red().to_string()
    }
}
```

### 4. Integration with Main Program

```rust
fn main() {
    let colors = ColorConfig::new();
    
    match run(&colors) {
        Ok(()) => {
            print_success("Operation completed successfully!", &colors);
        },
        Err(e) => {
            print_error(&e.to_string(), &colors);
            std::process::exit(1);
        }
    }
}

fn run(colors: &ColorConfig) -> Result<(), Box<dyn std::error::Error>> {
    let filename = "test.txt";
    
    print_info(&format!("Processing file: {}", 
        format_file_path(filename, colors)), colors);
    
    // Simulate processing
    for i in 1..=100 {
        if i % 25 == 0 {
            println!("Progress: {}", 
                format_percentage(i as f64, colors));
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    Ok(())
}
```

### 5. Manual ANSI Codes (Alternative)

If you prefer not to use the `colored` crate:

```rust
struct AnsiColors;

impl AnsiColors {
    const RED: &'static str = "\x1b[31m";
    const GREEN: &'static str = "\x1b[32m";
    const YELLOW: &'static str = "\x1b[33m";
    const BLUE: &'static str = "\x1b[34m";
    const MAGENTA: &'static str = "\x1b[35m";
    const CYAN: &'static str = "\x1b[36m";
    const BOLD: &'static str = "\x1b[1m";
    const RESET: &'static str = "\x1b[0m";
    
    fn red(text: &str, enabled: bool) -> String {
        if enabled {
            format!("{}{}{}", Self::RED, text, Self::RESET)
        } else {
            text.to_string()
        }
    }
    
    fn green(text: &str, enabled: bool) -> String {
        if enabled {
            format!("{}{}{}", Self::GREEN, text, Self::RESET)
        } else {
            text.to_string()
        }
    }
}
```

### Testing Your Color Implementation

```bash
# Test with colors (interactive terminal)
cargo run --bin ex06-color-output

# Test without colors (redirected)
cargo run --bin ex06-color-output > output.txt
cat output.txt

# Test with NO_COLOR
NO_COLOR=1 cargo run --bin ex06-color-output

# Test in different terminals
TERM=dumb cargo run --bin ex06-color-output
```

### C# Comparison

```csharp
// C# approach
if (!Console.IsOutputRedirected) {
    Console.ForegroundColor = ConsoleColor.Red;
    Console.WriteLine("Error message");
    Console.ResetColor();
}

// Rust with colored crate
if colors.enabled {
    println!("{}", "Error message".red());
}
```

The key is to always check terminal capabilities before applying colors!