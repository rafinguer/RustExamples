#[allow(unused_variables)]

use std::ops::Deref;

fn main() {
    // Drop trait -> what to do when the instance ends its scope
    let x = 5;
    let y = MyBox::new(x);

    if *y == 5 {  // double defering
        println!("(y) Value is 5");
    }

    // You cannot invoke directly the Drop trait
    let z = MyBox::new(20);
    println!("drop(z)");
    drop(z);  // This function is from Rust core. Drops the MyBox instance 

    println!("End of main() (y will be dropped)");
}  // End of scope. y instance will invoke its Drop trait

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

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Hi. This is the Drop code. Bye bye")
    }
}