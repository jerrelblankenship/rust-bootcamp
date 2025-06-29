// Exercise 3: Lifetimes - Fix the lifetime errors!
//
// Your task: Make all the lifetime examples compile and work correctly
// This demonstrates lifetime annotations and management from Module 02
//
// INSTRUCTIONS:
// 1. Read each function and understand what it's trying to do
// 2. Fix the compilation errors by adding proper lifetime annotations
// 3. Run `rustc ex03-lifetimes.rs && ./ex03-lifetimes` to test
// 4. Uncomment exercises one by one as you solve them

fn main() {
    println!("=== Exercise 3: Lifetimes (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each exercise one by one and fix the errors
    exercise_3_1();
    // exercise_3_2();
    // exercise_3_3();
    // exercise_3_4();
    // exercise_3_5();
    // exercise_3_6();
    // exercise_3_7();
    // exercise_3_8();
    
    println!("\n🎉 All lifetime exercises completed!");
}

// Exercise 3.1: Function returning references
// PROBLEM: Rust doesn't know which input the output lifetime relates to
fn exercise_3_1() {
    println!("Exercise 3.1: Function returning references");
    
    let string1 = String::from("hello");
    let string2 = String::from("world!");
    
    let result = longest(&string1, &string2);  // COMPILE ERROR!
    println!("Longest string: {}", result);
    
    // TODO: Fix the longest function below to compile
    
    println!("✅ Exercise 3.1 complete\n");
}

// TODO: Fix this function by adding lifetime annotations
// HINT: The returned reference must live as long as the parameters
fn longest(x: &str, y: &str) -> &str {  // FIXME: Missing lifetime annotations!
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Exercise 3.2: Lifetime elision rules
// PROBLEM: Understanding when lifetimes can be omitted
fn exercise_3_2() {
    println!("Exercise 3.2: Lifetime elision rules");
    
    let text = String::from("hello world");
    let first = first_word(&text);
    println!("First word: {}", first);
    
    // This works! But why doesn't first_word need lifetime annotations?
    // TODO: Answer these questions:
    // 1. How many input lifetimes does first_word have? ____
    // 2. How many output lifetimes does it have? ____
    // 3. Which elision rule applies? ____________________
    
    println!("✅ Exercise 3.2 complete\n");
}

// This function works without explicit lifetimes - can you figure out why?
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Exercise 3.3: Structs with lifetimes
// PROBLEM: Structs that hold references need lifetime parameters
fn exercise_3_3() {
    println!("Exercise 3.3: Structs with lifetimes");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: &novel[0..13],  // COMPILE ERROR!
    };
    
    println!("Excerpt: {}", excerpt.part);
    
    // TODO: Fix the ImportantExcerpt struct below
    
    println!("✅ Exercise 3.3 complete\n");
}

// TODO: Fix this struct by adding lifetime parameters
struct ImportantExcerpt {
    part: &str,  // FIXME: Missing lifetime parameter!
}

// Exercise 3.4: Methods with lifetimes
// PROBLEM: Implementing methods on structs with lifetimes
fn exercise_3_4() {
    println!("Exercise 3.4: Methods with lifetimes");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let mut excerpt = ImportantExcerpt {
        part: &novel[0..13],
    };
    
    // This should work once you implement the methods
    let announcement = excerpt.announce_and_return_part("Breaking news!");  // COMPILE ERROR!
    println!("Announcement result: {}", announcement);
    
    println!("✅ Exercise 3.4 complete\n");
}

// TODO: Implement methods for ImportantExcerpt
impl ImportantExcerpt {
    // TODO: Add a method that returns the excerpt with an announcement
    // The method should take &self and an announcement string
    // It should return the part field
    // fn announce_and_return_part(/* TODO: parameters */) -> /* TODO: return type */ {
    //     println!("Attention please: {}", /* announcement parameter */);
    //     // TODO: return self.part
    // }
}

// Exercise 3.5: Multiple lifetime parameters
// PROBLEM: Functions with multiple references might need multiple lifetimes
fn exercise_3_5() {
    println!("Exercise 3.5: Multiple lifetime parameters");
    
    let name = String::from("Alice");
    let title = String::from("Dr.");
    
    let full_title = format_title(&title, &name);  // COMPILE ERROR!
    println!("Full title: {}", full_title);
    
    // TODO: Fix the format_title function below
    
    println!("✅ Exercise 3.5 complete\n");
}

// TODO: Fix this function - does it need one lifetime or two?
fn format_title(title: &str, name: &str) -> &str {  // FIXME: Missing lifetimes!
    // This function always returns the title, so what lifetime should it have?
    title
}

// Exercise 3.6: Static lifetime
// PROBLEM: Understanding the 'static lifetime
fn exercise_3_6() {
    println!("Exercise 3.6: Static lifetime");
    
    let greeting = get_greeting();
    println!("Greeting: {}", greeting);
    
    // TODO: What makes the return value have a 'static lifetime?
    // ANSWER: ________________________________
    
    let custom = get_custom_message("Hello");  // COMPILE ERROR!
    println!("Custom: {}", custom);
    
    println!("✅ Exercise 3.6 complete\n");
}

fn get_greeting() -> &'static str {
    "Hello, world!"  // String literals have 'static lifetime
}

// TODO: Fix this function to work with 'static lifetime
fn get_custom_message(msg: &str) -> &'static str {  // FIXME: Can't return 'static!
    msg  // This parameter doesn't live for 'static!
}

// Exercise 3.7: Lifetime bounds in generics
// PROBLEM: Generic types that contain references need lifetime bounds
fn exercise_3_7() {
    println!("Exercise 3.7: Lifetime bounds in generics");
    
    let text = String::from("example text");
    let wrapper = Wrapper { value: &text };  // COMPILE ERROR!
    
    println!("Wrapped: {}", wrapper.value);
    
    // TODO: Fix the Wrapper struct below
    
    println!("✅ Exercise 3.7 complete\n");
}

// TODO: Fix this generic struct with lifetime bounds
struct Wrapper<T> {
    value: T,  // FIXME: If T contains references, we need lifetime bounds!
}

// Exercise 3.8: Complex lifetime relationships
// PROBLEM: Real-world lifetime scenarios
fn exercise_3_8() {
    println!("Exercise 3.8: Complex lifetime relationships");
    
    let mut cache = Cache::new();
    
    let data1 = String::from("cached data 1");
    let data2 = String::from("cached data 2");
    
    cache.store("key1", &data1);  // COMPILE ERROR!
    cache.store("key2", &data2);
    
    if let Some(value) = cache.get("key1") {
        println!("Retrieved: {}", value);
    }
    
    println!("✅ Exercise 3.8 complete\n");
}

// TODO: Fix this cache implementation with proper lifetimes
use std::collections::HashMap;

struct Cache {
    map: HashMap<String, &str>,  // FIXME: Missing lifetime parameter!
}

impl Cache {
    fn new() -> Self {
        Cache {
            map: HashMap::new(),
        }
    }
    
    // TODO: Fix these methods with proper lifetime annotations
    fn store(&mut self, key: &str, value: &str) {  // FIXME: Missing lifetimes!
        self.map.insert(key.to_string(), value);
    }
    
    fn get(&self, key: &str) -> Option<&str> {  // FIXME: Missing lifetimes!
        self.map.get(key).copied()
    }
}

// BONUS CHALLENGES (uncomment to try):

// Challenge 1: Implement a function that chooses between three strings
// fn choose_string(a: &str, b: &str, c: &str, choice: u8) -> &str {
//     match choice {
//         0 => a,
//         1 => b,
//         _ => c,
//     }
// }

// Challenge 2: Create a struct that holds two different types of references
// struct TwoRefs {
//     first: &str,
//     second: &str,
// }

// Challenge 3: Implement a function with lifetime subtyping
// fn longest_with_context(x: &str, y: &str, context: &str) -> &str {
//     println!("Context: {}", context);
//     if x.len() > y.len() { x } else { y }
// }

// Challenge 4: Fix this complex lifetime scenario
// struct Database {
//     connections: Vec<&Connection>,
// }
// 
// struct Connection {
//     url: &str,
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_when_implemented_longest() {
        // This test will work once you implement lifetimes correctly
        // let s1 = "short";
        // let s2 = "longer string";
        // let result = longest(s1, s2);
        // assert_eq!(result, "longer string");
    }
    
    #[test]
    fn test_when_implemented_excerpt() {
        // This test will work once you fix the struct
        // let text = String::from("Hello world");
        // let excerpt = ImportantExcerpt { part: &text[0..5] };
        // assert_eq!(excerpt.part, "Hello");
    }
    
    // TODO: Add your own tests to verify your implementations
}
