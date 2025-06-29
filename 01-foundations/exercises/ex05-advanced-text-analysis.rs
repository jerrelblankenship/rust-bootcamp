// Advanced Exercise: Text Analysis Tool
//
// This exercise demonstrates more advanced Rust concepts while still using
// only the foundations covered in Module 01. It's perfect for learners who
// want to challenge themselves further.

use std::collections::HashMap;

fn main() {
    println!("=== Advanced Exercise: Text Analysis Tool ===\n");
    
    let sample_text = "The quick brown fox jumps over the lazy dog. \
                       The dog was very lazy indeed. \
                       Quick brown foxes are quite remarkable animals.";
    
    let analyzer = TextAnalyzer::new(sample_text);
    analyzer.print_analysis();
    
    // Test with a poem
    println!("\n" + "=".repeat(50).as_str());
    println!("Analysis of a poem:");
    
    let poem = "Roses are red,\n\
               Violets are blue,\n\
               Rust is awesome,\n\
               And so are you!";
    
    let poem_analyzer = TextAnalyzer::new(poem);
    poem_analyzer.print_analysis();
    
    // Test the text processor
    println!("\n" + "=".repeat(50).as_str());
    println!("Text Processing Examples:");
    
    test_text_processor();
}

// Main struct for text analysis
#[derive(Debug)]
struct TextAnalyzer {
    text: String,
    words: Vec<String>,
    word_count: HashMap<String, usize>,
    stats: TextStats,
}

#[derive(Debug)]
struct TextStats {
    character_count: usize,
    word_count: usize,
    sentence_count: usize,
    paragraph_count: usize,
    average_word_length: f64,
    longest_word: String,
    shortest_word: String,
}

impl TextAnalyzer {
    fn new(text: &str) -> Self {
        let cleaned_text = text.to_lowercase();
        let words = Self::extract_words(&cleaned_text);
        let word_count = Self::count_words(&words);
        let stats = Self::calculate_stats(text, &words);
        
        TextAnalyzer {
            text: text.to_string(),
            words,
            word_count,
            stats,
        }
    }
    
    fn extract_words(text: &str) -> Vec<String> {
        text.chars()
            .filter_map(|c| {
                if c.is_alphabetic() || c.is_whitespace() {
                    Some(c)
                } else {
                    Some(' ')
                }
            })
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    fn count_words(words: &[String]) -> HashMap<String, usize> {
        let mut count = HashMap::new();
        for word in words {
            *count.entry(word.clone()).or_insert(0) += 1;
        }
        count
    }
    
    fn calculate_stats(original_text: &str, words: &[String]) -> TextStats {
        let character_count = original_text.chars().count();
        let word_count = words.len();
        
        let sentence_count = original_text
            .chars()
            .filter(|&c| c == '.' || c == '!' || c == '?')
            .count()
            .max(1);
        
        let paragraph_count = original_text
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .count()
            .max(1);
        
        let total_word_length: usize = words.iter().map(|w| w.len()).sum();
        let average_word_length = if word_count > 0 {
            total_word_length as f64 / word_count as f64
        } else {
            0.0
        };
        
        let longest_word = words
            .iter()
            .max_by_key(|w| w.len())
            .cloned()
            .unwrap_or_default();
        
        let shortest_word = words
            .iter()
            .min_by_key(|w| w.len())
            .cloned()
            .unwrap_or_default();
        
        TextStats {
            character_count,
            word_count,
            sentence_count,
            paragraph_count,
            average_word_length,
            longest_word,
            shortest_word,
        }
    }
    
    fn print_analysis(&self) {
        println!("📊 Text Analysis Results:");
        println!("─".repeat(40));
        println!("📝 Basic Statistics:");
        println!("   Characters: {}", self.stats.character_count);
        println!("   Words: {}", self.stats.word_count);
        println!("   Sentences: {}", self.stats.sentence_count);
        println!("   Paragraphs: {}", self.stats.paragraph_count);
        println!("   Average word length: {:.2} characters", self.stats.average_word_length);
        println!("   Longest word: '{}'", self.stats.longest_word);
        println!("   Shortest word: '{}'", self.stats.shortest_word);
        
        println!("\n🔤 Most Common Words:");
        let mut word_pairs: Vec<_> = self.word_count.iter().collect();
        word_pairs.sort_by(|a, b| b.1.cmp(a.1));
        
        for (word, count) in word_pairs.iter().take(5) {
            println!("   '{}': {} time(s)", word, count);
        }
        
        if self.stats.word_count > 0 {
            let reading_time = (self.stats.word_count as f64 / 200.0).ceil() as usize;
            println!("\n⏱️  Estimated reading time: {} minute(s)", reading_time);
        }
        
        println!("\n🎯 Text Complexity:");
        let complexity = self.analyze_complexity();
        println!("   Complexity level: {}", complexity.level);
        println!("   Reason: {}", complexity.reason);
    }
    
    fn analyze_complexity(&self) -> ComplexityAnalysis {
        let avg_word_len = self.stats.average_word_length;
        let unique_words = self.word_count.len();
        let total_words = self.stats.word_count;
        let vocabulary_ratio = if total_words > 0 {
            unique_words as f64 / total_words as f64
        } else {
            0.0
        };
        
        match (avg_word_len, vocabulary_ratio) {
            (len, ratio) if len > 6.0 && ratio > 0.7 => ComplexityAnalysis {
                level: "Advanced".to_string(),
                reason: "Long average word length with high vocabulary diversity".to_string(),
            },
            (len, _) if len > 5.5 => ComplexityAnalysis {
                level: "Intermediate".to_string(),
                reason: "Moderately long words used".to_string(),
            },
            (_, ratio) if ratio > 0.8 => ComplexityAnalysis {
                level: "Intermediate".to_string(),
                reason: "High vocabulary diversity".to_string(),
            },
            _ => ComplexityAnalysis {
                level: "Basic".to_string(),
                reason: "Simple vocabulary and word structure".to_string(),
            },
        }
    }
}

#[derive(Debug)]
struct ComplexityAnalysis {
    level: String,
    reason: String,
}

// Additional text processing utilities
enum TextTransform {
    ToUppercase,
    ToLowercase,
    TitleCase,
    ReverseWords,
    RemoveVowels,
    PigLatin,
}

struct TextProcessor;

impl TextProcessor {
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
    
    fn to_title_case(text: &str) -> String {
        text.split_whitespace()
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                for ch in chars.iter_mut().skip(1) {
                    *ch = ch.to_lowercase().next().unwrap_or(*ch);
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    fn reverse_words(text: &str) -> String {
        text.split_whitespace()
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    fn remove_vowels(text: &str) -> String {
        text.chars()
            .filter(|&c| !matches!(c.to_lowercase().next().unwrap_or(c), 'a' | 'e' | 'i' | 'o' | 'u'))
            .collect()
    }
    
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
}

fn test_text_processor() {
    let sample = "Hello World";
    
    let transforms = vec![
        (TextTransform::ToUppercase, "Uppercase"),
        (TextTransform::ToLowercase, "Lowercase"),
        (TextTransform::TitleCase, "Title Case"),
        (TextTransform::ReverseWords, "Reverse Words"),
        (TextTransform::RemoveVowels, "Remove Vowels"),
        (TextTransform::PigLatin, "Pig Latin"),
    ];
    
    println!("Original: '{}'", sample);
    for (transform, name) in transforms {
        let result = TextProcessor::transform(sample, transform);
        println!("{}: '{}'", name, result);
    }
}

// Word game utilities
struct WordGame;

impl WordGame {
    fn is_palindrome(word: &str) -> bool {
        let cleaned: String = word.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        
        cleaned == cleaned.chars().rev().collect::<String>()
    }
    
    fn is_anagram(word1: &str, word2: &str) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        
        let mut chars1: Vec<char> = word1.to_lowercase().chars().collect();
        let mut chars2: Vec<char> = word2.to_lowercase().chars().collect();
        
        chars1.sort_unstable();
        chars2.sort_unstable();
        
        chars1 == chars2
    }
    
    fn word_score(word: &str) -> u32 {
        word.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| {
                let c = c.to_lowercase().next().unwrap();
                (c as u32) - ('a' as u32) + 1
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_analyzer() {
        let analyzer = TextAnalyzer::new("Hello world! This is a test.");
        assert!(analyzer.stats.word_count > 0);
        assert!(analyzer.stats.character_count > 0);
        assert_eq!(analyzer.stats.sentence_count, 2);
    }
    
    #[test]
    fn test_word_extraction() {
        let words = TextAnalyzer::extract_words("hello, world! how are you?");
        assert_eq!(words, vec!["hello", "world", "how", "are", "you"]);
    }
    
    #[test]
    fn test_text_transforms() {
        assert_eq!(TextProcessor::transform("hello", TextTransform::ToUppercase), "HELLO");
        assert_eq!(TextProcessor::transform("HELLO", TextTransform::ToLowercase), "hello");
        assert_eq!(TextProcessor::transform("hello world", TextTransform::TitleCase), "Hello World");
        assert_eq!(TextProcessor::transform("hello world", TextTransform::ReverseWords), "olleh dlrow");
    }
    
    #[test]
    fn test_word_games() {
        assert!(WordGame::is_palindrome("racecar"));
        assert!(!WordGame::is_palindrome("hello"));
        
        assert!(WordGame::is_anagram("listen", "silent"));
        assert!(!WordGame::is_anagram("hello", "world"));
        
        assert_eq!(WordGame::word_score("a"), 1);
        assert_eq!(WordGame::word_score("z"), 26);
    }
    
    #[test]
    fn test_pig_latin() {
        assert_eq!(TextProcessor::transform("hello", TextTransform::PigLatin), "ellohay");
        assert_eq!(TextProcessor::transform("apple", TextTransform::PigLatin), "appleway");
    }
}
