# Exercise 4 - Level 2 Hint (Specific Solutions)

## Implementing Pipe-Friendly Behavior

Let's fix the specific issues with code examples.

### 1. Reading from stdin or File

Replace the rigid file-only approach:

```rust
use std::io::{self, BufRead, BufReader};

fn get_reader(args: &[String]) -> io::Result<Box<dyn BufRead>> {
    if args.len() > 1 {
        // Read from file if provided
        let file = File::open(&args[1])?;
        Ok(Box::new(BufReader::new(file)))
    } else {
        // Read from stdin if no file
        Ok(Box::new(BufReader::new(io::stdin())))
    }
}
```

### 2. Silent Operation in Pipelines

Detect if output is going to a terminal:

```rust
use std::io::IsTerminal;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Only show messages when running interactively
    let interactive = io::stdout().is_terminal();
    
    if interactive {
        eprintln!("Starting data processor...");
    }
    
    // Process data...
    
    if interactive {
        eprintln!("Processing completed!");
    }
}
```

### 3. Proper Error Handling for Pipelines

```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let reader = get_reader(&args)?;
    
    // Process lines
    for line in reader.lines() {
        let line = line?;
        
        // Process and output only the data
        let processed = process_line(&line);
        println!("{}", processed);
    }
    
    Ok(())
}
```

### 4. Handle SIGPIPE Properly

When output is piped and the receiving program closes:

```rust
use std::io::{self, Write};

fn write_output(data: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    // This will return Err on SIGPIPE
    if let Err(e) = writeln!(handle, "{}", data) {
        if e.kind() == io::ErrorKind::BrokenPipe {
            // Exit silently on broken pipe
            process::exit(0);
        }
        return Err(e);
    }
    
    Ok(())
}
```

### 5. Efficient Streaming

Don't load entire file into memory:

```rust
// BAD: Loads everything
let contents = std::fs::read_to_string(filename)?;
for line in contents.lines() {
    // process
}

// GOOD: Streams line by line
let reader = BufReader::new(File::open(filename)?);
for line in reader.lines() {
    let line = line?;
    // process immediately
    println!("{}", process_line(&line));
}
```

### Complete Example Structure

```rust
use std::env;
use std::io::{self, BufRead, BufReader, IsTerminal};
use std::fs::File;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&"--verbose".to_string());
    
    // Get input source
    let reader = get_reader(&args)?;
    
    // Only show status in verbose mode or interactive terminal
    if verbose || (io::stderr().is_terminal() && args.len() > 1) {
        eprintln!("Processing input...");
    }
    
    // Process input
    for line in reader.lines() {
        let line = line?;
        let processed = process_line(&line);
        
        // Output only the data
        println!("{}", processed);
    }
    
    Ok(())
}

fn get_reader(args: &[String]) -> io::Result<Box<dyn BufRead>> {
    // Implementation from above
}
```

### Testing Your Pipeline Tool

```bash
# Should work in all these scenarios:

# Direct file input
cargo run --bin ex04-pipe-friendly input.txt

# Stdin input
echo "test data" | cargo run --bin ex04-pipe-friendly

# In a pipeline
cat data.txt | cargo run --bin ex04-pipe-friendly | sort | uniq

# With verbose flag
cat data.txt | cargo run --bin ex04-pipe-friendly -- --verbose
```

### C# Equivalent Patterns

```csharp
// C#: Detecting piped input
if (Console.IsInputRedirected) {
    // Read from Console.In
}

// Rust: Same concept
if !io::stdin().is_terminal() {
    // Read from stdin
}
```

You're getting close! Focus on making the tool completely silent during normal operation, only outputting the processed data.