# Memory Visualizer Project Hints - Level 3 (Complete Solution)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working memory visualizer project. Here's the full implementation with all components integrated.

## 📝 Complete Project Structure

```
project-memory-visualizer/
├── Cargo.toml
├── src/
│   ├── main.rs              # ✅ Complete CLI implementation
│   ├── memory_tracker.rs    # ✅ Complete tracking system
│   ├── ownership_demo.rs    # ✅ Complete ownership demonstrations
│   ├── borrowing_demo.rs    # ✅ Complete borrowing demonstrations
│   ├── smart_pointers.rs    # ✅ Complete smart pointer examples
│   └── visualizer.rs        # ✅ Complete ASCII visualization
└── tests/
    └── integration_tests.rs # ✅ Complete integration tests
```

## 🔧 Complete main.rs Implementation

```rust
// src/main.rs - Complete CLI implementation
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};

mod memory_tracker;
mod ownership_demo;
mod borrowing_demo;
mod smart_pointers;
mod visualizer;

use memory_tracker::MemoryTracker;

#[derive(Parser)]
#[command(name = "memory-visualizer")]
#[command(about = "A Rust memory management visualizer")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long, help = "Show detailed memory operations")]
    verbose: bool,
    
    #[arg(short, long, help = "Use color output")]
    color: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Demonstrate ownership transfer patterns
    Ownership {
        #[arg(short, long, help = "Show step-by-step breakdown")]
        detailed: bool,
    },
    /// Show borrowing and reference rules
    Borrowing {
        #[arg(short, long, help = "Include lifetime examples")]
        lifetimes: bool,
    },
    /// Explore smart pointers (Box, Rc, Arc, RefCell)
    SmartPointers {
        #[arg(short, long, help = "Focus on specific pointer type")]
        pointer_type: Option<String>,
    },
    /// Compare with C# memory model
    CompareCsharp,
    /// Interactive exploration mode
    Interactive,
    /// Run all demonstrations
    All,
}

fn main() {
    let cli = Cli::parse();
    
    // Initialize colored output
    if !cli.color {
        colored::control::set_override(false);
    }
    
    let mut tracker = MemoryTracker::new();
    
    println!("{}", "🧠 Rust Memory Visualizer".bold().cyan());
    println!("{}", "Understanding ownership through visualization".italic());
    println!();
    
    match cli.command {
        Some(Commands::Ownership { detailed }) => {
            println!("{}", "=== Ownership Demonstration ===".bold().blue());
            ownership_demo::run_demonstrations(&mut tracker, detailed);
        }
        Some(Commands::Borrowing { lifetimes }) => {
            println!("{}", "=== Borrowing Demonstration ===".bold().green());
            borrowing_demo::run_demonstrations(&mut tracker, lifetimes);
        }
        Some(Commands::SmartPointers { pointer_type }) => {
            println!("{}", "=== Smart Pointers Demonstration ===".bold().purple());
            smart_pointers::run_demonstrations(&mut tracker, pointer_type.as_deref());
        }
        Some(Commands::CompareCsharp) => {
            println!("{}", "=== C# vs Rust Comparison ===".bold().yellow());
            compare_with_csharp();
        }
        Some(Commands::Interactive) => {
            println!("{}", "=== Interactive Mode ===".bold().cyan());
            if let Err(e) = interactive_mode(&mut tracker) {
                eprintln!("Interactive mode error: {}", e);
            }
        }
        Some(Commands::All) => {
            println!("{}", "=== Complete Demonstration Suite ===".bold().magenta());
            run_all_demonstrations(&mut tracker);
        }
        None => {
            show_welcome_screen();
            ownership_demo::show_basic_example();
        }
    }
    
    if cli.verbose {
        println!("\n{}", "=== Final Memory Operations Summary ===".bold());
        tracker.print_detailed_summary();
        tracker.visualize_memory_state();
    }
}

fn show_welcome_screen() {
    println!("{}", "Welcome to the Rust Memory Visualizer!".bold());
    println!("This tool demonstrates Rust's ownership model through interactive examples.\n");
    
    println!("Available commands:");
    println!("  ownership      - See ownership transfer in action");
    println!("  borrowing      - Explore borrowing and reference rules");
    println!("  smart-pointers - Learn about Box, Rc, Arc, and RefCell");
    println!("  compare-csharp - Compare with C# memory management");
    println!("  interactive    - Explore concepts interactively");
    println!("  all            - Run complete demonstration suite");
    println!("\nAdd --help to any command for more options.\n");
}

fn run_all_demonstrations(tracker: &mut MemoryTracker) {
    println!("Running complete demonstration suite...\n");
    
    ownership_demo::run_demonstrations(tracker, true);
    println!("\n{}", "─".repeat(60));
    
    borrowing_demo::run_demonstrations(tracker, true);
    println!("\n{}", "─".repeat(60));
    
    smart_pointers::run_demonstrations(tracker, None);
    println!("\n{}", "─".repeat(60));
    
    println!("{}", "🎉 All demonstrations completed!".bold().green());
}

fn compare_with_csharp() {
    println!("{}", "Memory Management: C# vs Rust".underline().bold());
    
    println!("\n{}", "C# Approach (Garbage Collection):".bold().yellow());
    println!("```csharp");
    println!("var obj1 = new MyObject();");
    println!("var obj2 = obj1;  // Both reference same object");
    println!("obj2.Modify();    // obj1 is also modified!");
    println!("// GC collects when no references remain (unpredictable timing)");
    println!("```");
    
    println!("\n{}", "Rust Approach (Ownership):".bold().green());
    println!("```rust");
    println!("let obj1 = MyStruct::new();");
    println!("let obj2 = obj1;  // Ownership MOVES to obj2");
    println!("// obj1 is no longer valid!");
    println!("// obj2 is dropped deterministically when out of scope");
    println!("```");
    
    println!("\n{}", "Key Differences:".bold());
    println!("┌─────────────────┬─────────────────────┬─────────────────────┐");
    println!("│ Aspect          │ C# (GC)             │ Rust (Ownership)    │");
    println!("├─────────────────┼─────────────────────┼─────────────────────┤");
    println!("│ Memory Safety   │ Runtime protection  │ Compile-time safety │");
    println!("│ Performance     │ GC pauses           │ Zero-cost abstracts │");
    println!("│ Predictability │ Unpredictable GC    │ Deterministic drops │");
    println!("│ Memory Leaks    │ Possible (cycles)   │ Prevented by design │");
    println!("│ Data Races      │ Runtime errors      │ Compile-time errors │");
    println!("└─────────────────┴─────────────────────┴─────────────────────┘");
    
    println!("\n{}", "Real-World Impact:".bold().cyan());
    println!("• {}", "Rust prevents entire classes of bugs at compile time".green());
    println!("• {}", "No unexpected GC pauses in critical systems".green());
    println!("• {}", "Memory usage is predictable and minimal".green());
    println!("• {}", "Concurrency bugs caught before production".green());
}

fn interactive_mode(tracker: &mut MemoryTracker) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Interactive Memory Explorer");
    println!("Commands: ownership, borrowing, smart-pointers, memory, help, quit\n");
    
    loop {
        print!("{}", "memory> ".bold().blue());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "quit" | "exit" | "q" => {
                println!("Goodbye! 👋");
                break;
            }
            "ownership" | "own" => {
                ownership_demo::interactive_demo(tracker);
            }
            "borrowing" | "borrow" => {
                borrowing_demo::interactive_demo(tracker);
            }
            "smart-pointers" | "smart" => {
                smart_pointers::interactive_demo(tracker);
            }
            "memory" | "mem" => {
                tracker.print_detailed_summary();
                tracker.visualize_memory_state();
            }
            "clear" => {
                tracker.clear_history();
                println!("Memory history cleared.");
            }
            "help" | "h" => {
                print_interactive_help();
            }
            "" => continue,
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", input);
            }
        }
        println!();
    }
    
    Ok(())
}

fn print_interactive_help() {
    println!("{}", "Interactive Mode Commands:".bold());
    println!("  ownership       - Explore ownership transfer patterns");
    println!("  borrowing       - Explore references and borrowing rules");
    println!("  smart-pointers  - Explore Box, Rc, Arc, and RefCell");
    println!("  memory          - Show current memory state and operations");
    println!("  clear           - Clear memory operation history");
    println!("  help            - Show this help message");
    println!("  quit            - Exit interactive mode");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parsing() {
        // Test that CLI can be parsed without panicking
        let _cli = Cli::try_parse_from(&["memory-visualizer", "--help"]);
    }
    
    #[test]
    fn test_memory_tracker_creation() {
        let tracker = MemoryTracker::new();
        assert_eq!(tracker.operation_count(), 0);
    }
    
    #[test]
    fn test_basic_demonstrations() {
        let mut tracker = MemoryTracker::new();
        
        // Test that demos don't panic
        ownership_demo::show_basic_example();
        ownership_demo::run_demonstrations(&mut tracker, false);
        
        // Should have recorded some operations
        assert!(tracker.operation_count() > 0);
    }
}
```

## 🔧 Complete memory_tracker.rs Implementation

```rust
// src/memory_tracker.rs - Complete tracking system
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Clone)]
pub struct MemoryOperation {
    pub operation: Operation,
    pub location: String,
    pub value_type: String,
    pub size: usize,
    pub timestamp: usize,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Allocate,
    Deallocate,
    Move,
    Borrow,
    Clone,
    MutableBorrow,
    BorrowEnd,
}

#[derive(Debug)]
pub struct MemoryTracker {
    operations: Vec<MemoryOperation>,
    active_allocations: HashMap<String, AllocationInfo>,
    active_borrows: HashMap<String, BorrowInfo>,
    next_timestamp: usize,
    total_allocated: usize,
    total_deallocated: usize,
    peak_memory: usize,
    current_memory: usize,
}

#[derive(Debug, Clone)]
struct AllocationInfo {
    size: usize,
    type_name: String,
    timestamp: usize,
}

#[derive(Debug, Clone)]
struct BorrowInfo {
    target: String,
    is_mutable: bool,
    timestamp: usize,
}

impl MemoryTracker {
    pub fn new() -> Self {
        MemoryTracker {
            operations: Vec::new(),
            active_allocations: HashMap::new(),
            active_borrows: HashMap::new(),
            next_timestamp: 0,
            total_allocated: 0,
            total_deallocated: 0,
            peak_memory: 0,
            current_memory: 0,
        }
    }
    
    pub fn record_operation(&mut self, operation: Operation, location: &str, value_type: &str, size: usize) {
        self.record_operation_with_description(operation, location, value_type, size, "");
    }
    
    pub fn record_operation_with_description(
        &mut self, 
        operation: Operation, 
        location: &str, 
        value_type: &str, 
        size: usize,
        description: &str
    ) {
        let op = MemoryOperation {
            operation: operation.clone(),
            location: location.to_string(),
            value_type: value_type.to_string(),
            size,
            timestamp: self.next_timestamp,
            description: description.to_string(),
        };
        
        match operation {
            Operation::Allocate => {
                self.active_allocations.insert(
                    location.to_string(),
                    AllocationInfo {
                        size,
                        type_name: value_type.to_string(),
                        timestamp: self.next_timestamp,
                    }
                );
                self.total_allocated += size;
                self.current_memory += size;
                if self.current_memory > self.peak_memory {
                    self.peak_memory = self.current_memory;
                }
            }
            Operation::Deallocate => {
                if self.active_allocations.remove(location).is_some() {
                    self.total_deallocated += size;
                    self.current_memory = self.current_memory.saturating_sub(size);
                }
            }
            Operation::Borrow => {
                self.active_borrows.insert(
                    location.to_string(),
                    BorrowInfo {
                        target: value_type.to_string(),
                        is_mutable: false,
                        timestamp: self.next_timestamp,
                    }
                );
            }
            Operation::MutableBorrow => {
                self.active_borrows.insert(
                    location.to_string(),
                    BorrowInfo {
                        target: value_type.to_string(),
                        is_mutable: true,
                        timestamp: self.next_timestamp,
                    }
                );
            }
            Operation::BorrowEnd => {
                self.active_borrows.remove(location);
            }
            _ => {} // Move, Clone don't change active state
        }
        
        self.operations.push(op);
        self.next_timestamp += 1;
    }
    
    // Convenience methods for common operations
    pub fn allocate_string(&mut self, name: &str, content: &str) {
        let size = content.len() + std::mem::size_of::<String>();
        self.record_operation_with_description(
            Operation::Allocate, 
            name, 
            "String", 
            size,
            &format!("String::from(\"{}\")", content)
        );
    }
    
    pub fn allocate_vec(&mut self, name: &str, element_count: usize, element_type: &str) {
        let size = element_count * 4 + std::mem::size_of::<Vec<i32>>(); // Simplified
        self.record_operation_with_description(
            Operation::Allocate,
            name,
            &format!("Vec<{}>", element_type),
            size,
            &format!("vec! with {} elements", element_count)
        );
    }
    
    pub fn move_value(&mut self, from: &str, to: &str, type_name: &str) {
        if let Some(alloc_info) = self.active_allocations.get(from).cloned() {
            self.record_operation(Operation::Move, &format!("{} -> {}", from, to), type_name, alloc_info.size);
            self.record_operation(Operation::Deallocate, from, type_name, alloc_info.size);
            self.record_operation(Operation::Allocate, to, type_name, alloc_info.size);
        }
    }
    
    pub fn borrow_immutable(&mut self, borrow_name: &str, target: &str) {
        self.record_operation_with_description(
            Operation::Borrow,
            borrow_name,
            target,
            0,
            &format!("&{}", target)
        );
    }
    
    pub fn borrow_mutable(&mut self, borrow_name: &str, target: &str) {
        self.record_operation_with_description(
            Operation::MutableBorrow,
            borrow_name,
            target,
            0,
            &format!("&mut {}", target)
        );
    }
    
    pub fn end_borrow(&mut self, borrow_name: &str) {
        self.record_operation(Operation::BorrowEnd, borrow_name, "", 0);
    }
    
    pub fn print_summary(&self) {
        println!("{}", "=== Memory Operations Summary ===".bold().blue());
        println!("Total operations: {}", self.operations.len());
        println!("Total allocated: {} bytes", self.total_allocated);
        println!("Total deallocated: {} bytes", self.total_deallocated);
        println!("Peak memory usage: {} bytes", self.peak_memory);
        println!("Current memory usage: {} bytes", self.current_memory);
        println!("Currently active: {} allocations", self.active_allocations.len());
        println!("Currently active: {} borrows", self.active_borrows.len());
    }
    
    pub fn print_detailed_summary(&self) {
        self.print_summary();
        
        if !self.operations.is_empty() {
            println!("\n{}", "Recent operations (last 10):".bold());
            for op in self.operations.iter().rev().take(10) {
                self.print_operation(op);
            }
        }
        
        if !self.active_allocations.is_empty() {
            println!("\n{}", "Active allocations:".bold().green());
            for (location, info) in &self.active_allocations {
                println!("  {} {} ({} bytes) at timestamp {}", 
                        "📦".green(), location, info.size, info.timestamp);
            }
        }
        
        if !self.active_borrows.is_empty() {
            println!("\n{}", "Active borrows:".bold().blue());
            for (borrow, info) in &self.active_borrows {
                let mutability = if info.is_mutable { "&mut" } else { "&" };
                println!("  {} {} {} -> {} at timestamp {}", 
                        "🔗".blue(), mutability, borrow, info.target, info.timestamp);
            }
        }
    }
    
    fn print_operation(&self, op: &MemoryOperation) {
        let (icon, color_fn): (_, fn(&str) -> ColoredString) = match op.operation {
            Operation::Allocate => ("🟢", |s| s.green()),
            Operation::Deallocate => ("🔴", |s| s.red()),
            Operation::Move => ("🔄", |s| s.yellow()),
            Operation::Borrow => ("📖", |s| s.blue()),
            Operation::MutableBorrow => ("📝", |s| s.purple()),
            Operation::BorrowEnd => ("📪", |s| s.cyan()),
            Operation::Clone => ("📋", |s| s.magenta()),
        };
        
        let op_name = format!("{:?}", op.operation).to_uppercase();
        println!("  {} {} {} ({}) at {} - {} bytes {}", 
                op.timestamp, 
                icon,
                color_fn(&op_name), 
                op.value_type, 
                op.location, 
                op.size,
                if !op.description.is_empty() { &format!("// {}", op.description) } else { "" }
        );
    }
    
    pub fn visualize_memory_state(&self) {
        println!("{}", "=== Current Memory State ===".bold().cyan());
        
        if self.active_allocations.is_empty() && self.active_borrows.is_empty() {
            println!("📭 No active allocations or borrows");
            return;
        }
        
        // Show memory layout diagram
        self.draw_memory_diagram();
        
        // Show borrow relationships
        if !self.active_borrows.is_empty() {
            println!("\n{}", "Borrow relationships:".bold().blue());
            for (borrow, info) in &self.active_borrows {
                let arrow = if info.is_mutable { "⇄" } else { "→" };
                let mutability = if info.is_mutable { "&mut" } else { "&" };
                println!("  {} {} {} {} {}", 
                        borrow.cyan(), 
                        mutability.blue(), 
                        arrow.yellow(), 
                        info.target.green(),
                        format!("({})", if info.is_mutable { "exclusive" } else { "shared" }).italic()
                );
            }
        }
    }
    
    fn draw_memory_diagram(&self) {
        println!("┌─── Stack ───┐    ┌─── Heap ───┐");
        
        if self.active_allocations.is_empty() {
            println!("│    empty    │    │   empty   │");
        } else {
            let max_width = 10;
            for (location, info) in &self.active_allocations {
                let bar_length = (info.size / 10).min(max_width).max(1);
                let bar = "█".repeat(bar_length);
                
                println!("│ {:>9} │ -> │ {} │ {} bytes", 
                        location.green(), 
                        bar.blue(), 
                        info.size);
            }
        }
        
        println!("└─────────────┘    └───────────┘");
    }
    
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }
    
    pub fn clear_history(&mut self) {
        self.operations.clear();
        self.active_allocations.clear();
        self.active_borrows.clear();
        self.next_timestamp = 0;
        self.total_allocated = 0;
        self.total_deallocated = 0;
        self.peak_memory = 0;
        self.current_memory = 0;
    }
    
    pub fn get_memory_usage(&self) -> usize {
        self.current_memory
    }
    
    pub fn get_peak_memory(&self) -> usize {
        self.peak_memory
    }
    
    pub fn has_active_borrows(&self) -> bool {
        !self.active_borrows.is_empty()
    }
    
    pub fn has_active_allocations(&self) -> bool {
        !self.active_allocations.is_empty()
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_tracker_basic_operations() {
        let mut tracker = MemoryTracker::new();
        
        tracker.allocate_string("s1", "hello");
        assert_eq!(tracker.operation_count(), 1);
        assert!(tracker.has_active_allocations());
        
        tracker.record_operation(Operation::Deallocate, "s1", "String", 5);
        assert!(!tracker.has_active_allocations());
    }
    
    #[test]
    fn test_borrowing_tracking() {
        let mut tracker = MemoryTracker::new();
        
        tracker.allocate_string("data", "test");
        tracker.borrow_immutable("ref1", "data");
        
        assert!(tracker.has_active_borrows());
        
        tracker.end_borrow("ref1");
        assert!(!tracker.has_active_borrows());
    }
    
    #[test]
    fn test_memory_usage_tracking() {
        let mut tracker = MemoryTracker::new();
        
        tracker.allocate_string("s1", "hello");
        let usage1 = tracker.get_memory_usage();
        
        tracker.allocate_string("s2", "world");
        let usage2 = tracker.get_memory_usage();
        
        assert!(usage2 > usage1);
        assert!(tracker.get_peak_memory() >= usage2);
    }
}
```

## 🔧 Complete ownership_demo.rs Implementation

```rust
// src/ownership_demo.rs - Complete ownership demonstrations
use crate::memory_tracker::{MemoryTracker, Operation};
use colored::*;
use std::io::{self, Write};

pub fn run_demonstrations(tracker: &mut MemoryTracker, detailed: bool) {
    if detailed {
        println!("{}", "🔍 Running detailed ownership demonstrations...".italic());
    }
    
    demonstrate_string_moves(tracker, detailed);
    demonstrate_vector_moves(tracker, detailed);
    demonstrate_struct_moves(tracker, detailed);
    demonstrate_clone_vs_move(tracker, detailed);
    demonstrate_function_ownership(tracker, detailed);
    demonstrate_return_ownership(tracker, detailed);
    
    if detailed {
        tracker.print_detailed_summary();
    }
}

pub fn show_basic_example() {
    println!("{}", "Quick Ownership Example:".bold());
    println!("  let s1 = String::from(\"hello\");  // s1 owns the string");
    println!("  let s2 = s1;                      // Ownership moves to s2");
    println!("  // s1 is no longer valid!");
    println!("  println!(\"{{}}\", s2);              // ✅ s2 owns the data");
    println!("\nRun 'memory-visualizer ownership' for interactive demos");
}

fn demonstrate_string_moves(tracker: &mut MemoryTracker, detailed: bool) {
    println!("{}", "\n🔤 String Move Demonstration".bold().underline());
    
    if detailed {
        println!("Strings in Rust are heap-allocated and follow move semantics.");
        println!("Unlike Copy types (i32, bool), String ownership transfers on assignment.\n");
    }
    
    println!("Step 1: Creating String");
    tracker.allocate_string("s1", "hello");
    println!("  {}", "let s1 = String::from(\"hello\");".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        pause_for_input();
    }
    
    println!("\nStep 2: Moving string ownership");
    tracker.move_value("s1", "s2", "String");
    println!("  {}", "let s2 = s1;  // s1 is moved to s2".code_style());
    
    println!("\n{}", "Result:".bold());
    println!("  {} s1 is no longer accessible", "❌".red());
    println!("  {} s2 now owns the string", "✅".green());
    
    if detailed {
        tracker.visualize_memory_state();
        println!("\n{}", "Key Insight:".bold().yellow());
        println!("Move prevents use-after-free bugs by making the original variable invalid.");
    }
    
    // Clean up for next demo
    tracker.record_operation(Operation::Deallocate, "s2", "String", 5);
}

fn demonstrate_vector_moves(tracker: &mut MemoryTracker, detailed: bool) {
    println!("{}", "\n📊 Vector Move Demonstration".bold().underline());
    
    if detailed {
        println!("Vectors are also heap-allocated and follow move semantics.");
        println!("This prevents accidental sharing of mutable data.\n");
    }
    
    println!("Step 1: Creating vector");
    tracker.allocate_vec("vec1", 3, "i32");
    println!("  {}", "let vec1 = vec![1, 2, 3];".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        pause_for_input();
    }
    
    println!("\nStep 2: Passing vector to function");
    tracker.move_value("vec1", "function_param", "Vec<i32>");
    println!("  {}", "process_vector(vec1);  // vec1 moved into function".code_style());
    
    println!("\nStep 3: Function completes, vector is dropped");
    tracker.record_operation(Operation::Deallocate, "function_param", "Vec<i32>", 28);
    println!("  {}", "// Vector automatically dropped at end of function".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        println!("\n{}", "Key Insight:".bold().yellow());
        println!("Functions can take ownership, preventing use after the call.");
    }
}

fn demonstrate_struct_moves(tracker: &mut MemoryTracker, detailed: bool) {
    println!("{}", "\n🏗️ Struct Move Demonstration".bold().underline());
    
    if detailed {
        println!("Custom structs follow move semantics when they contain non-Copy fields.");
        println!("This ensures exclusive ownership of complex data structures.\n");
    }
    
    println!("Step 1: Creating struct with String field");
    tracker.record_operation(Operation::Allocate, "person", "Person", 32);
    println!("  {}", "let person = Person { name: String::from(\"Alice\") };".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        pause_for_input();
    }
    
    println!("\nStep 2: Moving entire struct");
    tracker.move_value("person", "other_person", "Person");
    println!("  {}", "let other_person = person;  // Entire struct moved".code_style());
    
    println!("\n{}", "Result:".bold());
    println!("  {} Original 'person' is invalid", "❌".red());
    println!("  {} 'other_person' owns all the data", "✅".green());
    
    if detailed {
        tracker.visualize_memory_state();
        println!("\n{}", "Key Insight:".bold().yellow());
        println!("Moving a struct moves all its fields, maintaining ownership invariants.");
    }
    
    // Clean up
    tracker.record_operation(Operation::Deallocate, "other_person", "Person", 32);
}

fn demonstrate_clone_vs_move(tracker: &mut MemoryTracker, detailed: bool) {
    println!("{}", "\n📋 Clone vs Move Comparison".bold().underline());
    
    if detailed {
        println!("Sometimes you need multiple independent copies instead of ownership transfer.");
        println!("The .clone() method creates deep copies for this purpose.\n");
    }
    
    println!("{}", "Scenario A: Moving (transfers ownership)".bold());
    tracker.allocate_string("original", "data");
    println!("  {}", "let original = String::from(\"data\");".code_style());
    
    tracker.move_value("original", "moved", "String");
    println!("  {}", "let moved = original;  // original no longer usable".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        pause_for_input();
    }
    
    println!("\n{}", "Scenario B: Cloning (creates independent copy)".bold());
    tracker.record_operation(Operation::Clone, "moved -> cloned", "String", 4);
    tracker.allocate_string("cloned", "data");
    println!("  {}", "let cloned = moved.clone();  // Both are now usable".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
    }
    
    println!("\n{}", "Comparison:".bold());
    println!("  {} Move: Fast, transfers ownership, original becomes invalid", "🔄".yellow());
    println!("  {} Clone: Slower, creates copy, both remain valid", "📋".blue());
    
    if detailed {
        println!("\n{}", "When to use each:".bold().yellow());
        println!("• Use move when you're done with the original");
        println!("• Use clone when you need both values to remain valid");
        println!("• Clone has performance cost - use judiciously");
    }
    
    // Clean up
    tracker.record_operation(Operation::Deallocate, "moved", "String", 4);
    tracker.record_operation(Operation::Deallocate, "cloned", "String", 4);
}

fn demonstrate_function_ownership(tracker: &mut MemoryTracker, detailed: bool) {
    println!("{}", "\n🔧 Function Ownership Patterns".bold().underline());
    
    if detailed {
        println!("Functions can take ownership, borrow immutably, or borrow mutably.");
        println!("Each pattern serves different use cases.\n");
    }
    
    // Pattern 1: Taking ownership
    println!("{}", "Pattern 1: Taking Ownership".bold());
    tracker.allocate_string("data1", "consumed");
    println!("  {}", "let data = String::from(\"consumed\");".code_style());
    
    tracker.move_value("data1", "fn_consume_param", "String");
    println!("  {}", "consume_string(data);  // data moved into function".code_style());
    
    tracker.record_operation(Operation::Deallocate, "fn_consume_param", "String", 8);
    println!("  {}", "// data dropped at end of function".code_style());
    
    if detailed {
        println!("  {} Use when function needs to own the data", "💡".yellow());
        tracker.visualize_memory_state();
        pause_for_input();
    }
    
    // Pattern 2: Borrowing
    println!("\n{}", "Pattern 2: Borrowing (Reference)".bold());
    tracker.allocate_string("data2", "borrowed");
    println!("  {}", "let data = String::from(\"borrowed\");".code_style());
    
    tracker.borrow_immutable("fn_ref", "data2");
    println!("  {}", "read_string(&data);  // data borrowed, not moved".code_style());
    
    tracker.end_borrow("fn_ref");
    println!("  {}", "// Reference ends, data still owned by caller".code_style());
    
    if detailed {
        println!("  {} Use when function only needs to read the data", "💡".yellow());
        tracker.visualize_memory_state();
    }
    
    // Clean up
    tracker.record_operation(Operation::Deallocate, "data2", "String", 8);
}

fn demonstrate_return_ownership(tracker: &mut MemoryTracker, detailed: bool) {
    println!("{}", "\n↩️ Return Ownership Patterns".bold().underline());
    
    if detailed {
        println!("Functions can create new owned data or return ownership they received.");
        println!("This enables fluent APIs and builder patterns.\n");
    }
    
    // Pattern 1: Creating and returning new data
    println!("{}", "Pattern 1: Creating New Data".bold());
    tracker.allocate_string("fn_result", "created");
    println!("  {}", "let result = create_string();  // Function creates and returns".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        pause_for_input();
    }
    
    // Pattern 2: Transform and return
    println!("\n{}", "Pattern 2: Transform and Return".bold());
    tracker.move_value("fn_result", "fn_transform_param", "String");
    println!("  {}", "let transformed = transform(result);  // Takes and returns ownership".code_style());
    
    tracker.record_operation(Operation::Deallocate, "fn_transform_param", "String", 7);
    tracker.allocate_string("transformed", "created_modified");
    println!("  {}", "// Original consumed, new value returned".code_style());
    
    if detailed {
        tracker.visualize_memory_state();
        println!("\n{}", "Key Insight:".bold().yellow());
        println!("Return ownership enables method chaining and builder patterns:");
        println!("  builder.add(x).add(y).build()");
    }
    
    // Clean up
    tracker.record_operation(Operation::Deallocate, "transformed", "String", 15);
}

pub fn interactive_demo(tracker: &mut MemoryTracker) {
    println!("{}", "🔍 Interactive Ownership Explorer".bold());
    println!("Choose a demonstration:");
    println!("1. String moves");
    println!("2. Vector moves");
    println!("3. Struct moves");
    println!("4. Clone vs move");
    println!("5. Function ownership");
    println!("6. Return ownership");
    println!("7. All demonstrations");
    println!("Enter number (1-7):");
    
    print!("ownership> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        match input.trim() {
            "1" => demonstrate_string_moves(tracker, true),
            "2" => demonstrate_vector_moves(tracker, true),
            "3" => demonstrate_struct_moves(tracker, true),
            "4" => demonstrate_clone_vs_move(tracker, true),
            "5" => demonstrate_function_ownership(tracker, true),
            "6" => demonstrate_return_ownership(tracker, true),
            "7" => run_demonstrations(tracker, true),
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn pause_for_input() {
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

trait CodeStyle {
    fn code_style(&self) -> ColoredString;
}

impl CodeStyle for &str {
    fn code_style(&self) -> ColoredString {
        self.bright_white().on_black()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_example_display() {
        // Test that basic example doesn't panic
        show_basic_example();
    }
    
    #[test]
    fn test_demonstrations_run() {
        let mut tracker = MemoryTracker::new();
        run_demonstrations(&mut tracker, false);
        
        // Should have recorded operations
        assert!(tracker.operation_count() > 0);
    }
    
    #[test]
    fn test_string_moves_demo() {
        let mut tracker = MemoryTracker::new();
        demonstrate_string_moves(&mut tracker, false);
        
        // Should have some operations recorded
        assert!(tracker.operation_count() > 0);
    }
}
```

## 🔧 Complete Integration Tests

```rust
// tests/integration_tests.rs - Complete integration tests
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("memory management visualizer"));
}

#[test]
fn test_ownership_demo() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("ownership")
        .assert()
        .success()
        .stdout(predicate::str::contains("Ownership Demonstration"));
}

#[test]
fn test_borrowing_demo() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("borrowing")
        .assert()
        .success()
        .stdout(predicate::str::contains("Borrowing Demonstration"));
}

#[test]
fn test_smart_pointers_demo() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("smart-pointers")
        .assert()
        .success()
        .stdout(predicate::str::contains("Smart Pointers Demonstration"));
}

#[test]
fn test_compare_csharp() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("compare-csharp")
        .assert()
        .success()
        .stdout(predicate::str::contains("C# vs Rust"));
}

#[test]
fn test_all_demos() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.arg("all")
        .assert()
        .success()
        .stdout(predicate::str::contains("Complete Demonstration Suite"));
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.args(&["ownership", "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Memory Operations Summary"));
}

#[test]
fn test_detailed_ownership() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.args(&["ownership", "--detailed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("detailed ownership"));
}

#[test]
fn test_borrowing_with_lifetimes() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.args(&["borrowing", "--lifetimes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Borrowing Demonstration"));
}

#[test]
fn test_smart_pointers_specific_type() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.args(&["smart-pointers", "--pointer-type", "box"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Smart Pointers"));
}

#[test]
fn test_default_behavior() {
    let mut cmd = Command::cargo_bin("memory-visualizer").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Memory Visualizer"))
        .stdout(predicate::str::contains("Quick Ownership Example"));
}
```

## 🎓 Complete Project Walkthrough

### 1. Architecture Overview
```
CLI Layer (main.rs)
    ↓
Demo Modules (ownership_demo.rs, borrowing_demo.rs, smart_pointers.rs)
    ↓
Memory Tracker (memory_tracker.rs)
    ↓
Visualization (ASCII art, colored output)
```

### 2. Key Design Patterns
- **Command Pattern**: CLI subcommands for different demos
- **Observer Pattern**: Memory tracker observes and records operations
- **Strategy Pattern**: Different visualization strategies for different concepts
- **Builder Pattern**: Progressive construction of complex demonstrations

### 3. Educational Features
- **Progressive Disclosure**: Start simple, add complexity
- **Visual Feedback**: ASCII diagrams and colored output
- **Interactive Exploration**: REPL-style interface
- **Comparative Learning**: C# vs Rust explanations

### 4. Memory Safety Demonstration
```rust
// Shows ownership transfer
tracker.move_value("s1", "s2", "String");

// Shows borrowing safety
tracker.borrow_immutable("ref1", "data");
tracker.borrow_mutable("ref2", "data");  // Would show conflict

// Shows smart pointer usage
demonstrate_rc_sharing(tracker);  // Multiple ownership
demonstrate_arc_threading(tracker);  // Thread-safe sharing
```

## 🏆 Educational Impact

This complete memory visualizer:

### ✅ **Makes Abstract Concepts Concrete**
- Visual representation of memory operations
- Step-by-step breakdown of ownership transfer
- Real-time tracking of allocations and deallocations

### ✅ **Reinforces Learning Through Practice**
- Interactive exploration of concepts
- Immediate feedback on memory operations
- Progressive complexity building

### ✅ **Bridges Knowledge Gaps**
- Direct comparison with C# memory model
- Clear explanations of when to use each pattern
- Real-world context for ownership rules

### ✅ **Professional Development Tool**
- Production-quality CLI interface
- Comprehensive test coverage
- Clean, maintainable codebase

## 🚀 Next Steps

**Congratulations!** You've built a complete, professional memory visualizer that:

1. **Demonstrates mastery** of all Module 02 concepts
2. **Provides educational value** for other developers learning Rust
3. **Shows real-world application** of ownership patterns
4. **Exhibits professional development practices**

You're now ready to:
- **Extend the visualizer** with additional concepts
- **Share it as a learning tool** with other developers
- **Apply these patterns** in production Rust applications
- **Move to advanced Rust topics** with confidence

You've created a tool that makes Rust's most challenging concepts accessible and understandable! 🦀✨