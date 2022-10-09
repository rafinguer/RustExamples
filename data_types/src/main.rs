// Rust uses static typed. Data types checking is made in compilation time (not in execution time)
fn main() {
    // Integers
    // +---------+--------+----------+
    // | Length  | Signed | Unsigned |
    // +---------+--------+----------+
    // |   8-bit | i8     | u8       |
    // |  16-bit | i16    | u16      |
    // |  32-bit | i32    | u32      |
    // |  64-bit | i64    | u64      |
    // | 128-bit | i128   | u128     |
    // |    arch | isize  | usize    |
    // +---------+--------+----------+
    // * arch -> It depends of the architecture in which the program is executing
    println!("*** Integers ***");
    let signed_integer:i8 = -12;
    println!("signed_integer: {}", signed_integer);
    let unsigned_integer:u16 = 61945;
    println!("unsigned_integer: {}", unsigned_integer);

    // Number notations
    // You can use the '_' separator to facilitate the lecture of the data
    // The separator is always optional
    println!("\n----------");
    println!("*** Number notations ***");
    let decimal = 61_945;      // _ separator used to separate the thousands units (61,945)
    let hexadecimal = 0xff_ff; // _ separator used to separate bytes (#FFFF)
    let octal = 0o77;          
    let binary = 0b1010_1111;  // _ separator used to separate nibbles (10101111)
    println!("decimal: {}", decimal);         // 61945
    println!("hexadecimal: {}", hexadecimal); // 65535
    println!("octal: {}", octal);             // 63
    println!("binary: {}", binary);           // 175

    // Floating point numbers
    // - f32: 32 bits floating point
    // - f64: 64 bits floating point (by default (in inferences))
    println!("\n----------");
    println!("*** Floating point numbers  ***");
    let pi_value:f32 = 3.141592;        // Float32 (explicit typed)
    let salary = 68967.359;  // Float64 (by default)
    println!("pi_value: {}", pi_value);
    println!("salary: {}", salary);

    // Boolean (true or false)
    // - bool
    println!("\n----------");
    println!("*** Boolean ***");
    let is_married = false;
    println!("is_married: {}", is_married);

    // Character
    // - char
    // Note: We can use Unicode characters
    // See Unicode characters at https://unicode-table.com/es/
    println!("\n----------");
    println!("*** Character ***");
    let initial_name:char = 'R';
    println!("initial_name: '{}'", initial_name);
    let emoji = '\u{1F44D}';
    println!("emoji: '{}'", emoji);


    // --- COMPOUND TYPES ---

    // Strings
    // Colection of characters
    println!("\n----------");
    println!("*** Strings ***");
    let first_name = "Rafael";      // Native String by inference
    let last_name: &str = "Hernamperez";  // Native String as Static typed
    let job_title:String = "IT Architect".to_string();  // Object String. Allows size changes
    let mut city = String::new();
    city = "Madrid".to_string();
    println!("first_name: {}", first_name); 
    println!("last_name: {}", last_name); 
    println!("job_title: {}", job_title); 
    println!("city: {}", city); 


    // Arrays
    // Collection of several values with the same type
    println!("\n----------");
    println!("*** Arrays ***");
    let mut qualifications = [5.5, 6.7, 8.1, 7.2, 9.4];  // By inference
    println!("Third qualification: {}", qualifications[2]);  // Index starts at 0
    qualifications[2] = 7.7;  // Changing a value in the array
    println!("Third qualification: {}", qualifications[2]);  // Index starts at 0
    let initials:[char; 6] = ['R', 'N', 'E', 'C', 'G', 'L']; // Static typed -> [type; length]
    println!("Second initial: {}", initials[1]);  // Index starts at 0


    // Tuples
    // Collection of several values with the same or different types
    println!("\n----------");
    println!("*** Tuples ***");
    let tuple_example1 = ('R', 51, 62_000.00, true);  // inference
    let tuple_example2:(char,i32,f64,bool) = ('N', 20, 1_200.00, false);  // typed
    // Note: tuples cannot be shown using stantard formatted println! macro. You must specify special formatter {:#?}
    println!("tuple_example1: {:#?}", tuple_example1);
    println!("tuple_example2: {:#?}\n", tuple_example2);
    // Decomposing a tuple into several variables 
    let (initial, age, salary, is_married) = tuple_example1;
    println!("initial: {}", initial);
    println!("age: {}", age);
    println!("salary: {}", salary);
    println!("is_married: {}\n", is_married);
    // You also access to each value in the tuple, referring its index after a point
    println!("initial: {}", tuple_example2.0);
    println!("age: {}", tuple_example2.1);
    println!("salary: {}", tuple_example2.2);
    println!("is_married: {}", tuple_example2.3);







}
