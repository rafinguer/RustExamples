#[allow(unused_variables)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;  // multiple producer single consumer

// How to communicate data betwwen threads?
// Using channels in order to communicate messages between threads

fn main() {

    // tx = transmitter (sender) - rx = receiver
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();  // tx & tx2 transmit to the same channel

    // Using the same variable by two threads
    let name = String::from("Rafael");

    println!("Hello, {}", name);

    // With move we allow to the thread take borrowed the variable
    // name is now owned by the thread and it cannot be used again
    // the same issue occurs with tx object
    // If not, Rust warns about this function could finish before the main thread
    thread::spawn( move || {
        println!("Welcome to the hell, {}", name);
        thread::sleep(Duration::from_secs(3));

        // send message to main thread
        let message = String::from("My message");
        tx.send(message).unwrap();

        // Send multiple messages to the main thread
        for count in 1..5 {
            let message = String::from(format!("Counter: {}", count));
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });

    // Second thread
    thread::spawn( move || {
        // Send multiple messages to the main thread
        for count in 100..110 {
            let message = String::from(format!("Counter: {}", count));
            tx2.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receive one message from the secondary thread
    let the_message = rx.recv().unwrap();
    println!("Received message: {}", the_message);

    // receive multiple messages from the secondary thread
    for received_message in rx {
        println!("Received message: {}", received_message);
    }


}
