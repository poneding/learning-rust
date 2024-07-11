use std::{thread, time::Duration};

// 并发（或者并行）问题：
// 1、竞态条件
// 2、死锁，两个线程相互等待对方，无限阻塞
// 3、难以调试以及排查 bug
fn _main_1() {
    // 使用 spawn 创建新线程，spawn 是派生的意思
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 如果在这里调用 handle.join()，那么会等待派生线程结束再执行后面的主线程代码
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }

    // 可能派生线程还未执行完，主线程就已经退出了
    // thread::spawn 返回一个 JoinHandle 对线，使用 handle.join() 来等待线程结束
    handle.join().unwrap();
}

// move 闭包与线程一起使用
fn main() {
    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    // 在派生线程使用了主线程的 v，println 使用到 v 的引用， 闭包尝试借用 v
    // 但是派生线程不知道 v 是否一直有效
    let handle = thread::spawn(move || {
        // 使用 move 将 v 的所有权移动到派生线程
        println!("Here's a vector: {v:?}");
    });

    // drop(v); // 例如，这里直接丢弃 v，那么派生线程闭包对 v 的引用不再有效

    handle.join().unwrap();
}
