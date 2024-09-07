use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};
// 1. 不知类型长度，使用 Box 只能指针
// 2. 类型别名
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            // 不断的从 receiver 中获取请求，并处理
            loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Self {
            id,
            // 一个已经运行的线程
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    // threads: Vec<JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// 创建一个线程数量为 `size` 的线程池。
    ///
    /// # Panic
    ///
    /// `new` 函数在 `size` 为 0 时会 panic。
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Arc，引用计数，共享所有权，允许多线程修改
        let receiver = Arc::new(Mutex::new(receiver));

        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            // threads,
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// 优雅停机与清理
// 当主线程收到终止信号，执行完所有子线程
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // 丢弃 sender，将拒绝新请求

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // take() 取出现线程，并将 worker 的线程替换为 None
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
