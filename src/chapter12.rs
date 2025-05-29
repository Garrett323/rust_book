/*
* 'The organizational problem of allocating responsibility for multiple tasks to
* the main function is common to many binary projects. As a result, the Rust
* community has developed a process to use as a guideline for splitting the
* separate concerns of a binary program when main starts getting large. The
* process has the following steps:
*
* • Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
* • As long as your command line parsing logic is small, it can remain in main.rs.
* • When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
*
* The responsibilities that remain in the main function after this process
* should be limited to the following:
*
* • Setting up any other configuration
* • Calling the command line parsing logic with the argument values
* • Handling the error if run returns an error
* • Calling a run function in lib.rs'
* [direct quote from the book]
*/

// since we moved the code to lib.rs we need to import it again
use rust_book::{execute, Config};

#[allow(dead_code)]

pub fn run() {
    println!("Chapter12: Building a CLI application");
    println!("This is a recap of all the things done in previous chapters..");
    println!("I highly suggest working throught this chapter on your own.");
    println!("*************************************************\n");
    mini_main();
}

// we need to accept cmd-line arguments
// like 'cargo run searchstring file.txt'
fn mini_main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    if let Err(e) = execute(config) {
        eprintln!("Application error {}", e);
        std::process::exit(1);
    };
}
