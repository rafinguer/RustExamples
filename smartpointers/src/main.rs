#[allow(unused_variables)]

fn main() {
    // References
    let x = 25;
    let y = &x;

    // References counters (like String or Vec<T>)

    // Smart pointers, usually implemented with structs, but implementing traits Deref and Drop
    // With Deref trait you can use smart pointer instances working with references and smart pointers
    // With Drop trait you can execute a given code once the smart pointer is out of the scope

    // Box<T> is a kind of smart pointer
    // It indicates to Rust how many memory use in compile time
    let a = 100;  // Uses stack memory
    let b = Box::new(100);  // Uses heap memory

    // Linked lists (recursive resource)
    // (value1, node1) -> (value2, node2) -> (value3, null)
    enum List {
        Node(i32, Box<List>),
        None,
    }

    /*
    let node3 = List::Node(10, Box::new(List::None));
    let node2 = List::Node(15, Box::new(node3));
    let node1 = List::Node(25, Box::new(node2));
    */

    // Short version
    use List::*;

    let node3 = Node(10, Box::new(None));
    let node2 = Node(15, Box::new(node3));
    let node1 = Node(25, Box::new(node2));
}
