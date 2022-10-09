// main() function. The program starts in this function
fn main() {
   say_hello();
   salutation("Rafael");
   print_square(6);
   
   let result_sum:i64 = sum(256, 9320);
   println!("Result of sum: {}", result_sum);

   let my_number = {
    let x = 45;
    let y = 32;
    x*y
   };

   println!("My number is {:?}", my_number);

}

// Function without parameters
fn say_hello() {
    println!("Hello, world!");
}

// Function with parameters
fn print_square(number:i64) {
    println!("Square of {} = {}", number, number*number);
}

// String parameter. Parameter passed by reference
fn salutation(name: &str) {
    println!("Hello, {}", name);
}

// Function that returns a value
fn sum(a:i64, b:i64) -> i64 {
    //return a+b;  // return is optional
    a+b  // If last statament is an expression, the function returns the result
}
