# Exercise 5 Hints - Level 2 (More Specific Guidance)

## 🎯 Detailed Code Analysis

You're ready for deeper understanding of the text analysis code. Let's break down the complex patterns and show you how to modify them effectively.

## 🔧 Understanding Complex Iterator Chains

### The Word Extraction Pattern
```rust
fn extract_words(text: &str) -> Vec<String> {
    text.chars()                           // Convert to iterator of characters
        .filter_map(|c| {                 // Transform and filter simultaneously
            if c.is_alphabetic() || c.is_whitespace() {
                Some(c)                    // Keep alphabetic chars and spaces
            } else {
                Some(' ')                  // Replace punctuation with space
            }
        })
        .collect::<String>()               // Rebuild into a String
        .split_whitespace()                // Split on spaces
        .map(|s| s.to_string())           // Convert &str to String
        .filter(|s| !s.is_empty())        // Remove empty strings
        .collect()                         // Collect into Vec<String>
}
```

**C# Equivalent**:
```csharp
static List<string> ExtractWords(string text) {
    var cleanedText = new string(text.Select(c => 
        char.IsLetter(c) || char.IsWhiteSpace(c) ? c : ' ').ToArray());
    
    return cleanedText.Split(' ', StringSplitOptions.RemoveEmptyEntries)
                     .ToList();
}
```

**Key Learning Points**:
- `filter_map()` combines filtering and transformation
- `collect::<String>()` specifies the target type explicitly
- Method chaining creates a pipeline of transformations

### The Word Counting Pattern
```rust
fn count_words(words: &[String]) -> HashMap<String, usize> {
    let mut count = HashMap::new();
    for word in words {
        *count.entry(word.clone()).or_insert(0) += 1;
    }
    count
}
```

**Breakdown**:
- `entry()` gets or creates a HashMap entry
- `or_insert(0)` provides default value if key doesn't exist
- `*` dereferences to modify the value
- `clone()` creates owned String from reference

**Alternative Functional Style**:
```rust
fn count_words_functional(words: &[String]) -> HashMap<String, usize> {
    words.iter()
         .fold(HashMap::new(), |mut acc, word| {
             *acc.entry(word.clone()).or_insert(0) += 1;
             acc
         })
}
```

## 🔧 Understanding Complex Match Expressions

### The Transform Match Pattern
```rust
fn transform(text: &str, transform: TextTransform) -> String {
    match transform {
        TextTransform::ToUppercase => text.to_uppercase(),
        TextTransform::ToLowercase => text.to_lowercase(),
        TextTransform::TitleCase => Self::to_title_case(text),
        TextTransform::ReverseWords => Self::reverse_words(text),
        TextTransform::RemoveVowels => Self::remove_vowels(text),
        TextTransform::PigLatin => Self::to_pig_latin(text),
    }
}
```

**C# Equivalent**:
```csharp
static string Transform(string text, TextTransform transform) {
    return transform switch {
        TextTransform.ToUppercase => text.ToUpper(),
        TextTransform.ToLowercase => text.ToLower(),
        TextTransform.TitleCase => ToTitleCase(text),
        TextTransform.ReverseWords => ReverseWords(text),
        TextTransform.RemoveVowels => RemoveVowels(text),
        TextTransform.PigLatin => ToPigLatin(text),
        _ => throw new ArgumentException("Unknown transform")
    };
}
```

**Key Differences**:
- Rust match is exhaustive (must handle all cases)
- No default case needed if all variants are covered
- `Self::` calls associated functions (static methods)

### Complex Pattern Matching
```rust
match (avg_word_len, vocabulary_ratio) {
    (len, ratio) if len > 6.0 && ratio > 0.7 => ComplexityAnalysis {
        level: "Advanced".to_string(),
        reason: "Long average word length with high vocabulary diversity".to_string(),
    },
    (len, _) if len > 5.5 => ComplexityAnalysis {
        level: "Intermediate".to_string(),
        reason: "Moderately long words used".to_string(),
    },
    // ... more patterns
}
```

**Key Features**:
- Tuple destructuring: `(len, ratio)`
- Guard clauses: `if len > 6.0 && ratio > 0.7`
- Wildcard: `_` ignores values
- Pattern matching replaces if/else chains

## 🔧 Understanding String Manipulation

### Title Case Implementation
```rust
fn to_title_case(text: &str) -> String {
    text.split_whitespace()                    // Split into words
        .map(|word| {                          // Process each word
            let mut chars: Vec<char> = word.chars().collect();
            if let Some(first) = chars.first_mut() {
                *first = first.to_uppercase().next().unwrap_or(*first);
            }
            for ch in chars.iter_mut().skip(1) {
                *ch = ch.to_lowercase().next().unwrap_or(*ch);
            }
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()              // Collect words
        .join(" ")                             // Join with spaces
}
```

**Breakdown**:
- `chars.first_mut()` gets mutable reference to first character
- `if let Some(first)` handles the case where word might be empty
- `iter_mut().skip(1)` iterates over remaining characters mutably
- `into_iter().collect()` converts Vec<char> back to String

### Pig Latin Implementation
```rust
fn to_pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            if word.is_empty() {
                return word.to_string();
            }
            
            let first_char = word.chars().next().unwrap().to_lowercase().next().unwrap();
            if matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u') {
                format!("{}way", word)
            } else {
                let rest: String = word.chars().skip(1).collect();
                format!("{}{}ay", rest, first_char)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
```

**Key Patterns**:
- `matches!()` macro for pattern matching
- `chars().skip(1)` drops first character
- `format!()` for string interpolation (like C# $"")

## 🔧 Modification Exercises

### Exercise A: Add New Statistics
Add vowel counting to `TextStats`:

```rust
// 1. Add field to TextStats
struct TextStats {
    // ... existing fields
    vowel_count: usize,
    consonant_count: usize,
}

// 2. Calculate in calculate_stats
fn calculate_stats(original_text: &str, words: &[String]) -> TextStats {
    // ... existing calculations
    
    let vowel_count = original_text.chars()
        .filter(|c| matches!(c.to_lowercase().next().unwrap_or(*c), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count();
    
    let consonant_count = original_text.chars()
        .filter(|c| c.is_alphabetic())
        .count() - vowel_count;
    
    TextStats {
        // ... existing fields
        vowel_count,
        consonant_count,
    }
}

// 3. Display in print_analysis
fn print_analysis(&self) {
    // ... existing prints
    println!("   Vowels: {}", self.stats.vowel_count);
    println!("   Consonants: {}", self.stats.consonant_count);
}
```

### Exercise B: Add New Transform
Add `AlternatingCase` transform:

```rust
// 1. Add to enum
enum TextTransform {
    // ... existing variants
    AlternatingCase,
}

// 2. Implement the transformation
fn to_alternating_case(text: &str) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                c.to_uppercase().next().unwrap_or(c)
            }
        })
        .collect()
}

// 3. Add to match expression
TextTransform::AlternatingCase => Self::to_alternating_case(text),

// 4. Add to test list
(TextTransform::AlternatingCase, "Alternating Case"),
```

### Exercise C: Add Word Games
Add alliteration detection:

```rust
impl WordGame {
    fn has_alliteration(text: &str, min_words: usize) -> bool {
        let words: Vec<&str> = text.split_whitespace().collect();
        
        for window in words.windows(min_words) {
            let first_letters: Vec<char> = window.iter()
                .filter_map(|word| word.chars().next())
                .map(|c| c.to_lowercase().next().unwrap_or(c))
                .collect();
            
            if first_letters.len() == min_words && 
               first_letters.iter().all(|&c| c == first_letters[0]) {
                return true;
            }
        }
        false
    }
}
```

## 🎓 Advanced Patterns to Notice

### Builder Pattern in Action
The `TextAnalyzer::new()` method demonstrates the builder pattern:
1. Take simple input (`&str`)
2. Build complex intermediate data (`words`, `word_count`)
3. Combine into final structure

### Separation of Concerns
Notice how responsibilities are split:
- `TextAnalyzer`: Main orchestration
- `TextStats`: Data holder
- `TextProcessor`: Utility functions
- `WordGame`: Game-specific logic

### Error Handling Preparation
Functions use patterns that will be important in Module 02:
- `unwrap_or()` for defaults
- `Option<T>` handling with `if let`
- Validation before processing

## 💡 Key Learning Insights

1. **Iterators are powerful**: Method chaining replaces many loops
2. **Enums enable clean branching**: Match expressions replace if/else chains
3. **Structs organize complexity**: Group related data and behavior
4. **Ownership flows through methods**: Input `&str`, output `String`

## ➡️ Next Level

Ready to see the complete implementation details? Try [Level 3 Hints](ex05-level3.md) for comprehensive code walkthroughs.

You're mastering Rust's expressiveness - this is advanced programming with foundation-level concepts! 🦀