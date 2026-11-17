#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&mut self, bias: u32) -> u32 {
        self.w * self.h + bias
    }
}

fn main() {
    let new_bias = 32;
    let mut rectangle = Rectangle { w: 30, h: 50 };
    rectangle.w = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area(new_bias)
    );
}
