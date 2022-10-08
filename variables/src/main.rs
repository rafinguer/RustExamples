fn main() {
    //let x = 20;  // Using inferrence. It calculates its type automatically
    let x:u8 = 20;    // Using typed data
    // Note: Once the variable is initialized, its value is inmutable (cannot be changed)
    println!("x = {}", x);

    // Mutable variable
    let mut y = 20;   // using inference in a mutable variable
    println!("y = {}", y);
    y *= 2;   // Being mutable variable, its value can be changed
    println!("y = {}", y);

    // Shadowing variables
    // You can declared again a variable, with the same or another data type
    // It will be a new variable and is inmutable
    let x = 32;  // Remember: x was defined at the beginning
    println!("x = {}", x);
    let x = "My name is Rafael"; // Changing its data type to string
    println!("x = {}", x);

    // Constants
    // They cannot be defined by inference. The data type is mandatory
    // They always are inmutable. Their value is always fixed in their initialization.
    // They don't allow the shadowing
    const PI:f32 = 3.141592;
    println!("PI = {}", PI);

}
