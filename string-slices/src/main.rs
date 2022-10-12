#[allow(unused_variables)]

// string is a sequence of characters u8 (UTF8)
// string is saved on the heap memory. It can be mutable
// string slice is a stack, reference to the heap, string literal
// string slice is inmmutable
// char is a character u32 (Unicode)
fn main() {
    let name_str = String::from("Rafael");  // String (heap)
    let surname_slice = "Hernamperez";  // string literal (hardcode). Is a reference (&str)
    
    let mut surname_str = surname_slice.to_string();  // conversion from string slice to string
    surname_str.push('z');  // Adding z to the end (it must be mutable)

    let name_slice = &name_str[..4]; // conversion from string to string slice (using its memory reference)

    println!("string: {}{}", name_str, surname_str);
    println!("string slice: {}{}", name_slice, surname_slice);
}
