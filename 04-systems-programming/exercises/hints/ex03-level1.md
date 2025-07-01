# Exercise 03 - Level 1 Hints 🟢

## Understanding Manual Memory Management

You're working with manual allocation and deallocation - the foundation of systems programming. This is like using Marshal.AllocHGlobal in C#, but with Rust's safety system.

## Key Questions to Consider

1. **Why is allocation unsafe?** Manual memory management bypasses Rust's ownership system
2. **What are the safety requirements?** Every allocation must be properly matched with deallocation
3. **How is this like C# unmanaged memory?**
   ```csharp
   IntPtr ptr = Marshal.AllocHGlobal(sizeof(int));
   Marshal.WriteInt32(ptr, 42);
   Marshal.FreeHGlobal(ptr);  // Must remember to free!
   ```

## Concepts to Review

- **Layout**: Describes size and alignment requirements
- **Raw pointers**: `*mut T` and `*const T` for manual memory access
- **Memory safety**: Preventing leaks, double-free, and use-after-free
- **RAII**: Resource Acquisition Is Initialization pattern

## Common Memory Issues

### Memory Leaks
- Allocating memory but forgetting to free it
- Functions returning early without cleanup

### Double Free
- Calling `dealloc` twice on the same pointer
- Using wrong layout for deallocation

### Use After Free
- Accessing memory after it's been deallocated
- Keeping pointers to freed memory

## Safety Strategy

1. **Match allocations**: Every `alloc` needs a corresponding `dealloc`
2. **Use correct layout**: Same layout for allocation and deallocation
3. **Check null pointers**: Handle allocation failures gracefully
4. **RAII patterns**: Use Drop trait for automatic cleanup

## C# vs Rust Manual Memory

| Aspect | C# Marshal | Rust alloc |
|--------|------------|------------|
| **Safety** | Runtime checks | Compile-time + manual |
| **Errors** | Exceptions | Undefined behavior |
| **Cleanup** | Manual or finalizer | Manual or Drop |
| **Performance** | Some overhead | Zero overhead |

Need more specific guidance? Check Level 2 hints after trying for 15+ more minutes!