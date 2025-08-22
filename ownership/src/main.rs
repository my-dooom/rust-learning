


fn print_string(s: String) -> String {
    println!("From function: {}", s);
    s
}

fn main() {
    let original = String::from("hehe");
    let original = print_string(original);
    println!("Still valid: {}", original); // ✅
}

