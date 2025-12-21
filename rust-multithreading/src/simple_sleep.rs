use std::{thread, time::Duration};

fn slow_logic(i: u64) {
    println!("Thread {}: {:?}", i, thread::current());
    thread::sleep(Duration::from_millis(i * 10));
}

fn main() {
    //

    for i in 0..10 {
        let handle = thread::spawn(move || {
            // Current standard library implementation gives main thread id 1
            // This may change in the future
            println!("Thread {}: {:?}", i, thread::current());
            thread::sleep(Duration::from_millis(i * 10));
        });
        // or
        // let handle = thread::spawn(move || slow_logic(i));

        // Without join main thread will finish without waiting for others
        handle.join().unwrap();
    }
}
