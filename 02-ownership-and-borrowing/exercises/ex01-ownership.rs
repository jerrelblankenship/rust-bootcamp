// Exercise 1: Ownership Basics - Fix the broken code!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (8 ownership challenges)
//
// Your task: Make all the functions compile and work correctly
// This exercise demonstrates ownership transfers and the move vs copy semantics
// 
// LEARNING STRATEGY:
// 1. Fix ONE exercise at a time - ownership concepts build on each other
// 2. Uncomment exercises gradually as you complete each one
// 3. Read the error messages carefully - Rust's compiler guides ownership learning
// 4. Think about data ownership: "Who owns this data and when?"
//
// INSTRUCTIONS:
// 1. Start with exercise_1_1 (it's already uncommented)
// 2. Fix compilation errors by applying ownership concepts
// 3. Uncomment the next exercise only after the current one works
// 4. Run `rustc ex01-ownership.rs && ./ex01-ownership` to test
// 5. All exercises should compile and run successfully when fixed

fn main() {
    println!("=== Exercise 1: Ownership Basics (Fix the Code!) ===\n");
    
    // 📊 PROGRESS: Complete exercises one by one, uncomment as you go
    // ✅ Exercise 1.1: Move vs Clone patterns
    exercise_1_1();
    
    // ❌ Exercise 1.2: Copy vs Move semantics (uncomment when 1.1 works)
    // exercise_1_2();
    
    // ❌ Exercise 1.3: Function ownership (uncomment when 1.2 works)
    // exercise_1_3();
    
    // ❌ Exercise 1.4: Ownership return patterns (uncomment when 1.3 works)
    // exercise_1_4();
    
    // ❌ Exercise 1.5: Clone vs Move decisions (uncomment when 1.4 works)
    // exercise_1_5();
    
    // ❌ Exercise 1.6: Collections ownership (uncomment when 1.5 works)
    // exercise_1_6();
    
    // ❌ Exercise 1.7: Stack implementation (uncomment when 1.6 works)
    // exercise_1_7();
    
    // ❌ Exercise 1.8: Drop and RAII (uncomment when 1.7 works)
    // exercise_1_8();
    
    println!("\n🎉 All exercises completed successfully!");
}

// Exercise 1.1: Fix the move error
// PROBLEM: This code doesn't compile because of ownership rules
// 🎯 LEARNING FOCUS: Move vs Clone - when to copy vs transfer ownership
fn exercise_1_1() {
    println!("Exercise 1.1: Fix the move error");
    
    let s1 = String::from("hello");
    let s2 = s1;  // FIXME: s1 is moved here - how can we use both?
    
    // OWNERSHIP VISUALIZATION:
    // Before: s1 ──► "hello"    After: s1 ✗ (moved)
    //                                   s2 ──► "hello"
    
    // TODO: Make both of these print statements work
    // HINT: What method creates an independent copy of a String?
    // THINK: Do I need two independent copies, or can I solve this differently?
    println!("s1 = {}, s2 = {}", s1, s2);  // COMPILE ERROR: Fix this!
    
    println!("✅ Exercise 1.1 complete\n");
    
    // 📊 CHECKPOINT: When this compiles, uncomment exercise_1_2() in main()
}

// Exercise 1.2: Understanding Copy vs Move semantics
// TASK: Predict which assignments copy and which move, then fix the broken ones
fn exercise_1_2() {
    println!("Exercise 1.2: Understanding Copy vs Move semantics");
    
    // This works - predict why!
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);  // Both work - why?
    
    // STACK vs HEAP VISUALIZATION:
    // Stack (Copy):        Heap (Move):
    // x = 5               s1 ──► "hello"
    // y = 5 (copied!)     s2 ──► "hello" (s1 invalid!)
    
    // FIXME: This doesn't compile - make it work
    let s1 = String::from("hello");
    let s2 = s1;  // What happens to s1 here?
    println!("s1 = {}, s2 = {}", s1, s2);  // COMPILE ERROR!
    
    // This works - predict why!
    let a = [1, 2, 3];
    let b = a;
    println!("a = {:?}, b = {:?}", a, b);  // Both work - why?
    
    // FIXME: This doesn't compile - make it work  
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // What happens to v1 here?
    println!("v1 = {:?}, v2 = {:?}", v1, v2);  // COMPILE ERROR!
    
    // TODO: Answer these questions:
    // 1. Which types implement Copy trait? _____________
    // 2. Which types require explicit cloning? _____________
    // 3. What's the difference between copy and move? _____________
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Functions and ownership
// PROBLEM: Functions that take ownership make values unusable afterward
fn exercise_1_3() {
    println!("Exercise 1.3: Functions and ownership");
    
    let s = String::from("hello");
    
    // FIXME: After calling this function, s is no longer usable
    takes_ownership(s);
    println!("Can I still use s? {}", s);  // COMPILE ERROR!
    
    // CHALLENGE: Make the above work by fixing either:
    // 1. The function call, OR
    // 2. The function signature (see takes_ownership below)
    
    println!("✅ Exercise 1.3 complete\n");
}

// TODO: Modify this function so it doesn't take ownership
// HINT: How can you access a String without owning it?
fn takes_ownership(some_string: String) {
    println!("Function received: {}", some_string);
} // some_string is dropped here when function ends

// TODO: Create alternative functions that handle ownership differently:

// Option 1: Borrow the string instead of taking ownership
// fn borrows_string(/* TODO: parameter type */) {
//     println!("Borrowed: {}", /* TODO: parameter name */);
//     // Nothing is dropped - we're just borrowing!
// }

// Option 2: Take ownership but return it back
// fn takes_and_returns(/* TODO: parameter */) -> /* TODO: return type */ {
//     println!("Got and returning: {}", /* TODO: parameter */);
//     // TODO: return the parameter
// }

// Exercise 1.4: Returning ownership patterns
// TASK: Understand when ownership is transferred vs returned
fn exercise_1_4() {
    println!("Exercise 1.4: Returning ownership patterns");
    
    let s1 = String::from("hello");
    let s2 = append_world(s1);
    
    // FIXME: Can we still use s1 here? Why or why not?
    println!("Original: {}", s1);  // COMPILE ERROR!
    println!("Result: {}", s2);
    
    // CHALLENGE: Modify the code above so both prints work
    // Consider: Should you change the function call or the function itself?
    
    println!("✅ Exercise 1.4 complete\n");
}

// This function takes ownership and returns ownership
fn append_world(mut s: String) -> String {
    s.push_str(" world");
    s // Return the modified string
}

// TODO: Create an alternative version that borrows instead
// fn append_world_borrow(/* TODO: parameter */) -> String {
//     // HINT: Create a new String instead of modifying the input
//     // TODO: implementation
// }

// Exercise 1.5: Clone vs Move - when to use each
// TASK: Understand the difference and fix the broken code
fn exercise_1_5() {
    println!("Exercise 1.5: Clone vs Move");
    
    let original = String::from("hello");
    
    // This should work multiple times
    use_string(&original);
    use_string(&original);  // Should be able to call again
    
    // This should work only once
    use_string_once(original);
    // use_string_once(original);  // FIXME: This won't work - why not?
    
    // CHALLENGE: What if you wanted to call use_string_once multiple times?
    // How would you modify the code?
    
    println!("✅ Exercise 1.5 complete\n");
}

fn use_string(s: &String) {
    println!("Using borrowed string: {}", s);
    // Original string is unaffected
}

fn use_string_once(s: String) {
    println!("Using owned string: {}", s);
    // s is dropped here, ownership was transferred to this function
}

// TODO: Create a function that can be called multiple times but creates a copy
// fn use_string_with_clone(/* TODO: parameter */) {
//     // TODO: Implementation that doesn't take ownership
//     // HINT: Use the clone() method
// }

// Exercise 1.6: Ownership in collections  
// PROBLEM: Adding values to collections can move them
fn exercise_1_6() {
    println!("Exercise 1.6: Ownership in collections");
    
    let mut vec = Vec::new();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    // FIXME: After pushing into vec, can we still use s1 and s2?
    vec.push(s1);
    vec.push(s2);
    println!("s1: {}", s1);  // COMPILE ERROR!
    println!("s2: {}", s2);  // COMPILE ERROR!
    println!("vec: {:?}", vec);
    
    // TODO: Fix the above code using one of these strategies:
    // 1. Clone before pushing
    // 2. Use a Vec of references
    // 3. Accept the move and don't use the variables afterward
    
    println!("✅ Exercise 1.6 complete\n");
}

// Exercise 1.7: Implement a Stack that demonstrates ownership
// TASK: Complete the Stack implementation
fn exercise_1_7() {
    println!("Exercise 1.7: Implement a Stack");
    
    let mut stack = Stack::new();
    stack.push(String::from("first"));
    stack.push(String::from("second"));
    stack.push(String::from("third"));
    
    println!("Stack size: {}", stack.len());
    
    // Pop items and show ownership transfer
    while let Some(item) = stack.pop() {
        println!("Popped: {} (now I own this string)", item);
        // item is dropped here
    }
    
    println!("Stack is empty: {}", stack.is_empty());
    
    println!("✅ Exercise 1.7 complete\n");
}

// TODO: Complete this Stack implementation
struct Stack<T> {
    // TODO: What field do we need to store the items?
    // items: /* TODO: type */,
}

impl<T> Stack<T> {
    fn new() -> Self {
        // TODO: Create a new empty stack
        todo!("Implement Stack::new")
    }
    
    fn push(&mut self, item: T) {
        // TODO: Add item to the stack
        // THINK: Does this take ownership of item?
        todo!("Implement push")
    }
    
    fn pop(&mut self) -> Option<T> {
        // TODO: Remove and return the top item
        // THINK: Does this give ownership to the caller?
        todo!("Implement pop")
    }
    
    fn len(&self) -> usize {
        // TODO: Return the number of items
        todo!("Implement len")
    }
    
    fn is_empty(&self) -> bool {
        // TODO: Check if stack is empty
        todo!("Implement is_empty")
    }
    
    // BONUS: Implement peek to look at top item without removing it
    // fn peek(&self) -> Option<&T> {
    //     // TODO: Return a reference to the top item
    //     // THINK: Why return Option<&T> instead of Option<T>?
    // }
}

// Exercise 1.8: Understanding Drop and RAII
// TASK: Observe when values are automatically cleaned up
fn exercise_1_8() {
    println!("Exercise 1.8: Understanding Drop and RAII");
    
    println!("Starting outer scope");
    {
        let d1 = Droppable::new("d1");
        println!("Starting inner scope");
        {
            let d2 = Droppable::new("d2");
            let d3 = Droppable::new("d3");
            
            // TODO: Predict when d2 and d3 will be dropped
            // THINK: In what order will they be dropped?
        }
        println!("Inner scope ended");
        // TODO: Predict when d1 will be dropped
    }
    println!("Outer scope ended");
    
    // CHALLENGE: What happens with move semantics?
    println!("\nTesting move semantics:");
    let d4 = Droppable::new("d4");
    let d5 = d4; // What happens to d4?
    // TODO: Predict how many drop messages you'll see
    
    println!("✅ Exercise 1.8 complete\n");
}

// TODO: Complete this struct that shows when it's dropped
struct Droppable {
    name: String,
}

impl Droppable {
    fn new(name: &str) -> Self {
        println!("🔧 Creating resource: {}", name);
        Droppable {
            name: name.to_string(),
        }
    }
}

// TODO: Implement the Drop trait so we can see when cleanup happens
// HINT: Drop trait has one method: fn drop(&mut self)
impl Drop for Droppable {
    fn drop(&mut self) {
        // TODO: Print a message when this value is dropped
        // println!("🗑️ Dropping resource: {}", self.name);
        todo!("Implement drop method")
    }
}

// BONUS CHALLENGES (uncomment to try):

// Challenge 1: Create a function that can use a String multiple times
// fn use_string_multiple_times(s: /* TODO: type */) {
//     for i in 0..3 {
//         println!("Using {} (iteration {})", s, i);
//     }
// }

// Challenge 2: Create a function that optionally takes ownership
// fn maybe_take_ownership(s: String, take: bool) -> Option<String> {
//     if take {
//         // TODO: Take ownership and return None
//     } else {
//         // TODO: Don't take ownership, return the string back
//     }
// }

// Challenge 3: Implement a custom String type that tracks copies
// struct TrackedString {
//     value: String,
//     id: u32,
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_when_implemented_append_world() {
        // This test will work once you implement the functions
        // let s = String::from("hello");
        // let result = append_world(s);
        // assert_eq!(result, "hello world");
    }
    
    #[test]
    fn test_when_implemented_stack() {
        // This test will work once you implement Stack
        // let mut stack = Stack::new();
        // assert!(stack.is_empty());
        
        // stack.push(1);
        // stack.push(2);
        // assert_eq!(stack.len(), 2);
        
        // assert_eq!(stack.pop(), Some(2));
        // assert_eq!(stack.pop(), Some(1));
        // assert_eq!(stack.pop(), None);
        // assert!(stack.is_empty());
    }
    
    // TODO: Add your own tests to verify your implementations
}
