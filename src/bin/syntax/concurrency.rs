// Concurrency and async.

use std::future::Future; // the trait every async value implements
use std::pin::pin; // pins a future in place so it can be polled
use std::sync::Arc;
use std::sync::mpsc; // multiple producer, single consumer channels
use std::task::{Context, Poll, Wake, Waker}; // the pieces a runtime uses to drive futures
use std::thread::{self, Thread};
use std::time::Duration;

// ---------------------------------------------------------------------------
// Async
// ---------------------------------------------------------------------------

// `async fn` doesn't run when called. It returns a Future: a value that produces the
// result later, when something polls it to completion.
async fn fetch_number(n: u32) -> u32 {
    n * 2 // a real program would wait on a network or file read here
}

async fn add_fetched() -> u32 {
    let a = fetch_number(1).await; // .await pauses until that future is done
    let b = fetch_number(2).await;
    a + b
}

// Futures need a runtime to run them. Real programs use a crate like Tokio:
//     #[tokio::main]
//     async fn main() { let x = add_fetched().await; }
// This repo has no dependencies, so here is a tiny runtime that runs one future on this thread.

// A Waker tells the runtime "poll me again". This one wakes up the thread that's waiting.
struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark(); // wake the thread parked in block_on
    }
}

// Polls the future until it's ready, sleeping (parking) whenever it says it isn't ready yet.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future); // futures must not move in memory while being polled
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker); // the context passed to every poll
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value, // finished: hand back the result
            Poll::Pending => thread::park(),    // not yet: sleep until the waker unparks us
        }
    }
}

pub fn run() {
    // ---------------------------------------------------------------------------
    // Threads
    // ---------------------------------------------------------------------------

    let data = vec![1, 2, 3];
    // thread::spawn starts a native OS thread. `move` gives the closure ownership of data,
    // because the thread might outlive this function.
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        sum // the thread's return value
    });
    let sum = handle.join().unwrap(); // join waits for the thread and returns its value
    println!("sum from thread = {sum}");

    // ---------------------------------------------------------------------------
    // Channels
    // ---------------------------------------------------------------------------

    // mpsc::channel gives a sender (tx) and a receiver (rx). Messages are moved through it.
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone(); // "multiple producer": each thread gets its own sender
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * id)); // so the messages arrive in order
            tx.send(format!("hello from thread {id}")).unwrap();
        });
    }
    drop(tx); // drop the original sender, so the channel closes when the threads finish
    // "single consumer": iterating over rx receives until every sender has been dropped.
    for msg in rx {
        println!("{msg}");
    }

    // ---------------------------------------------------------------------------
    // Running async code
    // ---------------------------------------------------------------------------

    let future = add_fetched(); // nothing has run yet
    let result = block_on(future); // now it runs to completion
    println!("async result = {result}"); // 2 + 4 = 6
}
