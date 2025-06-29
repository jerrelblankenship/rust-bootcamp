// Exercise 4: Smart Pointers - Fix the smart pointer code!
//
// Your task: Complete the smart pointer examples and fix compilation errors
// This demonstrates Box, Rc, Arc, and RefCell from Module 02
//
// INSTRUCTIONS:
// 1. Read each function and understand what it's trying to do
// 2. Fix the compilation errors by using appropriate smart pointers
// 3. Run `rustc ex04-smart-pointers.rs && ./ex04-smart-pointers` to test
// 4. Uncomment exercises one by one as you solve them

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    println!("=== Exercise 4: Smart Pointers (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each exercise one by one and fix the errors
    exercise_4_1();
    // exercise_4_2();
    // exercise_4_3();
    // exercise_4_4();
    // exercise_4_5();
    // exercise_4_6();
    // exercise_4_7();
    // exercise_4_8();
    
    println!("\n🎉 All smart pointer exercises completed!");
}

// Exercise 4.1: Box<T> for heap allocation
// PROBLEM: Recursive types have infinite size on the stack
fn exercise_4_1() {
    println!("Exercise 4.1: Box<T> for heap allocation");
    
    // Try to create a simple recursive list
    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));  // COMPILE ERROR!
    
    println!("Created list: {:?}", list);
    
    // TODO: Fix the List enum below to compile
    
    println!("✅ Exercise 4.1 complete\n");
}

// TODO: Fix this recursive type using Box
#[derive(Debug)]
enum List {
    Cons(i32, List),  // FIXME: This has infinite size!
    Nil,
}

// Exercise 4.2: Box with heap-allocated data
// PROBLEM: Large data structures should live on the heap
fn exercise_4_2() {
    println!("Exercise 4.2: Box with heap-allocated data");
    
    // Create a large array on the stack - this might overflow!
    let large_array = [0; 1000000];  // FIXME: Potential stack overflow!
    
    println!("Array size: {}", large_array.len());
    
    // TODO: Put the large array on the heap using Box
    // let heap_array = /* TODO: Box the array */;
    // println!("Heap array size: {}", heap_array.len());
    
    println!("✅ Exercise 4.2 complete\n");
}

// Exercise 4.3: Rc<T> for shared ownership
// PROBLEM: We want multiple owners of the same data
fn exercise_4_3() {
    println!("Exercise 4.3: Rc<T> for shared ownership");
    
    let data = String::from("shared data");
    
    // We want multiple variables to own the same data
    let owner1 = data;          // data is moved here
    let owner2 = data;          // COMPILE ERROR! data already moved
    let owner3 = data;          // COMPILE ERROR! data already moved
    
    println!("Owner 1: {}", owner1);
    println!("Owner 2: {}", owner2);
    println!("Owner 3: {}", owner3);
    
    // TODO: Use Rc to enable shared ownership
    // HINT: Rc::new() and Rc::clone()
    
    println!("✅ Exercise 4.3 complete\n");
}

// Exercise 4.4: Rc reference counting
// PROBLEM: Understanding when Rc data is deallocated
fn exercise_4_4() {
    println!("Exercise 4.4: Rc reference counting");
    
    let data = Rc::new(String::from("counted data"));
    println!("Initial reference count: {}", Rc::strong_count(&data));
    
    // TODO: Create more references and observe the count
    let ref1 = data;  // FIXME: This moves data instead of cloning the Rc!
    println!("After ref1: {}", Rc::strong_count(&data));  // COMPILE ERROR!
    
    // TODO: Create ref2 and ref3 properly
    // let ref2 = /* TODO: Clone the Rc */;
    // let ref3 = /* TODO: Clone the Rc */;
    
    // TODO: Drop references and observe the count
    // drop(ref1);
    // println!("After dropping ref1: {}", Rc::strong_count(&/* which ref? */));
    
    println!("✅ Exercise 4.4 complete\n");
}

// Exercise 4.5: RefCell<T> for interior mutability
// PROBLEM: We want to modify data through an immutable reference
fn exercise_4_5() {
    println!("Exercise 4.5: RefCell<T> for interior mutability");
    
    let counter = Counter::new();
    
    // This should work - incrementing through an immutable reference
    counter.increment();  // COMPILE ERROR!
    counter.increment();
    
    println!("Final count: {}", counter.get());
    
    // TODO: Fix the Counter implementation below
    
    println!("✅ Exercise 4.5 complete\n");
}

// TODO: Fix this struct to allow interior mutability
struct Counter {
    count: i32,  // FIXME: Can't modify through &self!
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
    
    // This method takes &self but needs to modify count
    fn increment(&self) {
        self.count += 1;  // FIXME: Can't modify through immutable reference!
    }
    
    fn get(&self) -> i32 {
        self.count  // FIXME: Might need to borrow from RefCell!
    }
}

// Exercise 4.6: Combining Rc and RefCell
// PROBLEM: Shared mutable data
fn exercise_4_6() {
    println!("Exercise 4.6: Combining Rc and RefCell");
    
    let shared_counter = SharedCounter::new();
    
    // Create multiple owners of the same mutable counter
    let counter1 = shared_counter;        // FIXME: Moves instead of sharing!
    let counter2 = shared_counter;        // COMPILE ERROR!
    let counter3 = shared_counter;        // COMPILE ERROR!
    
    // All should be able to increment the same counter
    counter1.increment();
    counter2.increment();
    counter3.increment();
    
    println!("Final count: {}", counter1.get());
    println!("Same count from counter2: {}", counter2.get());
    
    // TODO: Fix the SharedCounter implementation below
    
    println!("✅ Exercise 4.6 complete\n");
}

// TODO: Implement shared mutable counter using Rc<RefCell<T>>
struct SharedCounter {
    // TODO: What type should this field be?
    // inner: /* TODO: Rc<RefCell<...>> */,
}

impl SharedCounter {
    fn new() -> Self {
        // TODO: Initialize with Rc<RefCell<i32>>
        todo!("Implement SharedCounter::new")
    }
    
    fn increment(&self) {
        // TODO: Borrow mutably from RefCell and increment
        todo!("Implement increment")
    }
    
    fn get(&self) -> i32 {
        // TODO: Borrow from RefCell and return value
        todo!("Implement get")
    }
}

// TODO: Implement Clone for SharedCounter
// impl Clone for SharedCounter {
//     fn clone(&self) -> Self {
//         // TODO: Clone the Rc, not the inner data
//     }
// }

// Exercise 4.7: Arc<T> for thread-safe sharing
// PROBLEM: Rc doesn't work across threads
fn exercise_4_7() {
    println!("Exercise 4.7: Arc<T> for thread-safe sharing");
    
    let data = Rc::new(String::from("thread data"));
    
    // Try to move data into a thread closure
    let handle = std::thread::spawn(move || {
        println!("Thread data: {}", data);  // COMPILE ERROR! Rc not Send
    });
    
    handle.join().unwrap();
    
    // TODO: Replace Rc with Arc to make it work across threads
    
    println!("✅ Exercise 4.7 complete\n");
}

// Exercise 4.8: Complex smart pointer combinations
// PROBLEM: Building a shared, mutable, thread-safe data structure
fn exercise_4_8() {
    println!("Exercise 4.8: Complex smart pointer combinations");
    
    let shared_list = SharedList::new();
    
    // Add some items
    shared_list.push(1);
    shared_list.push(2);
    shared_list.push(3);
    
    println!("List length: {}", shared_list.len());
    println!("List contents: {:?}", shared_list.get_all());
    
    // TODO: Implement SharedList below
    
    println!("✅ Exercise 4.8 complete\n");
}

// TODO: Implement a thread-safe shared list
use std::sync::Mutex;

struct SharedList {
    // TODO: What combination of smart pointers do we need?
    // Need: shared ownership + thread safety + interior mutability
    // inner: /* TODO: Arc<Mutex<Vec<i32>>> */,
}

impl SharedList {
    fn new() -> Self {
        // TODO: Initialize the shared list
        todo!("Implement SharedList::new")
    }
    
    fn push(&self, item: i32) {
        // TODO: Lock the mutex and push item
        todo!("Implement push")
    }
    
    fn len(&self) -> usize {
        // TODO: Lock the mutex and return length
        todo!("Implement len")
    }
    
    fn get_all(&self) -> Vec<i32> {
        // TODO: Lock the mutex and clone the vec
        todo!("Implement get_all")
    }
}

// TODO: Implement Clone for SharedList  
// impl Clone for SharedList {
//     fn clone(&self) -> Self {
//         // TODO: Clone the Arc, not the inner data
//     }
// }

// BONUS CHALLENGES (uncomment to try):

// Challenge 1: Implement a reference-counted linked list
// #[derive(Debug)]
// enum RcList<T> {
//     Cons(T, Rc<RcList<T>>),
//     Nil,
// }
//
// impl<T> RcList<T> {
//     fn new() -> Rc<Self> {
//         Rc::new(RcList::Nil)
//     }
//     
//     fn prepend(self: Rc<Self>, elem: T) -> Rc<RcList<T>> {
//         Rc::new(RcList::Cons(elem, self))
//     }
// }

// Challenge 2: Create a tree structure with parent/child relationships
// struct TreeNode {
//     value: i32,
//     children: RefCell<Vec<Rc<TreeNode>>>,
//     parent: RefCell<Option<Weak<TreeNode>>>,  // Weak to avoid cycles!
// }

// Challenge 3: Implement a simple cache with interior mutability
// use std::collections::HashMap;
// 
// struct Cache<K, V> {
//     map: RefCell<HashMap<K, V>>,
// }
//
// impl<K, V> Cache<K, V>
// where
//     K: Eq + std::hash::Hash,
// {
//     fn new() -> Self {
//         Cache {
//             map: RefCell::new(HashMap::new()),
//         }
//     }
//     
//     fn get(&self, key: &K) -> Option<V>
//     where
//         V: Clone,
//     {
//         // TODO: Implement cache lookup
//     }
//     
//     fn insert(&self, key: K, value: V) {
//         // TODO: Implement cache insertion
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_when_implemented_list() {
        // This test will work once you implement List with Box
        // let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        // match list {
        //     List::Cons(val, _) => assert_eq!(val, 1),
        //     List::Nil => panic!("Expected Cons"),
        // }
    }
    
    #[test]
    fn test_when_implemented_shared_counter() {
        // This test will work once you implement SharedCounter
        // let counter = SharedCounter::new();
        // let counter2 = counter.clone();
        
        // counter.increment();
        // counter2.increment();
        
        // assert_eq!(counter.get(), 2);
        // assert_eq!(counter2.get(), 2);
    }
    
    // TODO: Add your own tests to verify your implementations
}
