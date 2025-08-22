### Tuples + Structs + Enums
---
- **Tuples**: Fixed-size, ordered collections of varrying types.
- **Structs**: Named data structures with explicitly named fields
- **Enums**: Tagged unions that represtent data with multiple variants


# Tuples

A **tuple** is an ordered collection of elements, where each element can have a different type. Tuples are defined using parentheses.

```rust
let point: (i32, i32) = (10, 20);
```

### Accessing Elements

Use dot notation with an index:
```rust
let x = point.0
let y = point.1
```

### Destructuring
```rust
let (a, b) = point;
```

### Tuple types
Tuples are distinct by size and type:
```rust
let a: (i32, f64, bool) = (5, 3.2, true);
```

Rust has special 0-length tuple: `()`, called **unit-type**, repesenting an empty value.

# Structs

A **struct** (short for structure) defines a custom data type composed of named fields.

### Definig a struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

### Instantiating

```rust
let user1 = User {
    username: String::from("alex"),
    email: String::from("alice@example.com"),
    active: true,
};
```

### Accessing fields

```rust
println!("{}", user1.email);
```

### Mutable structs

```rust
let mut user2 = user1;
user2.email = String::from("mutated@example.com");
```

### Struct update syntax

```rust
let user3 = User {
    email: String::from("clone@example.com"),
    ..user2
};
```

### Tuple-Structs

Tuple Structs are like tuples, but with a named type. Fields are accessed by index.

```rust
struct Color(u8, u8, u8);
let black = Color(0, 0, 0);

println!("{}", black.1);
```

Used when field names are unnecessary but the type needs to be distinct.

### Unit-Like Structs

Structs with no fields, useful for traits or marker types.

```rust
struct Marker;
```

# Enums


**Enums** define a type by listing possible *variants*. Each variant can hold data, like a tagged union.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

### Using enums

```rust
let dir = Direction::Left;
```
### Enums with data 

Variants can hold additional data:

```rust

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8, u8),
}


let msg = Message::Move {x:10, y:20};
```

### Pattern Matching with Enums

Use `match` to exhuastively handle all enum cases:

```rust
match msg {
    Message::Quit => println!("Quit"),
    Message::Move {x, y} => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Text: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
}
```

| Feature      | Description                                 | Syntax Highlights           |
|--------------|---------------------------------------------|-----------------------------|
| Tuple        | Ordered collection of mixed types           | `(i32, bool)`               |
| Struct       | Named fields, user-defined type             | `struct Name {}`            |
| Tuple Struct | Named type with unnamed fields (like tuple) | `struct Name(T1, T2)`       |
| Unit Struct  | Zero-field struct                           | `struct Name;`              |
| Enum         | Type with named variants, optional data     | `enum Name {}`              |

