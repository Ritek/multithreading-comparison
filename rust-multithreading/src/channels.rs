use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    // This works because tx is cloned and second thread can take ownership of the original tx
    let tx1 = tx.clone();
    thread::spawn(move || {
        let messages = vec!["message 1", "message 2", "message 3", "message 4"];

        thread::sleep(Duration::from_millis(1000));
        for message in messages.iter() {
            tx1.send(format!("thread-{} : {}", 1, message)).unwrap();
        }
    });

    thread::spawn(move || {
        let messages = vec!["message 1", "message 2", "message 3", "message 4"];

        thread::sleep(Duration::from_millis(1000));
        for message in messages.iter() {
            tx.send(format!("thread-{} : {}", 2, message)).unwrap();
        }
    });

    // let receiver = rx.recv().unwrap();
    for received in rx {
        println!("Got message: {}", received);
    }
}
