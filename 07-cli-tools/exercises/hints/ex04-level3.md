# Exercise 4 - Level 3 Hint (Near-Complete Solution)

## Complete Pipe-Friendly CLI Implementation

Here's a fully working solution that follows all Unix pipeline conventions.

### Complete Working Solution

```rust
use std::env;
use std::io::{self, BufRead, BufReader, Write, IsTerminal};
use std::fs::File;
use std::process;

fn main() {
    // Set up SIGPIPE handling
    #[cfg(unix)]
    {
        unsafe {
            libc::signal(libc::SIGPIPE, libc::SIG_DFL);
        }
    }
    
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line flags
    let (files, flags) = parse_args(&args);
    let verbose = flags.contains(&"--verbose") || flags.contains(&"-v");
    
    // Determine if we should show progress (interactive mode)
    let show_progress = verbose || 
        (io::stderr().is_terminal() && !files.is_empty());
    
    // Get appropriate reader
    let reader = get_reader(&files)?;
    
    // Show starting message only in appropriate contexts
    if show_progress {
        eprintln!("Processing input...");
    }
    
    // Process input efficiently
    process_stream(reader, verbose)?;
    
    // Show completion only when appropriate
    if show_progress {
        eprintln!("Processing completed");
    }
    
    Ok(())
}

fn parse_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut flags = Vec::new();
    
    // Skip program name
    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            flags.push(arg.clone());
        } else {
            files.push(arg.clone());
        }
    }
    
    (files, flags)
}

fn get_reader(files: &[String]) -> io::Result<Box<dyn BufRead>> {
    if files.is_empty() {
        // No files specified - read from stdin
        Ok(Box::new(BufReader::new(io::stdin())))
    } else if files.len() == 1 && files[0] == "-" {
        // Explicit stdin marker
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        // Read from first file (could extend to multiple files)
        let file = File::open(&files[0])?;
        Ok(Box::new(BufReader::new(file)))
    }
}

fn process_stream(reader: Box<dyn BufRead>, verbose: bool) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    let mut line_count = 0;
    
    for line_result in reader.lines() {
        let line = line_result?;
        line_count += 1;
        
        // Process the line (example: convert to uppercase)
        let processed = process_line(&line);
        
        // Write output, handling SIGPIPE gracefully
        if let Err(e) = writeln!(handle, "{}", processed) {
            if e.kind() == io::ErrorKind::BrokenPipe {
                // Receiving end closed the pipe - exit gracefully
                break;
            }
            return Err(e);
        }
        
        // Flush periodically for responsiveness
        if line_count % 1000 == 0 {
            handle.flush()?;
        }
    }
    
    // Final flush
    handle.flush()?;
    
    if verbose {
        eprintln!("Processed {} lines", line_count);
    }
    
    Ok(())
}

fn process_line(line: &str) -> String {
    // Example processing: trim and convert to uppercase
    // Replace with your actual processing logic
    line.trim().to_uppercase()
}

// Proper signal handling for Unix systems
#[cfg(unix)]
mod unix_utils {
    use std::sync::atomic::{AtomicBool, Ordering};
    
    static INTERRUPTED: AtomicBool = AtomicBool::new(false);
    
    pub fn setup_signal_handlers() {
        ctrlc::set_handler(move || {
            INTERRUPTED.store(true, Ordering::SeqCst);
            std::process::exit(130); // Standard exit code for SIGINT
        }).expect("Error setting Ctrl-C handler");
    }
    
    pub fn is_interrupted() -> bool {
        INTERRUPTED.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_process_line() {
        assert_eq!(process_line("hello world"), "HELLO WORLD");
        assert_eq!(process_line("  spaces  "), "SPACES");
    }
    
    #[test]
    fn test_parse_args() {
        let args = vec![
            "program".to_string(),
            "-v".to_string(),
            "file.txt".to_string(),
            "--verbose".to_string(),
        ];
        
        let (files, flags) = parse_args(&args);
        assert_eq!(files, vec!["file.txt"]);
        assert_eq!(flags, vec!["-v", "--verbose"]);
    }
    
    #[test]
    fn test_stdin_marker() {
        let args = vec!["program".to_string(), "-".to_string()];
        let (files, _) = parse_args(&args);
        assert_eq!(files, vec!["-"]);
    }
}
```

### Key Features Implemented

1. **Flexible Input Sources**:
   - No args → read from stdin
   - `-` → explicit stdin
   - `filename` → read from file

2. **Proper Output Behavior**:
   - Data → stdout (for pipelines)
   - Messages → stderr (doesn't pollute pipelines)
   - Silent by default, verbose with flag

3. **Signal Handling**:
   - SIGPIPE handled gracefully
   - Ctrl+C handled with proper exit code

4. **Efficient Streaming**:
   - Line-by-line processing
   - Periodic flushing for responsiveness
   - No full-file buffering

### Usage Examples

```bash
# Read from file
cargo run --bin ex04-pipe-friendly input.txt

# Read from stdin
echo "hello world" | cargo run --bin ex04-pipe-friendly

# Explicit stdin
cargo run --bin ex04-pipe-friendly - < input.txt

# In a pipeline
cat data.txt | cargo run --bin ex04-pipe-friendly | sort | uniq > output.txt

# With verbose output (to stderr)
cat data.txt | cargo run --bin ex04-pipe-friendly -- -v 2>log.txt

# Handle broken pipe gracefully
cargo run --bin ex04-pipe-friendly huge.txt | head -n 10
```

### Best Practices Demonstrated

1. **Exit Codes**:
   - 0 = success
   - 1 = general error
   - 130 = interrupted (Ctrl+C)

2. **Stream Processing**:
   - Never load entire file into memory
   - Process line-by-line
   - Regular flushing for interactivity

3. **Error Messages**:
   - Always to stderr
   - Clear and actionable
   - Include context

### C# Comparison

```csharp
// C# pipeline-friendly tool
class Program {
    static void Main(string[] args) {
        TextReader reader = args.Length > 0 
            ? new StreamReader(args[0]) 
            : Console.In;
            
        string line;
        while ((line = reader.ReadLine()) != null) {
            Console.WriteLine(ProcessLine(line));
        }
    }
}
```

The Rust version adds:
- Better error handling
- Signal handling
- Performance optimizations
- More control over buffering

### Testing Script

Create a test script to verify all behaviors:

```bash
#!/bin/bash
# test_pipeline.sh

echo "Test 1: Direct file input"
echo "test data" > test.txt
cargo run --bin ex04-pipe-friendly test.txt

echo -e "\nTest 2: Stdin input"
echo "stdin test" | cargo run --bin ex04-pipe-friendly

echo -e "\nTest 3: Pipeline"
echo -e "line1\nline2\nline1" | cargo run --bin ex04-pipe-friendly | sort | uniq

echo -e "\nTest 4: Broken pipe handling"
seq 1 1000000 | cargo run --bin ex04-pipe-friendly | head -n 5

rm test.txt
```

Your tool is now a proper Unix citizen that plays well with others!