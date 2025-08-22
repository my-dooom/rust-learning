fn main() {
    let mut i = 50;

    let ni = loop {
        if i == 0 {
            break 69;
        }
        i -= 1;
    };

    println!("{}", ni);

    for x in 0..5 {
        println!("{}", x);
    }

    let names: [&'static str; 3] = ["Mikus", "Sabcia", "Lausia"];

    for name in names.iter() {
        println!("{}", name);
    }
}
