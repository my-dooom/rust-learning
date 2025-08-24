fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let chars = vec!['a', 'z', 'b'];

    println!("Largest number : {}", largest(&nums));
    println!("Largest char: {}", largest(&chars));
}
