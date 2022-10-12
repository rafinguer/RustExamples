#[allow(unused_variables)]

use std::rc::Rc;

fn main() {
    // Reference counted smart pointer - One value with many owners
    // We use Rc instead of Box because Box doesn't allow multiple owners for the same value
    enum List {
        Node(i32, Rc<List>),
        None
    }

    use List::*;

    // node0 -> 
    //          node2 -> node3 -> null
    // node1 ->
    
    // node0 and node1 are owners of node2
    let node3 = Node(10, Rc::new(None));
    let node2 = Node(5, Rc::new(node3));
    let node2_rc = Rc::new(node2);
    println!("Strong references count: {}", Rc::strong_count(&node2_rc));
    let node1 = Node(27, Rc::clone(&node2_rc));  // deep clone
    println!("Strong references count: {}", Rc::strong_count(&node2_rc));
    let node0 = Node(32, Rc::clone(&node2_rc));  // deep clone
    println!("Strong references count: {}", Rc::strong_count(&node2_rc));
}
