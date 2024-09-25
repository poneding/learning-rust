use std::time::Duration;

use tokio::time::sleep;
use understanding_async_await_4::mpmc;

#[tokio::main]
async fn main() {
    let mut tx_tasks = Vec::new();
    let mut rx_tasks = Vec::new();

    let (tx, rx) = mpmc::channel(10);

    for idx in 0..2 {
        let rx = rx.clone();

        let jh = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(value) => {
                        println!("rx-{idx:0>2}: received value: {value}");
                        sleep(Duration::from_micros(100)).await;
                    }
                    Err(_) => {
                        println!("rx-{idx:0>2}: channel closed.");
                        break;
                    }
                }
            }
        });

        rx_tasks.push(jh);
    }

    for idx in 0..3 {
        let tx = tx.clone();

        let jh = tokio::spawn(async move {
            for i in 0..2 {
                let value = format!("tx-{idx:0>2}: value-{i}");
                println!("tx-{idx:0>2}: sending value: {value}");
                if tx.send(value).await.is_err() {
                    println!("tx-{idx:0>2}: channel closed.");
                    break;
                }
                sleep(Duration::from_micros(80)).await;
            }
        });

        tx_tasks.push(jh);
    }

    for jh in tx_tasks {
        _ = jh.await;
    }
}
