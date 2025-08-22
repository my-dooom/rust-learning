# Primitive Types
primitive types are:
- Copyable (except str and [T])
- stored on the stack
- passed by value

## Signed integers
Signed integers store both positive and negative values using two's complement representation

| Type | Size (b) | Range |
|-       |:-:|--:|
| i8     | 8 | -128 to 127
| i16    | 16 | -32,768 to 32,767
| i32    | 32 | -2,147,483,648 to 2,147,483,647
| i64    | 64 | ±9.2×10¹⁸
| i128   | 128 | Extremly large
| isize | arch-dependent | Same size as a pointer (32/64-bit)

```rust
let a: i32 = -100;
let b: isize = 42;
```
---

## Unsigned integers

| Type | Size (b) | Range |
|-       |:-:|--:|
| u8     | 8 | 0 to 255
| u16    | 16 | 0 to 65,535
| u32    | 32 | 0 to 2^32 - 1
| u64    | 64 | 0 to 18.4×10¹⁸
| u128   | 128 | Extremly large
| usize | arch-dependent | Size of pointer in memory

```rust
let x: u8 = 255;
let y: usize = 1024;
```

## Floating point numbers

Floating-point numbers follow the IEEE 754 standard.

| Type | Size (b) | Range |
|-       |:-:|--:|
| f32 | 32 | ~6 decimal digits
| f64 | 64 | ~15 decimal digits

```rust
let temp_f: f32 = 98.6;
let distance_km = 42.195 // f64 by default
```

## Boolean

The `bool` type has only two values: `true` and `false`

```rust
let is_valid: bool = true;
let is_ready = false;
```
Used in control flow and logical expressions.

## Character

The `char` type represents a Unicode Scalar Value. Unlike C/C++'s `char`, it's not 8-bit ASCII - it's a 32-bit value.

```rust
let a: char = 'A';
let b = 'π';
let c = '🚀';
```

## The unit type `()`

The unit type represents the abscence of a value. It is used:

- As the return type of functions that return nothing
- in tuple structs with no fields

```rust
fn do_nothing() {
    // implicitly returns ()
}
```

## Function Pointers

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let f: fn(i32, i32) -> i32 = add;
let result = f(2, 3);
```

## Raw pointers
Raw pointers are unsafe references used in FFI, low level memory access, or unsafe code.

```rust
let x = 5;
let ptr: *const i32 = &x;
let mut y = 10;
let mut_ptr: *mut i32 = &mut y;
```

- `*const T`: raw immutable pointer
- `*mut T`: raw mutable pointer

Raw pointer dereferencing requires `unsafe` block and does not obey borrow rules

## Slices and Strings (Special primitives)

### Slices `[T]`:
Dynamically-sized view into sequence of elements. Usually accessed via references.

```rust
let arr = [1, 2, 3, 4];

let slice: &[i32] = &arr[1..3]; // [2, 3]
```

### String slices `str`:
UTF-8 text, dynamically sized, usually used as `&str`.

```rust
let name: &str = "Rustacean"
```

These are technically DSTs (Dynamically Sized Types) and not primitive in the strictest sense, but are built-in and fundamental.
