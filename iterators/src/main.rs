// An iterator is a public trait from the standard Rust library

fn main() {
    // Standard iterator
    let list = [1, 2, 3, 4, 5];

    // iter() function returns an iterator over an slice or a vector
    for i in list.iter() {
        println!("{}", i);
    }

    // Customized iterator
    println!("----------");
    let mut c = Counter::new();
    c.next();  // count = 1
    c.next();  // count = 2
    c.next();  // count = 3
    c.next();  // count = 4
    c.next();  // count = 5
    let i = c.next();  // *** END OF COUNTER ***

    match i {
        Some(count) => println!("Iterator: {}", c.count),
        //None => println!("Iterator: None"),
        //None => (),   // Do nothing
        None => println!("Iterator has reached the end"),
    }
}

struct Counter {
    count: i32,
    max: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0, max: 5 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    // fn next(&mut self) -> std::option:Option<<Self as std::iter::Iterator>::Item> {
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
