// -----------------------------第 1 段示例代码-----------------------------
// use std::time::Duration;
// use async_std::task::{sleep, spawn};
//
// async fn do3() {
//     for i in 1..=5 {
//         println!("do3 {}", i);
//         sleep(Duration::from_millis(500)).await;
//     }
// }
//
// async fn do4() {
//     for i in 1..=5 {
//         println!("do4 {}", i);
//         sleep(Duration::from_millis(1000)).await;
//     }
// }
//
// #[async_std::main]
// async fn main() {
//     let do3_async = spawn(do3());
//     do4().await;
//     do3_async.await;
// }


// 0. https://crates.io/crates/async-std
// 1. 需要添加包 async-std = "1.12.0"，写到 Cargo.toml 文件的 dependencies 下
// 2. 函数前添加 async
// 3. 主函数 main 添加 `#[async_std::main]` 属性，且主函数也需要添加 async
// 4. spawn(do3()) 而不是 spawn(do3)
// 5. do4() 调用后面添加 .await
// 6. sleep 后添加 .await

// -----------------------------第 1 段示例代码-----------------------------

// -----------------------------第 2 段示例代码-----------------------------
use futures::executor::block_on;
use std::future::Future;

async fn lesson() -> String { String::from("Learning") }

fn study1() -> impl Future<Output=String> {
    async {
        let x = lesson().await;
        x + " >> Target"
    }
}

fn go_study() -> impl Future<Output=String> {
    let r = |x: String| async move {
        let y: String = study1().await;
        y + &*x
    };
    r(String::from(": Golang"))
}

fn main() {
    let result = block_on(go_study());
    println!("{}", result);
}
// 0. https://crates.io/crates/futures
// 1. 添加依赖 futures = "0.3.21"
// -----------------------------第 2 段示例代码-----------------------------