// cargo is a tool that organizes the code and facilitates the compilation
// $ cargo new cargo_hello_world
//
// The file 'Cargo.toml' defines the dependencies
// The code is located on 'src' folder
// To check the code before the build:
// $ cargo check
//
// To build the application:
// $ cargo build
//
// To build and run the application:
// $ cargo run
//
// In both cases, the compilation will located on 'target' folder.
// The executable file will be located con target/debug folder
//
// You can generate a release with:
// $ cargo build --release
//
// The executables will be located on 'release' folder
// The size of the executable will be optimized
fn main() {
    println!("Hello, world!");
}
