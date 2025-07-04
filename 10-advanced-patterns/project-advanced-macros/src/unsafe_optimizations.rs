// ❌ BROKEN: Missing required imports for unsafe operations
// use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

// ❌ BROKEN: This struct has memory safety issues
pub struct FastBuffer<T> {
    // ❌ BROKEN: Raw pointer without proper lifetime management
    data: *mut T,
    len: usize,
    capacity: usize,
    // ❌ BROKEN: No phantom data for proper variance
}

impl<T> FastBuffer<T> {
    // ❌ BROKEN: Constructor with unsafe allocation issues
    pub fn new(capacity: usize) -> Self {
        unsafe {
            // ❌ BROKEN: Using wrong allocation method
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            let data = std::alloc::alloc(layout) as *mut T;
            
            // ❌ BROKEN: No null pointer check
            Self {
                data,
                len: 0,
                capacity,
            }
        }
    }
    
    // ❌ BROKEN: Push without bounds checking
    pub unsafe fn push_unchecked(&mut self, item: T) {
        // ❌ BROKEN: No debug assertion or safety check
        let ptr = self.data.add(self.len);
        // ❌ BROKEN: Writing to potentially unallocated memory
        ptr::write(ptr, item);
        self.len += 1;
    }
    
    // ❌ BROKEN: Get without bounds checking or validation
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        // ❌ BROKEN: No bounds checking even in debug
        &*self.data.add(index)
    }
    
    // ❌ BROKEN: Safe wrapper that's not actually safe
    pub fn push_safe(&mut self, item: T) -> Result<(), T> {
        if self.len < self.capacity {
            // ❌ BROKEN: Calling unsafe function without proper validation
            unsafe { self.push_unchecked(item) };
            Ok(())
        } else {
            Err(item)
        }
    }
    
    // ❌ BROKEN: Safe wrapper with potential UB
    pub fn get_safe(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // ❌ BROKEN: Could still be unsafe if data pointer is invalid
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }
    
    // ❌ BROKEN: Iterator that yields invalid references
    pub fn iter(&self) -> FastBufferIter<T> {
        FastBufferIter {
            buffer: self,
            index: 0,
        }
    }
}

// ❌ BROKEN: Drop implementation that might double-free or leak
impl<T> Drop for FastBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            // ❌ BROKEN: No proper cleanup of individual elements
            // ❌ BROKEN: Direct deallocation without proper element dropping
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            std::alloc::dealloc(self.data as *mut u8, layout);
        }
    }
}

// ❌ BROKEN: Iterator with lifetime and safety issues
pub struct FastBufferIter<'a, T> {
    buffer: &'a FastBuffer<T>,
    index: usize,
}

impl<'a, T> Iterator for FastBufferIter<'a, T> {
    type Item = &'a T;
    
    // ❌ BROKEN: Iterator that can yield invalid references
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.buffer.len {
            // ❌ BROKEN: Unsafe access without proper validation
            let item = unsafe { self.buffer.get_unchecked(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// ❌ BROKEN: Unsafe string operations with memory issues
pub struct UnsafeString {
    // ❌ BROKEN: Raw pointer to potentially invalid memory
    data: *mut u8,
    len: usize,
    capacity: usize,
}

impl UnsafeString {
    // ❌ BROKEN: Constructor that doesn't properly initialize
    pub fn new() -> Self {
        Self {
            // ❌ BROKEN: Null pointer without proper handling
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    // ❌ BROKEN: Push method with buffer overflow potential
    pub unsafe fn push_char_unchecked(&mut self, ch: char) {
        let mut buffer = [0; 4];
        let char_bytes = ch.encode_utf8(&mut buffer).as_bytes();
        
        // ❌ BROKEN: No capacity checking
        let start_ptr = self.data.add(self.len);
        
        // ❌ BROKEN: Potential buffer overflow
        ptr::copy_nonoverlapping(char_bytes.as_ptr(), start_ptr, char_bytes.len());
        self.len += char_bytes.len();
    }
    
    // ❌ BROKEN: String conversion without validation
    pub unsafe fn as_str(&self) -> &str {
        // ❌ BROKEN: Creating string slice from potentially invalid UTF-8
        let slice = std::slice::from_raw_parts(self.data, self.len);
        std::str::from_utf8_unchecked(slice)
    }
}

// ❌ BROKEN: Drop implementation with memory safety issues
impl Drop for UnsafeString {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                // ❌ BROKEN: Deallocating without proper layout information
                std::alloc::dealloc(
                    self.data,
                    std::alloc::Layout::from_size_align_unchecked(self.capacity, 1),
                );
            }
        }
    }
}

// ❌ BROKEN: Transmute operations that violate safety
pub unsafe fn transmute_slice<T, U>(slice: &[T]) -> &[U] {
    // ❌ BROKEN: Transmuting between potentially incompatible types
    std::mem::transmute(slice)
}

// ❌ BROKEN: Pointer arithmetic without bounds checking
pub unsafe fn advance_ptr_unchecked<T>(ptr: *mut T, offset: isize) -> *mut T {
    // ❌ BROKEN: No validation of pointer validity or bounds
    ptr.offset(offset)
}

// ❌ BROKEN: Memory copying without overlap checking
pub unsafe fn fast_copy<T>(src: *const T, dst: *mut T, count: usize) {
    // ❌ BROKEN: Using copy_nonoverlapping without overlap validation
    ptr::copy_nonoverlapping(src, dst, count);
}

// ❌ BROKEN: Uninitialized memory creation
pub unsafe fn create_uninitialized_vec<T>(size: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(size);
    // ❌ BROKEN: Setting length without initializing elements
    vec.set_len(size);
    vec
}

// ❌ BROKEN: Aliasing violation creator
pub unsafe fn create_aliased_mut_refs<T>(data: &mut [T]) -> (&mut T, &mut T) {
    if data.len() >= 2 {
        let ptr = data.as_mut_ptr();
        // ❌ BROKEN: Creating aliased mutable references
        let ref1 = &mut *ptr;
        let ref2 = &mut *ptr.add(1);
        // ❌ BROKEN: But what if we return the same element twice?
        (ref1, ref2)
    } else {
        // ❌ BROKEN: Returning references to the same element
        let ptr = data.as_mut_ptr();
        (&mut *ptr, &mut *ptr)
    }
}

// ❌ BROKEN: Use-after-free scenario
pub unsafe fn use_after_free_example() -> String {
    let layout = std::alloc::Layout::new::<String>();
    let ptr = std::alloc::alloc(layout) as *mut String;
    
    // Initialize the string
    ptr::write(ptr, "Hello".to_string());
    
    // Use the string
    let value = ptr::read(ptr);
    let result = value.clone();
    
    // Deallocate
    std::alloc::dealloc(ptr as *mut u8, layout);
    
    // ❌ BROKEN: Use after free
    let invalid_value = ptr::read(ptr);
    format!("{} {}", result, invalid_value)
}

// ❌ BROKEN: Test module with unsafe issues
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fast_buffer() {
        let mut buffer = FastBuffer::new(10);
        
        // ❌ BROKEN: These operations might cause UB
        buffer.push_safe(42).unwrap();
        buffer.push_safe(24).unwrap();
        
        assert_eq!(*buffer.get_safe(0).unwrap(), 42);
        assert_eq!(*buffer.get_safe(1).unwrap(), 24);
    }
    
    #[test]
    fn test_unsafe_string() {
        let mut s = UnsafeString::new();
        
        unsafe {
            // ❌ BROKEN: This will likely cause UB due to null pointer
            s.push_char_unchecked('H');
            s.push_char_unchecked('i');
            
            let str_view = s.as_str();
            assert_eq!(str_view, "Hi");
        }
    }
    
    #[test]
    fn test_transmute_operations() {
        let int_slice = &[1u32, 2u32, 3u32];
        
        unsafe {
            // ❌ BROKEN: This transmute might be invalid
            let byte_slice: &[u8] = transmute_slice(int_slice);
            assert_eq!(byte_slice.len(), 12); // Might not be true on all platforms
        }
    }
}