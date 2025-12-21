use std::{
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

fn main() {
    let messages = Arc::new(vec!["message 1", "message 2", "message 3", "message 4"]);

    let (tx, rx) = mpsc::channel();

    for i in 0..2 {
        let messages = Arc::clone(&messages);
        let tx = tx.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            for message in messages.iter() {
                tx.send(format!("thread-{} : {}", i, message)).unwrap();
            }
        });
    }

    drop(tx);

    // let receiver = rx.recv().unwrap();
    for received in rx {
        println!("Got message: {}", received);
    }
}
