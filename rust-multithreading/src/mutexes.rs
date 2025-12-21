use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // The order of Mutex inside Ark makes is logical because:
    // Arc<Mutex<T>> - multiple threads own the same value and access is synchronized
    // Arc lets threads share the value
    // Mutex protects the interior mutation
    // Arc is sharing mechanism while Mutex is access-control mechanism
    // Arc<T> owns T and enables shared ownership across threads
    // Mutex provides exclusive mutable access to T
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Mutex is more useful than Atomic[Type] because it allows:
    // 1. Mutate multiple values together
    // 2. Perform complex logic atomically
    // 3. Borrow the value

    for _ in 0..10 {
        // Arc::clone does not create a new instance, just clones the pointer
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock is automatically released at the end of the scope
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        // Mutating global state is fine since it is not inside thread
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
