#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let rectangle = Rectangle { w: 30, h: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle),
    );
    //println!("{rectangle:#?}");

    let scale = 2;
    let rectangle_2 = Rectangle {
        w: dbg!(30 * scale),
        h: 50,
    };

    dbg!(&rectangle_2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.w * rectangle.h
}
