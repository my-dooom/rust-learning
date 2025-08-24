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

fn main() {
    let r = Rectangle::new(3, 2);

    println!("Area = {}", r.get_area());
    println!("Perimeter = {}", r.get_perimeter());
}
