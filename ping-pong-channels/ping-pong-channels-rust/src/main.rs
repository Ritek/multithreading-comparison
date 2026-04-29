use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, mpsc};
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
    let score = Arc::new(AtomicUsize::new(0));

    let (tx1, rx1): (Sender<BallState>, Receiver<BallState>) = mpsc::channel();
    let (tx2, rx2): (Sender<BallState>, Receiver<BallState>) = mpsc::channel();
    let mut players = Vec::new();

    tx1.send(BallState::PING)
        .expect("Could not send starting message");
    println!("[PING]");

    let score_ref1 = Arc::clone(&score);
    let p1 = thread::spawn(move || {
        loop {
            let msg = rx2.recv().expect("p1 could not read message");

            if msg != BallState::PONG {
                break;
            }

            if !is_received() {
                tx1.send(BallState::DONE)
                    .expect("p1 could not send final message");
                break;
            }

            let sent = tx1.send(BallState::PING);
            match sent {
                Ok(_) => {
                    score_ref1.fetch_add(1, Ordering::Relaxed);
                    println!("[PING] | score: {:?}", score_ref1);
                }
                Err(_) => break,
            }
        }
    });
    players.push(p1);

    let score_ref2 = Arc::clone(&score);
    let p2 = thread::spawn(move || {
        loop {
            let msg = rx1.recv().expect("p2 could not read message");

            if msg != BallState::PING {
                break;
            }

            if !is_received() {
                tx2.send(BallState::DONE)
                    .expect("p2 could not send final message");
                break;
            }

            let sent = tx2.send(BallState::PONG);
            match sent {
                Ok(_) => {
                    score_ref2.fetch_add(1, Ordering::Relaxed);
                    println!("[PONG] | score: {:?}", score_ref2);
                }
                Err(_) => break,
            }
        }
    });
    players.push(p2);

    for player in players {
        player.join().expect("one of the player panicked");
    }
}
