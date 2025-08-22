struct User {
    name: &'static str,
    age: u8,
    income: u32,
}

fn main() {
    ////// TUPLES ///////
    let point: (i32, i32) = (0, 1);

    let (a, b) = point;

    println!("{} - {} ", a, b);

    println!("{} - {}", point.0, point.1);

    ////// STRUCTS /.//////
    let phil = User {
        name: "Phil",
        age: 27,
        income: 10000,
    };

    println!(
        "{} is {} years old and earns {}",
        phil.name, phil.age, phil.income
    );
}
