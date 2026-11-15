fn main() {
    let s = String::from("hello world");

    let word = first_word(&s[0..6]);
    println!("the first word is: {word}");
    let word = first_word(&s[..]);
    println!("the first word is: {word}");
    let word = first_word(&s);
    println!("the first word is: {word}");

    let word = first_word("ashdjkashdjk aszjkldas");

    println!("the first word is: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
