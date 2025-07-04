# Exercise 05 - Level 3 Hints: Phantom Problems

## 🎯 Complete Solutions

Here are the fixes for each checkpoint:

## 🔧 Checkpoint 1-2: State Machine Fix

```rust
use std::marker::PhantomData;

struct StateMachine<State> {
    data: String,
    _state: PhantomData<State>,
}

impl StateMachine<Initial> {
    fn new(data: String) -> Self {
        Self {
            data,
            _state: PhantomData,
        }
    }
    
    fn start_processing(self) -> StateMachine<Processing> {
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl StateMachine<Processing> {
    fn process(&mut self) {
        self.data = format!("Processing: {}", self.data);
    }
    
    fn finish(self) -> StateMachine<Complete> {
        StateMachine {
            data: format!("Completed: {}", self.data),
            _state: PhantomData,
        }
    }
}

impl StateMachine<Complete> {
    fn get_result(&self) -> &str {
        &self.data
    }
    
    fn reset(self) -> StateMachine<Initial> {
        StateMachine {
            data: "Reset".to_string(),
            _state: PhantomData,
        }
    }
}
```

## 🔧 Checkpoint 3-4: Measurement Units Fix

```rust
struct Measurement<T, Unit> {
    value: T,
    _unit: PhantomData<Unit>,
}

impl<T> Measurement<T, Meters>
where
    T: Copy + std::ops::Mul<f64, Output = T>,
{
    fn new(value: T) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }
    
    fn to_feet(self) -> Measurement<T, Feet> {
        Measurement {
            value: self.value * 3.28084,
            _unit: PhantomData,
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
            _unit: PhantomData,
        }
    }
    
    fn to_meters(self) -> Measurement<T, Meters> {
        Measurement {
            value: self.value / 3.28084,
            _unit: PhantomData,
        }
    }
}
```

## 🔧 Checkpoint 5: Data Processor Fix

```rust
struct DataProcessor<InputType, OutputType> {
    name: String,
    _input: PhantomData<InputType>,
    _output: PhantomData<OutputType>,
}

impl<I, O> DataProcessor<I, O> {
    fn new(name: String) -> Self {
        Self {
            name,
            _input: PhantomData,
            _output: PhantomData,
        }
    }
}

impl<I, O> Processable<I, O> for DataProcessor<I, O> {
    fn process(&self, input: I) -> O {
        // Implementation would depend on actual types
        unimplemented!("Process implementation depends on concrete types")
    }
}
```

## 🔧 Checkpoint 6: Serializer Fix

```rust
struct JsonSerializer<T> {
    _phantom: PhantomData<T>,
}

struct XmlSerializer<T> {
    _phantom: PhantomData<T>,
}

impl<T> Serializer<Json> for JsonSerializer<T> {
    type Output = String;
    
    fn serialize<U>(&self, data: U) -> Self::Output
    where
        U: serde::Serialize,
    {
        // Mock JSON serialization
        format!("JSON: {:?}", std::any::type_name::<U>())
    }
}

impl<T> Serializer<Xml> for XmlSerializer<T> {
    type Output = String;
    
    fn serialize<U>(&self, data: U) -> Self::Output
    where
        U: serde::Serialize,
    {
        // Mock XML serialization
        format!("XML: {:?}", std::any::type_name::<U>())
    }
}
```

## 🔧 Checkpoint 7: API Client Fix

```rust
struct ApiClient<State> {
    base_url: String,
    _state: PhantomData<State>,
}

impl ApiClient<Unauthenticated> {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            _state: PhantomData,
        }
    }
    
    fn authenticate(self, token: &str) -> ApiClient<Authenticated> {
        println!("Authenticating with token: {}", token);
        ApiClient {
            base_url: self.base_url,
            _state: PhantomData,
        }
    }
}

impl ApiClient<Authenticated> {
    fn make_request(&self, endpoint: &str) -> Result<String, String> {
        Ok(format!("GET {}/{}", self.base_url, endpoint))
    }
    
    fn logout(self) -> ApiClient<Unauthenticated> {
        println!("Logging out");
        ApiClient {
            base_url: self.base_url,
            _state: PhantomData,
        }
    }
}
```

## 🔧 Checkpoint 8: Buffer with Lifetimes Fix

```rust
struct Buffer<'a, T> {
    data: Vec<T>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, T> Buffer<'a, T> {
    fn new(data: Vec<T>) -> Self {
        Self {
            data,
            _lifetime: PhantomData,
        }
    }
    
    fn get_slice(&self) -> &[T] {
        &self.data
    }
}

// Alternative: If you don't need the lifetime parameter
struct SimpleBuffer<T> {
    data: Vec<T>,
}

impl<T> SimpleBuffer<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data }
    }
    
    fn get_slice(&self) -> &[T] {
        &self.data
    }
}
```

## 🎮 Testing Your Solution

```rust
fn main() {
    // Test state machine
    let machine = StateMachine::new("test data".to_string());
    let processing = machine.start_processing();
    let mut processing = processing;
    processing.process();
    let complete = processing.finish();
    println!("State machine result: {}", complete.get_result());
    
    // Test measurements
    let distance_m = Measurement::<f64, Meters>::new(10.0);
    let distance_ft = distance_m.to_feet();
    println!("Distance: {} meters -> {} feet", 10.0, distance_ft.value);
    
    // Test API client
    let client = ApiClient::new("https://api.example.com".to_string());
    let authenticated = client.authenticate("secret-token");
    let response = authenticated.make_request("users");
    println!("API response: {:?}", response);
}
```

## 🔍 Key Takeaways

1. **PhantomData is for unused type parameters** - tells compiler about generic types
2. **Zero-cost abstractions** - no runtime overhead for type safety
3. **State machines with phantom types** - compile-time state verification
4. **Unit types for measurements** - prevent unit conversion errors
5. **API state tracking** - ensure correct usage patterns

## 🎯 Verification

Your code should now compile and provide strong compile-time guarantees without any runtime overhead. The phantom types should prevent incorrect usage patterns!