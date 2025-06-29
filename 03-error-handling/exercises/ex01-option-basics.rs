// Exercise 1: Option<T> Basics - Fix the Broken Code!
//
// Your mission: Make this code compile and work correctly by fixing
// all the compilation errors and implementing the missing functionality.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one  
// 3. Run `rustc ex01-option-basics.rs` frequently to check progress
// 4. Understand that Option<T> replaces nullable types from C#
// 5. Check solutions/ex01-option-basics.rs if you get completely stuck

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 1: Option<T> Basics (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each exercise one by one and fix the errors
    exercise_1_1();
    // exercise_1_2();
    // exercise_1_3();
    // exercise_1_4();
    // exercise_1_5();
    // exercise_1_6();
    // exercise_1_7();
    
    println!("\n🎉 All exercises completed successfully!");
}

// Exercise 1.1: Basic Option handling - BROKEN CODE TO FIX!
fn exercise_1_1() {
    println!("Exercise 1.1: Basic Option handling");
    
    // FIXME: This function doesn't compile - implement it!
    fn get_user_name(user_id: u32) -> String {
        let user = find_user(user_id);
        // TODO: Handle the Option<User> from find_user
        // HINT: What if the user is found? What if not found?
        // C# equivalent: user?.Name ?? "Unknown"
        todo!("Return user name or 'Unknown' if not found")
    }
    
    // Test with existing and non-existing users
    println!("User 1: {}", get_user_name(1));
    println!("User 999: {}", get_user_name(999));
    
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Option chaining - BROKEN CODE TO FIX!
fn exercise_1_2() {
    println!("Exercise 1.2: Option chaining");
    
    // FIXME: This function has compilation errors
    fn get_user_email_domain(user_id: u32) -> Option<String> {
        // TODO: Get the user, then get their email, then extract the domain
        // HINT: Use .map() and .and_then() for chaining
        // Step 1: find_user(user_id) -> Option<User>
        // Step 2: user.email -> String  
        // Step 3: extract domain after '@' -> Option<String>
        
        let user = find_user(user_id);
        // FIXME: How do you get the email from an Option<User>?
        // FIXME: How do you extract the domain from an email string?
        todo!("Chain operations to get email domain")
    }
    
    // Test cases
    match get_user_email_domain(1) {
        Some(domain) => println!("User 1 email domain: {}", domain),
        None => println!("User 1 email domain not found"),
    }
    
    match get_user_email_domain(999) {
        Some(domain) => println!("User 999 email domain: {}", domain),
        None => println!("User 999 email domain not found"),
    }
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Working with collections and Option - BROKEN CODE TO FIX!
fn exercise_1_3() {
    println!("Exercise 1.3: Working with collections and Option");
    
    let scores = vec![Some(85), None, Some(92), Some(78), None, Some(88)];
    
    // FIXME: This function doesn't compile - implement it!
    fn calculate_average(scores: &[Option<i32>]) -> Option<f64> {
        // TODO: Calculate average of only the Some(score) values
        // HINT: filter_map() can help extract only the Some values
        // QUESTION: What should you return if all scores are None?
        
        // Step 1: Extract only the valid scores (filter out None)
        // Step 2: Calculate sum and count
        // Step 3: Return None if no valid scores, otherwise return Some(average)
        
        todo!("Calculate average of valid scores")
    }
    
    match calculate_average(&scores) {
        Some(avg) => println!("Average score: {:.2}", avg),
        None => println!("No scores available"),
    }
    
    println!("✅ Exercise 1.3 complete\n");
}

// Exercise 1.4: Option with error handling - BROKEN CODE TO FIX!
fn exercise_1_4() {
    println!("Exercise 1.4: Option with error handling");
    
    // FIXME: These functions are missing implementations
    fn safe_parse_int(s: &str) -> Option<i32> {
        // TODO: Parse string to i32, return None if it fails
        // HINT: s.parse() returns Result<i32, _>, how do you convert to Option?
        todo!("Safely parse string to integer")
    }
    
    fn safe_divide(a: i32, b: i32) -> Option<f64> {
        // TODO: Return None if dividing by zero, otherwise return Some(result)
        todo!("Safely divide two numbers")
    }
    
    // FIXME: This function doesn't compile
    fn parse_and_divide(a_str: &str, b_str: &str) -> Option<f64> {
        // TODO: Parse both strings, then divide them
        // HINT: Use the ? operator to chain Option operations
        // OR use and_then() for explicit chaining
        
        // Step 1: Parse a_str to get Option<i32>
        // Step 2: Parse b_str to get Option<i32>  
        // Step 3: If both succeed, divide them
        
        todo!("Parse both strings and divide them")
    }
    
    // Test cases
    let test_cases = [
        ("10", "2"),
        ("15", "3"),
        ("20", "0"),  // Division by zero
        ("abc", "5"), // Parse error
        ("8", "def"), // Parse error
    ];
    
    for (a, b) in test_cases {
        match parse_and_divide(a, b) {
            Some(result) => println!("{} / {} = {:.2}", a, b, result),
            None => println!("{} / {} = Error", a, b),
        }
    }
    
    println!("✅ Exercise 1.4 complete\n");
}

// Exercise 1.5: Option in structs - BROKEN CODE TO FIX!
fn exercise_1_5() {
    println!("Exercise 1.5: Option in structs");
    
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,  // Email is optional (like nullable in C#)
        phone: Option<String>,  // Phone is optional
    }
    
    impl Person {
        // FIXME: This method doesn't compile - implement it!
        fn preferred_contact(&self) -> Option<&str> {
            // TODO: Return email if available, otherwise phone, otherwise None
            // HINT: You can use .as_deref() to convert Option<String> to Option<&str>
            // HINT: .or_else() can provide a fallback Option
            todo!("Return preferred contact method")
        }
        
        // FIXME: This method doesn't compile - implement it!
        fn contact_info(&self) -> String {
            // TODO: Format contact info nicely
            // If email exists: "Name <email>"
            // If no email but phone exists: "Name (phone)"
            // If neither: just "Name"
            // HINT: Match on (&self.email, &self.phone)
            todo!("Format contact information")
        }
    }
    
    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
            phone: Some("555-1234".to_string()),
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            email: None,
            phone: Some("555-5678".to_string()),
        },
        Person {
            name: "Carol".to_string(),
            age: 35,
            email: Some("carol@example.com".to_string()),
            phone: None,
        },
        Person {
            name: "Dave".to_string(),
            age: 28,
            email: None,
            phone: None,
        },
    ];
    
    for person in &people {
        println!("Contact info: {}", person.contact_info());
        match person.preferred_contact() {
            Some(contact) => println!("  Preferred: {}", contact),
            None => println!("  No contact available"),
        }
    }
    
    println!("✅ Exercise 1.5 complete\n");
}

// Exercise 1.6: Converting between Option and Result - BROKEN CODE TO FIX!
fn exercise_1_6() {
    println!("Exercise 1.6: Converting between Option and Result");
    
    // FIXME: These conversion functions are missing
    fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
        // TODO: Convert Option to Result
        // HINT: Use .ok_or() or .ok_or_else()
        todo!("Convert Option to Result")
    }
    
    fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
        // TODO: Convert Result to Option (ignore the error)
        // HINT: Use .ok()
        todo!("Convert Result to Option")
    }
    
    // Test the conversions
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("Something went wrong");
    
    // Convert Option to Result
    println!("Some(42) -> Result: {:?}", option_to_result(some_value, "No value"));
    println!("None -> Result: {:?}", option_to_result(none_value, "No value"));
    
    // Convert Result to Option
    println!("Ok(100) -> Option: {:?}", result_to_option(ok_result));
    println!("Err -> Option: {:?}", result_to_option(err_result));
    
    println!("✅ Exercise 1.6 complete\n");
}

// Exercise 1.7: Option with iterators - BROKEN CODE TO FIX!
fn exercise_1_7() {
    println!("Exercise 1.7: Option with iterators");
    
    let usernames = vec!["alice", "bob", "", "carol", "dave", ""];
    
    // FIXME: These functions don't compile - implement them!
    fn get_valid_usernames(usernames: &[&str]) -> Vec<String> {
        // TODO: Filter out empty usernames and collect valid ones
        // HINT: Use .filter_map() to both filter and transform
        todo!("Filter out empty usernames")
    }
    
    fn find_user_starting_with(usernames: &[&str], letter: char) -> Option<String> {
        // TODO: Find first valid username starting with the given letter
        // HINT: Chain .filter_map() and .find()
        todo!("Find user starting with letter")
    }
    
    fn get_username_lengths(usernames: &[&str]) -> Vec<usize> {
        // TODO: Get lengths of all valid usernames (skip empty ones)
        // HINT: Use .filter_map() to skip empty and get lengths
        todo!("Get lengths of valid usernames")
    }
    
    let valid_users = get_valid_usernames(&usernames);
    println!("Valid usernames: {:?}", valid_users);
    
    match find_user_starting_with(&usernames, 'c') {
        Some(user) => println!("Found user starting with 'c': {}", user),
        None => println!("No user found starting with 'c'"),
    }
    
    let lengths = get_username_lengths(&usernames);
    println!("Username lengths: {:?}", lengths);
    
    println!("✅ Exercise 1.7 complete\n");
}

// Helper functions for the exercises (these work correctly!)
fn find_user(user_id: u32) -> Option<User> {
    let users = get_test_users();
    users.into_iter().find(|u| u.id == user_id)
}

fn get_test_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@test.org".to_string(),
        },
        User {
            id: 3,
            name: "Carol".to_string(),
            email: "carol@company.net".to_string(),
        },
    ]
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // FIXME: These tests won't compile until you implement the functions
    #[test]
    fn test_find_user() {
        assert!(find_user(1).is_some());
        assert!(find_user(999).is_none());
    }
    
    #[test]
    fn test_get_user_name() {
        // TODO: Uncomment when you implement get_user_name
        // fn get_user_name(user_id: u32) -> String { ... }
        // assert_eq!(get_user_name(1), "Alice");
        // assert_eq!(get_user_name(999), "Unknown");
    }
    
    #[test]
    fn test_email_domain_extraction() {
        // TODO: Uncomment when you implement get_user_email_domain
        // assert_eq!(get_user_email_domain(1), Some("example.com".to_string()));
        // assert_eq!(get_user_email_domain(999), None);
    }
    
    #[test]
    fn test_safe_parsing() {
        // TODO: Uncomment when you implement safe_parse_int
        // assert_eq!(safe_parse_int("42"), Some(42));
        // assert_eq!(safe_parse_int("abc"), None);
    }
    
    // TODO: Add more tests as you implement the functions
}

// COMPILATION CHALLENGES:
// 1. Start with exercise_1_1 - implement get_user_name
// 2. Use match or .map() and .unwrap_or() 
// 3. Move to exercise_1_2 - chain Option operations
// 4. Learn .and_then() for chaining operations that return Option
// 5. Exercise_1_3 - work with collections of Options
// 6. Use .filter_map() to extract Some values
// 7. Exercise_1_4 - convert between Result and Option
// 8. Learn the ? operator for early returns
// 9. Exercise_1_5 - handle optional struct fields
// 10. Exercise_1_6 - explicit conversions between Option and Result
// 11. Exercise_1_7 - combine Option with iterator methods
//
// KEY CONCEPTS FOR C# DEVELOPERS:
// - Option<T> replaces nullable types
// - No null reference exceptions possible!
// - .map() is like LINQ Select()
// - .filter_map() is like Where() + Select() combined
// - .and_then() is for chaining operations that might fail
// - ? operator is similar to null-conditional operator ?.
//
// Remember: The goal is to make null reference errors impossible at compile time!
