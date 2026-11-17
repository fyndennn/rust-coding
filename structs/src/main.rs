struct Color(i32, i32, i32);
struct Color_rgb {
    r: i32,
    g: i32,
    b: i32,
}

struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user1 = build_user(
        String::from("example@gmail.com"),
        String::from("exampleusername"),
    );

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("UPDATE_SYNTAX_COOL@gmail.com"),
        ..user2
    };

    println!("{}", user2.active);
}
