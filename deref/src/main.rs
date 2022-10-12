#[allow(unused_variables)]

use std::ops::Deref;

fn main() {
    // Defer trait -> deferencing memory references
    let x = 5;
    let y = &x;
    let z = &y;

    if x == 5 {
        println!("(x) Value is 5");
    }

    // Value of y is the memory reference to x
    // Accessing to its value it is not allowed
    // You must defer the memory reference with Defer trait using * symbol (pointer)
    if *y == 5 {
        println!("(y) Value is 5");
    }

    // z has a memory reference to y, which is the memory reference to x
    if **z == 5 {  // double defering
        println!("(z) Value is 5");
    }

    // Defering using Box<T>
    let n = Box::new(x);

    if *n == 5 {  // double defering
        println!("(n) Value is 5");
    }

    // Defering using MyBox
    let i = MyBox::new(x);

    if *i == 5 {  // double defering
        println!("(i) Value is 5");
    }
}

// Creating our own Box version to demonstrate the Defer trait
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}