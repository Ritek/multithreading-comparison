use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, PartialEq, Eq)]
enum Ball {
    PING,
    PONG,
    DONE(String),
}

fn is_received() -> bool {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 100 != 99
}

fn main() {
    let status = Arc::new((Mutex::new(Ball::PING), Condvar::new()));
    let mut handles = Vec::new();

    let ref1 = Arc::clone(&status);
    let p1 = thread::spawn(move || {
        let (lock, cvar) = &*ref1;
        loop {
            let mut ball = lock.lock().unwrap();

            while !matches!(*ball, Ball::PONG | Ball::DONE(_)) {
                ball = cvar.wait(ball).unwrap();
            }

            match &*ball {
                Ball::DONE(_) => break,
                Ball::PONG => {
                    if is_received() {
                        *ball = Ball::PING;
                    } else {
                        *ball = Ball::DONE(String::from("dropped by p1"));
                    }
                }
                _ => unreachable!(),
            }

            let done = matches!(*ball, Ball::DONE(_));
            drop(ball);
            cvar.notify_all();

            if done {
                break;
            } else {
                println!("[PING] p1 hit the ball");
            }
        }
    });
    handles.push(p1);

    let ref2 = Arc::clone(&status);
    let p2 = thread::spawn(move || {
        let (lock, cvar) = &*ref2;
        loop {
            let mut ball = lock.lock().unwrap();

            while !matches!(*ball, Ball::PING | Ball::DONE(_)) {
                ball = cvar.wait(ball).unwrap();
            }

            match &*ball {
                Ball::DONE(_) => break,
                Ball::PING => {
                    if is_received() {
                        *ball = Ball::PONG;
                    } else {
                        *ball = Ball::DONE(String::from("dropped by p2"));
                    }
                }
                _ => unreachable!(),
            }

            let done = matches!(*ball, Ball::DONE(_));
            drop(ball);
            cvar.notify_all();

            if done {
                break;
            } else {
                println!("[PONG] p2 hit the ball");
            }
        }
    });
    handles.push(p2);

    for handle in handles {
        handle.join().unwrap();
    }

    let (lock, _) = &*status;
    println!("Result: {:?}", *lock.lock().unwrap());
}
