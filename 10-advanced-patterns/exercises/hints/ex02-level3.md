# Exercise 02 - Level 3 Hints: Procedural Macro Panic

## 🎯 Complete Solutions

Here are the complete fixes for the procedural macro simulation:

## 🔧 Complete Working Code

```rust
use std::fmt;

// Fixed token parsing
impl TokenStreamParser for SimpleParser {
    fn parse_struct_fields(&self, input: &str) -> Vec<Field> {
        let mut fields = Vec::new();
        
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            if let Some(colon_pos) = line.find(':') {
                let name_part = line[..colon_pos].trim();
                let type_part = line[colon_pos + 1..].trim();
                
                let name = name_part.replace("pub ", "").trim().to_string();
                let type_name = type_part.trim_end_matches(',').trim().to_string();
                
                if !name.is_empty() && !type_name.is_empty() {
                    fields.push(Field { name, type_name });
                }
            }
        }
        
        fields
    }
    
    fn generate_impl_block(&self, fields: &[Field]) -> String {
        let mut impl_block = String::new();
        impl_block.push_str("impl SampleStruct {\n");
        
        for field in fields {
            // Generate getter method
            impl_block.push_str(&format!(
                "    pub fn {}(&self) -> &{} {{\n",
                field.name, field.type_name
            ));
            impl_block.push_str(&format!(
                "        &self.{}\n",
                field.name
            ));
            impl_block.push_str("    }\n");
            
            // Generate setter method with proper type handling
            if field.type_name.starts_with('&') {
                // Handle reference types differently
                impl_block.push_str(&format!(
                    "    pub fn set_{}(&mut self, value: {}) {{\n",
                    field.name, field.type_name
                ));
            } else {
                impl_block.push_str(&format!(
                    "    pub fn set_{}(&mut self, value: {}) {{\n",
                    field.name, field.type_name
                ));
            }
            impl_block.push_str(&format!(
                "        self.{} = value;\n",
                field.name
            ));
            impl_block.push_str("    }\n");
        }
        
        impl_block.push_str("}\n");
        impl_block
    }
}

// Fixed Debug implementation generation
impl MacroExpansion for BrokenExpansion {
    fn expand_derive_debug(&self, struct_name: &str, fields: &[Field]) -> String {
        let mut debug_impl = String::new();
        debug_impl.push_str(&format!("impl fmt::Debug for {} {{\n", struct_name));
        debug_impl.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
        
        debug_impl.push_str(&format!("        f.debug_struct(\"{}\")\n", struct_name));
        
        for field in fields {
            debug_impl.push_str(&format!(
                "            .field(\"{}\", &self.{})\n",
                field.name, field.name
            ));
        }
        
        debug_impl.push_str("            .finish()\n");
        debug_impl.push_str("    }\n");
        debug_impl.push_str("}\n");
        
        debug_impl
    }
    
    fn expand_attribute_macro(&self, method_name: &str) -> String {
        format!(
            "pub fn {}_cached(&self) -> Result<CachedValue, Box<dyn std::error::Error>> {{\n    // Cache implementation would go here\n    Ok(CachedValue::new())\n}}",
            method_name
        )
    }
}

// Fixed SQL macro
macro_rules! sql_query {
    (SELECT $($field:ident),+ FROM $table:ident WHERE $condition:expr) => {
        {
            let fields = vec![$(stringify!($field)),+];
            let table = stringify!($table);
            let query = format!("SELECT {} FROM {} WHERE {}", 
                fields.join(", "), table, $condition);
            SqlQuery::new(query)
        }
    };
    (INSERT INTO $table:ident ($($field:ident),+) VALUES ($($value:expr),+)) => {
        {
            let fields = vec![$(stringify!($field)),+];
            let values = vec![$(format!("{:?}", $value)),+];
            let table = stringify!($table);
            let query = format!("INSERT INTO {} ({}) VALUES ({})", 
                table, fields.join(", "), values.join(", "));
            SqlQuery::new(query)
        }
    };
}

// Helper types
struct CachedValue;

impl CachedValue {
    fn new() -> Self {
        Self
    }
}
```

## 🔧 Fixed Test Cases

```rust
#[cfg(test)]
mod proc_macro_tests {
    use super::*;
    
    #[test]
    fn test_token_parsing() {
        let parser = SimpleParser;
        let input = "pub name: String,\npub age: u32,";
        let fields = parser.parse_struct_fields(input);
        
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "name");
        assert_eq!(fields[0].type_name, "String");
        assert_eq!(fields[1].name, "age");
        assert_eq!(fields[1].type_name, "u32");
    }
    
    #[test]
    fn test_debug_expansion() {
        let expander = BrokenExpansion;
        let fields = vec![
            Field { name: "name".to_string(), type_name: "String".to_string() },
            Field { name: "age".to_string(), type_name: "u32".to_string() },
        ];
        
        let debug_impl = expander.expand_derive_debug("Person", &fields);
        
        assert!(debug_impl.contains("impl fmt::Debug for Person"));
        assert!(debug_impl.contains("debug_struct"));
        assert!(debug_impl.contains(".field(\"name\", &self.name)"));
        assert!(debug_impl.contains(".field(\"age\", &self.age)"));
    }
}
```

## 🎯 Verification

After applying these fixes, run:
```bash
rustc ex02-proc-macro-panic.rs && ./ex02-proc-macro-panic
```

## 🎓 What You Learned

1. **Proc macro architecture** - How they work at the AST level
2. **Token parsing** - How to extract information from Rust syntax
3. **Code generation** - How to create valid Rust code programmatically
4. **Error handling** - How to handle parsing and generation errors
5. **Testing** - How to test procedural macro implementations

## 🤔 C# Comparison

This is like creating Source Generators that:
- Parse C# syntax trees with Roslyn
- Generate code based on attributes and types
- Handle compilation errors and edge cases
- Provide good developer experience

**Congratulations!** You've mastered the concepts behind Rust's procedural macro system! 🦀