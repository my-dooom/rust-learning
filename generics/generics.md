# Generics

Generics allow you to write code that works with **multiple types**, without duplicating logic. Instead of rewriting separate versions of a function for each type, you write one version using a **type parameter.**

### Repetition without generics

```rust
fn largest_i32(list &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn largest_char(list &[char]) -> char {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}
```

Both functions do the same thing but for different types.
This duplication can be removed using generics.
---

### Generic version:

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}
```

- `T` is a generic type parameter
- `T: PartialOrd` means `T` must support `>`
- `T: Copy` means the value can be copied (needed for `max = item`)

---
### Usage
```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let chars = vec!['a', 'z', 'b'];

    println!("Largest number : {}", largest(&nums));
    println!("Largest char: {}", largest(&nums));
}
```

## Genetics with traits

use **trait bounds** to restrict a generic type to only those that implement a specific trait.

```rust
fn print_area<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
}

fn print_area_alt(shape: impl Shape) {
    println!("Area: {}", shape.area());
}
```