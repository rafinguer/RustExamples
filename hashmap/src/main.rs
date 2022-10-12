#[allow(unused_variables)]

use std::collections::HashMap;

// Collection of structures under a key
fn main() {
    let mut qualifications: HashMap<String, u32> = HashMap::new(); // It can be inferred
    qualifications.insert("Red Team".to_string(), 100);
    qualifications.insert("Green Team".to_string(), 120);
    qualifications.insert("Blue Team".to_string(), 150);

    println!("Red Team: {:?}", qualifications.get("Red Team"));
    println!("Green Team: {:?}", qualifications.get("Green Team"));
    println!("Blue Team: {:?}", qualifications.get("Blue Team"));

    // With insert you can modify an existing qualification
    // Is you use entry, if the qualification is not existing it will be created,
    // in other case, it will be updated
    qualifications.entry(String::from("Green Team")).or_insert(175);
    println!("----------");

    //for (key, value) in qualifications.iter() {   // also valid
    for (key, value) in &qualifications {
        println!("{:?} {}", key, value);
    }

}
