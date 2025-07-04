# Exercise 02 - Level 2 Hints: Procedural Macro Panic

## 🎯 Specific Issues to Fix

The main issues are in the simulated proc macro implementations:

## 🔧 Checkpoint 1: Missing Dependencies

**Problem**: A real proc macro crate would need:
```toml
[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"

[lib]
proc-macro = true
```

**Solution**: This exercise simulates the behavior, so focus on the implementation logic.

## 🔧 Checkpoint 2: Token Parsing Issues

**Problem**: The `parse_struct_fields` method is too simplistic:
```rust
let parts: Vec<&str> = line.split(':').collect();
if parts.len() >= 2 {
    let name = parts[0].trim().replace("pub ", "");
    let type_name = parts[1].trim().trim_end_matches(',');
    // This parsing is too naive
}
```

**Solution**: Improve the parsing logic:
```rust
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
```

## 🔧 Checkpoint 3: Code Generation Issues

**Problem**: The `expand_derive_debug` method generates invalid Debug implementation.

**Solution**: Fix the format string construction:
```rust
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
```

## 🔧 Checkpoint 4: SQL Macro Issues

**Problem**: The `INSERT INTO` pattern is missing parentheses:
```rust
(INSERT INTO $table:ident ($($field:ident),+) VALUES ($($value:expr),+)) => {
    // This pattern is correct
}
```

**Solution**: The pattern syntax is actually correct, but ensure the macro call matches it.

## ⏰ Time Check

Still struggling after 30 minutes? Move to Level 3 for complete solutions.

**Hint**: Focus on improving the parsing logic and code generation - these are the core issues!