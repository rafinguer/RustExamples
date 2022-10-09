fn main() {
    // if-let
    let edad:Option<i8> = Some(51);

    /* 
    // This fragment with match is verbose and add useless code (line with _ => ())
    match edad {
        Some(value) => println!("Age: {}", value),
        _ => (),
    }
    */

    
    if let Some(value) = edad {  // This is more clear and useful
        println!("Age: {}", value);
    }

    // while-let
    let mut unread_msg = Some(20);
    
    /*
    // This fragment is verbose and the code is large and hard
    loop {
        match unread_msg {
            Some(value) => {
                if value > 0 {
                    println!("You have {} messages pending to read", value);
                    unread_msg = Some(value-1);
                } else {
                    println!("No messages pending to read");
                    unread_msg = None;
                }
            }
            None => { break; }
        }
    }
    */

    while let Some(value) = unread_msg {  // This is more clear and useful
        if value > 0 {
            println!("You have {} messages pending to read", value);
            unread_msg = Some(value-1);
        } else {
            println!("No messages pending to read");
            unread_msg = None;
        }
    }

}
