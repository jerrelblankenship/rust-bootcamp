# Exercise 4 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working implementation. Here's the full solution with detailed explanations.

## 📝 Complete ex04-smart-pointers.rs Implementation

```rust
// Exercise 4: Smart Pointers - Complete Solutions
//
// This file demonstrates all smart pointer concepts from Module 02

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

fn main() {
    println!("=== Exercise 4: Smart Pointers (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_4_1();
    exercise_4_2();
    exercise_4_3();
    exercise_4_4();
    exercise_4_5();
    exercise_4_6();
    exercise_4_7();
    exercise_4_8();
    
    println!("\n🎉 All smart pointer exercises completed!");
    
    // Demonstrate advanced patterns
    demonstrate_advanced_patterns();
}

// Exercise 4.1: Box<T> for heap allocation - SOLVED
fn exercise_4_1() {
    println!("Exercise 4.1: Box<T> for heap allocation");
    
    // SOLUTION: Use Box to break recursive type cycle
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3, 
                Box::new(List::Nil))))));
    
    println!("Created list: {:?}", list);
    
    // Demonstrate accessing boxed data
    print_list(&list);
    println!("List length: {}", list_length(&list));
    
    println!("✅ Exercise 4.1 complete\n");
}

// SOLUTION: Recursive type using Box
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),  // Box provides indirection
    Nil,
}

impl List {
    // Helper methods for the list
    fn new() -> List {
        List::Nil
    }
    
    fn prepend(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }
}

fn print_list(list: &List) {
    match list {
        List::Cons(head, tail) => {
            print!("{} -> ", head);
            print_list(tail);
        }
        List::Nil => println!("Nil"),
    }
}

fn list_length(list: &List) -> usize {
    match list {
        List::Cons(_, tail) => 1 + list_length(tail),
        List::Nil => 0,
    }
}

// Exercise 4.2: Box with heap-allocated data - SOLVED
fn exercise_4_2() {
    println!("Exercise 4.2: Box with heap-allocated data");
    
    // SOLUTION: Use Box to allocate large data on heap
    let heap_array = Box::new([0; 1000000]);  // 1 million integers on heap
    
    println!("Array size: {}", heap_array.len());
    
    // Demonstrate operations on boxed data
    println!("First element: {}", heap_array[0]);
    println!("Last element: {}", heap_array[heap_array.len() - 1]);
    
    // Box automatically dereferences
    let sum: i32 = heap_array.iter().take(10).sum();
    println!("Sum of first 10 elements: {}", sum);
    
    // Show memory usage difference
    demonstrate_stack_vs_heap();
    
    println!("✅ Exercise 4.2 complete\n");
}

fn demonstrate_stack_vs_heap() {
    println!("Memory allocation comparison:");
    
    // Small data on stack (fast, limited space)
    let stack_array = [1, 2, 3, 4, 5];
    println!("Stack array: {:?}", stack_array);
    
    // Large data on heap (slower access, unlimited space)
    let heap_vec = Box::new(vec![1; 100]);
    println!("Heap vector size: {}", heap_vec.len());
    
    // Box has pointer size on stack, data on heap
    println!("Box size: {} bytes", std::mem::size_of::<Box<Vec<i32>>>());
    println!("Vec size: {} bytes", std::mem::size_of::<Vec<i32>>());
}

// Exercise 4.3: Rc<T> for shared ownership - SOLVED
fn exercise_4_3() {
    println!("Exercise 4.3: Rc<T> for shared ownership");
    
    // SOLUTION: Use Rc for multiple owners
    let data = Rc::new(String::from("shared data"));
    
    // Create multiple owners by cloning the Rc (not the data)
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    let owner3 = Rc::clone(&data);
    
    println!("Owner 1: {}", owner1);
    println!("Owner 2: {}", owner2);
    println!("Owner 3: {}", owner3);
    println!("Original: {}", data);
    
    // Demonstrate reference counting
    println!("Reference count: {}", Rc::strong_count(&data));
    
    // Show memory efficiency - all point to same data
    demonstrate_rc_efficiency();
    
    println!("✅ Exercise 4.3 complete\n");
}

fn demonstrate_rc_efficiency() {
    let expensive_data = Rc::new(generate_expensive_data());
    
    // Multiple owners share the same data
    let users = vec![
        Rc::clone(&expensive_data),
        Rc::clone(&expensive_data),
        Rc::clone(&expensive_data),
    ];
    
    println!("Sharing expensive data among {} users", users.len());
    println!("Data only exists once in memory");
    println!("Reference count: {}", Rc::strong_count(&expensive_data));
}

fn generate_expensive_data() -> Vec<i32> {
    (0..1000).collect()  // Simulate expensive computation
}

// Exercise 4.4: Rc reference counting - SOLVED
fn exercise_4_4() {
    println!("Exercise 4.4: Rc reference counting");
    
    let data = Rc::new(String::from("counted data"));
    println!("Initial reference count: {}", Rc::strong_count(&data));
    
    // SOLUTION: Clone the Rc instead of moving
    let ref1 = Rc::clone(&data);
    println!("After ref1: {}", Rc::strong_count(&data));
    
    let ref2 = Rc::clone(&data);
    println!("After ref2: {}", Rc::strong_count(&ref1));
    
    let ref3 = Rc::clone(&data);
    println!("After ref3: {}", Rc::strong_count(&ref2));
    
    // Demonstrate dropping references
    {
        let temp_ref = Rc::clone(&data);
        println!("With temp_ref: {}", Rc::strong_count(&data));
    } // temp_ref dropped here
    println!("After temp_ref dropped: {}", Rc::strong_count(&data));
    
    // Explicit drops
    drop(ref1);
    println!("After dropping ref1: {}", Rc::strong_count(&ref2));
    
    drop(ref2);
    println!("After dropping ref2: {}", Rc::strong_count(&ref3));
    
    drop(ref3);
    println!("After dropping ref3: {}", Rc::strong_count(&data));
    
    // Demonstrate weak references
    demonstrate_weak_references();
    
    println!("✅ Exercise 4.4 complete\n");
}

fn demonstrate_weak_references() {
    use std::rc::Weak;
    
    println!("\nWeak reference demonstration:");
    let strong = Rc::new(String::from("Strong reference"));
    let weak: Weak<String> = Rc::downgrade(&strong);
    
    println!("Strong count: {}", Rc::strong_count(&strong));
    println!("Weak count: {}", Rc::weak_count(&strong));
    
    // Try to upgrade weak to strong
    if let Some(upgraded) = weak.upgrade() {
        println!("Weak upgrade successful: {}", upgraded);
    }
    
    drop(strong);  // Drop strong reference
    
    // Weak reference can't be upgraded now
    if weak.upgrade().is_none() {
        println!("Weak upgrade failed - data was dropped");
    }
}

// Exercise 4.5: RefCell<T> for interior mutability - SOLVED
fn exercise_4_5() {
    println!("Exercise 4.5: RefCell<T> for interior mutability");
    
    let counter = Counter::new();
    
    // Works - can modify through immutable reference
    counter.increment();
    counter.increment();
    
    println!("Final count: {}", counter.get());
    
    // Demonstrate runtime borrowing rules
    counter.demonstrate_borrowing_rules();
    
    println!("✅ Exercise 4.5 complete\n");
}

// SOLUTION: Counter using RefCell for interior mutability
struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { 
            count: RefCell::new(0) 
        }
    }
    
    // Takes &self but can modify through RefCell
    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }
    
    fn get(&self) -> i32 {
        *self.count.borrow()
    }
    
    fn add(&self, value: i32) {
        *self.count.borrow_mut() += value;
    }
    
    fn reset(&self) {
        *self.count.borrow_mut() = 0;
    }
    
    fn demonstrate_borrowing_rules(&self) {
        println!("Demonstrating RefCell borrowing rules:");
        
        // Multiple immutable borrows OK
        {
            let borrow1 = self.count.borrow();
            let borrow2 = self.count.borrow();
            println!("Multiple immutable borrows: {} {}", borrow1, borrow2);
        } // Borrows end here
        
        // Single mutable borrow OK
        {
            let mut mutable_borrow = self.count.borrow_mut();
            *mutable_borrow += 10;
            println!("Mutable borrow: {}", mutable_borrow);
        } // Mutable borrow ends here
        
        // This would panic at runtime:
        // let _immut = self.count.borrow();
        // let _mut = self.count.borrow_mut();  // PANIC!
    }
}

// Exercise 4.6: Combining Rc and RefCell - SOLVED
fn exercise_4_6() {
    println!("Exercise 4.6: Combining Rc and RefCell");
    
    let shared_counter = SharedCounter::new();
    
    // Create multiple owners
    let counter1 = shared_counter.clone();
    let counter2 = shared_counter.clone();
    let counter3 = shared_counter.clone();
    
    // All can increment the same counter
    counter1.increment();
    counter2.increment();
    counter3.increment();
    
    println!("Final count: {}", counter1.get());
    println!("Same count from counter2: {}", counter2.get());
    println!("Same count from counter3: {}", counter3.get());
    
    // Demonstrate shared mutation
    counter1.add(10);
    println!("After adding 10: {}", counter2.get());
    
    // Show reference counting
    println!("Shared references: {}", counter1.reference_count());
    
    println!("✅ Exercise 4.6 complete\n");
}

// SOLUTION: SharedCounter using Rc<RefCell<T>>
struct SharedCounter {
    inner: Rc<RefCell<i32>>,
}

impl SharedCounter {
    fn new() -> Self {
        SharedCounter {
            inner: Rc::new(RefCell::new(0)),
        }
    }
    
    fn increment(&self) {
        *self.inner.borrow_mut() += 1;
    }
    
    fn get(&self) -> i32 {
        *self.inner.borrow()
    }
    
    fn add(&self, value: i32) {
        *self.inner.borrow_mut() += value;
    }
    
    fn reset(&self) {
        *self.inner.borrow_mut() = 0;
    }
    
    fn reference_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

// SOLUTION: Clone implementation that clones the Rc, not the data
impl Clone for SharedCounter {
    fn clone(&self) -> Self {
        SharedCounter {
            inner: Rc::clone(&self.inner),
        }
    }
}

// Exercise 4.7: Arc<T> for thread-safe sharing - SOLVED
fn exercise_4_7() {
    println!("Exercise 4.7: Arc<T> for thread-safe sharing");
    
    // SOLUTION: Use Arc instead of Rc for thread safety
    let data = Arc::new(String::from("thread data"));
    
    // Clone Arc to move into thread
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("Thread data: {}", data_clone);
        thread::sleep(std::time::Duration::from_millis(100));
        println!("Thread finished processing: {}", data_clone);
    });
    
    // Main thread can still use original
    println!("Main thread data: {}", data);
    
    handle.join().unwrap();
    
    // Demonstrate multiple threads sharing data
    demonstrate_multi_thread_sharing();
    
    println!("✅ Exercise 4.7 complete\n");
}

fn demonstrate_multi_thread_sharing() {
    println!("Multi-thread sharing demonstration:");
    
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    // Spawn multiple threads
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("Thread {}: data = {:?}", i, data);
            let sum: i32 = data.iter().sum();
            println!("Thread {}: sum = {}", i, sum);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Reference count: {}", Arc::strong_count(&shared_data));
}

// Exercise 4.8: Complex smart pointer combinations - SOLVED
fn exercise_4_8() {
    println!("Exercise 4.8: Complex smart pointer combinations");
    
    let shared_list = SharedList::new();
    
    // Add some items
    shared_list.push(1);
    shared_list.push(2);
    shared_list.push(3);
    
    println!("List length: {}", shared_list.len());
    println!("List contents: {:?}", shared_list.get_all());
    
    // Demonstrate sharing and concurrent access
    demonstrate_concurrent_access(shared_list.clone());
    
    println!("After concurrent access: {:?}", shared_list.get_all());
    
    println!("✅ Exercise 4.8 complete\n");
}

// SOLUTION: Thread-safe shared list using Arc<Mutex<T>>
struct SharedList {
    inner: Arc<Mutex<Vec<i32>>>,
}

impl SharedList {
    fn new() -> Self {
        SharedList {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn push(&self, item: i32) {
        let mut list = self.inner.lock().unwrap();
        list.push(item);
    }
    
    fn len(&self) -> usize {
        let list = self.inner.lock().unwrap();
        list.len()
    }
    
    fn get_all(&self) -> Vec<i32> {
        let list = self.inner.lock().unwrap();
        list.clone()
    }
    
    fn pop(&self) -> Option<i32> {
        let mut list = self.inner.lock().unwrap();
        list.pop()
    }
    
    fn clear(&self) {
        let mut list = self.inner.lock().unwrap();
        list.clear();
    }
    
    fn contains(&self, item: &i32) -> bool {
        let list = self.inner.lock().unwrap();
        list.contains(item)
    }
}

// SOLUTION: Clone implementation for SharedList
impl Clone for SharedList {
    fn clone(&self) -> Self {
        SharedList {
            inner: Arc::clone(&self.inner),
        }
    }
}

fn demonstrate_concurrent_access(shared_list: SharedList) {
    let mut handles = vec![];
    
    // Spawn threads that modify the shared list
    for i in 10..13 {
        let list = shared_list.clone();
        let handle = thread::spawn(move || {
            list.push(i);
            println!("Thread added: {}", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
}

// BONUS IMPLEMENTATIONS - Advanced Smart Pointer Patterns

fn demonstrate_advanced_patterns() {
    println!("=== Advanced Smart Pointer Patterns ===\n");
    
    // Pattern 1: Reference-counted linked list
    demonstrate_rc_list();
    
    // Pattern 2: Tree with parent/child relationships
    demonstrate_tree_structure();
    
    // Pattern 3: Cache with interior mutability
    demonstrate_cache_pattern();
}

// Challenge 1: Reference-counted linked list
#[derive(Debug)]
enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

impl<T> RcList<T> {
    fn new() -> Rc<Self> {
        Rc::new(RcList::Nil)
    }
    
    fn prepend(self: Rc<Self>, elem: T) -> Rc<RcList<T>> {
        Rc::new(RcList::Cons(elem, self))
    }
    
    fn head(&self) -> Option<&T> {
        match self {
            RcList::Cons(head, _) => Some(head),
            RcList::Nil => None,
        }
    }
    
    fn tail(&self) -> Option<&RcList<T>> {
        match self {
            RcList::Cons(_, tail) => Some(tail),
            RcList::Nil => None,
        }
    }
}

fn demonstrate_rc_list() {
    println!("Reference-counted linked list:");
    
    let list = RcList::new();
    let list = list.prepend(1);
    let list = list.prepend(2);
    let list = list.prepend(3);
    
    println!("List: {:?}", list);
    
    // Multiple owners of the same list
    let list_copy = Rc::clone(&list);
    let list_copy2 = Rc::clone(&list);
    
    println!("Reference count: {}", Rc::strong_count(&list));
    
    println!();
}

// Challenge 2: Tree structure with parent/child relationships
use std::rc::Weak;

struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Option<Weak<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        })
    }
    
    fn add_child(self: &Rc<Self>, child: Rc<TreeNode>) {
        child.parent.borrow_mut().replace(Rc::downgrade(self));
        self.children.borrow_mut().push(child);
    }
    
    fn print_tree(&self, depth: usize) {
        println!("{}{}", "  ".repeat(depth), self.value);
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
}

fn demonstrate_tree_structure() {
    println!("Tree structure with weak parent references:");
    
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    let grandchild = TreeNode::new(4);
    
    root.add_child(child1.clone());
    root.add_child(child2.clone());
    child1.add_child(grandchild);
    
    root.print_tree(0);
    
    // Demonstrate that parent references don't prevent cleanup
    println!("Strong count for root: {}", Rc::strong_count(&root));
    println!("Weak count for root: {}", Rc::weak_count(&root));
    
    println!();
}

// Challenge 3: Cache with interior mutability
struct Cache<K, V> {
    map: RefCell<HashMap<K, V>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        Cache {
            map: RefCell::new(HashMap::new()),
        }
    }
    
    fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.map.borrow().get(key).cloned()
    }
    
    fn insert(&self, key: K, value: V) {
        self.map.borrow_mut().insert(key, value);
    }
    
    fn len(&self) -> usize {
        self.map.borrow().len()
    }
    
    fn clear(&self) {
        self.map.borrow_mut().clear();
    }
    
    fn contains_key(&self, key: &K) -> bool {
        self.map.borrow().contains_key(key)
    }
}

fn demonstrate_cache_pattern() {
    println!("Cache with interior mutability:");
    
    let cache = Cache::new();
    
    // Insert through immutable reference
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    cache.insert("key3", "value3");
    
    // Read through immutable reference
    println!("Cache lookup 'key1': {:?}", cache.get(&"key1"));
    println!("Cache lookup 'key2': {:?}", cache.get(&"key2"));
    println!("Cache size: {}", cache.len());
    
    // Cache is immutable but we can still modify its contents
    let cache_ref = &cache;  // Immutable reference
    cache_ref.insert("key4", "value4");  // But can still insert!
    
    println!("After inserting through immutable ref: {}", cache.len());
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box_list() {
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        assert_eq!(list_length(&list), 2);
    }
    
    #[test]
    fn test_rc_sharing() {
        let data = Rc::new(String::from("test"));
        let copy1 = Rc::clone(&data);
        let copy2 = Rc::clone(&data);
        
        assert_eq!(Rc::strong_count(&data), 3);
        assert_eq!(*data, "test");
        assert_eq!(*copy1, "test");
        assert_eq!(*copy2, "test");
    }
    
    #[test]
    fn test_refcell_mutability() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
        
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
        
        counter.add(5);
        assert_eq!(counter.get(), 7);
    }
    
    #[test]
    fn test_shared_counter() {
        let counter = SharedCounter::new();
        let counter2 = counter.clone();
        
        counter.increment();
        counter2.increment();
        
        assert_eq!(counter.get(), 2);
        assert_eq!(counter2.get(), 2);
        assert_eq!(counter.reference_count(), 2);
    }
    
    #[test]
    fn test_shared_list() {
        let list = SharedList::new();
        let list2 = list.clone();
        
        list.push(1);
        list2.push(2);
        
        assert_eq!(list.len(), 2);
        assert!(list.contains(&1));
        assert!(list.contains(&2));
        
        let contents = list.get_all();
        assert_eq!(contents.len(), 2);
    }
    
    #[test]
    fn test_cache() {
        let cache = Cache::new();
        
        cache.insert("key1", 42);
        cache.insert("key2", 100);
        
        assert_eq!(cache.get(&"key1"), Some(42));
        assert_eq!(cache.get(&"key2"), Some(100));
        assert_eq!(cache.get(&"key3"), None);
        assert_eq!(cache.len(), 2);
    }
    
    #[test]
    fn test_rc_list_operations() {
        let list = RcList::new();
        let list = list.prepend(1);
        let list = list.prepend(2);
        
        assert_eq!(list.head(), Some(&2));
        
        if let Some(tail) = list.tail() {
            assert_eq!(tail.head(), Some(&1));
        }
    }
}
```

## 🎓 Complete Code Walkthrough

### 1. Box<T> - Heap Allocation
```rust
// Problem: Recursive types have infinite size
enum List {
    Cons(i32, List),  // ERROR: infinite size
    Nil,
}

// Solution: Use Box for indirection
enum List {
    Cons(i32, Box<List>),  // Box has known size (pointer)
    Nil,
}

// Box automatically dereferences
let boxed = Box::new(42);
println!("{}", *boxed);  // Explicit dereference
println!("{}", boxed);   // Automatic dereference
```

### 2. Rc<T> - Reference Counting
```rust
// Problem: Multiple owners needed
let data = String::from("shared");
let owner1 = data;        // data moved
let owner2 = data;        // ERROR: already moved

// Solution: Use Rc for shared ownership
let data = Rc::new(String::from("shared"));
let owner1 = Rc::clone(&data);  // Increment ref count
let owner2 = Rc::clone(&data);  // Increment ref count

// All references point to same data
println!("Count: {}", Rc::strong_count(&data));  // 3
```

### 3. RefCell<T> - Interior Mutability
```rust
// Problem: Can't mutate through immutable reference
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&self) {
        self.count += 1;  // ERROR: can't mutate through &self
    }
}

// Solution: Use RefCell for runtime borrowing
struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn increment(&self) {
        *self.count.borrow_mut() += 1;  // Runtime mutable borrow
    }
}
```

### 4. Arc<T> - Thread-Safe Reference Counting
```rust
// Problem: Rc doesn't work across threads
let data = Rc::new(String::from("thread data"));
thread::spawn(move || {
    println!("{}", data);  // ERROR: Rc is not Send
});

// Solution: Use Arc for thread-safe sharing
let data = Arc::new(String::from("thread data"));
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    println!("{}", data_clone);  // Works: Arc is Send + Sync
});
```

### 5. Complex Combinations
```rust
// Single-threaded shared mutable data
type SharedData = Rc<RefCell<i32>>;

// Multi-threaded shared mutable data
type ThreadSafeSharedData = Arc<Mutex<i32>>;

// Reference-counted collections
type SharedList = Rc<RefCell<Vec<i32>>>;

// Thread-safe collections
type ThreadSafeList = Arc<Mutex<Vec<i32>>>;
```

## 🏆 Advanced Smart Pointer Patterns

### Weak References (Prevent Cycles)
```rust
use std::rc::{Rc, Weak};

struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    parent: RefCell<Weak<Parent>>,  // Weak prevents cycles
}

// Create relationships without memory leaks
let parent = Rc::new(Parent { children: RefCell::new(vec![]) });
let child = Rc::new(Child { parent: RefCell::new(Rc::downgrade(&parent)) });
parent.children.borrow_mut().push(child);
```

### Custom Smart Pointers
```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}
```

### Copy-on-Write (COW) Pattern
```rust
use std::borrow::Cow;

fn process_data(input: Cow<str>) -> String {
    if input.contains("modify") {
        let mut owned = input.into_owned();  // Clone only if needed
        owned.push_str(" - modified");
        owned
    } else {
        input.to_string()  // Use borrowed data
    }
}
```

## 🎯 Key Learning Achievements

By completing this exercise, you've mastered:

### ✅ **Smart Pointer Fundamentals**
- Understanding when and why to use each smart pointer type
- Memory management patterns beyond basic ownership
- Performance implications of different smart pointer choices

### ✅ **Complex Ownership Scenarios**
- Shared ownership with reference counting
- Interior mutability for flexible APIs
- Thread-safe sharing across concurrent contexts

### ✅ **Memory Safety Guarantees**
- Preventing memory leaks with proper reference management
- Breaking reference cycles with weak references
- Runtime vs compile-time borrowing trade-offs

### ✅ **Real-World Patterns**
- Building data structures with shared components
- Creating thread-safe shared state
- Implementing caches and containers with smart pointers

### ✅ **C# to Rust Translation**
You can now handle complex C# patterns in Rust:
- Object references → Smart pointer combinations
- Shared mutable state → `Rc<RefCell<T>>` or `Arc<Mutex<T>>`
- Collections with sharing → Reference-counted data structures
- Thread-safe operations → Arc and Mutex combinations

## 🚀 Next Steps

**Congratulations!** You've mastered Rust's smart pointer system. You're ready for:

1. **Module 02 Exercise 5**: Advanced Ownership Patterns
2. **Advanced Rust concepts**: Async programming, trait objects, and unsafe code
3. **Real-world applications**: Building complex systems with Rust's memory safety

Smart pointers are essential tools for advanced Rust programming - you now have the complete toolkit! 🦀