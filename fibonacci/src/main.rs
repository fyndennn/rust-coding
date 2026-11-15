use std::io;

fn fibonnaci_sequence(n: i64) -> i64 {
    let mut a = 1;
    let mut b = 0;

    let mut i = 0;
    let mut c = a + b;

    while i < n {
        c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    return c;
}

fn main() {
    let mut n = String::new();
    println!("Which number from fibonacci sequence do you want to know?");
    io::stdin()
        .read_line(&mut n)
        .expect("Couldn't parse value given");
    let n: i64 = n.trim().parse().expect("wrong n'th value");
    let ans: i64 = fibonnaci_sequence(n);
    println!("your answer is {ans}");
}
