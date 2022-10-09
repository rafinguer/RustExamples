// Classical and standard struct
struct User {
    name: String,
    age: u8,
    email: String,
    is_active: bool,
}

// functions implemention to the struct
impl User {
    fn is_allowed(&self) -> bool {
        self.age >= 18
    }
}

// Tuple struct
// Used for short structures, defining only their data types (not their names)
struct Point(i32, i32, i32);

fn main() {
    let user = User {
        name: "Rafael".to_string(),
        age: 51,
        email: String::from("rafael@gmail.com"),
        is_active: true,
    };

    print_user(&user);

    print_line();

    let other_user = new_user("Edu".to_string(), "edu@yahoo.com".to_string());
    print_user(&other_user);

    print_line();
 
    let third_user:User = User {
        name: String::from("Nerea"),
        email: String::from("nerea@yahoo.com"),
        ..user  // This expression assumes the rest of properties as the 'user' instance 
    };

    print_user(&third_user);

    // Implementing a tuple struct
    print_line();

    let main_point = Point(12, 15, 2);
    println!("main_point: ({}, {}, {})", main_point.0, main_point.1, main_point.2);

}


// This function creates a new user
fn new_user(name:String, email:String) -> User {
    return User {
        //name: name,   // valid, althouth the names of property and value are redundant
        //email: email,
        name,   // if property and value has the same name, you can use only one name
        email,
        age: 0,
        is_active: true,
    };
}

// This function prints a print_line
fn print_line() {
    println!("----------");
}

// This function prints the values of an user
// Note: You must pass the parameter by reference
//       If you don't do this, the original values are moved to this function
//       and errors are produced when you try to used again the original value
//       One example is in the third_user, which uses 'user' object as '..user'
fn print_user(user: &User) {
    //println!("user = {:#?}", user);  // Does not work
    println!("Name = {}", user.name);
    println!("Age = {}", user.age);
    println!("Email = {}", user.email);
    println!("Is_Active = {}", user.is_active);
    println!("Is this user allowed? {}", user.is_allowed());
}

