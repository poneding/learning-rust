#[allow(unused)]
fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    route(IpAddrKind::V4); // 调用函数
    route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let lookback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let lookback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let lookback = IpAddr::V6(String::from("::1"));

    let quit = Message::Quit;
    quit.call();

    let move_msg = Message::Move { x: 1, y: 1 };
    move_msg.call();

    let write = Message::Write(String::from("hello from main"));
    write.call();

    let change_color = Message::ChangeColor(1, 1, 1);
    change_color.call();

    let num = MyOption::Some(1);
    let char = MyOption::Some('e');
    let none1: MyOption<i32> = MyOption::None;

    // rust 内置的 Option，Some，None 都可以直接使用，而不需要前缀
}

// 枚举
enum IpAddrKind {
    V4,
    V6,
}

// 枚举关联
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

#[allow(unused)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(_ip_kind: IpAddrKind) {} // 枚举作为函数参数

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[allow(unused)]
struct Ipv4Addr {
    n1: u8,
    n2: u8,
    n3: u8,
    n4: u8,
}
#[allow(unused)]
struct Ipv6Addr {
    address: String,
}
#[allow(unused)]
enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[allow(unused)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message: {self:?}")
    }
}

enum MyOption<T> {
    Some(T),
    None,
}
