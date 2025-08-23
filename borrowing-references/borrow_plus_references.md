# Borrowing and References

Rust lets you access a value without taking ownership by **borrowing** it using **references**. Borrowing enables functions to read or modify data safely without transfering ownership or triggering deallocation.


## Immutable borrowing (`&T`)

An **immutable borrow** allows read-only access to a value without taking ownership

### Example: Reading a String

```rust
fn calc_len(s: &String) -> usize {
    s.len()
}

fn main() {
    let original = String::from("hehe");
    let len = calc_len(&original); // pass by reference
    println!("Length: {}", len);
    println!("Still valid: {}", original); // ✅ still usable
}
```

- `&original` creates a reference to the `String`.
- `calc_len` borrows it - no clone, no move.
- The complier ensures the reference is valid while used

## Mutable borrowing (`&mut T`)

A **mutable borrow** allows the function to modify the original value.

### Example: modifying a String

```rust
fn append_word(s: &mut String) {
    s.push_str(" world");


fn main() {
    let mut original = String::from("hehe");
    append_world(&mut original);
    println!("Modified: {}", original); // "hehe world"
}
```

- `original` must be delcared `mut` to allow mutable borrow
- only one mutable reference is allowed at a time.

## The rules of Borrowing

Rust enforces this rules at complie time to ensure **memory safety**
1. You can have **any number** of **immutable references** (`&T`), or
2. You can have exactly one mutable reference (`&mut T`),
3. You cannot have a mutable reference while immutable references are active,
4. References **must always be valie** i.e., the borrowed value cannot go out of scope.






