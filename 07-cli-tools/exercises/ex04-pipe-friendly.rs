// Exercise 4: Pipe-Friendly CLI - Fix Unix Pipeline Integration!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 pipe features to fix)
//
// Your task: Make this CLI tool work properly in Unix pipelines and scripts
// This demonstrates proper stdin/stdout handling and pipeline etiquette
//
// INSTRUCTIONS:
// 1. Test the tool: `echo "test data" | cargo run --bin ex04-pipe-friendly`
// 2. Fix broken stdin reading and stdout formatting
// 3. Test in pipeline: `cat file.txt | program | sort | uniq`
// 4. Ensure tool respects Unix pipeline conventions
//
// LEARNING STRATEGY:
// - Understand stdin vs file input patterns
// - Fix stdout to be pipeline-friendly (no extra output)
// - Handle signals properly (SIGPIPE, Ctrl+C)
// - Add proper exit codes for scripts
//
// PIPELINE FEATURES TO FIX:
// [] Read from stdin when no file specified
// [] Silent operation for pipeline use
// [] Proper exit codes for scripting
// [] Handle SIGPIPE gracefully
// [] Efficient streaming (don't load everything to memory)

use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::process;

fn main() {
    // TERRIBLE: Always prints verbose messages even in pipelines
    println!("Starting data processor...");  // FIXME: This breaks pipelines!
    println!("Initializing...");             // FIXME: Should be silent in pipes!
    
    let args: Vec<String> = env::args().collect();
    
    // TERRIBLE: Doesn't handle stdin input
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);  // FIXME: Should support stdin!
        process::exit(1);
    }
    
    let filename = &args[1];
    
    // TERRIBLE: Only reads from files, never stdin
    match process_file(filename) {
        Ok(()) => {
            println!("Processing completed successfully!");  // FIXME: Breaks pipelines!
            process::exit(0);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

// FIXME: This function only works with files
fn process_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    // TERRIBLE: Loads everything into memory
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    
    // TERRIBLE: Chatty output during processing
    eprintln!("Processing {} lines...", lines.len());  // FIXME: Should be quiet!
    
    // Process and output
    for (i, line) in lines.iter().enumerate() {
        // TERRIBLE: Progress updates break pipeline output
        if i % 100 == 0 {
            eprintln!("Processed {} lines...", i);  // FIXME: Not pipeline-friendly!
        }
        
        // The actual processing (just uppercase for demo)
        println!("{}", line.to_uppercase());
    }
    
    Ok(())
}

// TODO: Create pipe-aware input reading
fn create_input_reader() -> io::Result<Box<dyn BufRead>> {
    // TODO: Detect if input is from stdin or file
    // HINT: Similar to checking Console.IsInputRedirected in C#
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        // TODO: Read from stdin for pipeline use
        // Example: `cat file.txt | program`
        todo!("Create stdin reader")
    } else {
        // TODO: Read from specified file
        // Example: `program file.txt`
        todo!("Create file reader")
    }
}

// TODO: Create pipe-aware processing
fn process_stream<R: BufRead>(reader: R, quiet: bool) -> io::Result<()> {
    // TODO: Stream processing instead of loading everything to memory
    // This is crucial for large inputs in pipelines
    
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        
        // TODO: Only show progress if NOT in a pipeline
        if !quiet && line_num % 1000 == 0 {
            eprintln!("Processed {} lines...", line_num);
        }
        
        // Process the line (example: convert to uppercase)
        let processed = line.to_uppercase();
        
        // IMPORTANT: Only output the processed data, no extra formatting
        println!("{}", processed);
        
        // TODO: Handle SIGPIPE properly
        // When downstream command exits (like `head -10`), we should exit gracefully
        if let Err(e) = io::stdout().flush() {
            if e.kind() == io::ErrorKind::BrokenPipe {
                // SIGPIPE received - downstream closed, exit gracefully
                process::exit(0);
            }
            return Err(e);
        }
    }
    
    Ok(())
}

// TODO: Detect if running in a pipeline
fn is_pipe_mode() -> bool {
    // TODO: Check if stdout is connected to a terminal or redirected
    // HINT: Use isatty/atty crate or check if stdin/stdout are TTY
    
    // For now, use simple heuristic
    use std::io::IsTerminal;
    !io::stdout().is_terminal()
}

// TODO: Handle command line arguments properly for pipes
fn parse_args() -> (Option<String>, bool) {
    let args: Vec<String> = env::args().collect();
    
    // TODO: Support multiple argument patterns:
    // program                    # Read from stdin
    // program file.txt          # Read from file  
    // program -q file.txt       # Quiet mode
    // program --quiet           # Quiet mode from stdin
    
    let quiet = is_pipe_mode() || args.contains(&"--quiet".to_string()) || args.contains(&"-q".to_string());
    
    // TODO: Parse filename or use stdin
    let filename = if args.len() > 1 && !args[1].starts_with('-') {
        Some(args[1].clone())
    } else {
        None
    };
    
    (filename, quiet)
}

// TODO: Implement proper main with pipeline support
fn main_fixed() -> io::Result<()> {
    let (filename, quiet) = parse_args();
    
    // Create appropriate input reader
    let reader: Box<dyn BufRead> = match filename {
        Some(file) => {
            let f = File::open(&file)?;
            Box::new(BufReader::new(f))
        },
        None => {
            // Read from stdin
            let stdin = io::stdin();
            Box::new(stdin.lock())
        }
    };
    
    // Process the stream
    process_stream(reader, quiet)?;
    
    Ok(())
}

// TODO: Handle Unix signals properly
fn setup_signal_handlers() {
    // TODO: Handle SIGPIPE (broken pipe) gracefully
    // When downstream command exits early (like `head -10`), 
    // this program should exit cleanly without error
    
    // HINT: Use signal-hook crate or manual signal handling
    // In C#, this would be like handling Console.CancelKeyPress
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_stream_processing() {
        let input = "hello\nworld\ntest";
        let cursor = Cursor::new(input);
        
        // TODO: Test that processing works correctly
        // Should output: HELLO\nWORLD\nTEST
        todo!("Test stream processing");
    }
    
    #[test]
    fn test_pipe_detection() {
        // TODO: Test pipe mode detection
        todo!("Test pipeline detection");
    }
    
    #[test]
    fn test_quiet_mode() {
        // TODO: Test that quiet mode suppresses progress messages
        todo!("Test quiet operation");
    }
}

// UNIX PIPELINE EXAMPLES:
//
// GOOD BEHAVIOR (what we're fixing towards):
// $ echo "test" | program                    # Reads from stdin
// $ cat file.txt | program | sort           # Quiet processing  
// $ program file.txt | head -10             # Handles SIGPIPE gracefully
// $ program --quiet large.txt > output.txt  # No progress messages
//
// BAD BEHAVIOR (current broken state):
// $ echo "test" | program                    # ERROR: "Usage: program <filename>"
// $ cat file.txt | program | sort           # Mixes progress messages with data
// $ program file.txt | head -10             # Crashes with "Broken pipe"
// $ program large.txt > output.txt          # Progress messages pollute output
//
// PIPELINE ETIQUETTE RULES:
// 1. Read from stdin if no file specified
// 2. Only output processed data to stdout  
// 3. Send messages/progress to stderr
// 4. Exit cleanly on SIGPIPE
// 5. Use proper exit codes (0=success, 1=error)
// 6. Be efficient with memory for large inputs
//
// C# COMPARISON:
// C#: Console.IsInputRedirected, Console.IsOutputRedirected
// Rust: std::io::IsTerminal trait, atty crate
//
// C#: Process.StandardInput.ReadLine()
// Rust: io::stdin().lock().lines()
//
// C#: Environment.Exit(exitCode)  
// Rust: process::exit(code)
//
// 📊 PROGRESS TRACKER:
// Feature 1 (Stdin support): [ ] Working
// Feature 2 (Quiet operation): [ ] Working  
// Feature 3 (Streaming): [ ] Working
// Feature 4 (SIGPIPE handling): [ ] Working
// Feature 5 (Proper exit codes): [ ] Working
//
// 🎯 SUCCESS CRITERIA:
// ✅ Works in: `echo "test" | program`
// ✅ Works in: `cat big.txt | program | sort`
// ✅ Works in: `program file.txt | head -10`
// ✅ No extra output in pipelines
// ✅ Efficient memory usage for large files
// ✅ Graceful handling of broken pipes
//
// 🚀 NEXT CHALLENGE:
// Move to ex05-progress-bars.rs for user feedback!