#[allow(unused_variables)]

use std::thread;
use std::sync::{Arc, Mutex};

// How to use the same data in multiple threads
// Mutexes: allows the access to the same data from different threads

fn main() {
    // Arc = Atomic Reference Counted (smart pointer)
    // Atomic = safe in order to share between threads
    let id = Mutex::new(25);

    // Single example with a fragment of code
    {
        let mut number = id.lock().unwrap();
        *number = 33;   
    }

    println!("id: {:?}", id);

    // Example using multiple threads
    let id2 = Arc::new(Mutex::new(25));

    let num1_clone = Arc::clone(&id2);
    let handle1 = thread::spawn(move || {
        let mut number = num1_clone.lock().unwrap();
        *number += 1;
    });

    let num2_clone = Arc::clone(&id2);
    let handle2 = thread::spawn(move || {
        let mut number = num2_clone.lock().unwrap();
        *number += 1;
    });

    let num3_clone = Arc::clone(&id2);
    let handle3 = thread::spawn(move || {
        let mut number = num3_clone.lock().unwrap();
        *number += 1;
    });

    let num4_clone = Arc::clone(&id2);
    let handle4 = thread::spawn(move || {
        let mut number = num4_clone.lock().unwrap();
        *number += 1;
    });

    // Multiple threads in a loop. Their handlers will be stored in a vector
    let mut handles = vec![];

    for _ in 0..10 {
        let num_clone = Arc::clone(&id2);
        let handle = thread::spawn(move || {
            let mut num = num_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle)
    }

    // Wait until the end of each thread of the vector
    for handle  in handles {
        handle.join().unwrap();
    }

    println!("id2: {:?}", id2);
}