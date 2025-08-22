# Ownership

Ownership is Rust's core system for managing memory **without a garbage collector**. It ensures memory safety at compile time through strict rules enforced by the complier.

Rust enforces a **single-owner model**, meaning each piece of data in memory has one and only one owner at any time.

## The Three rules of Ownership
1. Each value in Rust has a **single owner**.
2. When the owner **goes out of scope**, the value is dropped (freed).
3. A value can be moved to a new owner, but **only one owner can exist at a time**.

## Scope and ownership

When a variable goes out of scope, rust automatically calls drop() to free its resources.

```rust
{
    let s = String::from("hehe");
    // s is valid here
}
// s is not out ouf scope and deallocated
```

## Ownership Move

Ownership is transferred (moved) when you assign a variable to another:

```rust
let s1 = String::from("hehe");
let s2 = s1;
// s1 is now invalid; s2 owns the string
```

After the move, using `s1` is a compile-time error

## Cloning to keep both

If you neewd two separate copies of the data, use `.clone()`:

```rust
let s1 = String::from("hehe");
let s2 = s1.clone(); //deepcopy

println!("{}", s1);  // ✅ valid
```

## Ownership with stack values

Simple values like integers implement the `copy` trait:

```rust
let a = 5;
let b = a; // a is still valid
```

Rust performs a bitwise copy for `Copy` types. No ownership transfer occurs.


## Movement Through a Function

Passing ownership to a function works just like a variable assignment - ownership is moved unless the type implements `Copy`.

```rust
fn take_ownership(s: String) {
    println!("Inside function: {}", s);

} // s is dropped here


fn main() {
    let s = String::from("hehe");
    take_ownership(s);
    // println!("{}", s); // ❌ Error: s was moved
}
```

To keep using the value after the function, you must return it:

```rust
fn take_and_return(s: String) -> String {
    println!("Processing: {}", s);
    s
}

fn main() {
    let s1 = String::from("hehe");
    let s2 = take_and_return(s1); // s1 is moved, s2 now owns it
    println!("After function: {}", s2); // ✅
}
```

Or you can **borrow it**!

# Summary:

| Action                      | Result                  |
|-----------------------------|-------------------------|
| Assign `String` to new var    | Moves ownership         |
| Pass `String` to a function   | Moves ownership         |
| Use after move              | Compile-time error      |
| Clone a value               | Deep copy on heap       |
| Copy types (e.g., `i32`)      | Implicit bitwise copy   |