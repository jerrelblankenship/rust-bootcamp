# Exercise 2 Hints - Level 1: Gentle Guidance 🟢

## Improving CLI Error Messages

**Struggling with error handling?** Let's think about user experience first.

### 🤔 First Questions to Ask Yourself

1. **What makes an error message helpful vs frustrating?**
   - Compare: "Error!" vs "File 'data.txt' not found. Check the filename and try again."
   - Which would you prefer as a user?

2. **How do you handle errors in C# console applications?**
   - In C#: `try-catch` blocks with meaningful exception messages
   - In Rust: `Result<T, E>` types with pattern matching
   - Both can provide context and suggestions

3. **What information does a user need when something goes wrong?**
   - What went wrong?
   - Why did it go wrong?
   - How can they fix it?
   - What should they try next?

### 🔍 General Approach

1. **Start with the worst errors**: Find `unwrap()` and `panic!()` calls
2. **One error type at a time**: Fix argument errors, then file errors, then parsing errors
3. **Think like a user**: What would help you if you encountered this error?
4. **Add context gradually**: Start with basic messages, then add suggestions

### 💡 Key Concepts

**Error Types**: Different problems need different messages:
- Missing arguments → Show usage example
- File not found → Check filename and path
- Permission denied → Check file permissions
- Parse errors → Show what was expected

**Error Context**: Add helpful information:
- What operation was being attempted?
- What input caused the problem?
- What are valid alternatives?

### 🎯 Focus Areas

- **Replace panics**: No more `unwrap()` on user input
- **Helpful messages**: Clear, actionable error descriptions
- **Error categories**: Different error types for different problems
- **Exit codes**: Proper process exit codes for scripts

### ⏱️ Time Check

If you've been working for **15+ minutes** and still stuck on basic error handling patterns, check Level 2 hints.

### 🚀 Next Steps

1. Find all the `unwrap()` calls in the code
2. Think about what could go wrong with each operation
3. Replace panics with proper error handling
4. Add helpful context to each error message

Remember: Great error messages turn frustration into understanding!