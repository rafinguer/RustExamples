#[allow(unused_variables)]

// lifetime is related to the time of memory references
// It can assure that one memory place is still valid for a reference

// The longest lifetime is for all application (like a "static")
const VALUE: i32 = 128;  // lifetime static

fn main() {
    let a;  // lifetime for main function scope
    
    {  // internal local scope
        let b = 10;   // lifetime for local scope
        a = b * 3; 
    }  // b is released

    println!("a: {}", a);   // b is dead

    // c has lifetime in main(). Uses VALUE (lifetime static) and 100 (lifetime call during the call)
    let c = something(&VALUE, &100);
    println!("c: {}", c);   // params and return lifetimes are not valid here

    // string literal is static, due to it is defined and saved into the binary file
    let name : &'static str = "Rafael";  // string literal of harcoded string literal
    println!("name: {}", name);
}

// Simple example
// Rust compiler, automatically added the lifetime in compile time
// The lifetime defines the life time of the parameters and returns of the function
// In this case, the lifetime is for all the life of the execution of the function
// In this example, a and b receives a life time
// It's also very important the lifetime of the variables used as parameters when invoke the function
//
// fn something(a:i32, b:i32) -> i32 {
// fn something<'a>(param_a: &'a i32, param_b: &'a i32) -> i32 {  // param_a, param_b have the sema lifetime('a)
fn something<'a, 'b>(param_a: &'a i32, param_b: &'b i32) -> i32 {
    return param_a + param_b;
}
