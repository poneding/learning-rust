use std::{
    future::{self, Future},
    task::Poll,
};

#[tokio::main]
async fn main() {
    for _ in 0..5 {
        ready().await;
    }

    // println!("before pending.");
    // pending().await;
    // println!("after pending."); // 不会执行到这一步了

    println!("before yield.");
    yielded().await;
    println!("after yield.");
}

// 先来一个最简单的 Future
struct Ready {}

impl Future for Ready {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("Ready!");
        Poll::Ready(())
    }
}

fn ready() -> Ready {
    Ready {}
}

// Pending
struct Pending {}

impl Future for Pending {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        println!("Pending!");
        Poll::Pending
    }
}

#[allow(unused)]
// fn pending() -> Pending {
fn pending() -> impl Future<Output = ()> {
    Pending {}
}

struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        println!("YieldNow!");
        if self.yielded {
            return Poll::Ready(());
        }

        self.yielded = true;

        cx.waker().wake_by_ref();

        Poll::Pending
    }
}

fn yielded() -> YieldNow {
    YieldNow { yielded: false }
}
