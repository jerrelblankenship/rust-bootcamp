// Exercise 1: Option<T> Basics - COMPLETE SOLUTIONS
//
// This file contains the complete working solutions for all Option exercises.
// Students should only reference this after attempting the broken starter code.

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 1: Option<T> Basics - SOLUTIONS ===\n");
    
    // Run all exercises
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_1_5();
    exercise_1_6();
    exercise_1_7();
}

// Exercise 1.1: Basic Option handling
fn exercise_1_1() {
    println!("Exercise 1.1: Basic Option handling");
    
    // SOLVED: Return the user's name or "Unknown" if not found
    fn get_user_name(user_id: u32) -> String {
        let user = find_user(user_id);
        match user {
            Some(u) => u.name,
            None => "Unknown".to_string(),
        }
        // Alternative using unwrap_or_else:
        // user.map(|u| u.name).unwrap_or_else(|| "Unknown".to_string())
    }
    
    // Test with existing and non-existing users
    println!("User 1: {}", get_user_name(1));
    println!("User 999: {}", get_user_name(999));
    
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Option chaining
fn exercise_1_2() {
    println!("Exercise 1.2: Option chaining");
    
    // SOLVED: Get the email domain using Option chaining methods
    fn get_user_email_domain(user_id: u32) -> Option<String> {
        find_user(user_id)
            .map(|user| user.email)
            .and_then(|email| {
                email.split('@').nth(1).map(|domain| domain.to_string())
            })
    }
    
    // Test cases
    match get_user_email_domain(1) {
        Some(domain) => println!("User 1 email domain: {}", domain),
        None => println!("User 1 email domain not found"),
    }
    
    match get_user_email_domain(2) {
        Some(domain) => println!("User 2 email domain: {}", domain),
        None => println!("User 2 email domain not found"),
    }
    
    match get_user_email_domain(999) {
        Some(domain) => println!("User 999 email domain: {}", domain),
        None => println!("User 999 email domain not found"),
    }
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Working with collections and Option
fn exercise_1_3() {
    println!("Exercise 1.3: Working with collections and Option");
    
    let scores = vec![Some(85), None, Some(92), Some(78), None, Some(88)];
    
    // SOLVED: Calculate the average of the scores that are present
    fn calculate_average(scores: &[Option<i32>]) -> Option<f64> {
        let valid_scores: Vec<i32> = scores.iter().filter_map(|&score| score).collect();
        
        if valid_scores.is_empty() {
            None
        } else {
            let sum: i32 = valid_scores.iter().sum();
            Some(sum as f64 / valid_scores.len() as f64)
        }
    }
    
    match calculate_average(&scores) {
        Some(avg) => println!("Average score: {:.2}", avg),
        None => println!("No scores available"),
    }
    
    println!("✅ Exercise 1.3 complete\n");
}

// Exercise 1.4: Option with error handling
fn exercise_1_4() {
    println!("Exercise 1.4: Option with error handling");
    
    // SOLVED: Safely parse a string to an integer
    fn safe_parse_int(s: &str) -> Option<i32> {
        s.parse().ok()
    }
    
    // SOLVED: Safely divide two numbers
    fn safe_divide(a: i32, b: i32) -> Option<f64> {
        if b == 0 {
            None
        } else {
            Some(a as f64 / b as f64)
        }
    }
    
    // SOLVED: Combine the above functions to parse two strings and divide them
    fn parse_and_divide(a_str: &str, b_str: &str) -> Option<f64> {
        let a = safe_parse_int(a_str)?;
        let b = safe_parse_int(b_str)?;
        safe_divide(a, b)
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

// Exercise 1.5: Option in structs
fn exercise_1_5() {
    println!("Exercise 1.5: Option in structs");
    
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,  // Email is optional
        phone: Option<String>,  // Phone is optional
    }
    
    impl Person {
        // SOLVED: Return email if available, otherwise phone, otherwise None
        fn preferred_contact(&self) -> Option<&str> {
            self.email.as_deref().or_else(|| self.phone.as_deref())
        }
        
        // SOLVED: Return formatted contact string
        fn contact_info(&self) -> String {
            match (&self.email, &self.phone) {
                (Some(email), _) => format!("{} <{}>", self.name, email),
                (None, Some(phone)) => format!("{} ({})", self.name, phone),
                (None, None) => self.name.clone(),
            }
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

// Exercise 1.6: Converting between Option and Result
fn exercise_1_6() {
    println!("Exercise 1.6: Converting between Option and Result");
    
    // SOLVED: Convert Option to Result
    fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
        opt.ok_or_else(|| error_msg.to_string())
    }
    
    // SOLVED: Convert Result to Option
    fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
        res.ok()
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

// Exercise 1.7: Option with iterators
fn exercise_1_7() {
    println!("Exercise 1.7: Option with iterators");
    
    let usernames = vec!["alice", "bob", "", "carol", "dave", ""];
    
    // SOLVED: Filter out empty usernames and collect valid ones
    fn get_valid_usernames(usernames: &[&str]) -> Vec<String> {
        usernames
            .iter()
            .filter_map(|&username| {
                if username.is_empty() {
                    None
                } else {
                    Some(username.to_string())
                }
            })
            .collect()
    }
    
    // SOLVED: Find the first user whose name starts with a specific letter
    fn find_user_starting_with(usernames: &[&str], letter: char) -> Option<String> {
        usernames
            .iter()
            .filter_map(|&username| {
                if username.is_empty() {
                    None
                } else {
                    Some(username)
                }
            })
            .find(|username| {
                username.chars().next().map_or(false, |c| c == letter)
            })
            .map(|username| username.to_string())
    }
    
    // SOLVED: Get the lengths of all valid usernames
    fn get_username_lengths(usernames: &[&str]) -> Vec<usize> {
        usernames
            .iter()
            .filter_map(|&username| {
                if username.is_empty() {
                    None
                } else {
                    Some(username.len())
                }
            })
            .collect()
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

// Helper functions for the exercises
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
    
    #[test]
    fn test_find_user() {
        assert!(find_user(1).is_some());
        assert!(find_user(999).is_none());
    }
    
    #[test]
    fn test_user_email_extraction() {
        let user = find_user(1).unwrap();
        assert!(user.email.contains('@'));
    }
    
    #[test]
    fn test_get_user_name() {
        // Create a local function for testing
        fn get_user_name(user_id: u32) -> String {
            let user = find_user(user_id);
            match user {
                Some(u) => u.name,
                None => "Unknown".to_string(),
            }
        }
        
        assert_eq!(get_user_name(1), "Alice");
        assert_eq!(get_user_name(999), "Unknown");
    }
    
    #[test]
    fn test_email_domain_extraction() {
        fn get_user_email_domain(user_id: u32) -> Option<String> {
            find_user(user_id)
                .map(|user| user.email)
                .and_then(|email| {
                    email.split('@').nth(1).map(|domain| domain.to_string())
                })
        }
        
        assert_eq!(get_user_email_domain(1), Some("example.com".to_string()));
        assert_eq!(get_user_email_domain(2), Some("test.org".to_string()));
        assert_eq!(get_user_email_domain(999), None);
    }
    
    #[test]
    fn test_calculate_average() {
        fn calculate_average(scores: &[Option<i32>]) -> Option<f64> {
            let valid_scores: Vec<i32> = scores.iter().filter_map(|&score| score).collect();
            
            if valid_scores.is_empty() {
                None
            } else {
                let sum: i32 = valid_scores.iter().sum();
                Some(sum as f64 / valid_scores.len() as f64)
            }
        }
        
        let scores = vec![Some(85), None, Some(92), Some(78), None, Some(88)];
        let avg = calculate_average(&scores).unwrap();
        assert!((avg - 85.75).abs() < 0.01);
        
        let empty_scores: Vec<Option<i32>> = vec![None, None];
        assert!(calculate_average(&empty_scores).is_none());
    }
}
