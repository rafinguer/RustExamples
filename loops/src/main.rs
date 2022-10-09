fn main() {
    // Infinite loop
    // Only can be breaked with Ctrl+C
    /*
    loop {
        println!("Infinite loop...");
    }
    */

    // Controlling a loop with conditions and break sentence
    // If the number is even then it will be shown in console
    // If not, continue sentence jumps to the next iteration of the loop
    let mut number = 0;

    println!("--- LOOP ---");

    loop {
        if number < 100 {
            number += 1;
            if number % 2 != 0 {  // odd number
                continue;  // jumps to beginning of the loop
            } 
            println!("{}", number);  // event number
        } else {  // When number is 100, the loop ends
            break;
        }
    }

    println!("--- WHILE ---");

    // While loop
    while number > 0 {
        println!("{}", number);
        number -= 10;
    }

    println!("--- FOR ---");

    // For loop
    for i in 1..10 {
        println!("{}", i);
    }

    println!("--- FOR IN COLLECTION ---");
    
    // For loop evaluating a collection
    let fruits = ["Apple", "Orange", "Lemon", "Melon", "Avocado", "Peach"];

    //for fruit in fruits {  // implicit
    for fruit in fruits.iter() {  // explicit
        println!("{}", fruit);
    }



}
