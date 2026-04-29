use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Eq)]
enum BallState {
    PING,
    PONG,
    DONE,
}

fn is_received() -> bool {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 100 != 99
}

fn main() {
    let status = (BallState::PING, Condvar::new());
    let mut score2 = Mutex::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let (ball, score) = status;
            let mut num = score2.lock().unwrap();
            *num += 1;
        });
        s.spawn(|| {
            let (ball, score) = status;
            let mut num = score2.lock().unwrap();
            *num += 1;
        });
    })
}
