pub fn run() {
    println!("Chapter12: Building a CLI application");
    println!("This is a recap of all the things done in previous chapters..");
    println!("I highly suggest working throught this chapter on your own.");
    mini_main();
}

// we need to accept cmd-line arguments
// like 'cargo run searchstring file.txt'
fn mini_main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}
