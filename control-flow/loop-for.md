# Control Flow

## `loop`

An infinite loop that runs until explicitly broken.


```rust
loop {
    println!("Running forever");
    break;  // exit manually
}
```

### Returning a value from `loop`

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter = 10 {
        break counter * 2;
    }
};
```

## `for` loops

Used to iterate over iterators (common with ranges and collections);

```rust
for i in 0..5 {
    println!("i = {}", i);
}
```

- `0..5` is a range (exclusive of 5)
- to include 5: `0..=5`
  
Iterating over a collection:

```rust
let names = ["Alice", "Bob", "Carol"];
for name in names.iter() {
    println!("Name: {}", name);
}
```
