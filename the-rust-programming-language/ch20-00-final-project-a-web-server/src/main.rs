use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 3. 使用线程池
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!"); // 连接建立完成
        // 1. 单线程的，多个请求是阻塞的
        // handle_connection(stream);

        // 2. 每个请求都新建立一个线程处理，如果并发请求很大，系统资源会被耗尽
        // thread::spawn(|| handle_connection(stream));

        // 3. 使用线程池
        pool.execute(|| {
            handle_connection(stream);
        });

        // 测试线程池处理请求
        // 第一步：异步请求 /sleep 4次
        //
        // for i in {1..4}
        // do
        // # 在这里替换为你想要执行的命令
        // curl localhost:7878/sleep &
        // done

        // 第二步： 立马请求 /，发现请求被阻塞
        // curl localhost:7878
    }
}

// HTTP 请求格式
// Method Request-URI HTTP-Version CRLF
// headers CRLF
//
// message-body

// HTTP 响应格式
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
//
// message-body

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 400 Not Found", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

// 1. 不知类型长度，使用 Box 只能指针
// 2. 类型别名
type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(unused)]
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            // 不断的从 receiver 中获取请求，并处理
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");

                job();
            }
        });
        Self {
            id,
            // 一个已经运行的线程
            thread,
        }
    }
}

#[allow(unused)]
pub struct ThreadPool {
    // threads: Vec<JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
