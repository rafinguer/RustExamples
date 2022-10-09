// A closure is a function defined inline
fn main() {
    // Variable defined as a function
    let next = next_value;
    println!("next(20): {}", next(20));

    // Basic closure
    /* 
    // Implicit version of closure. Defining types
    let previous:i32 = |number:i32| -> i32 {
        number - 1   // Returns the previous value
    };
    */

    // Explicit version of closure. Using inference
    let previous = |number| {
        number - 1
    };

    println!("previous(20): {}", previous(20));

    // Closure with several arguments
    let sum = |number1, number2| {
        number1 + number2
    };

    println!("sum(10, 12): {}", sum(10, 12));
}

// simple function
fn next_value(number: i32) -> i32 {
    number + 1   // returns the next value
}
