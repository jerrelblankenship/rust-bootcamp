# Exercise 3 Hints - Level 1: Gentle Guidance 🟢

## Configuration Management Fundamentals

**Working on configuration loading?** Let's think about how configuration should work.

### 🤔 First Questions to Ask Yourself

1. **How do you handle configuration in C# applications?**
   - appsettings.json with hierarchical overrides
   - Environment variables (Development, Production)
   - Command line arguments for overrides
   - IConfiguration with dependency injection

2. **What should happen when configuration is missing or invalid?**
   - Graceful fallback to defaults?
   - Clear error message about what's wrong?
   - Suggestions for fixing the configuration?

3. **What's the right precedence order for configuration sources?**
   - Command line (highest priority - immediate override)
   - Environment variables (deployment-specific)
   - Configuration files (project defaults)
   - Built-in defaults (lowest priority - fallback)

### 🔍 General Approach

1. **Start with defaults**: Every setting should have a sensible default
2. **Layer on file config**: Override defaults with file settings
3. **Add environment variables**: Allow deployment-specific overrides
4. **Top with CLI args**: Allow immediate overrides for testing
5. **Validate the result**: Ensure final config makes sense

### 💡 Key Concepts

**Configuration Hierarchies**: Like ASP.NET Core's configuration system:
- Multiple sources can contribute values
- Later sources override earlier ones
- Each source is optional (graceful degradation)

**Serde + TOML**: Rust's equivalent to JSON configuration:
- `serde` handles serialization/deserialization
- TOML is human-friendly like YAML but stricter
- Derive macros make it easy (like DataContract in C#)

**Environment Variable Patterns**: 
- Prefix to avoid conflicts (MYAPP_DATABASE_URL)
- Nested config via underscore (MYAPP_SERVER_PORT)
- Standard environment variables (PORT, DATABASE_URL)

### 🎯 Focus Areas

- **Default Implementation**: What are sensible defaults?
- **File Loading**: How to find and parse config files?
- **Environment Variables**: Which ones should override what?
- **Validation**: What makes a configuration valid/invalid?

### ⏱️ Time Check

If you've been working for **15+ minutes** and still stuck on basic TOML parsing or serde, check Level 2 hints.

### 🚀 Next Steps

1. Start with the Default trait - what should default values be?
2. Get basic TOML file loading working
3. Add environment variable reading
4. Implement the precedence chain
5. Add comprehensive validation

Remember: Good configuration makes deployment easy and debugging clear!