# Exercise 01 - Level 1 Hints 🟢

## Getting Started with Systems Programming

You're encountering low-level memory and unsafe code concepts. This is where Rust shows its systems programming power!

## Key Questions to Consider

1. **What's a struct syntax error?** Look at the field definitions carefully - what punctuation might be missing?

2. **Memory layout matters** - How does Rust organize data in memory compared to C#?

3. **In C#, what happens here?**
   ```csharp
   public struct SimpleStruct {
       public byte Field1;
       public string Field2;
       public int Field3;
   }
   ```
   C# handles memory layout automatically. Rust gives you control!

## Concepts to Review

- **Struct syntax**: Field separation and trailing commas
- **Memory representation**: How Rust lays out data
- **Stack vs heap**: Where different types live
- **Alignment**: How data is positioned in memory

## Step-by-Step Approach

1. **Fix one error at a time** - Don't try to fix everything at once
2. **Compile after each fix** - `rustc ex01-memory-layout.rs`
3. **Read error messages** - Rust's compiler is very helpful
4. **Uncomment next step** - Only after current step compiles

## Next Steps

1. Look at the struct definition in `step_1_struct_syntax()`
2. Check the punctuation between fields
3. What comes after each field declaration?

Need more specific guidance? Check Level 2 hints after trying for 15+ more minutes!