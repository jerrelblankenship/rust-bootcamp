# Exercise 5 - Level 2 Hint (Specific Solutions)

## Implementing Progress Bars Step by Step

Let's build proper progress indication with code examples.

### 1. Basic Progress Structure

```rust
use std::time::Instant;
use std::io::{self, IsTerminal, Write};

struct ProgressBar {
    total: u64,
    current: u64,
    start_time: Instant,
    last_update: Instant,
    is_terminal: bool,
}

impl ProgressBar {
    fn new(total: u64) -> Self {
        Self {
            total,
            current: 0,
            start_time: Instant::now(),
            last_update: Instant::now(),
            is_terminal: io::stderr().is_terminal(),
        }
    }
}
```

### 2. Progress Detection and Updates

```rust
impl ProgressBar {
    fn update(&mut self, current: u64) {
        self.current = current;
        
        // Only update if enough time has passed (avoid flickering)
        let now = Instant::now();
        if now.duration_since(self.last_update).as_millis() < 100 {
            return;
        }
        
        if self.is_terminal {
            self.render();
        }
        
        self.last_update = now;
    }
    
    fn render(&self) {
        if self.total == 0 {
            return;
        }
        
        let percentage = (self.current * 100) / self.total;
        let bar_width = 40;
        let filled = (self.current * bar_width) / self.total;
        
        // Create visual bar
        let bar: String = (0..bar_width)
            .map(|i| if i < filled { '█' } else { '░' })
            .collect();
        
        // Calculate rate and ETA
        let elapsed = self.start_time.elapsed();
        let rate = if elapsed.as_secs() > 0 {
            self.current as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };
        
        let eta = if rate > 0.0 {
            let remaining = self.total - self.current;
            format!("ETA: {}", format_duration(remaining as f64 / rate))
        } else {
            "ETA: --".to_string()
        };
        
        // Print progress (to stderr, not stdout)
        eprint!("\r[{}] {}% {}/{} ({}/s, {})", 
            bar, percentage, 
            format_bytes(self.current), 
            format_bytes(self.total),
            format_bytes(rate as u64),
            eta);
        
        // Flush to ensure immediate display
        let _ = io::stderr().flush();
    }
    
    fn finish(&mut self) {
        if self.is_terminal {
            eprintln!("\r{}", " ".repeat(80)); // Clear line
            eprintln!("✅ Complete! Processed {} in {}", 
                format_bytes(self.total),
                format_duration(self.start_time.elapsed().as_secs_f64()));
        }
    }
}
```

### 3. Helper Functions for Formatting

```rust
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn format_duration(seconds: f64) -> String {
    if seconds >= 3600.0 {
        let hours = (seconds / 3600.0) as u32;
        let minutes = ((seconds % 3600.0) / 60.0) as u32;
        format!("{}h {}m", hours, minutes)
    } else if seconds >= 60.0 {
        let minutes = (seconds / 60.0) as u32;
        let secs = (seconds % 60.0) as u32;
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", seconds as u32)
    }
}
```

### 4. Integration with File Processing

```rust
fn process_file_with_progress(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let file_size = metadata(filename)?.len();
    let reader = BufReader::new(file);
    
    // Create progress bar only for interactive terminals
    let mut progress = ProgressBar::new(file_size);
    
    let mut bytes_processed = 0u64;
    
    for line_result in reader.lines() {
        let line = line_result?;
        
        // Simulate processing work
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Update progress
        bytes_processed += line.len() as u64 + 1; // +1 for newline
        progress.update(bytes_processed);
        
        // Do actual work
        validate_line(&line)?;
    }
    
    progress.finish();
    Ok(())
}
```

### 5. Using the indicatif Library (Alternative)

If you want to use a professional library:

```rust
use indicatif::{ProgressBar, ProgressStyle};

fn process_with_indicatif(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let file_size = metadata(filename)?.len();
    let reader = BufReader::new(file);
    
    // Create progress bar
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {bytes_per_sec} ETA: {eta}")
        .unwrap()
        .progress_chars("█░"));
    
    let mut bytes_processed = 0u64;
    
    for line_result in reader.lines() {
        let line = line_result?;
        
        // Process line
        validate_line(&line)?;
        
        // Update progress
        bytes_processed += line.len() as u64 + 1;
        pb.set_position(bytes_processed);
    }
    
    pb.finish_with_message("Processing complete!");
    Ok(())
}
```

### 6. Smart Terminal Detection

```rust
fn should_show_progress() -> bool {
    use std::io::IsTerminal;
    
    // Show progress only if:
    // - stderr is a terminal (not redirected)
    // - Not in a pipe or redirect
    io::stderr().is_terminal() && 
    !std::env::var("NO_COLOR").is_ok() // Respect NO_COLOR env var
}
```

### Testing Your Progress Bar

```bash
# Test with different scenarios:

# Interactive terminal (should show progress)
cargo run --bin ex05-progress-bars large-file.txt

# Redirected output (should not show progress)
cargo run --bin ex05-progress-bars large-file.txt > output.txt

# Piped output (should not show progress) 
cargo run --bin ex05-progress-bars large-file.txt | cat

# Set NO_COLOR to disable progress
NO_COLOR=1 cargo run --bin ex05-progress-bars large-file.txt
```

### C# Comparison

```csharp
// C# progress reporting
var progress = new Progress<int>(percent => {
    if (!Console.IsOutputRedirected) {
        Console.Write($"\rProgress: {percent}%");
    }
});

// Rust equivalent
let mut progress = ProgressBar::new(total);
progress.update(current);
```

The key insight is that progress information goes to stderr, not stdout, so it doesn't interfere with the actual program output!