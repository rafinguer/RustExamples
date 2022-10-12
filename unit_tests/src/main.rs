
// To run the tests, launch the following command:
// $ cargo test

fn main() {
    // assert!(expression)                   // Check if expression is true
    // assert_eq!(expressionA, expressionB)  // Check if expressionA is equal to expressionB
    // assert_ne!(expressionA, expressionB)  // Check if expressionA is not equal to expressionB
    
}

// This function returns the sum of two values
fn sum(a:i32, b:i32) -> i32 {
    a + b
}

// Rust identifies a function with #[test] as a test function
#[test]
fn check_sum() {
    assert_eq!(sum(2, 2), 4);
}

#[test]
fn check_non_sum(){
    assert_ne!(sum(2, 2), 3);
}