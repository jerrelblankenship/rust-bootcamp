use std::fmt::Display;

// ❌ BROKEN: This trait violates object safety rules
trait ComponentRenderer {
    fn name(&self) -> &str;
    
    // ❌ BROKEN: Generic methods break object safety
    fn render<T: Display>(&self, data: T) -> String;
    
    // ❌ BROKEN: Associated types break object safety for trait objects
    type RenderOutput;
    fn render_typed(&self) -> Self::RenderOutput;
    
    // ❌ BROKEN: Self return types break object safety
    fn clone_component(&self) -> Self;
}

// ❌ BROKEN: This trait also has object safety issues
trait ComponentAdvanced {
    // ❌ BROKEN: Static methods break object safety
    fn create_default() -> Self where Self: Sized;
    
    // ❌ BROKEN: Generic associated types
    type Config<T>;
    fn configure<T>(&mut self, config: Self::Config<T>);
}

// ❌ BROKEN: Missing feature gate for macro usage
// #[cfg(feature = "macros")]
// use advanced_macros::AdvancedDebug;

// ❌ BROKEN: Derive macro will fail due to missing dependencies
#[derive(AdvancedDebug)]
pub struct ButtonComponent {
    pub label: String,
    pub enabled: bool,
    // ❌ BROKEN: This self-reference creates issues
    pub self_ref: Option<*const ButtonComponent>,
}

// ❌ BROKEN: Implementation tries to implement non-object-safe trait
impl ComponentRenderer for ButtonComponent {
    fn name(&self) -> &str {
        "Button"
    }
    
    // ❌ BROKEN: This generic method can't be used in trait objects
    fn render<T: Display>(&self, data: T) -> String {
        format!("Button[{}] data: {} enabled: {}", self.label, data, self.enabled)
    }
    
    // ❌ BROKEN: Associated type makes trait non-object-safe
    type RenderOutput = String;
    fn render_typed(&self) -> Self::RenderOutput {
        format!("Button[{}] enabled: {}", self.label, self.enabled)
    }
    
    // ❌ BROKEN: Can't clone component with self-reference
    fn clone_component(&self) -> Self {
        ButtonComponent {
            label: self.label.clone(),
            enabled: self.enabled,
            self_ref: None, // Can't clone self-reference safely
        }
    }
}

impl ComponentAdvanced for ButtonComponent {
    fn create_default() -> Self {
        ButtonComponent {
            label: "Default".to_string(),
            enabled: true,
            self_ref: None,
        }
    }
    
    // ❌ BROKEN: Generic associated type not properly implemented
    type Config<T> = T;
    fn configure<T>(&mut self, config: Self::Config<T>) {
        // ❌ BROKEN: No actual configuration logic
    }
}

// ❌ BROKEN: Another component with similar issues
#[derive(AdvancedDebug)]
pub struct TextComponent {
    pub content: String,
    pub font_size: u32,
    // ❌ BROKEN: Lifetime issues with string references
    pub style_ref: Option<&'static str>,
}

impl ComponentRenderer for TextComponent {
    fn name(&self) -> &str {
        "Text"
    }
    
    fn render<T: Display>(&self, data: T) -> String {
        format!("Text[{}] data: {} size: {}", self.content, data, self.font_size)
    }
    
    type RenderOutput = String;
    fn render_typed(&self) -> Self::RenderOutput {
        format!("Text[{}] size: {}", self.content, self.font_size)
    }
    
    fn clone_component(&self) -> Self {
        TextComponent {
            content: self.content.clone(),
            font_size: self.font_size,
            style_ref: self.style_ref,
        }
    }
}

impl ComponentAdvanced for TextComponent {
    fn create_default() -> Self {
        TextComponent {
            content: "Default Text".to_string(),
            font_size: 12,
            style_ref: None,
        }
    }
    
    type Config<T> = T;
    fn configure<T>(&mut self, _config: Self::Config<T>) {
        // ❌ BROKEN: No implementation
    }
}

// ❌ BROKEN: This function tries to use trait objects with non-object-safe traits
pub fn create_components() -> Vec<Box<dyn ComponentRenderer>> {
    vec![
        Box::new(ButtonComponent {
            label: "Click me".to_string(),
            enabled: true,
            self_ref: None,
        }),
        Box::new(TextComponent {
            content: "Hello, world!".to_string(),
            font_size: 16,
            style_ref: Some("default"),
        }),
    ]
}

// ❌ BROKEN: This function uses generic methods that can't be called on trait objects
pub fn render_all_components(components: Vec<Box<dyn ComponentRenderer>>) -> Vec<String> {
    components
        .iter()
        .map(|comp| {
            // ❌ BROKEN: Can't call generic method on trait object
            comp.render("test data")
        })
        .collect()
}

// ❌ BROKEN: This downcasting attempt is incorrect
use std::any::Any;

pub fn downcast_component(component: &dyn ComponentRenderer) -> Option<&ButtonComponent> {
    // ❌ BROKEN: ComponentRenderer doesn't implement Any
    // ❌ BROKEN: Can't downcast from non-Any trait object
    component.as_any().downcast_ref::<ButtonComponent>()
}