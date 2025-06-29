// Memory Visualizer - ASCII art for memory operations
//
// TODO: Complete the visualization functions to show memory layout

use colored::*;
use std::collections::HashMap;

pub struct MemoryVisualizer {
    stack_items: Vec<StackItem>,
    heap_items: Vec<HeapItem>,
}

#[derive(Debug, Clone)]
struct StackItem {
    name: String,
    value_type: String,
    points_to_heap: Option<usize>, // Index into heap_items
}

#[derive(Debug, Clone)]
struct HeapItem {
    id: usize,
    value_type: String,
    size: usize,
    content: String,
}

impl MemoryVisualizer {
    pub fn new() -> Self {
        MemoryVisualizer {
            stack_items: Vec::new(),
            heap_items: Vec::new(),
        }
    }
    
    pub fn add_stack_value(&mut self, name: &str, value_type: &str) {
        // TODO: Add a stack-allocated value
        let item = StackItem {
            name: name.to_string(),
            value_type: value_type.to_string(),
            points_to_heap: None,
        };
        self.stack_items.push(item);
    }
    
    pub fn add_heap_value(&mut self, name: &str, value_type: &str, size: usize, content: &str) -> usize {
        // TODO: Add a heap-allocated value and return its ID
        let id = self.heap_items.len();
        let item = HeapItem {
            id,
            value_type: value_type.to_string(),
            size,
            content: content.to_string(),
        };
        self.heap_items.push(item);
        
        // TODO: Update stack item to point to this heap item
        // HINT: Find the stack item with the given name and set points_to_heap
        
        id
    }
    
    pub fn remove_stack_value(&mut self, name: &str) {
        // TODO: Remove a stack value by name
        self.stack_items.retain(|item| item.name != name);
    }
    
    pub fn remove_heap_value(&mut self, id: usize) {
        // TODO: Remove a heap value by ID
        self.heap_items.retain(|item| item.id != id);
        
        // TODO: Update any stack items that pointed to this heap item
        for stack_item in &mut self.stack_items {
            if stack_item.points_to_heap == Some(id) {
                stack_item.points_to_heap = None;
            }
        }
    }
    
    pub fn visualize(&self) {
        println!("\n{}", "=== Memory Layout ===".bold());
        self.draw_stack();
        self.draw_heap();
        self.draw_references();
    }
    
    fn draw_stack(&self) {
        println!("\n{}", "STACK".bold().blue());
        println!("┌─────────────────────┐");
        
        if self.stack_items.is_empty() {
            println!("│ (empty)             │");
        } else {
            // TODO: Draw stack items from top to bottom
            for (i, item) in self.stack_items.iter().enumerate().rev() {
                let pointer_symbol = if item.points_to_heap.is_some() { "→" } else { " " };
                println!("│ {:<8} {:<7} {} │", 
                    item.name.green(), 
                    item.value_type.yellow(),
                    pointer_symbol.red()
                );
            }
        }
        
        println!("└─────────────────────┘");
        println!(" {}", "Stack grows down ↓".dimmed());
    }
    
    fn draw_heap(&self) {
        println!("\n{}", "HEAP".bold().purple());
        
        if self.heap_items.is_empty() {
            println!("┌─────────────────────┐");
            println!("│ (empty)             │");
            println!("└─────────────────────┘");
            return;
        }
        
        // TODO: Draw heap items with their sizes
        for item in &self.heap_items {
            let blocks = (item.size / 8).max(1); // Each block represents 8 bytes
            let visualization = "█".repeat(blocks);
            
            println!("┌─────────────────────┐");
            println!("│ ID: {:<3} Size: {:<6} │", item.id.to_string().cyan(), format!("{}B", item.size).yellow());
            println!("│ Type: {:<12} │", item.value_type.blue());
            println!("│ Data: {:<12} │", item.content.green());
            println!("│ Mem:  {:<12} │", visualization.purple());
            println!("└─────────────────────┘");
        }
        
        println!(" {}", "Heap grows up ↑".dimmed());
    }
    
    fn draw_references(&self) {
        println!("\n{}", "REFERENCES".bold().cyan());
        
        let mut has_references = false;
        for stack_item in &self.stack_items {
            if let Some(heap_id) = stack_item.points_to_heap {
                println!("{} {} → Heap[{}]", 
                    "📍".yellow(),
                    stack_item.name.green(),
                    heap_id.to_string().cyan()
                );
                has_references = true;
            }
        }
        
        if !has_references {
            println!("(no active references)");
        }
    }
    
    pub fn visualize_operation(&self, operation: &str, description: &str) {
        println!("\n{} {}", "OPERATION:".bold(), operation.yellow().bold());
        println!("{}", description.italic());
        self.visualize();
    }
    
    // TODO: Add method to show ownership transfer
    pub fn show_ownership_transfer(&self, from: &str, to: &str) {
        println!("\n{}", "=== Ownership Transfer ===".bold().red());
        println!("{} {} → {}", "MOVE:".bold(), from.green(), to.green());
        println!("{}", "Previous owner is now invalid".red().italic());
        
        // TODO: Show visual representation of the transfer
        println!("Before: {} owns the data", from.green());
        println!("After:  {} owns the data", to.green());
        println!("        {} is no longer valid", from.strikethrough().red());
    }
    
    // TODO: Add method to show borrowing
    pub fn show_borrowing(&self, owner: &str, borrower: &str, mutable: bool) {
        let borrow_type = if mutable { "MUTABLE BORROW" } else { "IMMUTABLE BORROW" };
        let symbol = if mutable { "🔓" } else { "🔒" };
        
        println!("\n{} {}", "=== Borrowing ===".bold().blue(), symbol);
        println!("{} {} borrows from {}", 
            borrow_type.bold(), 
            borrower.cyan(), 
            owner.green()
        );
        
        if mutable {
            println!("{}", "• Exclusive access granted".yellow());
            println!("{}", "• Can modify through reference".yellow());
        } else {
            println!("{}", "• Shared access granted".green());
            println!("{}", "• Multiple immutable borrows allowed".green());
        }
        
        // TODO: Show ASCII representation of borrowing
        println!("Owner:    {} ← still owns data", owner.green());
        println!("Borrower: {} ← temporary access", borrower.cyan());
    }
    
    // TODO: Add method to show smart pointer operations
    pub fn show_smart_pointer_op(&self, pointer_type: &str, operation: &str, details: &str) {
        let symbol = match pointer_type {
            "Box" => "📦",
            "Rc" => "🔗",
            "Arc" => "⚛️",
            "RefCell" => "🔄",
            _ => "📍",
        };
        
        println!("\n{} {} {}", symbol, pointer_type.bold().purple(), operation.yellow());
        println!("{}", details.italic());
        
        // TODO: Add specific visualizations for each smart pointer type
        match pointer_type {
            "Box" => self.visualize_box_operation(operation, details),
            "Rc" => self.visualize_rc_operation(operation, details),
            "RefCell" => self.visualize_refcell_operation(operation, details),
            _ => {}
        }
    }
    
    fn visualize_box_operation(&self, operation: &str, details: &str) {
        // TODO: Show Box-specific visualization
        println!("Box operation: {}", operation);
        println!("Stack: [ptr] → Heap: [data]");
    }
    
    fn visualize_rc_operation(&self, operation: &str, details: &str) {
        // TODO: Show Rc reference counting
        println!("Rc operation: {}", operation);
        if operation.contains("clone") {
            println!("Reference count increased");
            println!("ptr1 → [data] ← ptr2");
        }
    }
    
    fn visualize_refcell_operation(&self, operation: &str, details: &str) {
        // TODO: Show RefCell runtime borrowing
        println!("RefCell operation: {}", operation);
        if operation.contains("borrow") {
            println!("Runtime borrow check: ✅");
        }
    }
}

// TODO: Implement helper functions for complex visualizations

pub fn visualize_memory_comparison() {
    println!("{}", "=== Memory Management Comparison ===".bold());
    
    println!("\n{}", "C# (Garbage Collection):".yellow().bold());
    println!("┌─────────────────────┐");
    println!("│ obj1 ──┐            │");
    println!("│        │    HEAP    │");
    println!("│ obj2 ──┴─→ [Object] │");
    println!("│               ↑     │");
    println!("│            GC Root  │");
    println!("└─────────────────────┘");
    println!("• Multiple references to same object");
    println!("• Garbage collector cleans up");
    println!("• Unpredictable cleanup timing");
    
    println!("\n{}", "Rust (Ownership):".green().bold());
    println!("┌─────────────────────┐");
    println!("│ owner ────→ [Data]  │");
    println!("│                     │");
    println!("│ (no other refs)     │");
    println!("│                     │");
    println!("│ Dropped when owner  │");
    println!("│ goes out of scope   │");
    println!("└─────────────────────┘");
    println!("• Single owner at a time");
    println!("• Deterministic cleanup");
    println!("• No garbage collection needed");
}

pub fn visualize_borrowing_rules() {
    println!("{}", "=== Borrowing Rules Visualization ===".bold());
    
    println!("\n{}", "Rule 1: Multiple Immutable Borrows OK".green().bold());
    println!("owner → [data] ← ref1");
    println!("               ← ref2");
    println!("               ← ref3");
    println!("✅ All can read simultaneously");
    
    println!("\n{}", "Rule 2: One Mutable Borrow Only".yellow().bold());
    println!("owner → [data] ← mut_ref");
    println!("✅ Exclusive write access");
    println!("❌ No other references allowed");
    
    println!("\n{}", "Rule 3: No Mixed Borrows".red().bold());
    println!("owner → [data] ← immut_ref");
    println!("               ← mut_ref");
    println!("❌ Cannot have both simultaneously");
}

// TODO: Add more visualization functions
pub fn visualize_smart_pointer_patterns() {
    println!("{}", "=== Smart Pointer Patterns ===".bold());
    
    // TODO: Implement visualizations for:
    // - Box<T> heap allocation
    // - Rc<T> reference counting
    // - Arc<T> thread-safe sharing
    // - RefCell<T> interior mutability
    // - Combined patterns like Rc<RefCell<T>>
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visualizer_creation() {
        let visualizer = MemoryVisualizer::new();
        assert!(visualizer.stack_items.is_empty());
        assert!(visualizer.heap_items.is_empty());
    }
    
    #[test]
    fn test_add_stack_value() {
        let mut visualizer = MemoryVisualizer::new();
        visualizer.add_stack_value("x", "i32");
        
        assert_eq!(visualizer.stack_items.len(), 1);
        assert_eq!(visualizer.stack_items[0].name, "x");
    }
    
    // TODO: Add more tests for visualization functionality
}
