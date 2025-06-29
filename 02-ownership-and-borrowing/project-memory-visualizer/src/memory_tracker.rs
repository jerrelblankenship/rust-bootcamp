// Memory Tracker - Core memory operation tracking
//
// TODO: Complete the missing implementations to track memory operations

use colored::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Allocate,
    Deallocate,
    Move,
    Borrow,
    MutableBorrow,
    Clone,
    Drop,
}

#[derive(Debug, Clone)]
pub struct MemoryOperation {
    pub operation: Operation,
    pub location: String,
    pub value_type: String,
    pub size: usize,
    pub step: usize,
}

pub struct MemoryTracker {
    operations: Vec<MemoryOperation>,
    active_allocations: HashMap<String, usize>,
    current_step: usize,
    total_allocated: usize,
    total_deallocated: usize,
}

impl MemoryTracker {
    pub fn new() -> Self {
        // TODO: Initialize the MemoryTracker with empty state
        // HINT: All fields should start with appropriate default values
        todo!("Initialize MemoryTracker struct")
    }
    
    pub fn track_allocation(&mut self, location: &str, type_name: &str, size: usize) {
        self.current_step += 1;
        
        // TODO: Record the allocation operation
        // HINT: Create a MemoryOperation and add it to self.operations
        // HINT: Update self.active_allocations and self.total_allocated
        
        println!("📦 {} Allocated {} bytes for {} at {}",
            "ALLOC".green().bold(),
            size.to_string().yellow(),
            type_name.blue(),
            location.cyan()
        );
    }
    
    pub fn track_deallocation(&mut self, location: &str, size: usize) {
        self.current_step += 1;
        
        // TODO: Record the deallocation operation
        // HINT: Remove from active_allocations, update total_deallocated
        // HINT: Add operation to the operations vec
        
        println!("🗑️  {} Deallocated {} bytes from {}",
            "FREE".red().bold(),
            size.to_string().yellow(),
            location.cyan()
        );
    }
    
    pub fn track_move(&mut self, from: &str, to: &str, type_name: &str) {
        self.current_step += 1;
        
        // TODO: Record the move operation
        // HINT: Update active_allocations to reflect new ownership
        // HINT: A move transfers ownership without changing memory size
        
        let operation = MemoryOperation {
            operation: Operation::Move,
            location: format!("{} -> {}", from, to),
            value_type: type_name.to_string(),
            size: 0, // Moves don't allocate/deallocate
            step: self.current_step,
        };
        
        self.operations.push(operation);
        
        println!("➡️  {} Ownership moved from {} to {}",
            "MOVE".purple().bold(),
            from.cyan(),
            to.cyan()
        );
    }
    
    pub fn track_borrow(&mut self, borrower: &str, borrowed_from: &str, mutable: bool) {
        self.current_step += 1;
        
        let operation_type = if mutable { Operation::MutableBorrow } else { Operation::Borrow };
        let symbol = if mutable { "🔓" } else { "🔒" };
        let borrow_type = if mutable { "BORROW_MUT".yellow() } else { "BORROW".green() };
        
        // TODO: Record the borrow operation
        // HINT: Create and store the MemoryOperation
        
        println!("{} {} {} borrows from {}",
            symbol,
            borrow_type.bold(),
            borrower.cyan(),
            borrowed_from.cyan()
        );
    }
    
    pub fn track_clone(&mut self, original: &str, clone: &str, type_name: &str, size: usize) {
        // TODO: Track both the clone operation and the new allocation
        // HINT: A clone creates a new allocation of the same size
        // HINT: This is different from a move - both values exist after cloning
        
        println!("🔄 {} Cloned {} to {} ({} bytes)",
            "CLONE".blue().bold(),
            original.cyan(),
            clone.cyan(),
            size.to_string().yellow()
        );
    }
    
    pub fn print_summary(&self) {
        println!("\n{}", "=== Memory Operations Summary ===".bold());
        println!("Total operations: {}", self.operations.len().to_string().yellow());
        println!("Total allocated: {} bytes", self.total_allocated.to_string().green());
        println!("Total deallocated: {} bytes", self.total_deallocated.to_string().red());
        
        let active_memory: usize = self.active_allocations.values().sum();
        println!("Currently active: {} bytes", active_memory.to_string().blue());
        
        if active_memory == 0 {
            println!("{}", "✅ All memory properly cleaned up!".green().bold());
        } else {
            println!("{}", "⚠️  Some memory still allocated".yellow().bold());
        }
        
        // TODO: Add more detailed summary information
        // IDEAS: Show operation breakdown, largest allocations, etc.
        
        self.print_operation_breakdown();
    }
    
    fn print_operation_breakdown(&self) {
        println!("\nOperation breakdown:");
        let mut counts = HashMap::new();
        
        // TODO: Count each operation type and display the results
        // HINT: Iterate through self.operations and count each Operation variant
        
        for (op_type, count) in counts {
            println!("  {:?}: {}", op_type, count);
        }
    }
    
    pub fn visualize_memory_state(&self) {
        println!("\n{}", "=== Current Memory State ===".bold());
        
        if self.active_allocations.is_empty() {
            println!("📭 No active allocations");
            return;
        }
        
        println!("Active allocations:");
        for (location, size) in &self.active_allocations {
            let blocks = size / 8; // Represent each 8 bytes as a block
            let visualization = "█".repeat(blocks.max(1));
            println!("  {}: {} ({} bytes)", 
                location.cyan(), 
                visualization.green(), 
                size.to_string().yellow()
            );
        }
        
        // TODO: Add more sophisticated visualization
        // IDEAS: Stack vs heap representation, reference arrows, etc.
    }
    
    // Helper method for testing
    pub fn get_active_allocations(&self) -> &HashMap<String, usize> {
        &self.active_allocations
    }
    
    pub fn get_total_operations(&self) -> usize {
        self.operations.len()
    }
    
    // TODO: Add method to get operations by type
    pub fn get_operations_by_type(&self, op_type: Operation) -> Vec<&MemoryOperation> {
        // HINT: Filter self.operations by the given operation type
        todo!("Filter operations by type")
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
    fn test_memory_tracker_creation() {
        let tracker = MemoryTracker::new();
        assert_eq!(tracker.get_total_operations(), 0);
        assert!(tracker.get_active_allocations().is_empty());
    }
    
    #[test]
    fn test_track_allocation() {
        let mut tracker = MemoryTracker::new();
        
        // TODO: Test allocation tracking
        // HINT: Call track_allocation and verify the state changes
        
        // tracker.track_allocation("var1", "String", 10);
        // Add assertions here
    }
    
    #[test]
    fn test_track_move() {
        let mut tracker = MemoryTracker::new();
        
        // TODO: Test move tracking
        // HINT: First allocate, then move, then verify state
        
        // Add your test implementation
    }
    
    #[test]
    fn test_memory_cleanup() {
        let mut tracker = MemoryTracker::new();
        
        // TODO: Test that allocation + deallocation results in clean state
        // This simulates proper RAII cleanup
        
        // Add your test implementation
    }
}
