#[allow(dead_code)]  // Omit alerts about non used code

/* 
// This enum is generic for data types that can be None (null)
// Rust already implements this enum
// This avoids the null pointer exception
enum Option<T> {  // T delegate any type (generic type)
    Some(T),      // Only can be this type (T)
    None,         // or None (null)
}
*/

// Struct with fields using option
struct User {
    name: String,
    age: Option<i8>,
}

impl User {
    fn get_age(&self) -> Option<i8> {
        self.age
    }
}

// Struct using default values with option
struct Employee {
    name: String,
    salary: Option<f32>,
}

impl Employee {
    fn get_salary(&self) -> f32 {
        // This method returns the default valur for this type in case of None
        //self.salary.unwrap_or_default()

        // We can control if the valu is none
        if self.salary.is_none() {
            25000.00   // returns default value (mininum salary)
        } else {
            self.salary.unwrap()  // forces type and value 
        }
    }
}

fn main() {
    let mut name : Option<String> = None;
    println!("Name: {:#?}", name);  // None
    name = Some(String::from("Rafael"));
    println!("Name: {:#?}", name);  // Some("Rafael")

    match name {   // "Name: Rafael"
        None => println!("Anonymous"),
        Some(name) => println!("Name: {}", name),
    }

    println!("----------");

    // Usen option with User
    let user = User {
        name: "Rafael".to_string(),
        age: Some(51),
    };

    let the_age = user.get_age();

    match the_age {
        Some(the_age) => println!("Age: {}", the_age),
        // _ => (),  // optional. "In other cases do nothing"
        None => println!("No age provided"),
    }

    println!("----------");

    let employee = Employee {
        name: "Rafael".to_string(),
        salary: None
    };

    println!("Employee: {}", employee.name);
    println!("Salary {}", employee.get_salary());

}
