use std::thread;
use std::time::Duration;

fn main() {
    
    // spawn opens a new thread, using a closure function
    thread::spawn( || {
        println!("I'm the first thread");
    });

    println!("I'm the principal thread");

    thread::sleep(Duration::from_millis(1000));

    // Second thread using spawn function as join handler
    let join_handle =     thread::spawn( || {
        println!("I'm the second thread");
    });
    
    println!("I'm the principal thread, again");

    // Waits for execution to complete
    join_handle.join().unwrap();

    println!("Both threads are finished");

    // Using the same variable by two threads
    let name = String::from("Rafael");
    let name_clone = name.clone();

    println!("Hello, {}", name);

    // With move we allow to the thread take borrowed the variable
    // name is now owned by the thread and it cannot be used again
    // If not, Rust warns about this function could finish before the main thread
    let join_handle2 =     thread::spawn( move || {
        println!("Welcome to the hell, {}", name_clone);
        thread::sleep(Duration::from_millis(3000));
    });

    // Waits for execution to complete
    join_handle2.join().unwrap();

    println!("Goodbye, {}", name);
}
