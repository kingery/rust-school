# RUST

## Data Types
- arrays have fixed length
- arrays are allocated on the stack
- vectors have variable length and are allocated on the heap

- "Statements" - code that does not return a value
  - e.g. `let x = 123`
- "Expressions" - code that does return a value
  - 1 + 2
  - calling a function
  - literals
  - scoped blocks of code

- a literal w/o a semicolon is an expression, a literal w/ a semicolon is a statement
  - `5` is an expression
  - `5;` is a statement

## Functions
- return values must have types
- the return value is the last expression in the function

## Ownership
- Stack:
  - known sizes only
  - faster than Heap allocation: compiler knows where to put the next thing
  - all local variables, pointers to inputs, etc, stored on stack
- Heap:
  - request a chunk of mem
  - allocator returns pointer to memory address
  - pointed to address is stored on the stack
  - slower access due to following pointer

Onwership is all about managing heap data.

Ownership has three rules:
- Each value has a variable called its "owner"
- A value can only have one owner at a time (it may change over time)
- Values are dropped when the owner goes out of scope

When copying a heap-allocated value, we copy the pointer value on the stack and mark the old variable as invalid
```
    let s1 = String::from("foo");
    let s2 = s1;
```
here `s1` "moves" to `s2`. Compare with cloning, which copies the value
```
    let s1 = String::from("bar");
    let s2 = s1.clone();
```

Simpler types that are put on the stack implement the "Copy trait", which allows for quick copying on the stack and thus avoids the invalidation.
```
    let s1 = 11;
    let s2 = s1;
```
Anything that goes on the Heap must implement the "Drop" trait which handles all the de-allocation when a variable goes out of scope. The compiler will prevent the implementation of both "Drop" and "Copy".

Passing variables into functions behaves the same way as declaring new variables from existing ones: either they are copied or moved depending on which trait (Copy or Drop) has been implemented for the type.

Functions can also transer ownership when passing return values. If we want to pass data to a function and not worry about having to return it, we can use "references"

### References
- like a pointer, but guaranteed to point to a valid part of memory
- can be made mutable: `s: &mut String`, `let mut foo = String::from("foo")`
  - only one mutable reference to a piece of data at a single time
  - cannot have a mutable and immutable reference to the same data in the same scope
    - variable scope defined by {} braces, or ends after the last usage
    - Compiler can identify scope via "Non-Lexical Lifetimes" (NLL)

### Slices
- slices are references
- like the python syntax
```
    let s = String::from("some string");
    &s[2..4]
```
- can omit first index to start at beginning
- can omit last index to go to end
- slices are immutable, so they can provide some safety when referencing mutable strings/arrays/etc
- slices have fixed size in memory: pointer to first element and length
- slices have distinct type
  - &str - string slice
  - &[i32] - array slice of type i32

## Structs
- `struct` keyword
- access/update with dot notation
- entire struct is mutable/not mutable--not mutability on field-by-field basis
- tuple structs: name a tuple and assign types for clarity
```
struct Point(u32, u32, u32);
```
