# Unsafe Rust Guide: From C# Unsafe Code to Rust Raw Power

## 🎯 Overview

Unsafe Rust provides escape hatches from the language's safety guarantees, similar to C#'s `unsafe` blocks but with more responsibility and power. Unlike C#, where the garbage collector provides some safety nets, unsafe Rust requires manual management of all memory safety invariants.

## 🔄 C# vs Rust Unsafe Comparison

| Aspect | C# Unsafe Code | Rust Unsafe Code |
|--------|----------------|------------------|
| **Purpose** | Performance, interop | Performance, interop, low-level control |
| **Memory Management** | GC still active | Manual management |
| **Pointer Types** | `int*`, `void*` | `*const T`, `*mut T` |
| **Safety Net** | Garbage collector | None - programmer responsibility |
| **Verification** | Some runtime checks | Zero runtime safety |
| **Scope** | Method/block level | Expression level |
| **Common Uses** | P/Invoke, performance | FFI, collections, allocators |

## 🚨 Undefined Behavior (UB)

### What Causes UB in Rust vs C#

#### Rust UB Scenarios

```rust
unsafe {
    // ❌ Dereferencing null/dangling pointers
    let ptr: *const i32 = std::ptr::null();
    let value = *ptr;  // UB
    
    // ❌ Out-of-bounds access
    let arr = [1, 2, 3];
    let ptr = arr.as_ptr();
    let value = *ptr.add(10);  // UB
    
    // ❌ Use after free
    let layout = std::alloc::Layout::new::<i32>();
    let ptr = std::alloc::alloc(layout) as *mut i32;
    std::alloc::dealloc(ptr as *mut u8, layout);
    let value = *ptr;  // UB
    
    // ❌ Data races
    let mut data = 42;
    let ptr1 = &mut data as *mut i32;
    let ptr2 = &mut data as *mut i32;
    *ptr1 = 1;  // Writing through aliased mutable pointers
    *ptr2 = 2;  // UB if concurrent
    
    // ❌ Invalid transmutation
    let x = 1u64;
    let y: u32 = std::mem::transmute(x);  // UB - size mismatch
}
```

#### C# Unsafe Scenarios

```csharp
unsafe
{
    // ❌ Dereferencing invalid pointers (less likely to be UB)
    int* ptr = (int*)0;
    int value = *ptr;  // Access violation, but defined behavior
    
    // ❌ Buffer overruns (often caught by runtime)
    int[] arr = {1, 2, 3};
    fixed (int* p = arr)
    {
        int value = *(p + 10);  // Might be caught by CLR
    }
    
    // ❌ Stack corruption
    int localVar = 42;
    int* ptr = &localVar;
    // ptr becomes invalid when method returns
    
    // ❌ Heap corruption
    IntPtr ptr = Marshal.AllocHGlobal(sizeof(int));
    Marshal.FreeHGlobal(ptr);
    Marshal.WriteInt32(ptr, 42);  // Use after free
}
```

**Key Difference**: C# often converts UB into exceptions, while Rust UB is truly undefined.

## 🔧 Raw Pointers

### Pointer Types and Operations

```rust
// Rust pointer types
fn pointer_examples() {
    let x = 42i32;
    
    // Immutable raw pointer
    let ptr_const: *const i32 = &x;
    
    // Mutable raw pointer  
    let mut y = 24i32;
    let ptr_mut: *mut i32 = &mut y;
    
    unsafe {
        // Reading (both pointer types)
        let value1 = *ptr_const;
        let value2 = *ptr_mut;
        
        // Writing (only mutable pointers)
        *ptr_mut = 100;
        
        // Pointer arithmetic
        let ptr2 = ptr_const.add(1);        // ptr + sizeof(i32)
        let ptr3 = ptr_const.offset(2);     // Can be negative
        let ptr4 = ptr_const.wrapping_add(1); // No overflow check
        
        // Null checks
        if !ptr_const.is_null() {
            println!("Valid pointer: {}", *ptr_const);
        }
    }
}
```

**C# Equivalent**:
```csharp
unsafe void PointerExamples()
{
    int x = 42;
    
    // Pointer to local variable
    int* ptrConst = &x;
    
    int y = 24;
    int* ptrMut = &y;
    
    // Reading
    int value1 = *ptrConst;
    int value2 = *ptrMut;
    
    // Writing
    *ptrMut = 100;
    
    // Pointer arithmetic
    int* ptr2 = ptrConst + 1;
    int* ptr3 = ptrConst + 2;
    
    // Null checks
    if (ptrConst != null)
    {
        Console.WriteLine($"Valid pointer: {*ptrConst}");
    }
}
```

### Safe Abstractions Over Unsafe Code

```rust
// Rust: Building safe APIs on unsafe foundations
pub struct SafeBuffer<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> SafeBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity)
            .expect("Layout calculation overflow");
        
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut T;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr
        };
        
        Self {
            ptr,
            len: 0,
            cap: capacity,
        }
    }
    
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.len >= self.cap {
            return Err(item);
        }
        
        unsafe {
            let ptr = self.ptr.add(self.len);
            std::ptr::write(ptr, item);
        }
        
        self.len += 1;
        Ok(())
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe {
                Some(&mut *self.ptr.add(index))
            }
        } else {
            None
        }
    }
}

impl<T> Drop for SafeBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            // Drop all elements
            for i in 0..self.len {
                std::ptr::drop_in_place(self.ptr.add(i));
            }
            
            // Deallocate memory
            if self.cap > 0 {
                let layout = std::alloc::Layout::array::<T>(self.cap)
                    .expect("Layout calculation overflow");
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

// Safe interface - clients can't cause UB
let mut buffer = SafeBuffer::new(10);
buffer.push(42).unwrap();
buffer.push(24).unwrap();
println!("Item: {:?}", buffer.get(0));
```

**C# Equivalent**:
```csharp
// C# unsafe collection (simplified)
public unsafe class UnsafeBuffer<T> : IDisposable where T : unmanaged
{
    private T* ptr;
    private int length;
    private int capacity;
    private bool disposed;
    
    public UnsafeBuffer(int capacity)
    {
        this.capacity = capacity;
        ptr = (T*)Marshal.AllocHGlobal(sizeof(T) * capacity);
        if (ptr == null)
            throw new OutOfMemoryException();
    }
    
    public bool TryAdd(T item)
    {
        if (length >= capacity) return false;
        
        ptr[length] = item;
        length++;
        return true;
    }
    
    public T? Get(int index)
    {
        if (index >= length) return null;
        return ptr[index];
    }
    
    public void Dispose()
    {
        if (!disposed && ptr != null)
        {
            Marshal.FreeHGlobal((IntPtr)ptr);
            ptr = null;
            disposed = true;
        }
    }
}
```

## 🔄 Transmutation and Type Punning

### Safe Transmutation

```rust
// Rust: Transmute with size/alignment checks
fn safe_transmute_example() {
    // ✅ Safe: Same size and alignment
    let bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let number: u32 = unsafe {
        std::mem::transmute(bytes)
    };
    
    // ✅ Better: Use from_ne_bytes (no unsafe needed)
    let number_safe = u32::from_ne_bytes(bytes);
    
    // ❌ Unsafe: Different sizes
    let big_num = 0x123456789ABCDEFu64;
    let small_num: u32 = unsafe {
        std::mem::transmute(big_num)  // UB on some platforms
    };
    
    // ✅ Safe alternative: Explicit casting
    let truncated = big_num as u32;
}
```

**C# Equivalent**:
```csharp
// C# type punning with unions or unsafe
[StructLayout(LayoutKind.Explicit)]
struct IntBytes
{
    [FieldOffset(0)] public int IntValue;
    [FieldOffset(0)] public byte Byte0;
    [FieldOffset(1)] public byte Byte1;
    [FieldOffset(2)] public byte Byte2;
    [FieldOffset(3)] public byte Byte3;
}

unsafe void TransmuteExample()
{
    // Safe way in C#
    int value = 0x12345678;
    byte* ptr = (byte*)&value;
    Console.WriteLine($"Bytes: {ptr[0]:X2} {ptr[1]:X2} {ptr[2]:X2} {ptr[3]:X2}");
    
    // Using explicit layout
    IntBytes union = new IntBytes { IntValue = 0x12345678 };
    Console.WriteLine($"Byte 0: {union.Byte0:X2}");
}
```

## 🧵 Concurrency and Data Races

### Avoiding Data Races

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// ❌ Data race with unsafe code
fn data_race_example() {
    let mut data = 42;
    let ptr = &mut data as *mut i32;
    
    // This creates a data race if used concurrently
    unsafe {
        *ptr = 100;
    }
}

// ✅ Safe concurrent access
fn safe_concurrent_example() {
    let data = Arc::new(Mutex::new(42));
    let mut handles = vec![];
    
    for i in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut guard = data_clone.lock().unwrap();
            *guard += i;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final value: {}", *data.lock().unwrap());
}
```

**C# Equivalent**:
```csharp
// C# thread safety
class SafeConcurrentExample
{
    private int data = 42;
    private readonly object lockObj = new object();
    
    public void UpdateConcurrently()
    {
        var tasks = new Task[10];
        for (int i = 0; i < 10; i++)
        {
            int captured = i;
            tasks[i] = Task.Run(() =>
            {
                lock (lockObj)
                {
                    data += captured;
                }
            });
        }
        
        Task.WaitAll(tasks);
        Console.WriteLine($"Final value: {data}");
    }
}
```

## 🔗 Foreign Function Interface (FFI)

### Calling C Functions

```rust
// Rust: FFI declarations
#[link(name = "m")]  // Link with math library
extern "C" {
    fn sqrt(x: f64) -> f64;
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
}

fn ffi_example() {
    unsafe {
        // Calling C math function
        let result = sqrt(16.0);
        println!("sqrt(16) = {}", result);
        
        // Manual memory management
        let ptr = malloc(1024);
        if !ptr.is_null() {
            // Use the memory...
            free(ptr);
        }
    }
}

// Exposing Rust functions to C
#[no_mangle]
pub extern "C" fn rust_function(x: i32, y: i32) -> i32 {
    x + y
}
```

**C# Equivalent**:
```csharp
// C# P/Invoke
class FFIExample
{
    [DllImport("msvcrt.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern double sqrt(double x);
    
    [DllImport("msvcrt.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr malloc(UIntPtr size);
    
    [DllImport("msvcrt.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free(IntPtr ptr);
    
    public void FFIUsage()
    {
        // Calling C function
        double result = sqrt(16.0);
        Console.WriteLine($"sqrt(16) = {result}");
        
        // Manual memory management
        IntPtr ptr = malloc(new UIntPtr(1024));
        if (ptr != IntPtr.Zero)
        {
            // Use the memory...
            free(ptr);
        }
    }
}

// Exposing C# to native code
[UnmanagedCallersOnly(EntryPoint = "csharp_function")]
public static int CSharpFunction(int x, int y)
{
    return x + y;
}
```

## 🛡️ Safe Abstractions Design

### Invariants and Contracts

```rust
// Rust: Maintaining invariants in unsafe code
pub struct SortedVec<T> {
    inner: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    
    pub fn insert(&mut self, item: T) {
        let pos = self.inner.binary_search(&item).unwrap_or_else(|e| e);
        self.inner.insert(pos, item);
    }
    
    // Safe: Maintains sorted invariant
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    
    // Unsafe: Could break sorted invariant
    unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        // SAFETY: Caller must ensure modifications maintain sort order
        self.inner.get_unchecked_mut(index)
    }
    
    // Safe wrapper with validation
    pub fn update_with<F>(&mut self, index: usize, f: F) -> bool
    where
        F: FnOnce(&T) -> T,
    {
        if let Some(item) = self.inner.get(index) {
            let new_item = f(item);
            self.inner.remove(index);
            self.insert(new_item);
            true
        } else {
            false
        }
    }
}
```

**C# Equivalent**:
```csharp
// C# safe wrapper over potentially unsafe operations
public class SortedList<T> where T : IComparable<T>
{
    private List<T> inner = new List<T>();
    
    public void Insert(T item)
    {
        int pos = inner.BinarySearch(item);
        if (pos < 0) pos = ~pos;
        inner.Insert(pos, item);
    }
    
    public T this[int index] => inner[index];
    
    // Safe mutation that maintains invariants
    public bool UpdateItem(int index, Func<T, T> updater)
    {
        if (index >= 0 && index < inner.Count)
        {
            T oldItem = inner[index];
            T newItem = updater(oldItem);
            inner.RemoveAt(index);
            Insert(newItem);
            return true;
        }
        return false;
    }
}
```

## 💡 Best Practices

### Unsafe Code Guidelines

1. **Minimize unsafe scope** - Keep unsafe blocks as small as possible
2. **Document safety requirements** - Explain what makes the code safe
3. **Validate inputs** - Check preconditions before unsafe operations
4. **Maintain invariants** - Ensure data structure consistency
5. **Provide safe APIs** - Don't expose raw unsafe operations

### Common Patterns

#### RAII for Resource Management

```rust
// Rust: RAII wrapper for C resources
struct CFile {
    file: *mut std::ffi::c_void,
}

impl CFile {
    fn open(path: &str) -> Option<Self> {
        extern "C" {
            fn fopen(path: *const i8, mode: *const i8) -> *mut std::ffi::c_void;
        }
        
        let c_path = std::ffi::CString::new(path).ok()?;
        let c_mode = std::ffi::CString::new("r").ok()?;
        
        unsafe {
            let file = fopen(c_path.as_ptr(), c_mode.as_ptr());
            if file.is_null() {
                None
            } else {
                Some(Self { file })
            }
        }
    }
}

impl Drop for CFile {
    fn drop(&mut self) {
        extern "C" {
            fn fclose(file: *mut std::ffi::c_void) -> i32;
        }
        
        unsafe {
            fclose(self.file);
        }
    }
}
```

**C# Equivalent**:
```csharp
// C# IDisposable pattern
public class CFileWrapper : IDisposable
{
    private IntPtr file;
    private bool disposed = false;
    
    [DllImport("msvcrt.dll")]
    private static extern IntPtr fopen(string path, string mode);
    
    [DllImport("msvcrt.dll")]
    private static extern int fclose(IntPtr file);
    
    public static CFileWrapper? Open(string path)
    {
        IntPtr file = fopen(path, "r");
        return file != IntPtr.Zero ? new CFileWrapper { file = file } : null;
    }
    
    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }
    
    protected virtual void Dispose(bool disposing)
    {
        if (!disposed && file != IntPtr.Zero)
        {
            fclose(file);
            file = IntPtr.Zero;
            disposed = true;
        }
    }
    
    ~CFileWrapper()
    {
        Dispose(false);
    }
}
```

Understanding unsafe Rust enables you to write high-performance, low-level code while maintaining the safety guarantees that make Rust unique. The key is building safe abstractions that encapsulate the unsafe operations and maintain invariants.