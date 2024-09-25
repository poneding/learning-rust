use understanding_async_await_4::mpmc::{self, Receiver, Sender};

#[tokio::main]
async fn main() {
    // let capacity = 1;
    let capacity = 2;
    let (sender, receiver) = mpmc::channel(capacity);

    // tokio::spawn 会将 sender 和 receiver 的所有权转移给新的 task
    // 在新的 task 中，sender 和 receiver 离开作用域后，会调用析构函数 drop()
    let receiver_handle = tokio::spawn(receive_loop(receiver));
    let sender_handle = tokio::spawn(send_two(sender));

    _ = sender_handle.await;
    _ = receiver_handle.await;

    println!("done");
}

async fn send_two(sender: Sender) {
    let values = vec!["Hello", "World"];

    for value in values {
        sender
            .send(value.to_string())
            .await
            .expect("the channel has closed early");
        println!("sent: {}", value)
    }
}

async fn receive_loop(receiver: Receiver) {
    loop {
        match receiver.recv().await {
            Ok(value) => println!("received: {}", value),
            Err(_) => {
                println!("channel closed, exiting.");
                break;
            }
        }
    }
}
