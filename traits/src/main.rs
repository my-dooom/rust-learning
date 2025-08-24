struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }

    fn get_area(&self) -> u32 {
        self.height * self.width
    }

    fn get_perimeter(&self) -> u32 {
        self.height * 2 + self.width + 2
    }
}

struct Square {
    side_lenght: u32,
}

impl Square {
    fn new(side_length: u32) -> Square {
        Square {
            side_lenght: side_length,
        }
    }
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side_lenght * self.side_lenght
    }
    fn perimeter(&self) -> u32 {
        4 * self.side_lenght
    }
}
fn main() {
    let r = Rectangle::new(3, 2);

    println!("Area = {}", r.get_area());
    println!("Perimeter = {}", r.get_perimeter());

    let square = Square::new(5);
    println!("Square area = {}", square.area());
    println!("Square perimeter = {}", square.perimeter());
}
