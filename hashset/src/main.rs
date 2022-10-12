use std::collections::HashSet;

// a hashset is a collection of unique values
fn main() {
    //let user_ids: HashSet<u32> = vec![12, 23, 33, 14, 35, 53];
    let mut user_ids: HashSet<u32> = HashSet::new();   // Unknown size of collection
    let id: u32 = 38;
    user_ids.insert(id);
    user_ids.insert(12);
    user_ids.insert(23);
    user_ids.insert(33);
    user_ids.insert(12);  // This won't be added to the collection (is repeated)
    user_ids.remove(&12);  // Remove this item from the collection

    for id in user_ids.iter() {  // 33, 23, 38
        println!("id: {}", id);
    }

    // SET OPERATIONS
    let ids: HashSet<u32> = HashSet::from_iter(vec![34, 35, 36,  7, 38]);

    // Union: unique items in the both collections
    let union = HashSet::union(&user_ids, &ids);
    println!("union: {:?}", union);  // 7, 38, 36, 34, 35, 23, 33

    // Difference: items in the first set but not in the second
    let difference = HashSet::difference(&user_ids, &ids);
    println!("difference: {:?}", difference);  //33, 23

    // Intersections: items matched on both sets
    let intersection = HashSet::intersection(&user_ids, &ids);
    println!("intersection: {:?}", intersection); // 38

}
