fn calc_len(s: &String) -> usize {
    s.len()
}

fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut msg = String::from("hehe");

    let len = calc_len(&msg); // immutable borrow
    println!("Length: {}", len);

    add_exclamation(&mut msg); // mutable borrow
    println!("Modified: {}", msg); // "hehe!"
}
