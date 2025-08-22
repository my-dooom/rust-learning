# Pattern Matching

Pattern matching in rust is a powerful feature that lets you destructure and branch on complex data types, such as enums, structs, tuples and more. The main tools are `match` and `if let`

## `match` expression

`match` is an exhausitve control structure that compares a value against patterns. All possible cases must be handled either explicitly or with a wildcard (`_`).

### Basic example:

```rust
let num = 3;

match num {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"),
}
```

Each arm has a pattern followed by a FAT_ARROW (`=>`) and a result block or expression. The `_` pattern matches anything not previously matched.

### With enums:

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Left;

match dir {
    Direction::Up => println!("Going up"),
    Direction::Down => println!("Going down"),
    Direction::Left => println!("Going left"),
    Direction::Right => println!("Going right"),
}
```

### Destructuring Enum Variants with Data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Text: {}", text),
}
```

## `if let`

`if let` is a shorthand for matching a single pattern and ignoring the rest.

```rust
let some_number = Some(5);

if let Some(x) = some_number {
    println!("The number is {}", x);
}
```

- Equivalent to a match with one arm and `_ => {}` for the rest.
- Useful for matching only the expected case concisely.
  
You can also use `esle` with `if let`:

```rust
if let Some(x) = some_number {
    println!("The number is {}", x);
} else {
    println!("No match!");
}
```

### Pattern matching with `while let`

Loops can also match patterns:

```rust
let mut stack vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("stack top = {}", top);
}
```
- Continues looping as long as the pattern matches.


# Rust Pattern Matching Constructs

| Construct   | Description                                      |
|-------------|--------------------------------------------------|
| `match`     | Exhaustive pattern matching across all variants  |
| `if let`    | Concise match for one specific pattern           |
| `while let` | Loop with a destructuring pattern                |