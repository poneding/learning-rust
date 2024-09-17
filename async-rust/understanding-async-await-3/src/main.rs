use core::{fmt, panic};
use std::{
    future::Future,
    os::macos::raw::stat,
    process::Output,
    sync::{Arc, Mutex, MutexGuard},
    task::Poll,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // hold_mutex_guard(Arc::new(Mutex::new(0_u64)))
    //     .await
    //     .expect("failed to perform operation.")

    let data = Arc::new(Mutex::new(0_u64));

    tokio::spawn(yieldless_mutex_access(Arc::clone(&data)));
    hold_mutex_guard(Arc::clone(&data))
        .await
        .expect("failed to perform operation.");
}

#[derive(Debug)]
struct DataAccessError {}

impl fmt::Display for DataAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "there was an error accessing the shared data.")
    }
}

async fn hold_mutex_guard(data: Arc<Mutex<u64>>) -> Result<(), DataAccessError> {
    let mut guard = data.lock().map_err(|_| DataAccessError {})?;
    println!("existing value in hold_mutex_guard: {}", *guard);

    tokio::task::yield_now().await;

    *guard += 3;
    println!("new value in hold_mutex_guard: {}", *guard);

    Ok(())
}

async fn yieldless_mutex_access(data: Arc<Mutex<u64>>) -> Result<(), DataAccessError> {
    let mut guard = data.lock().map_err(|_| DataAccessError {})?;

    println!("existing value in yeildless_mutex_access: {}", *guard);

    *guard += 3;
    println!("new value in yeildless_mutex_access: {}", *guard);

    Ok(())
}

enum HoldMutextGuard<'a> {
    Init {
        data: Arc<Mutex<u64>>,
    },
    Yielded {
        guard: MutexGuard<'a, u64>,
        _data: Arc<Mutex<u64>>,
    },
    Done,
}

impl<'a> Future for HoldMutextGuard<'a> {
    type Output = Result<(), DataAccessError>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state = &mut *self;
        match state {
            Self::Init { data } => {
                let guard = unsafe {
                    std::mem::transmute::<MutexGuard<'_, u64>, MutexGuard<'static, u64>>(
                        data.lock().map_err(|_| DataAccessError {})?,
                    )
                };
                println!("existing value: {}", *guard);

                cx.waker().wake_by_ref();

                *state = Self::Yielded {
                    guard: guard,
                    _data: Arc::clone(&data),
                };

                Poll::Pending
            }
            Self::Yielded { guard, _data } => {
                println!("new value: {}", *guard);
                *state = Self::Done;

                Poll::Ready(Ok(()))
            }
            Self::Done => panic!("please stop polling me!"),
        }
    }
}

fn hold_mutex_guard2(data: Arc<Mutex<u64>>) -> impl Future<Output = Result<(), DataAccessError>> {
    HoldMutextGuard::Init { data }
}
