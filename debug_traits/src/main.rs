use std::fmt;

// macros (println!, print!, format!...) use traits for printing (Debug or {:?}), Copy, Hash, Clone
// When you use {:?} in a println! you are using a Debug trait
// When you use {} in a println! you are using a Display trait
// Reference: https://doc.rust-lang.org/rust-by-example/trait/derive.html

#[derive(Debug)]  // if you don't use this, the line with {:?} will have errors
struct User {
    name: String,
    age: i8
}

// Implementing Display trait 
impl fmt::Display for User {
    // f is the formatter
    //fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} ({})", self.name, self.age)
    }
}

fn main() {
 let user = User {
    name: String::from("Rafael"),
    age: 51
 };

 println!("{:?}", user);  // Using Debug trait
 println!("{}", user);   // Using Display trait
}
