enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrString {
    V4(String),
    V6(String),
}

enum IpAddrInteger {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// 

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // let home = IpAddrString::V4(String::from("127.0.0.1"));

    // let loopback = IpAddrString::V6(String::from("::1"));

    let home = IpAddrInteger::V4(127, 0, 0, 1);

    let loopback = IpAddrInteger::V6(String::from("::1"));

    let some_number = Option::Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = Option::None;

    // 

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {

}