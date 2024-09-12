// extern crate tokio;

// use tokio::time;

use core::panic;
use std::{future::Future, task::Poll};

#[tokio::main]
async fn main() {
    // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    // println!("Hello, Async Rust!")

    // hello("Async Rust!").await;

    async_hello("Async Rust!").await;
}

// async fn hello(name: &'static str) {
//     println!("Hello, {name}");
// }

// 自定义 Future

// 状态机
// [initial] -(constr)-> [Init{name:String}] -(poll)-> [Done] -(drop)-> [Final]
enum Hello {
    // Future 的开始状态
    Init {
        name: &'static str,
        f: fn(name: &'static str),
    },
    // Future 完成后的状态
    Done,
}

impl Future for Hello {
    type Output = ();

    // poll 触发，但是已经 Done 的直接 panic
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // 匹配
        match *self {
            // 默认初始状态是 Init，匹配上则执行打印 Hello
            Hello::Init { name, f } => f(name),
            // 匹配到 Done，说明已经 poll 过，panic
            Hello::Done => panic!("Please stop polling me!"),
        };

        // 执行完，状态更新为 Done
        *self = Hello::Done;
        Poll::Ready(())
    }
}

fn hello(name: &'static str) {
    println!("Hello, {name}");
}

// 现在 它 等同于 async fn hello
fn async_hello(name: &'static str) -> impl Future<Output = ()> {
    Hello::Init {
        name: name,
        f: hello,
    }
}
