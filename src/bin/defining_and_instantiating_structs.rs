struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {

    let mut user1 = User {
        active: true, 
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheruser1@example.com");

    println!("{}", user1.email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user2@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user2.email);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("{}", user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true, 
        username,
        email,
        sign_in_count: 1,
    }
}