# Lesson 04: Smart Pointers

Learn when and how to use Rust's smart pointers to handle complex ownership scenarios. This lesson covers Box, Rc, Arc, RefCell, and how to combine them for powerful data structures.

## 🎯 Learning Objectives

- Understand when single ownership isn't sufficient
- Master Box<T> for heap allocation and recursive types
- Use Rc<T> and Arc<T> for shared ownership scenarios
- Apply RefCell<T> for interior mutability patterns
- Combine smart pointers for complex data structures
- Choose the right smart pointer for your specific use case
- Avoid common pitfalls like reference cycles

## 📚 Introduction

So far, we've learned that Rust enforces single ownership to ensure memory safety. But real-world programs often need more flexible ownership patterns: multiple owners of the same data, heap allocation for large objects, or interior mutability for certain design patterns.

Smart pointers are data structures that act like pointers but provide additional metadata and capabilities. Coming from C#, you can think of them as more explicit versions of reference counting or weak references, but with compile-time guarantees.

## 📦 Box<T> - Heap Allocation

`Box<T>` is the simplest smart pointer. It allocates values on the heap instead of the stack.

### When to Use Box<T>

1. **Recursive types** with unknown size at compile time
2. **Large data structures** you want to move without copying
3. **Trait objects** for dynamic dispatch

### Basic Usage

```rust
fn main() {
    // Allocate an integer on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Automatic dereferencing
    let sum = *b + 10;
    println!("sum = {}", sum);
} // b goes out of scope, heap memory is automatically freed
```

### Recursive Types

This won't compile because the size is infinite:

```rust
// ERROR: recursive type has infinite size
enum List {
    Cons(i32, List),
    Nil,
}
```

**Fix with Box:**
```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> List {
        Nil
    }
    
    fn prepend(self, elem: i32) -> List {
        Cons(elem, Box::new(self))
    }
    
    fn len(&self) -> usize {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    
    println!("List: {:?}", list);
    println!("Length: {}", list.len());
}
```

## 🔄 Rc<T> - Reference Counting

`Rc<T>` (Reference Counted) enables multiple ownership through reference counting. Use it when multiple parts of your program need to read the same data.

### When to Use Rc<T>

- Multiple parts need to read the same data
- You can't determine which part will finish using it last
- **Single-threaded scenarios only**

### Basic Usage

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("Initial count: {}", Rc::strong_count(&data));
    
    {
        let data2 = Rc::clone(&data);  // Increment reference count
        let data3 = Rc::clone(&data);  // Note: Rc::clone, not data.clone()
        
        println!("Count with 3 refs: {}", Rc::strong_count(&data));
        println!("data2: {:?}", data2);
        println!("data3: {:?}", data3);
    } // data2 and data3 dropped, count decremented
    
    println!("Final count: {}", Rc::strong_count(&data));
}
```

### Graph Structure with Rc

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            neighbors: RefCell::new(vec![]),
        })
    }
    
    fn add_neighbor(&self, neighbor: Rc<Node>) {
        self.neighbors.borrow_mut().push(neighbor);
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    
    // Create bidirectional connections
    node1.add_neighbor(Rc::clone(&node2));
    node1.add_neighbor(Rc::clone(&node3));
    node2.add_neighbor(Rc::clone(&node1));
    node3.add_neighbor(Rc::clone(&node1));
    
    println!("Node 1 neighbors: {}", node1.neighbors.borrow().len());
    println!("Node 1 reference count: {}", Rc::strong_count(&node1));
}
```

## 🧵 Arc<T> - Atomic Reference Counting

`Arc<T>` (Atomically Reference Counted) is the thread-safe version of `Rc<T>`.

### When to Use Arc<T>

- Need to share data between multiple threads
- Want reference counting with thread safety

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let numbers = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let sum: i32 = numbers.iter().sum();
            println!("Thread {} calculated sum: {}", i, sum);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Main thread can still use: {:?}", numbers);
}
```

## 🔐 RefCell<T> - Interior Mutability

`RefCell<T>` allows mutation of data even when behind an immutable reference by moving borrowing rules from compile-time to runtime.

### When to Use RefCell<T>

- Need to mutate data inside an otherwise immutable structure
- Implementing mock objects for testing
- Certain design patterns require interior mutability

### Basic Usage

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Immutable borrow
    {
        let value = data.borrow();
        println!("Value: {}", *value);
    } // Borrow ends here
    
    // Mutable borrow
    {
        let mut value = data.borrow_mut();
        *value += 10;
    } // Mutable borrow ends here
    
    println!("New value: {}", *data.borrow());
}
```

### Runtime Borrowing Rules

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    let r1 = data.borrow();
    let r2 = data.borrow();  // OK - multiple immutable borrows
    
    // This would panic at runtime:
    // let r3 = data.borrow_mut();  // PANIC: already borrowed
    
    drop(r1);
    drop(r2);
    
    let r3 = data.borrow_mut();  // OK now
    println!("Mutable access: {}", *r3);
}
```

## 🎯 Combining Smart Pointers

Real-world scenarios often require combining smart pointers:

### Rc<RefCell<T>> - Shared Mutable Data

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Rc<RefCell<Counter>> {
        Rc::new(RefCell::new(Counter { value: 0 }))
    }
    
    fn increment(&self) {
        self.value += 1;
    }
    
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let counter = Counter::new();
    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);
    
    // Multiple owners can mutate the same data
    counter1.borrow_mut().increment();
    counter2.borrow_mut().increment();
    counter.borrow_mut().increment();
    
    println!("Final count: {}", counter.borrow().get());
}
```

### Tree with Parent References

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
    
    fn print_tree(&self, depth: usize) {
        println!("{}{}", "  ".repeat(depth), self.value);
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
}

fn main() {
    let root = TreeNode::new(0);
    let child1 = TreeNode::new(1);
    let child2 = TreeNode::new(2);
    let grandchild = TreeNode::new(3);
    
    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    TreeNode::add_child(&child1, grandchild);
    
    println!("Tree structure:");
    root.print_tree(0);
    
    // Check parent relationships
    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("Child 1's parent value: {}", parent.value);
    }
}
```

## 💡 Smart Pointer Selection Guide

| Use Case | Smart Pointer | Reason |
|----------|---------------|---------|
| Heap allocation | `Box<T>` | Single owner, heap allocation |
| Recursive types | `Box<T>` | Fixed size on stack |
| Shared ownership (single-threaded) | `Rc<T>` | Multiple readers |
| Shared ownership (multi-threaded) | `Arc<T>` | Thread-safe sharing |
| Interior mutability | `RefCell<T>` | Runtime borrow checking |
| Shared mutable (single-threaded) | `Rc<RefCell<T>>` | Multiple owners, mutation |
| Shared mutable (multi-threaded) | `Arc<Mutex<T>>` | Thread-safe mutation |
| Break reference cycles | `Weak<T>` | Prevent memory leaks |

## 🔄 Comparison with C#

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| All reference types | `Box<T>` | Explicit heap allocation |
| Object references | `Rc<T>` | Explicit reference counting |
| Thread-safe sharing | `Arc<T>` | No GC, manual counting |
| `lock` statement | `Mutex<T>` | Type-level thread safety |
| Weak references | `Weak<T>` | Prevents reference cycles |
| Field mutation | `RefCell<T>` | Runtime borrow checking |

## 💻 Practice Exercises

### Exercise 1: Binary Tree with Box

```rust
// Implement a binary search tree using Box<T>
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        // Implement constructor
    }
    
    fn insert(&mut self, value: i32) {
        // Implement insertion logic
    }
    
    fn contains(&self, value: i32) -> bool {
        // Implement search logic
    }
}

fn main() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);
    root.insert(7);
    
    println!("Tree: {:?}", root);
    println!("Contains 7: {}", root.contains(7));
    println!("Contains 12: {}", root.contains(12));
}
```

### Exercise 2: Shared Cache with Rc

```rust
use std::rc::Rc;
use std::collections::HashMap;

// Implement a shared cache that multiple components can read from
struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    fn new() -> Rc<Cache> {
        // Implement constructor
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        // Implement get method
    }
}

// Simulate multiple components using the same cache
fn component_a(cache: Rc<Cache>) {
    if let Some(value) = cache.get("config") {
        println!("Component A found: {}", value);
    }
}

fn component_b(cache: Rc<Cache>) {
    if let Some(value) = cache.get("setting") {
        println!("Component B found: {}", value);
    }
}

fn main() {
    let cache = Cache::new();
    
    component_a(Rc::clone(&cache));
    component_b(Rc::clone(&cache));
}
```

### Exercise 3: Mock Object with RefCell

```rust
use std::cell::RefCell;

// Create a mock database that tracks method calls
struct MockDatabase {
    calls: RefCell<Vec<String>>,
    responses: RefCell<HashMap<String, String>>,
}

impl MockDatabase {
    fn new() -> Self {
        // Implement constructor
    }
    
    fn set_response(&self, key: String, value: String) {
        // Add response to mock
    }
    
    fn get(&self, key: &str) -> Option<String> {
        // Record the call and return mock response
    }
    
    fn get_calls(&self) -> Vec<String> {
        // Return all recorded calls
    }
}

fn main() {
    let db = MockDatabase::new();
    db.set_response("user:1".to_string(), "Alice".to_string());
    
    // Use the database
    println!("User: {:?}", db.get("user:1"));
    println!("User: {:?}", db.get("user:2"));
    
    // Check what calls were made
    println!("Calls made: {:?}", db.get_calls());
}
```

## 🚀 Mini-Project: Multi-Owner Graph

Build a graph data structure where nodes can have multiple owners:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
struct GraphNode {
    id: usize,
    value: String,
    edges: RefCell<Vec<Weak<GraphNode>>>,
}

struct Graph {
    nodes: HashMap<usize, Rc<GraphNode>>,
    next_id: usize,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            next_id: 0,
        }
    }
    
    fn add_node(&mut self, value: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Rc::new(GraphNode {
            id,
            value,
            edges: RefCell::new(vec![]),
        });
        
        self.nodes.insert(id, node);
        id
    }
    
    fn add_edge(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        let to_node = self.nodes.get(&to)
            .ok_or("Target node not found")?;
        
        if let Some(from_node) = self.nodes.get(&from) {
            from_node.edges.borrow_mut().push(Rc::downgrade(to_node));
            Ok(())
        } else {
            Err("Source node not found")
        }
    }
    
    fn print_graph(&self) {
        for (id, node) in &self.nodes {
            print!("Node {} ({}): ", id, node.value);
            
            let valid_edges: Vec<_> = node.edges.borrow()
                .iter()
                .filter_map(|weak_ref| weak_ref.upgrade())
                .collect();
            
            let edge_ids: Vec<_> = valid_edges.iter()
                .map(|node| node.id)
                .collect();
            
            println!("-> {:?}", edge_ids);
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    
    let a = graph.add_node("A".to_string());
    let b = graph.add_node("B".to_string());
    let c = graph.add_node("C".to_string());
    
    graph.add_edge(a, b).unwrap();
    graph.add_edge(a, c).unwrap();
    graph.add_edge(b, c).unwrap();
    graph.add_edge(c, a).unwrap();
    
    graph.print_graph();
}
```

## 🔑 Key Takeaways

1. **Box<T> for heap allocation**: Use when you need owned heap data or recursive types
2. **Rc<T> for shared ownership**: Multiple owners in single-threaded scenarios
3. **Arc<T> for thread-safe sharing**: Multiple owners across threads
4. **RefCell<T> for interior mutability**: Mutate data through immutable references
5. **Combine as needed**: `Rc<RefCell<T>>` for shared mutable state
6. **Weak<T> prevents cycles**: Use to break reference cycles and prevent memory leaks
7. **Choose wisely**: Each smart pointer has specific use cases and trade-offs

## 📚 Additional Resources

- [Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust by Example - Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html)
- [Understanding Rc and RefCell](https://ricardomartins.cc/2016/06/08/interior-mutability)

## ✅ Checklist

Before moving on, ensure you can:
- [ ] Choose the appropriate smart pointer for different scenarios
- [ ] Use Box<T> for heap allocation and recursive types
- [ ] Share ownership safely with Rc<T> and Arc<T>
- [ ] Apply interior mutability patterns with RefCell<T>
- [ ] Combine smart pointers for complex data structures
- [ ] Avoid reference cycles using Weak<T>

---

**Congratulations!** You've mastered Rust's ownership system - the foundation that makes Rust both safe and fast. You now understand the concepts that eliminate entire classes of bugs that plague other systems programming languages.

Next Module: [03 - Error Handling](../03-error-handling/README.md) - Learn Rust's approach to handling errors without exceptions →
