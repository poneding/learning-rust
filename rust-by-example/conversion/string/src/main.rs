#![allow(unused)]

use core::fmt;
use std::str::FromStr;

fn main() {
    let circle = Circle { radius: 1 };
    println!("{}", circle);

    let circle2 = Circle2 { radius: 2 };
    println!("{}", circle2.to_string());

    let msg: Result<Message, ()> = "msg: hello world!".parse();
    match msg {
        Ok(msg) => println!("{:?}", msg),
        Err(_) => println!("Error"),
    }
}

// 实现 fmt::Display 会自动实现 ToString，而且还可以用来打印
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// 实现 ToString
struct Circle2 {
    radius: i32,
}

impl ToString for Circle2 {
    fn to_string(&self) -> String {
        format!("Circle of radius: {:?}", self.radius)
    }
}

/// 实现 FromStr trait，用于从字符串解析出结构体
#[derive(Debug)]
struct Message {
    value: String,
}

impl FromStr for Message {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("msg:") {
            Ok(Message { value: s.into() })
        } else {
            Err(())
        }
    }
}
