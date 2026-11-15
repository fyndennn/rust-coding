fn main() {
    let song: [&str; 14] = [
        "On the twelfth day of Christmas",
        "My true love gave to me",
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And a partridge in a pear tree",
    ];
    let mut i = 0;
    while i < 14 {
        for p in 0..=i {
            println!("{}", song[p]);
        }
        println!("\n");
        i += 1;
    }
}
