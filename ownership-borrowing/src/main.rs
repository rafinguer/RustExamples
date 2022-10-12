#[allow(unused_variables)]

// ownership -> is owner of
// borrowing -> borrow, request something 

// Rust has not garbage collection
// Each data has an ownership (owner)

// Stack memory:
//    - Data structure stack
//    - Fixed size
//    - Fast. Only its necessary to move the pointer
//    - Released when the end of scope is reached ({<scope>})
// Heap memory:
//    - Flexible
//    - Expensive to assign or recover data
//    - Released when no owner available 
fn main() {
    // Simple explanation
    // Initially, name is the owner of a initialized value
    // After, the owner is name_copy (moved)
    let name = String::from("Rafael"); 
    let name_copy = name;  // name value was borrowed to name_copy (moved). Assignement moves the ownership
    println!("{}", name_copy);  // name_copy is the new owbner of the value
    //println!("{}", name);  // This doesn't work, because it was moved to name_copy

    // Ownwership and borrowing with Stack memory. Native data are managed in the stack
    let mut age: i8 = 50;  // age variable is ownership of the value in the Stack
    increase_age(&mut age);  // Passing the reference
    println!("Age: {}", age);

    // Ownership and borrowing with Heap memory. Complex data are managed in the heap
    let mut job = String::from("Developer");
    change_job_title(&mut job, String::from("IT Architect"));
    println!("Job title: {}", job);

}

// This function receives the reference of the age variable (in Stack)
fn increase_age(years: &mut i8) {  // years parameter borrows the value of the age variable
    *years += 1   // changing the value in the reference (affects to age variable)
}

// This function receives the reference of the job_title variable (in Heap)
fn change_job_title(job_title: &mut String, new_job_tile: String) {
    *job_title = new_job_tile;
}
