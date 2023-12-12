use std::thread;
use std::time::Duration;

fn main() {
    create_thread();
    thread::sleep(Duration::from_secs(1));
    create_thread2();
}


fn create_thread() {
    thread::spawn(|| {
        for i in 1..=10 {
            println!("sub thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..=5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1))
    }
}

// 第一个函数，绝大多数情况是子线程为运行完，主线程已经运行结束并退出
fn create_thread2() {
    let sub = thread::spawn(|| {
        for i in 1..=10 {
            println!("sub thread2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..=5 {
        println!("main thread2: {}", i);
        thread::sleep(Duration::from_millis(1))
    }
    // 使用 join 方法，将子线程加入主线程等待队列
    sub.join().unwrap();
}