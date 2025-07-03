// Exercise 5: Progress Bars - Fix the Missing User Feedback!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (4 progress features to fix)
//
// Your task: Add proper progress indication for long-running operations
// This demonstrates user experience design and responsive CLI interfaces
//
// INSTRUCTIONS:
// 1. Run program to see it process with no feedback (bad UX!)
// 2. Add progress bars for file operations
// 3. Test: `cargo run --bin ex05-progress-bars -- large-file.txt`
// 4. Make progress responsive and informative
//
// LEARNING STRATEGY:
// - Start with basic progress indication
// - Add estimated time remaining
// - Handle different terminal capabilities
// - Make progress bars disappear in non-interactive mode
//
// PROGRESS FEATURES TO FIX:
// [] Add basic progress bar for file processing
// [] Show percentage, rate, and ETA
// [] Hide progress in non-interactive terminals
// [] Handle terminal resize gracefully

use std::env;
use std::fs::{File, metadata};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    // TERRIBLE: No feedback during long operations
    println!("Starting to process {}...", filename);
    
    match process_file_with_progress(filename) {
        Ok(()) => println!("✅ Processing completed!"),
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

// FIXME: This function gives no progress feedback
fn process_file_with_progress(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let file_size = metadata(filename)?.len();
    let reader = BufReader::new(file);
    
    println!("Processing {} bytes...", file_size);
    
    // TERRIBLE: Just processes silently with no progress indication
    let mut processed_lines = 0;
    let mut bytes_processed = 0u64;
    
    for line_result in reader.lines() {
        let line = line_result?;
        
        // Simulate some processing work
        thread::sleep(Duration::from_millis(10));
        
        // Count progress
        bytes_processed += line.len() as u64 + 1; // +1 for newline
        processed_lines += 1;
        
        // TERRIBLE: No visual progress indication!
        if processed_lines % 100 == 0 {
            // Just print dots - not helpful!
            print!(".");  // FIXME: This is not a good progress indicator!
            io::stdout().flush().unwrap();
        }
        
        // Do the actual work (example: validate line format)
        validate_line(&line)?;
    }
    
    println!(); // New line after dots
    println!("Processed {} lines", processed_lines);
    
    Ok(())
}

// TODO: Create a proper progress bar structure
struct ProgressBar {
    // TODO: Track progress state
    // total: u64,
    // current: u64, 
    // start_time: Instant,
    // last_update: Instant,
    // is_terminal: bool,
}

impl ProgressBar {
    // TODO: Create new progress bar
    fn new(total: u64) -> Self {
        // TODO: Initialize progress tracking
        // Detect if output is a terminal (don't show progress in pipes)
        todo!("Initialize progress bar")
    }
    
    // TODO: Update progress
    fn update(&mut self, current: u64) {
        // TODO: Update progress and redraw if enough time passed
        // Show: [████████░░░░░░] 45% (1.2MB/s, ETA: 2m 30s)
        todo!("Update progress display")
    }
    
    // TODO: Finish progress bar
    fn finish(&mut self) {
        // TODO: Clean up progress bar and show completion
        todo!("Finish progress bar")
    }
    
    // TODO: Render progress bar
    fn render(&self) {
        // TODO: Draw progress bar with:
        // - Percentage complete
        // - Visual bar with blocks
        // - Current/total values  
        // - Processing rate
        // - Estimated time remaining
        
        // Example: [████████░░░░░░] 45% 450MB/1GB (1.2MB/s, ETA: 2m 30s)
        todo!("Render progress bar")
    }
}

// TODO: Detect if we should show progress
fn should_show_progress() -> bool {
    // TODO: Only show progress bars in interactive terminals
    // Don't show in:
    // - Pipes: program | other_command
    // - Redirects: program > file.txt
    // - Non-TTY environments
    
    // HINT: Use isatty/atty crate or std::io::IsTerminal
    use std::io::IsTerminal;
    io::stderr().is_terminal()
}

// TODO: Calculate processing rate
fn calculate_rate(bytes: u64, duration: Duration) -> f64 {
    // TODO: Calculate bytes per second
    // Handle edge cases (zero duration, etc.)
    if duration.as_secs_f64() > 0.0 {
        bytes as f64 / duration.as_secs_f64()
    } else {
        0.0
    }
}

// TODO: Format rate for display  
fn format_rate(bytes_per_sec: f64) -> String {
    // TODO: Format as human-readable rate
    // Examples: "1.2 MB/s", "45.3 KB/s", "2.1 GB/s"
    if bytes_per_sec >= 1_000_000_000.0 {
        format!("{:.1} GB/s", bytes_per_sec / 1_000_000_000.0)
    } else if bytes_per_sec >= 1_000_000.0 {
        format!("{:.1} MB/s", bytes_per_sec / 1_000_000.0)
    } else if bytes_per_sec >= 1_000.0 {
        format!("{:.1} KB/s", bytes_per_sec / 1_000.0)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

// TODO: Calculate and format ETA
fn format_eta(remaining_bytes: u64, bytes_per_sec: f64) -> String {
    // TODO: Calculate estimated time remaining
    // Format as: "2m 30s", "45s", "1h 15m"
    
    if bytes_per_sec <= 0.0 {
        return "∞".to_string();
    }
    
    let remaining_seconds = remaining_bytes as f64 / bytes_per_sec;
    
    if remaining_seconds >= 3600.0 {
        let hours = (remaining_seconds / 3600.0) as u32;
        let minutes = ((remaining_seconds % 3600.0) / 60.0) as u32;
        format!("{}h {}m", hours, minutes)
    } else if remaining_seconds >= 60.0 {
        let minutes = (remaining_seconds / 60.0) as u32;
        let seconds = (remaining_seconds % 60.0) as u32;
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", remaining_seconds as u32)
    }
}

// TODO: Improved processing with real progress
fn process_file_improved(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let file_size = metadata(filename)?.len();
    let reader = BufReader::new(file);
    
    // TODO: Create progress bar if in interactive terminal
    let mut progress = if should_show_progress() {
        Some(ProgressBar::new(file_size))
    } else {
        None
    };
    
    let mut bytes_processed = 0u64;
    let mut processed_lines = 0;
    
    for line_result in reader.lines() {
        let line = line_result?;
        
        // Simulate processing work
        thread::sleep(Duration::from_millis(5));
        
        // Update progress
        bytes_processed += line.len() as u64 + 1;
        processed_lines += 1;
        
        // TODO: Update progress bar
        if let Some(ref mut pb) = progress {
            pb.update(bytes_processed);
        }
        
        // Do actual work
        validate_line(&line)?;
    }
    
    // TODO: Finish progress bar
    if let Some(ref mut pb) = progress {
        pb.finish();
    }
    
    Ok(())
}

// Example processing function
fn validate_line(line: &str) -> io::Result<()> {
    // Example validation: check line isn't too long
    if line.len() > 1000 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Line too long: {} characters", line.len())
        ));
    }
    Ok(())
}

// TODO: Handle terminal resize
fn get_terminal_width() -> usize {
    // TODO: Get current terminal width for responsive progress bars
    // Default to 80 if can't determine
    
    // HINT: Use terminal_size crate or ioctl calls
    80
}

// TODO: Create different progress bar styles
enum ProgressStyle {
    Bar,        // [████████░░░░░░] 
    Dots,       // ................
    Spinner,    // |/-\
    Percentage, // 45%
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_calculation() {
        let rate = calculate_rate(1_000_000, Duration::from_secs(1));
        assert_eq!(rate, 1_000_000.0);
    }
    
    #[test]
    fn test_rate_formatting() {
        assert_eq!(format_rate(1_500_000.0), "1.5 MB/s");
        assert_eq!(format_rate(500.0), "500 B/s");
    }
    
    #[test]
    fn test_eta_formatting() {
        // TODO: Test ETA calculations
        todo!("Test ETA formatting");
    }
    
    #[test]
    fn test_progress_bar() {
        // TODO: Test progress bar updates
        todo!("Test progress bar functionality");
    }
}

// PROGRESS BAR EXAMPLES:
//
// GOOD PROGRESS (what we're building):
// Processing large-file.txt...
// [████████████░░░░░░░░] 60% 1.2GB/2.0GB (15.3 MB/s, ETA: 52s)
//
// Multiple operations:
// Step 1/3: Reading file...     [██████████] 100% (2.1s)
// Step 2/3: Processing data...  [███░░░░░░░] 30% (45.2 KB/s, ETA: 12s)
//
// BAD PROGRESS (current broken state):
// Processing large-file.txt...
// .....................................................
// (no indication of progress, rate, or time remaining)
//
// PROGRESS BAR COMPONENTS:
// [████████░░░░░░░░] - Visual bar (filled/empty blocks)
// 60%              - Percentage complete
// 1.2GB/2.0GB      - Current/total values
// (15.3 MB/s)      - Processing rate
// ETA: 52s         - Estimated time remaining
//
// RESPONSIVE BEHAVIOR:
// - Show progress only in interactive terminals
// - Hide in pipes: program | other_command
// - Adjust width to terminal size
// - Update at reasonable frequency (not too fast/slow)
//
// C# COMPARISON:
// C#: Console.SetCursorPosition() for updating in place
// Rust: \r and ANSI escape codes for cursor control
//
// C#: Console.IsOutputRedirected to detect non-interactive
// Rust: std::io::IsTerminal trait
//
// 📊 PROGRESS TRACKER:
// Feature 1 (Basic progress bar): [ ] Working
// Feature 2 (Rate calculation): [ ] Working
// Feature 3 (ETA estimation): [ ] Working  
// Feature 4 (Terminal detection): [ ] Working
//
// 🎯 SUCCESS CRITERIA:
// ✅ Shows visual progress during long operations
// ✅ Displays rate and ETA information
// ✅ Adapts to terminal width
// ✅ Hidden in non-interactive mode
// ✅ Updates smoothly without flickering
//
// 🚀 NEXT CHALLENGE:
// Move to ex06-color-output.rs for terminal styling!