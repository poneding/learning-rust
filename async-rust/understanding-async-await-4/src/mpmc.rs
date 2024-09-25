use core::fmt;
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{Display, Formatter},
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

pub fn channel(capacity: usize) -> (Sender, Receiver) {
    // Arc 线程安全的引用计数
    // Mutex 锁，调用 lock() 返回锁保护，MutexGuard
    let inner = Arc::new(Mutex::new(Channel::new(capacity)));

    (Sender::new(inner.clone()), Receiver::new(inner.clone()))
}

#[derive(Debug)] // {:?}
pub struct ChannelClosedError {}
// fmt::Display -> {}, #[derive(Debug)] -> {:?}
impl Display for ChannelClosedError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "channel closed")
    }
}
impl Error for ChannelClosedError {}

pub struct Sender {
    inner: Arc<Mutex<Channel>>,
}

impl Sender {
    fn new(inner: Arc<Mutex<Channel>>) -> Self {
        {
            match inner.lock() {
                Ok(mut inner_guard) => inner_guard.inc_senders(),
                Err(_) => panic!("mpmc channel has becom corrupted"),
            }
        }

        Self { inner }
    }

    pub async fn send(&self, value: String) -> Result<(), ChannelClosedError> {
        // Send is a future
        Send {
            value,
            inner: self.inner.clone(),
        }
        .await
    }
}

impl Clone for Sender {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        match self.inner.lock() {
            Ok(mut inner_guard) => {
                println!("drop sender");
                inner_guard.dec_senders();
            }
            Err(_) => panic!("mpmc channel has become corrupted"),
        }
    }
}

struct Send {
    value: String,
    inner: Arc<Mutex<Channel>>,
}

/// impl Future for Send

impl Future for Send {
    type Output = Result<(), ChannelClosedError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let Ok(mut inner_guard) = self.inner.lock() else {
            panic!("mpmc channel has become corrupted");
        };

        match inner_guard.send(self.value.clone()) {
            Ok(()) => Poll::Ready(Ok(())),
            Err(ChannelSendError::Closed) => Poll::Ready(Err(ChannelClosedError {})),
            Err(ChannelSendError::Full) => {
                inner_guard.register_sender_waker(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

pub struct Receiver {
    inner: Arc<Mutex<Channel>>,
}

impl Receiver {
    fn new(inner: Arc<Mutex<Channel>>) -> Self {
        {
            match inner.lock() {
                Ok(mut inner_guard) => inner_guard.inc_receivers(),
                Err(_) => panic!("mpmc channel has becom corrupted"),
            }
        }

        Self { inner }
    }

    pub async fn recv(&self) -> Result<String, ChannelClosedError> {
        // Send is a future
        Recv {
            inner: self.inner.clone(),
        }
        .await
    }
}

impl Clone for Receiver {
    fn clone(&self) -> Self {
        Self::new(self.inner.clone())
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        match self.inner.lock() {
            Ok(mut inner_guard) => {
                println!("drop receiver");
                inner_guard.dec_receivers()
            }
            Err(_) => panic!("mpmc channel has become corrupted"),
        }
    }
}

struct Recv {
    inner: Arc<Mutex<Channel>>,
}

/// impl Future for Send

impl Future for Recv {
    type Output = Result<String, ChannelClosedError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let Ok(mut inner_guard) = self.inner.lock() else {
            panic!("mpmc channel has become corrupted");
        };

        match inner_guard.recv() {
            Ok(value) => Poll::Ready(Ok(value)),
            Err(ChannelRecvError::Closed) => Poll::Ready(Err(ChannelClosedError {})),
            Err(ChannelRecvError::Empty) => {
                inner_guard.register_receiver_waker(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

struct Channel {
    buffer: VecDeque<String>, // 双端队列
    capacity: usize,
    closed: bool,

    senders: usize,
    receivers: usize,

    sender_wakers: VecDeque<Waker>,
    receiver_wakers: VecDeque<Waker>,
}

impl Channel {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            closed: false,
            senders: 0,
            receivers: 0,
            sender_wakers: VecDeque::new(),
            receiver_wakers: VecDeque::new(),
        }
    }

    fn register_sender_waker(&mut self, waker: Waker) {
        self.sender_wakers.push_back(waker);
    }

    fn register_receiver_waker(&mut self, waker: Waker) {
        self.receiver_wakers.push_back(waker);
    }

    fn wake_next_sender(&mut self) {
        if let Some(waker) = self.sender_wakers.pop_front() {
            waker.wake();
        }
    }

    fn wake_next_receiver(&mut self) {
        if let Some(waker) = self.receiver_wakers.pop_front() {
            waker.wake();
        }
    }

    fn inc_senders(&mut self) {
        self.senders += 1;
    }

    fn dec_senders(&mut self) {
        self.senders -= 1;
        if self.senders == 0 {
            self.close();
        }
    }

    fn inc_receivers(&mut self) {
        self.receivers += 1;
    }

    fn dec_receivers(&mut self) {
        self.receivers -= 1;
        if self.receivers == 0 {
            self.close();
        }
    }

    fn send(&mut self, value: String) -> Result<(), ChannelSendError> {
        if self.closed {
            return Err(ChannelSendError::Closed);
        }

        if self.buffer.len() < self.capacity {
            self.buffer.push_back(value);
            self.wake_next_receiver();
            Ok(())
        } else {
            Err(ChannelSendError::Full)
        }
    }

    fn recv(&mut self) -> Result<String, ChannelRecvError> {
        match self.buffer.pop_front() {
            Some(value) => {
                self.wake_next_sender();
                Ok(value)
            }
            None => {
                if self.closed {
                    Err(ChannelRecvError::Closed)
                } else {
                    Err(ChannelRecvError::Empty)
                }
            }
        }
    }

    fn close(&mut self) {
        self.closed = true;

        while let Some(waker) = self.sender_wakers.pop_front() {
            waker.wake();
        }

        while let Some(waker) = self.receiver_wakers.pop_front() {
            waker.wake();
        }
    }
}

#[derive(Debug)]
enum ChannelSendError {
    Full,
    Closed,
}

impl Display for ChannelSendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelSendError::Full => write!(f, "channel full"),
            ChannelSendError::Closed => write!(f, "channel closed"),
        }
    }
}

impl Error for ChannelSendError {}

#[derive(Debug)]
enum ChannelRecvError {
    Empty,
    Closed,
}

impl Display for ChannelRecvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelRecvError::Empty => write!(f, "channel empty"),
            ChannelRecvError::Closed => write!(f, "channel closed"),
        }
    }
}

impl Error for ChannelRecvError {}
