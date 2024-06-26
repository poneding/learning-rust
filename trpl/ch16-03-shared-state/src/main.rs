use std::{rc::Rc, sync::Arc, sync::Mutex, thread};

// 不要通过共享内存来通讯，do not communicate by sharing memory.

// 互斥器：mutex，mutual exclusion，任意时刻，只允许一个线程访问资源
// 1、获取锁
// 2、访问资源
// 3、释放锁
fn _main_1() {
    let m = Mutex::new(1);

    {
        let mut num = m.lock().unwrap(); // 获取锁, 返回 MutexGuard，其实现了 Deref 来指向内部数据，实现了 Drop 来在离开作用域时自动释放锁
        *num += 1;
    }

    println!("m = {:?}", m)
}

fn main() {
    // 在线程间共享 Mutex<T>
    let _counter1 = Mutex::new(0);

    // Rc：引用计数
    // Arc：原子引用计数

    let _counter2 = Rc::new(Mutex::new(0)); // Rc<Mutex<i32>> 没有实现 Send Trait

    // Rc<T> 并不能安全的在线程间共享，使用 Arc 原子操作
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // 使用多所有权方式来调整
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // let mut num = _counter1.lock().unwrap(); // counter 所有权被移动到派生线程
            // let mut num = _counter2.lock().unwrap(); // Rc<T> 不能安全的在线程间发送数据

            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()) // 丢失所有权
}
