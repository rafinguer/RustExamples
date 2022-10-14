// Macro = code that writes code (meta-programming)
// It can used to write DSL (Domain Specific Language)

// Basic Macro
macro_rules! three {
    () => {
        3
    };
}

macro_rules! next {
    ($x:expr) => {
        $x + 1
    };
}

// Medium complex macro
// This builds a Vector, with the values passed (as *)
macro_rules! collection {
    ($($x:expr), *) => {
        {
            let mut coll = Vec::new();
            $(
                coll.push($x);
            )*
            coll
        }
    };
}

// This macro crates a new funtcion
macro_rules! new_function {
    ($name:ident) => {   // ident = identifier
        fn $name() {
            println!("Hello, I'm {:?}()", stringify!($name)); 
        }
    };
}

fn main() {
    println!("Three = {}", three!());
    println!("Next = {}", next!(50));

    let my_coll = collection!(23, 12, 44, 233, 12, 52);
    println!("my_coll = {:?}", my_coll);

    // We can also use brackets ([]) instead of parentheses
    let jobs = collection!["Developer", "Analyst", "Architect", "Scrum Master", "Product Owner"];
    for job in jobs {
        println!("{}", job);
    }

    // Create a function named "hello()"
    new_function!(hello);
    hello();  // Call to the function created with new_function! macro
}
