#[allow(unused_variables)]

use std::fs::File;
use std::io::ErrorKind;

// Rust doesn't use exceptions, like Java
// Instead, Rust uses the Result<T, E> struct for recoverable errrors
// In case of unrecoverable errors (critical errors), uses the panic! macro
fn main() {
    // Recoverable errors:
    //    Result<T, E> struct. T = Result. E = Error
    let file = File::open("inexistent_file.txt");

    // file is not an file object, but it is a Result type
    match file {
        Ok(the_file) => read_file(the_file),
        /* 
        Err(error) => { 
            println!("Error [{}]: {}", error.kind(), error);
        } */
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("Error: File not found"),
            // _ => println!("Unknown error"),
            other_error => println!("Unknown error: {}", error.kind()),
        }
    }

    // This line, if error, launches a panic directly. If ok, returns a File object
    // let file2 = File::open("inexistent_file.txt").unwrap();

    // The following line, if error, launches a panic directly with an error message.
    // If ok, returns a File object
    // let file2 = File::open("inexistent_file.txt").expect("File 'inexistent_file.txt' is nos found");


    // Unrecoverable errors:
    //    panic!("error message");
    // For backtrace (more detail), execute the app using the env variable RUST_BACKTRACE=1:
    //    RUST_BACKTRACE=1 cargo run

    /*
    panic!("Demo panic error: Don't worry. It's a test");
    */
}

fn read_file(file: File) {
    println!("The file exists and will be read");
}
