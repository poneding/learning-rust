use std::{sync::mpsc, thread, time::Duration};

// Golang channel 的目标：
// Do not communicate by sharing memory; instead, share memory by communicating.
// 不要通过共享内存来通讯，而是通过通讯来共享内存。
// Rust 实现 channel（信道）- sender，receiver
fn _main_1() {
    // mpsc: multiple producer, single consumer

    //  mscp::channel() 返回信道的发送者和接收者，使用 let 解构元组
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        // 使用 move 将 sender 所有权移动到派生线程
        let val = String::from("Hello");
        thread::sleep(Duration::from_secs(2));
        sender.send(val).unwrap()
    });

    // let received = receiver.recv().unwrap();
    // println!("Received: {received}");

    // recv() 会阻塞当前线程直到接收到一个值，try_recv() 不阻塞线程
    let recv_handle = thread::spawn(move || {
        for _ in 1..5 {
            let received = receiver.try_recv();
            if received.is_ok() {
                let reveived_val = received.unwrap();
                println!("Received: {reveived_val}");
                break;
            }

            // is_ok() == false，本次尝试接收失败，也可以通过 is_err() 来判断是否失败
            // if received.is_err() {
            thread::sleep(Duration::from_secs(1));
            // }
        }
    });

    recv_handle.join().unwrap(); // 等待线程结束
}

fn _main_2() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");
        sender.send(val).unwrap();

        // println!("Send: {}", val); // val 所有权已经被移动到 send 函数，这里不能再使用 val
    });

    let reveivec = receiver.recv().unwrap();
    println!("Received: {}", reveivec);
}

fn _main_3() {
    // sender 发送多个值
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receiver 实现了迭代器，可以使用 for 循环接收多个值
    for receiced in receiver {
        println!("Received: {}", receiced);
    }
}

fn main() {
    // 克隆发送者，multiple producer，single consumer
    let (sender, receiver) = mpsc::channel();

    let cloned_sender = sender.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("cloned"),
            String::from("sender"),
        ];

        for val in vals {
            cloned_sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("origin"),
            String::from("sender"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receiced in receiver {
        println!("Received: {}", receiced);
    }
}
