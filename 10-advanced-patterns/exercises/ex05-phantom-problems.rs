// Exercise 05: Phantom Problems - Phantom Types and Zero-Cost Abstractions
//
// Learning Objectives:
// - Understand phantom types and their use cases
// - Debug zero-cost abstraction issues
// - Learn compile-time state tracking
// - Compare with C# generic type parameters and state machines
//
// C# Analogy: Like generic type parameters that exist only at compile time
// to enforce type safety, similar to state machines with compile-time verification.
//
// Your Mission: Fix the broken phantom type implementations to enable
// compile-time state tracking and zero-cost abstractions.

use std::marker::PhantomData;

// ❌ CHECKPOINT 1: Basic Phantom Type Syntax
// This should use phantom types for compile-time state tracking
// C# equivalent: Generic type parameter for compile-time type safety
struct StateMachine<State> {
    data: String,
    // ❌ Missing phantom data field
    _state: State,
}

// State types (these are zero-sized)
struct Initial;
struct Processing;
struct Complete;

// ❌ CHECKPOINT 2: Phantom Type State Transitions
// These methods should enforce state transitions at compile time
// C# equivalent: State machine with compile-time state verification
impl StateMachine<Initial> {
    fn new(data: String) -> Self {
        Self {
            data,
            // ❌ Wrong phantom data initialization
            _state: Initial,
        }
    }
    
    // ❌ This should return StateMachine<Processing>
    fn start_processing(self) -> StateMachine<Initial> {
        StateMachine {
            data: self.data,
            _state: Processing,
        }
    }
}

impl StateMachine<Processing> {
    fn process(&mut self) {
        self.data = format!("Processing: {}", self.data);
    }
    
    // ❌ This should return StateMachine<Complete>
    fn finish(self) -> StateMachine<Processing> {
        StateMachine {
            data: format!("Completed: {}", self.data),
            _state: Complete,
        }
    }
}

impl StateMachine<Complete> {
    fn get_result(&self) -> &str {
        &self.data
    }
    
    // ❌ This should reset to Initial state
    fn reset(self) -> StateMachine<Complete> {
        StateMachine {
            data: "Reset".to_string(),
            _state: Initial,
        }
    }
}

// ❌ CHECKPOINT 3: Phantom Types with Generic Parameters
// This should use phantom types for units of measurement
// C# equivalent: Generic wrapper types for type safety
struct Measurement<T, Unit> {
    value: T,
    // ❌ Missing phantom data for unit
    _unit: Unit,
}

// Unit types
struct Meters;
struct Feet;
struct Seconds;

// ❌ CHECKPOINT 4: Phantom Type Conversions
// These conversions should be compile-time safe
// C# equivalent: Implicit/explicit conversion operators
impl<T> Measurement<T, Meters>
where
    T: Copy + std::ops::Mul<f64, Output = T>,
{
    fn new(value: T) -> Self {
        Self {
            value,
            // ❌ Wrong phantom data usage
            _unit: Meters,
        }
    }
    
    // ❌ This conversion logic is wrong
    fn to_feet(self) -> Measurement<T, Feet> {
        Measurement {
            value: self.value * 3.28084,
            _unit: Feet,
        }
    }
}

impl<T> Measurement<T, Feet>
where
    T: Copy + std::ops::Mul<f64, Output = T>,
{
    fn new(value: T) -> Self {
        Self {
            value,
            // ❌ Wrong phantom data usage
            _unit: Feet,
        }
    }
    
    // ❌ This conversion logic is wrong
    fn to_meters(self) -> Measurement<T, Meters> {
        Measurement {
            value: self.value / 3.28084,
            _unit: Meters,
        }
    }
}

// ❌ CHECKPOINT 5: Phantom Types with Trait Bounds
// This should use phantom types with trait constraints
// C# equivalent: Generic constraints with phantom type parameters
trait Processable<Input, Output> {
    fn process(&self, input: Input) -> Output;
}

struct DataProcessor<InputType, OutputType> {
    name: String,
    // ❌ Missing phantom data for type parameters
    _input: InputType,
    _output: OutputType,
}

impl<I, O> DataProcessor<I, O> {
    fn new(name: String) -> Self {
        Self {
            name,
            // ❌ Wrong phantom data initialization
            _input: std::marker::PhantomData,
            _output: std::marker::PhantomData,
        }
    }
}

// ❌ CHECKPOINT 6: Phantom Types with Associated Types
// This should use phantom types with associated types
// C# equivalent: Generic interface with phantom type parameters
trait Serializer<Format> {
    type Output;
    
    fn serialize<T>(&self, data: T) -> Self::Output
    where
        T: serde::Serialize;
}

struct JsonSerializer<T> {
    // ❌ Missing phantom data
    _phantom: T,
}

struct XmlSerializer<T> {
    // ❌ Missing phantom data
    _phantom: T,
}

// JSON format marker
struct Json;
// XML format marker  
struct Xml;

// ❌ CHECKPOINT 7: Phantom Types for API Safety
// This should use phantom types for API state tracking
// C# equivalent: Builder pattern with compile-time state verification
struct ApiClient<State> {
    base_url: String,
    // ❌ Missing phantom data
    _state: State,
}

struct Unauthenticated;
struct Authenticated;

impl ApiClient<Unauthenticated> {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            // ❌ Wrong phantom data
            _state: Unauthenticated,
        }
    }
    
    // ❌ This should return ApiClient<Authenticated>
    fn authenticate(self, token: &str) -> Self {
        println!("Authenticating with token: {}", token);
        ApiClient {
            base_url: self.base_url,
            _state: Authenticated,
        }
    }
}

impl ApiClient<Authenticated> {
    fn make_request(&self, endpoint: &str) -> Result<String, String> {
        Ok(format!("GET {}/{}", self.base_url, endpoint))
    }
    
    // ❌ This should return ApiClient<Unauthenticated>
    fn logout(self) -> Self {
        println!("Logging out");
        ApiClient {
            base_url: self.base_url,
            _state: Unauthenticated,
        }
    }
}

// ❌ CHECKPOINT 8: Phantom Types with Lifetimes
// This should use phantom types with lifetime parameters
// C# equivalent: Generic type with lifetime-like constraints
struct Buffer<'a, T> {
    data: Vec<T>,
    // ❌ Missing phantom data for lifetime
    _lifetime: &'a (),
}

impl<'a, T> Buffer<'a, T> {
    // ❌ This lifetime binding is wrong
    fn new(data: Vec<T>) -> Self {
        Self {
            data,
            _lifetime: &(),
        }
    }
    
    // ❌ This should properly handle lifetime constraints
    fn get_slice(&self) -> &[T] {
        &self.data
    }
}

// Helper traits for testing
trait DisplayValue {
    fn display(&self) -> String;
}

impl<T> DisplayValue for Measurement<T, Meters>
where
    T: std::fmt::Display,
{
    fn display(&self) -> String {
        format!("{} meters", self.value)
    }
}

impl<T> DisplayValue for Measurement<T, Feet>
where
    T: std::fmt::Display,
{
    fn display(&self) -> String {
        format!("{} feet", self.value)
    }
}

// Mock serde for serialization example
mod serde {
    pub trait Serialize {}
    impl<T> Serialize for T {}
}

fn main() {
    println!("=== Phantom Problems Exercise ===");
    
    // Test Checkpoint 1 & 2: State machine
    let machine = StateMachine::new("test data".to_string());
    let processing = machine.start_processing();
    let mut processing = processing;
    processing.process();
    let complete = processing.finish();
    println!("State machine result: {}", complete.get_result());
    
    // Test Checkpoint 3 & 4: Measurements
    let distance_m = Measurement::<f64, Meters>::new(10.0);
    let distance_ft = distance_m.to_feet();
    println!("Distance: {} -> {}", 
             distance_m.display(), 
             distance_ft.display());
    
    // Test Checkpoint 5: Data processor
    let processor = DataProcessor::<String, String>::new("test".to_string());
    println!("Created processor: {}", processor.name);
    
    // Test Checkpoint 6: Serializers
    let json_serializer = JsonSerializer::<Json> {
        _phantom: PhantomData,
    };
    let xml_serializer = XmlSerializer::<Xml> {
        _phantom: PhantomData,
    };
    
    // Test Checkpoint 7: API client
    let client = ApiClient::new("https://api.example.com".to_string());
    let authenticated = client.authenticate("secret-token");
    let response = authenticated.make_request("users");
    println!("API response: {:?}", response);
    
    // Test Checkpoint 8: Buffer with lifetimes
    let data = vec![1, 2, 3, 4, 5];
    let buffer = Buffer::new(data);
    let slice = buffer.get_slice();
    println!("Buffer slice: {:?}", slice);
    
    println!("🎉 Phantom type concepts demonstrated!");
}

// C# Comparison Notes:
//
// 1. Phantom types are like generic type parameters that exist only at compile time
// 2. State machines with phantom types are like compile-time state verification
// 3. Unit types for measurements are like strongly-typed wrappers
// 4. Phantom data is like generic type constraints without runtime overhead
// 5. API state tracking is like builder pattern with compile-time verification
// 6. Lifetime phantom types are like generic lifetime constraints
// 7. Zero-cost abstractions provide safety without runtime overhead
// 8. More compile-time guarantees than C# generic constraints

// Key Differences from C#:
// - True zero-cost abstractions (no runtime overhead)
// - More precise compile-time state tracking
// - Better memory layout control
// - More complex syntax but stronger guarantees
// - No reflection or runtime type information needed