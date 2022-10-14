// The result of an async function is a future object
// await launches the async function and returns the type of the function (String)
// An async call to the function must be in other async function
// main() function cannot be async
// To launch an async function we must use a runtime
// we need ue tools provided by the community
// Async Runtimes are tools used in order to execute async applications
// These libraries provide Reactor and Executors
// Async runtimes from the community:
//    - Tokio: https://tokio.rs
//    - async-std: https://async.rs
//    - smol
// Note: see crate packages for each runtime (added tokio in Cargo.toml)

// Tokio runtime allows to main be a sync function
#[tokio::main]
async fn main() {
    let content = read_file("myfile.txt").await;
    println!("Content: {}", content);
}

// Simulate reading of file
async fn read_file(path_file: &str) -> String {
    println!("Reading from file {}...", path_file);
    std::thread::sleep(std::time::Duration::from_secs(2));
    "This is the content of the file".to_string()
}
