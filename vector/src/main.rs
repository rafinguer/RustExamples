#[allow(unused_variables)]

// Vector is a collection of value of the same type
fn main() {

    // Implicit declaration
    let v1: Vec<i32> = Vec::new();

    // Explicit declaration, using macros
    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("Third value: {}", v2[2]);  // Accessing to a value
    println!("Fourth value: {:?}", v2.get(3).unwrap()); // Accessing to a value (is an Option)

    v2.push(20);   // Append a new value to the vector

    // Itering into the vector
    for value in v2.iter() {
        println!("- {} -", value);
    }

    /*
    for value in &mut vs {
        *i += 10;  // updating each value increasing by 10
    }

    for value in &v2 {
        println!("* {} *", value);
    }
    */


}
