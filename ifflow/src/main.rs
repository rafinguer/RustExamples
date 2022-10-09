fn main() {
    let qualification = 8;

    if qualification < 5 {
        println!("Not qualified");
    } else if qualification < 8 {
        println!("Sufficiently qualified");
    } else {
        println!("Excellently qualified");
    }

    let result = if qualification < 5 { "Not qualified" } else { "Qualified" };
    println!("Result: {}", result);
}
