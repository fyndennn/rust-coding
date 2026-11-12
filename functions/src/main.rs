fn main() {
    let x = plus_one(5);
    println!("This is the value: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
