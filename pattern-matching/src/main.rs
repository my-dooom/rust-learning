enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn match_check(x: i32, y: i32) -> i32 {
    if ((x <= -1) || (y <= -1)) {
        println!("We hit the catch all!");
        return 0;
    } else {
        println!("Move to: {}, {}", x, y);
        return 1;
    }
}

fn main() {
    ///// MATCH ///////
    let x = Message::Move { x: 1, y: 2 };
    // let x = Message::Quit;
    let y: i32 = match x {
        Message::Move { x, y } => match_check(x, y),
        _ => match_check(-1, -1),
    };

    println!("Return = {}", y);

    ///// IF LET ///////
    let arr: [i32; 500] = [0; 500];

    // let fivehundredandone = arr.get(501);

    if let Some(x) = arr.get(501) {
        println!("{} is the value", x);
    } else {
        println!("TOO MUCH BUDDY!");
    }
}
