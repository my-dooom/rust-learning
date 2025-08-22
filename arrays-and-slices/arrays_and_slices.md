# Arrays
An **array** is a fixed-size, stack-allocated collection of elements of the same type

```rust
let a: [i32; 4] = [10, 20, 30, 40];
```

- `[T, N]` is the type of an array with `N` elements of type `T`
- The length is known at compile time.
- Arrays are stored entirely on the stack


### Accessing elements
```rust
let first = a[0];
let last = a[3];
```

Indexing beyond the array length will cause a **panic at runtime.**

### Default initialization 

```rust
let zeros = [0; 8]; // [0, 0, 0, 0, 0, 0, 0, 0]
```

# Slices

A **slice** is a dynamically-sized view into a contiguous sequence. Slices do not own the data they reference—they are borrowed views into arrays or vectors.

```rust
let a = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..4]; // [2, 3, 4]
```

- `&[T]` is the type of an immutable slice.
- `&mut [T]` is a mutable slice.
- The range `1..4` includes indices 1, 2 and 3 (exclusive of 4)

### Slicing syntax

```rust
let full = &a[..];          //entire array
let first_half = &a[..3];   // a[0], a[1], a[2]
let last_half = &a[2..];    // a[2], a[3], a[4]
```

### Mutability

**if the array itself is mutable**, you can create a mutable slice

```rust
let mut data = [1, 2, 3];

let slice = &mut data[0..2];

slice[0] = 10;
```

### Length and iteration

You can access the length of both arrays and slices using `.len()`

```rust
println!("Length =  {}", a.len())
println!("Length = {}", slice.len())
```

You can iterate using a `for` loop:

```rust
for element in a.iter() {
    println!("{}", element);
}
```

# Arrays vs Slices

| Feature    | Array                   | Slice                       |
|------------|-------------------------|-----------------------------|
| Size       | Fixed at compile time   | Dynamic at runtime          |
| Memory     | Stack-allocated         | View into other data        |
| Ownership  | Owns data               | Borrowed reference          |
| Syntax     | `[T; N]`                | `&[T]` or `&mut`            |