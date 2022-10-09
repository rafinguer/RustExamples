struct Human;
struct Cat;

// trait is similar to interfaces in other languages
trait Speak {
    fn say_hello(&self) -> String;
    fn language() -> String {  // Default behaviour
        "I don't speak amy lanaguage".to_string()
    }
}

impl Speak for Human {
    fn say_hello(&self) -> String {
        "Hello, friends".to_string()
    }

    fn language() -> String {
        "I speak human language".to_string()
    }
}

impl Speak for Cat {
    fn say_hello(&self) -> String {
        "Miau".to_string()
    }

    fn language() -> String {
        "I speak cat language".to_string()
    }
}

fn main() {
    let rafael = Human;
    let kitty = Cat;

    println!("Human says: '{}'", rafael.say_hello());
    println!("{}", Human::language()); // No self use. Must be called with <struct>::<method>
    println!("----------");
    println!("Cat says: '{}'", kitty.say_hello());
    println!("{}", Cat::language()); // No self use. Must be called with <struct>::<method>
}
