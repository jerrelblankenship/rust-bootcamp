# Progressive Hint System 🎯

Building CLI tools? Follow this hint progression when stuck:

## When to Use Hints

1. **First 15 minutes**: Read compiler errors and clap docs
2. **After 15 minutes**: Check Level 1 hints
3. **After 30 minutes**: Check Level 2 hints  
4. **After 45 minutes**: Check Level 3 hints

## Hint Levels

### 🟢 Level 1: CLI Concepts
- Points to relevant crate documentation
- Asks about user experience design
- Suggests similar C# console app patterns

### 🟡 Level 2: Implementation Guidance
- Shows specific clap/structopt patterns
- Provides error handling strategies
- Demonstrates cross-platform considerations

### 🔴 Level 3: Working Examples
- Shows most of the implementation
- Explains design decisions
- Leaves integration details to complete

## CLI Development Process

1. **Design First** - What's the ideal user experience?
2. **Parse Arguments** - Use derive macros for type safety
3. **Handle Errors** - Provide helpful error messages
4. **Test Experience** - Try edge cases and bad input
5. **Polish Output** - Colors, progress, formatting

## Example Hint Progression

**Exercise**: Fix broken argument parser
- **Level 1**: "How would PowerShell handle this? Check clap's derive docs"
- **Level 2**: "You need `#[clap(short, long)]` attributes. In C#, this is like..."
- **Level 3**: "Use `#[derive(Parser)]` with this structure..."

Remember: Great CLI tools are a joy to use. Focus on user experience!