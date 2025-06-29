// Memory Visualizer - Incomplete Implementation
//
// A command-line tool that demonstrates Rust's ownership model.
// 
// YOUR TASK: Complete the missing implementations marked with TODO/FIXME
// This project demonstrates all concepts from Module 02.

use clap::{Parser, Subcommand};
use colored::*;

mod memory_tracker;
mod ownership_demo;
mod borrowing_demo;
mod smart_pointers;
mod visualizer;

use memory_tracker::MemoryTracker;

#[derive(Parser)]
#[command(name = "memory-visualizer")]
#[command(about = "A Rust memory management visualizer")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Demonstrate ownership transfer
    Ownership,
    /// Show borrowing and references
    Borrowing,
    /// Explore smart pointers (Box, Rc, Arc)
    SmartPointers,
    /// Compare with C# memory model
    CompareCsharp,
    /// Interactive exploration mode
    Interactive,
}

fn main() {
    let cli = Cli::parse();
    
    // TODO: Create a MemoryTracker instance
    // HINT: Look at the MemoryTracker::new() method
    let mut tracker = todo!("Create MemoryTracker instance");
    
    match cli.command {
        Some(Commands::Ownership) => {
            println!("{}", "=== Ownership Demonstration ===".bold().blue());
            ownership_demo::run_demonstrations(&mut tracker);
        }
        Some(Commands::Borrowing) => {
            println!("{}", "=== Borrowing Demonstration ===".bold().green());
            borrowing_demo::run_demonstrations(&mut tracker);
        }
        Some(Commands::SmartPointers) => {
            println!("{}", "=== Smart Pointers Demonstration ===".bold().purple());
            smart_pointers::run_demonstrations(&mut tracker);
        }
        Some(Commands::CompareCsharp) => {
            println!("{}", "=== C# vs Rust Comparison ===".bold().yellow());
            compare_with_csharp();
        }
        Some(Commands::Interactive) => {
            println!("{}", "=== Interactive Mode ===".bold().cyan());
            interactive_mode(&mut tracker);
        }
        None => {
            // Default: show a quick overview of all concepts
            println!("{}", "🧠 Memory Visualizer - Rust Ownership Demo".bold());
            println!("Run with --help to see available commands\n");
            
            println!("Quick overview:");
            ownership_demo::show_basic_example();
        }
    }
    
    if cli.verbose {
        println!("\n{}", "=== Memory Operations Summary ===".bold());
        tracker.print_summary();
    }
}

fn compare_with_csharp() {
    println!("{}",
        "Memory Management: C# vs Rust".underline()
    );
    
    println!("\n{}", "C# Approach (Garbage Collection):".bold().yellow());
    println!("```csharp");
    println!("var obj1 = new MyObject();");
    println!("var obj2 = obj1;  // Both reference same object");
    println!("obj2.Modify();    // obj1 is also modified!");
    println!("// GC collects when no references remain");
    println!("```");
    
    println!("\n{}", "Rust Approach (Ownership):".bold().green());
    println!("```rust");
    println!("let obj1 = MyStruct::new();");
    println!("let obj2 = obj1;  // Ownership MOVES to obj2");
    println!("// obj1 is no longer valid!");
    println!("// obj2 is dropped deterministically");
    println!("```");
    
    println!("\n{}", "Key Differences:".bold());
    println!("• C#: Multiple references to same object, unpredictable cleanup");
    println!("• Rust: Single owner, predictable cleanup, no shared mutation");
    
    // TODO: Add more detailed comparison
    // TASK: Expand this function to show more examples
    // Consider: null references, memory leaks, data races
}

fn interactive_mode(tracker: &mut MemoryTracker) {
    use std::io::{self, Write};
    
    println!("Interactive Memory Explorer");
    println!("Commands: ownership, borrowing, smart-pointers, summary, quit");
    println!();
    
    loop {
        print!("memory> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                match input {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye! 👋");
                        break;
                    }
                    "ownership" => {
                        ownership_demo::interactive_demo(tracker);
                    }
                    "borrowing" => {
                        borrowing_demo::interactive_demo(tracker);
                    }
                    "smart-pointers" => {
                        smart_pointers::interactive_demo(tracker);
                    }
                    "summary" => {
                        tracker.print_summary();
                    }
                    "help" => {
                        print_interactive_help();
                    }
                    "" => continue,
                    _ => {
                        println!("Unknown command: {}. Type 'help' for available commands.", input);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn print_interactive_help() {
    println!("Available commands:");
    println!("  ownership       - Explore ownership transfer");
    println!("  borrowing       - Explore references and borrowing");
    println!("  smart-pointers  - Explore Box, Rc, Arc");
    println!("  summary         - Show memory operations summary");
    println!("  help            - Show this help");
    println!("  quit            - Exit interactive mode");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // TODO: Add tests for the main functionality
        // HINT: Use assert_cmd to test CLI commands
        
        // Test that the program runs without crashing
        let mut tracker = MemoryTracker::new();
        // Add your tests here
    }
}
