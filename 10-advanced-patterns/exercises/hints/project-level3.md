# Project Level 3 Hints: Advanced Macro System Integration

## 🎯 Complete Working Implementation

Here's the complete fixed project structure and implementation:

## 🔧 Complete Project Structure

```
project-advanced-macros/
├── Cargo.toml
├── macros/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── src/
    ├── lib.rs
    ├── components/
    │   └── mod.rs
    ├── async_component.rs
    └── unsafe_optimizations.rs
```

## 🔧 Root Cargo.toml

```toml
[package]
name = "advanced-macros"
version = "0.1.0"
edition = "2021"
authors = ["Student <student@example.com>"]
description = "Advanced Rust patterns integration project"

[features]
default = ["async", "macros"]
async = ["tokio"]
macros = ["advanced-macros"]
unsafe-optimizations = []

[dependencies]
tokio = { version = "1.0", features = ["macros", "rt", "time"], optional = true }
advanced-macros = { path = "macros", optional = true }

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "performance"
harness = false

[[example]]
name = "integration_demo"
required-features = ["macros", "async"]
```

## 🔧 Macro Crate (macros/Cargo.toml)

```toml
[package]
name = "advanced-macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

## 🔧 Macro Implementation (macros/src/lib.rs)

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(AdvancedDebug)]
pub fn derive_advanced_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let debug_fields = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_debug = fields.named.iter().map(|f| {
                        let field_name = &f.ident;
                        let field_str = field_name.as_ref().unwrap().to_string();
                        quote! {
                            .field(#field_str, &self.#field_name)
                        }
                    });
                    quote! {
                        f.debug_struct(stringify!(#name))
                            #(#field_debug)*
                            .finish()
                    }
                }
                _ => quote! {
                    write!(f, stringify!(#name))
                }
            }
        }
        _ => quote! {
            write!(f, stringify!(#name))
        }
    };
    
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #debug_fields
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn cache_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;
    
    let cache_name = quote::format_ident!("{}_cache", fn_name);
    
    let expanded = quote! {
        thread_local! {
            static #cache_name: std::cell::RefCell<std::collections::HashMap<String, String>> = 
                std::cell::RefCell::new(std::collections::HashMap::new());
        }
        
        fn #fn_name(#fn_inputs) #fn_output {
            let cache_key = format!("{:?}", (#fn_inputs));
            
            #cache_name.with(|cache| {
                if let Some(cached) = cache.borrow().get(&cache_key) {
                    return cached.clone();
                }
                
                let result = (|| #fn_block)();
                cache.borrow_mut().insert(cache_key, result.clone());
                result
            })
        }
    };
    
    TokenStream::from(expanded)
}
```

## 🔧 Main Library (src/lib.rs)

```rust
#[cfg(feature = \"macros\")]
pub use advanced_macros::{AdvancedDebug, cache_result};

pub mod components;
#[cfg(feature = \"async\")]
pub mod async_component;
#[cfg(feature = \"unsafe-optimizations\")]
pub mod unsafe_optimizations;

pub use components::*;
#[cfg(feature = \"async\")]
pub use async_component::*;

// Re-export commonly used types
pub type BoxedRenderer = Box<dyn ComponentRenderer>;
pub type PinnedAsyncComponent = std::pin::Pin<Box<AsyncComponent>>;
```

## 🔧 Components (src/components/mod.rs)

```rust
use std::fmt::Display;

// Object-safe base trait
pub trait ComponentRenderer {
    fn name(&self) -> &str;
    fn render_simple(&self) -> String;
}

// Non-object-safe extension trait
pub trait ComponentAdvanced: ComponentRenderer {
    fn render_with_data<T: Display>(&self, data: T) -> String {
        format!(\"{}: {}\", self.name(), data)
    }
}

#[cfg(feature = \"macros\")]
use advanced_macros::AdvancedDebug;

#[cfg_attr(feature = \"macros\", derive(AdvancedDebug))]
pub struct ButtonComponent {
    pub label: String,
    pub enabled: bool,
}

impl ComponentRenderer for ButtonComponent {
    fn name(&self) -> &str {
        \"Button\"
    }
    
    fn render_simple(&self) -> String {
        format!(\"Button[{}] enabled: {}\", self.label, self.enabled)
    }
}

impl ComponentAdvanced for ButtonComponent {}

#[cfg_attr(feature = \"macros\", derive(AdvancedDebug))]
pub struct TextComponent {
    pub content: String,
    pub font_size: u32,
}

impl ComponentRenderer for TextComponent {
    fn name(&self) -> &str {
        \"Text\"
    }
    
    fn render_simple(&self) -> String {
        format!(\"Text[{}] size: {}\", self.content, self.font_size)
    }
}

impl ComponentAdvanced for TextComponent {}

// Factory function for trait objects
pub fn create_components() -> Vec<Box<dyn ComponentRenderer>> {
    vec![
        Box::new(ButtonComponent {
            label: \"Click me\".to_string(),
            enabled: true,
        }),
        Box::new(TextComponent {
            content: \"Hello, world!\".to_string(),
            font_size: 16,
        }),
    ]
}
```

## 🔧 Async Component (src/async_component.rs)

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::future::Future;
use std::task::{Context, Poll};

pub struct AsyncComponent {
    data: String,
    self_ref: *const String,
    _phantom: PhantomPinned,
}

impl AsyncComponent {
    pub fn new(data: String) -> Pin<Box<Self>> {
        let component = Box::new(AsyncComponent {
            data,
            self_ref: std::ptr::null(),
            _phantom: PhantomPinned,
        });
        
        let mut pinned = Box::into_pin(component);
        
        unsafe {
            let self_ptr = pinned.as_mut().get_unchecked_mut();
            self_ptr.self_ref = &self_ptr.data;
        }
        
        pinned
    }
    
    pub fn get_data(&self) -> &str {
        &self.data
    }
    
    pub fn get_self_ref(&self) -> &str {
        unsafe { &*self.self_ref }
    }
}

impl Future for AsyncComponent {
    type Output = String;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(format!(\"Async result: {}\", self.get_data()))
    }
}

#[cfg(feature = \"async\")]
pub async fn process_async_component(mut component: Pin<Box<AsyncComponent>>) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    component.as_mut().await
}
```

## 🔧 Unsafe Optimizations (src/unsafe_optimizations.rs)

```rust
pub struct FastBuffer<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> FastBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    pub unsafe fn push_unchecked(&mut self, item: T) {
        debug_assert!(self.data.len() < self.capacity);
        let len = self.data.len();
        let ptr = self.data.as_mut_ptr().add(len);
        std::ptr::write(ptr, item);
        self.data.set_len(len + 1);
    }
    
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        debug_assert!(index < self.data.len());
        &*self.data.as_ptr().add(index)
    }
    
    pub fn push_safe(&mut self, item: T) -> Result<(), T> {
        if self.data.len() < self.capacity {
            unsafe { self.push_unchecked(item) };
            Ok(())
        } else {
            Err(item)
        }
    }
    
    pub fn get_safe(&self, index: usize) -> Option<&T> {
        if index < self.data.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }
}

impl<T> Drop for FastBuffer<T> {
    fn drop(&mut self) {
        // Vec's drop will handle the cleanup
    }
}
```

## 🎯 Verification Commands

```bash
# Build with all features
cargo build --all-features

# Run tests
cargo test

# Run the example
cargo run --example integration_demo --features=\"macros,async\"

# Run benchmarks
cargo bench
```

## 🎓 What You Learned

1. **Procedural Macros** - Creating derive and attribute macros
2. **Trait Objects** - Object safety and dynamic dispatch
3. **Pin and Async** - Self-referential structs and async futures
4. **Unsafe Code** - Safe abstractions over unsafe operations
5. **Feature Flags** - Conditional compilation and optional dependencies
6. **Integration** - Combining advanced patterns in a real project

## 🤔 C# Comparison

This project demonstrates concepts similar to:
- **Source Generators** for compile-time code generation
- **Async/await** with state machines and pinning
- **Unsafe code** with pointer operations and manual memory management
- **Interface objects** with virtual method dispatch
- **Conditional compilation** with preprocessor directives

**Congratulations!** You've mastered advanced Rust patterns integration! 🦀