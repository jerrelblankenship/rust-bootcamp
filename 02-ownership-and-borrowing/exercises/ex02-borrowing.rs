// Exercise 2: Borrowing and References - Fix the broken code!
//
// Your task: Make all the borrowing examples compile and work correctly
// This demonstrates borrowing rules and reference concepts from Module 02
//
// INSTRUCTIONS:
// 1. Read each function and understand what it's trying to do
// 2. Fix the compilation errors by applying borrowing concepts
// 3. Run `rustc ex02-borrowing.rs && ./ex02-borrowing` to test
// 4. Uncomment exercises one by one as you solve them

fn main() {
    println!("=== Exercise 2: Borrowing and References (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each exercise one by one and fix the errors
    exercise_2_1();
    // exercise_2_2();
    // exercise_2_3();
    // exercise_2_4();
    // exercise_2_5();
    // exercise_2_6();
    // exercise_2_7();
    // exercise_2_8();
    
    println!("\n🎉 All borrowing exercises completed!");
}

// Exercise 2.1: Basic immutable borrowing
// PROBLEM: We want to use data without taking ownership
fn exercise_2_1() {
    println!("Exercise 2.1: Basic immutable borrowing");
    
    let data = String::from("borrowed data");
    
    // TODO: Create an immutable reference to data
    let reference = data;  // FIXME: This moves data instead of borrowing!
    
    // TODO: Make both of these work
    println!("Original: {}", data);     // COMPILE ERROR!
    println!("Reference: {}", reference);
    
    // TODO: Calculate length using the reference
    let length = reference.len();  // This should work once reference is fixed
    println!("Length through reference: {}", length);
    
    // QUESTION: What's the difference between moving and borrowing?
    // ANSWER: ________________________________
    
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Mutable borrowing
// PROBLEM: We want to modify data through a reference
fn exercise_2_2() {
    println!("Exercise 2.2: Mutable borrowing");
    
    let data = String::from("mutable data");  // FIXME: Is this mutable?
    
    // TODO: Create a mutable reference
    let mut_ref = &data;  // FIXME: This creates immutable reference!
    
    // TODO: Modify data through the mutable reference
    mut_ref.push_str(" - modified");  // COMPILE ERROR!
    
    println!("Modified data: {}", data);
    
    // CHALLENGE: Why do we need both `mut` in two different places?
    // 1. `let mut data` does what? ____________________
    // 2. `&mut data` does what? _____________________
    
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: The borrowing rules - multiple references
// PROBLEM: Understanding when you can have multiple references
fn exercise_2_3() {
    println!("Exercise 2.3: Borrowing rules");
    
    let mut data = String::from("rules demo");
    
    // Try 1: Multiple immutable references
    let r1 = &data;
    let r2 = &data;
    println!("Multiple immutable refs: {} and {}", r1, r2);  // This works!
    
    // Try 2: Multiple mutable references  
    let mut_ref1 = &mut data;  // FIXME: This conflicts with r1 and r2 above!
    let mut_ref2 = &mut data;  // FIXME: Two mutable references!
    
    println!("Two mutable refs: {} and {}", mut_ref1, mut_ref2);  // COMPILE ERROR!
    
    // TODO: Fix the above code to follow Rust's borrowing rules
    // HINT: References have scopes - think about when they're last used
    
    println!("✅ Exercise 2.3 complete\n");
}

// Exercise 2.4: Reference scope and lifetime
// PROBLEM: References can't outlive the data they point to
fn exercise_2_4() {
    println!("Exercise 2.4: Reference scope and lifetime");
    
    let reference;
    {
        let temp_data = String::from("temporary");
        reference = &temp_data;  // FIXME: temp_data doesn't live long enough!
        
        println!("Inside scope: {}", reference);
    }
    // temp_data is dropped here
    
    println!("Outside scope: {}", reference);  // COMPILE ERROR!
    
    // TODO: Fix this by ensuring the reference has a valid lifetime
    // HINT: What if the data lived in the outer scope instead?
    
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Functions with borrowing
// PROBLEM: Functions should borrow, not take ownership
fn exercise_2_5() {
    println!("Exercise 2.5: Functions with borrowing");
    
    let mut text = String::from("hello");
    
    // This function takes ownership - we lose our data!
    let length = calculate_length_badly(text);  // FIXME: text is moved!
    println!("Length: {}", length);
    println!("Original text: {}", text);  // COMPILE ERROR!
    
    // TODO: Fix by making the function borrow instead of take ownership
    
    println!("✅ Exercise 2.5 complete\n");
}

// TODO: Fix this function to borrow instead of take ownership
fn calculate_length_badly(s: String) -> usize {
    s.len()
}  // s is dropped here - wasteful!

// TODO: Create a corrected version
// fn calculate_length_correctly(/* TODO: parameter */) -> usize {
//     // TODO: Implementation that borrows
// }

// Exercise 2.6: Modifying through mutable references
// PROBLEM: We want to modify data through function parameters
fn exercise_2_6() {
    println!("Exercise 2.6: Modifying through mutable references");
    
    let mut message = String::from("Hello");
    
    // This function should modify our string, but it takes ownership
    append_exclamation_badly(message);  // FIXME: message is moved!
    println!("After function: {}", message);  // COMPILE ERROR!
    
    // TODO: Fix by making the function take a mutable reference
    
    println!("✅ Exercise 2.6 complete\n");
}

// TODO: Fix this function to take a mutable reference
fn append_exclamation_badly(s: String) {
    // This doesn't work because we can't modify a moved value
    // and we can't return it back easily
    println!("Cannot modify: {}", s);
}

// TODO: Create a corrected version
// fn append_exclamation_correctly(/* TODO: parameter */) {
//     // TODO: Implementation that modifies through reference
// }

// Exercise 2.7: Slices and borrowing
// PROBLEM: Working with parts of collections
fn exercise_2_7() {
    println!("Exercise 2.7: Slices and borrowing");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // TODO: Create a slice of the first 3 elements
    let first_three = numbers;  // FIXME: This moves the whole vector!
    
    println!("First three: {:?}", first_three);
    println!("Original numbers: {:?}", numbers);  // COMPILE ERROR!
    
    // TODO: Fix to create a slice instead of moving
    // HINT: Use range syntax like [start..end]
    
    // BONUS: Try creating slices with different ranges
    // - First 2 elements: ____________
    // - Last 2 elements: _____________
    // - Middle element: ______________
    
    println!("✅ Exercise 2.7 complete\n");
}

// Exercise 2.8: String slices
// PROBLEM: Getting parts of strings efficiently
fn exercise_2_8() {
    println!("Exercise 2.8: String slices");
    
    let sentence = String::from("The quick brown fox");
    
    // TODO: Extract words without moving the whole string
    let first_word = get_first_word_badly(sentence);  // FIXME: sentence moved!
    println!("First word: {}", first_word);
    
    let second_word = get_second_word_badly(sentence);  // COMPILE ERROR!
    println!("Second word: {}", second_word);
    
    // TODO: Fix by using string slices (&str) instead of taking ownership
    
    println!("✅ Exercise 2.8 complete\n");
}

// TODO: Fix these functions to return string slices
fn get_first_word_badly(s: String) -> String {
    // This is inefficient - we're moving the whole string just to get one word
    let words: Vec<&str> = s.split_whitespace().collect();
    words[0].to_string()  // Converting back to String - more allocation!
}

fn get_second_word_badly(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[1].to_string()
}

// TODO: Create efficient versions using string slices
// fn get_first_word_correctly(/* TODO: parameter */) -> /* TODO: return type */ {
//     // TODO: Implementation using slices
// }

// fn get_second_word_correctly(/* TODO: parameter */) -> /* TODO: return type */ {
//     // TODO: Implementation using slices  
// }

// BONUS CHALLENGES (uncomment to try):

// Challenge 1: Fix this function that has multiple borrowing issues
// fn process_data(data: &mut Vec<i32>) {
//     let first = &data[0];        // Immutable borrow
//     data.push(6);                // FIXME: Mutable borrow while immutable exists!
//     println!("First: {}", first); // Using immutable borrow after mutable
// }

// Challenge 2: Create a function that returns the longest string slice
// fn find_longest<'a>(s1: &str, s2: &str) -> &str {  // FIXME: Missing lifetime!
//     if s1.len() > s2.len() { s1 } else { s2 }
// }

// Challenge 3: Fix this struct that tries to store a reference
// struct Container {
//     data: &str,  // FIXME: Missing lifetime parameter!
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_when_implemented_calculate_length() {
        // This test will work once you implement the correct function
        // let s = String::from("hello");
        // let len = calculate_length_correctly(&s);
        // assert_eq!(len, 5);
        // assert_eq!(s, "hello");  // s should still be usable
    }
    
    #[test] 
    fn test_when_implemented_string_slices() {
        // This test will work once you implement the slice functions
        // let sentence = String::from("hello world");
        // let first = get_first_word_correctly(&sentence);
        // let second = get_second_word_correctly(&sentence);
        // assert_eq!(first, "hello");
        // assert_eq!(second, "world");
        // assert_eq!(sentence, "hello world");  // sentence should still be usable
    }
    
    // TODO: Add your own tests to verify your implementations
}
