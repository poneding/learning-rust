use core::panic;
use std::collections::HashMap;

use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // 监听制定地址，等待 TCP 连接
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // 等待并接受一个 TCP 连接
        let (socket, _) = listener.accept().await.unwrap();

        // process(socket).await; // 阻塞

        // 每次新建一个任务来处理新的连接
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

/// 处理一个新的客户端连接
async fn process(socket: TcpStream) {
    let mut cache = HashMap::new();
    let mut conn = Connection::new(socket);

    // 读到数据帧(Redis 命令 + 数据)
    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        // 写入一个响应
        let response = Frame::Simple(String::from("world"));
        conn.write_frame(&response).await.unwrap();
    }

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            // Set 命令
            Command::Set(cmd) => {
                cache.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            }
            // Get 命令
            Command::Get(cmd) => {
                if let Some(val) = cache.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 将响应写入连接
        conn.write_frame(&response).await.unwrap();
    }
}
