// Exercise 1: Argument Parser - Fix the Broken CLI!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix this broken command-line argument parser to make it work properly
// This demonstrates clap usage and proper CLI design patterns
//
// INSTRUCTIONS:
// 1. Fix ONE error at a time - don't try to fix everything at once!
// 2. Compile after each fix: `cargo run --bin ex01-arg-parser -- --help`
// 3. Test with various arguments to see what breaks
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with compilation errors (missing dependencies, wrong syntax)
// - Then fix logical errors (argument parsing, validation)
// - Finally polish the user experience (help messages, error handling)
//
// COMPLETED CONCEPTS:
// [] Add clap dependency to Cargo.toml
// [] Fix derive macro syntax
// [] Fix argument attributes
// [] Add proper help text
// [] Handle subcommands correctly
// [] Implement proper error handling

use clap::{Parser, Subcommand};  // FIXME: This import might not work!

// FIXME: This struct has several issues with clap derive
#[derive(Parser)]
#[clap(author, version, about)]  // FIXME: Missing actual values!
pub struct Cli {
    /// The input file to process
    #[clap(short, long)]
    input: String,  // FIXME: What if user doesn't provide this?
    
    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
    
    /// Number of threads to use
    #[clap(short = 't', long, default_value = "1")]
    threads: usize,  // FIXME: Should validate this is > 0
    
    #[clap(subcommand)]
    command: Commands,  // FIXME: Commands enum doesn't exist yet!
}

// TODO: Implement the Commands enum
// HINT: In C#, this would be like having different action methods in a console app

// FIXME: This function doesn't handle errors properly
fn main() {
    let cli = Cli::parse();  // COMPILE ERROR: This will panic on bad input!
    
    println!("Processing file: {}", cli.input);
    
    if cli.verbose {
        println!("Verbose mode enabled");
        println!("Using {} threads", cli.threads);
    }
    
    // TODO: Handle the subcommand
    // HINT: Use match statement like a switch in C#
    println!("Command: {:?}", cli.command);  // COMPILE ERROR: Commands not defined!
    
    // ✅ CHECKPOINT 1: Run `cargo run --bin ex01-arg-parser -- --help`
    // Should show help without crashing
    
    // TODO: Implement actual command logic
    execute_command(&cli);
}

// FIXME: This function needs implementation
fn execute_command(cli: &Cli) {
    todo!("Implement command execution")  // Remove this and add real logic
    
    // TODO: Match on cli.command and handle each subcommand
    // HINT: Similar to a switch statement in C# console apps
    
    // ✅ CHECKPOINT 2: Commands should execute without panicking
    // Each subcommand should do something meaningful
}

// TODO: Add validation functions
// HINT: In C#, you might use data annotations or manual validation

fn validate_input_file(path: &str) -> Result<(), String> {
    // TODO: Check if file exists and is readable
    // HINT: Use std::path::Path and std::fs
    todo!("Implement file validation")
}

fn validate_thread_count(count: usize) -> Result<(), String> {
    // TODO: Ensure thread count is reasonable (1-16)
    // HINT: Similar to validating numeric input in C# console apps
    todo!("Implement thread validation")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parsing() {
        // TODO: Test that CLI parsing works correctly
        // HINT: Use clap's try_parse() instead of parse()
        todo!("Implement CLI parsing tests")
    }
    
    #[test]
    fn test_validation() {
        // TODO: Test input validation functions
        todo!("Implement validation tests")
    }
}

// COMPILATION CHALLENGES:
// 1. Add clap dependency to Cargo.toml
// 2. Define the Commands enum with subcommands
// 3. Fix the derive macro attributes
// 4. Handle parsing errors gracefully
// 5. Implement command execution logic
// 6. Add proper validation
//
// LEARNING OBJECTIVES:
// - Clap derive macros vs manual parsing
// - Subcommand patterns in CLI tools
// - Error handling for user input
// - CLI UX best practices
// - Argument validation and defaults
//
// C# COMPARISON:
// C#: CommandLineParser.Default.ParseArguments<Options>(args)
// Rust: Cli::parse() with derive macros
//
// C#: switch (options.Command) { case "build" => ... }
// Rust: match cli.command { Commands::Build => ... }
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Basic parsing): [ ] Complete
// Checkpoint 2 (Subcommands): [ ] Complete  
// Checkpoint 3 (Validation): [ ] Complete
// Checkpoint 4 (Error handling): [ ] Complete
// Checkpoint 5 (Testing): [ ] Complete
// Checkpoint 6 (Polish): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Clap derive macros and attributes
// ✅ Subcommand patterns for complex CLIs
// ✅ Proper error handling and validation
// ✅ CLI user experience design
// ✅ Testing command-line applications
//
// 🚀 Ready for the next challenge?
// Move on to ex02-error-handling.rs to improve CLI error messages!
// Or test your work with: `cargo run --bin ex01-arg-parser -- process input.txt --verbose`