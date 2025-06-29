# Lesson 04: Smart Pointers

## 🎯 Learning Objectives

By the end of this lesson, you will:
- Understand when and why to use smart pointers
- Master Box<T> for heap allocation
- Use Rc<T> and Arc<T> for shared ownership
- Apply RefCell<T> for interior mutability
- Build data structures using smart pointers
- Choose the right smart pointer for your use case

## 📚 Introduction

So far, we've seen that Rust enforces single ownership. But what if you need multiple owners? Or what if you need to mutate data that's behind an immutable reference? This is where smart pointers come in.

Smart pointers are data structures that act like pointers but have additional metadata and capabilities. Coming from C#, you might think of them as similar to reference counting or weak references, but with compile-time guarantees.

## 📦 Box<T> - Heap Allocation

`Box<T>` is the simplest smart pointer. It allocates values on the heap rather than the stack.

### When to Use Box

1. **Recursive types** (unknown size at compile time)
2. **Large data** you want to move without copying
3. **Trait objects** (dynamic dispatch)

### Basic Usage

```rust
fn main() {
    // Allocate an integer on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Dereference like a regular reference
    let sum = *b + 10;
    println!("sum = {}", sum);
} // b is dropped, heap memory is freed
```

### Recursive Types

Without Box, this won't compile:

```rust
// ERROR: recursive type has infinite size
enum List {
    Cons(i32, List),
    Nil,
}
```

With Box:

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
    
    println!("list = {:?}", list);
}
```

### C# Comparison

```csharp
// C# - all reference types are heap allocated
class Node {
    public int Value { get; set; }
    public Node Next { get; set; }  // Always a heap reference
}

// Rust - explicit heap allocation
struct Node {
    value: i32,
    next: Option<Box<Node>>,  // Explicitly heap allocated
}
```

## 🔄 Rc<T> - Reference Counting

`Rc<T>` enables multiple ownership through reference counting. It's similar to C#'s reference counting for COM objects, but it's not thread-safe.

### When to Use Rc

- Multiple parts of your program need to read the same data
- You can't determine which part will finish using it last
- Single-threaded scenarios only

### Basic Usage

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);
    println!("Initial count: {}", Rc::strong_count(&data));
    
    {
        let data2 = Rc::clone(&data);  // Increment reference count
        println!("Count after clone: {}", Rc::strong_count(&data));
        
        let data3 = Rc::clone(&data);
        println!("Count with 3 refs: {}", Rc::strong_count(&data));
    } // data2 and data3 dropped, count decremented
    
    println!("Final count: {}", Rc::strong_count(&data));
}
```

### Graph Structure Example

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    neighbors: Vec<Rc<Node>>,
}

fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        neighbors: vec![],
    });
    
    let node2 = Rc::new(Node {
        value: 2,
        neighbors: vec![Rc::clone(&node1)],
    });
    
    let node3 = Rc::new(Node {
        value: 3,
        neighbors: vec![Rc::clone(&node1), Rc::clone(&node2)],
    });
    
    println!("Node 3 has {} neighbors", node3.neighbors.len());
}
```

## 🧵 Arc<T> - Atomic Reference Counting

`Arc<T>` is the thread-safe version of `Rc<T>`. Use it when you need to share data between threads.

### Arc vs Rc

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
            println!("Thread {} sum: {}", i, sum);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## 🔐 RefCell<T> - Interior Mutability

`RefCell<T>` enables mutation even when behind an immutable reference. It moves Rust's borrowing rules from compile time to runtime.

### When to Use RefCell

- You need to mutate data inside an immutable structure
- Mock objects in tests
- Implementing certain patterns (like observer)

### Basic Usage

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Borrow immutably
    {
        let value = data.borrow();
        println!("Value: {}", *value);
    } // borrow ends here
    
    // Borrow mutably
    {
        let mut value = data.borrow_mut();
        *value += 10;
    } // mutable borrow ends here
    
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
}
```

## 🎯 Combining Smart Pointers

Often you'll combine smart pointers for complex scenarios:

### Rc<RefCell<T>> - Shared Mutable Data

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct SharedData {
    value: i32,
}

fn main() {
    let data = Rc::new(RefCell::new(SharedData { value: 0 }));
    
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);
    
    // Multiple owners can mutate
    data1.borrow_mut().value += 10;
    data2.borrow_mut().value += 20;
    
    println!("Final value: {}", data.borrow().value);  // 30
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
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = TreeNode::new(0);
    let child1 = TreeNode::new(1);
    let child2 = TreeNode::new(2);
    
    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    
    println!("Root has {} children", root.children.borrow().len());
}
```

## 💡 Choosing the Right Smart Pointer

| Use Case | Smart Pointer | Why |
|----------|--------------|-----|
| Heap allocation | `Box<T>` | Single owner, heap data |
| Shared ownership (single-threaded) | `Rc<T>` | Multiple readers |
| Shared ownership (multi-threaded) | `Arc<T>` | Thread-safe |
| Interior mutability | `RefCell<T>` | Mutate through & |
| Shared mutable (single-threaded) | `Rc<RefCell<T>>` | Multiple owners can mutate |
| Shared mutable (multi-threaded) | `Arc<Mutex<T>>` | Thread-safe mutation |

## 🐛 Common Pitfalls

### Reference Cycles

```rust
use std::rc::Rc;
use std::cell::RefCell;

// This creates a reference cycle - memory leak!
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

// Solution: Use Weak<T> for one direction
struct BetterNode {
    next: Option<Rc<RefCell<BetterNode>>>,
    prev: Option<Weak<RefCell<BetterNode>>>,  // Weak reference
}
```

### RefCell Panics

```rust
let data = RefCell::new(5);
let r1 = data.borrow_mut();
let r2 = data.borrow_mut();  // PANIC at runtime!
```

## 📝 Key Takeaways

1. **Box<T>**: Heap allocation with single ownership
2. **Rc<T>**: Reference counting for shared ownership
3. **Arc<T>**: Thread-safe reference counting
4. **RefCell<T>**: Interior mutability with runtime checks
5. **Weak<T>**: Break reference cycles
6. **Combine as Needed**: `Rc<RefCell<T>>` for shared mutability

## 🔗 Comparison with C#

| C# Concept | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| All ref types | `Box<T>` | Explicit heap allocation |
| Reference counting | `Rc<T>`/`Arc<T>` | Manual, no GC |
| `lock` statement | `Mutex<T>` | Type-based locking |
| Weak references | `Weak<T>` | Prevents cycles |
| Mutable fields | `RefCell<T>` | Runtime borrow checking |

## ✏️ Practice Exercises

1. **Linked List**: Implement a linked list using `Box<T>`

2. **Graph Structure**: Build a graph where nodes can have multiple connections using `Rc<T>`

3. **Mock Object**: Create a mock object for testing using `RefCell<T>`

4. **Thread-Safe Counter**: Implement a counter that can be incremented from multiple threads

---

Congratulations! You've completed the Ownership and Borrowing module. These concepts are fundamental to writing safe and efficient Rust code.

Next Module: [03 - Error Handling](../03-error-handling/README.md) →
