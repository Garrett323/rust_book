#[allow(unused)]
pub fn run() {
    println!("Chapter 9");
    "There are two types of errors, Recoverable handled with the Result<T, E> type";
    "and unrecoverable errors (usually results of bugs) which can be done with the panic! macro.";
    "adding 'panic = abort' under changes the behaviour of panic! being either handled by rust itself or the os";
    "having rust handle the error results in a larger binary";
    example_panic();
    recoverable_error();
    propagating_errors();
}

fn example_panic() {
    // panic!("This crashes the program");
    let _v = vec![1, 2, 3];
    // _v[99]; // accesing memory which is not valid also crashes the program.
    // C would return the memory stored at this address (if the OS allows), which might not crash
    // but could introduce bugs
}

use core::panic;
use std::{
    fs::File,
    io::{self, Read},
};
#[allow(unused_variables)]
fn recoverable_error() {
    let f = File::open("hello.txt"); // doesnt exist => results in Err
                                     // Using match to handle the error
                                     // let f = match f {
                                     //     Ok(file) => file,
                                     //     Err(error) => {
                                     //         panic!("Problem opening the file: {:?}", error);
                                     //     }
                                     // };
                                     // matching on different errors
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
    // more idiomatic version of the same code using closures (lambdas)
    // which is a concept being discussed in chapter13
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    let f = File::open("hello.txt").unwrap(); // shortcut to get value or panic
    let f = File::open("hello.txt").expect("Same as above but with message!");
}

fn propagating_errors() {
    fn read_username_verbose() -> Result<String, io::Error> {
        // passing errors up the chain might make it easier to handle them
        let f = File::open("hello.txt");
        // maybe it fails here, so we return the error
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut name = String::new();
        // maybe it fails here, so we return the error
        match f.read_to_string(&mut name) {
            Ok(_) => Ok(name),
            Err(e) => Err(e),
        } // last expression gets returned automatically if not followed by an ;
    }
    // same as above
    // ? operator does the same as the above match statements
    // propagates errors up
    // but calls 'from' first (Trait to convert errors to return type)
    // using ? requires to return a Result<T, E> type
    // the main() function is restrictued on what types of results may be returned
    fn read_username_simple() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut name = String::new();
        f.read_to_string(&mut name)?;
        Ok(name)
    }

    // error occured in another function but is hanlded here
    let name = match read_username_verbose() {
        Ok(name) => name,
        Err(e) => panic!("Coulnd't read the username: {:?}", e),
    };
    println!("{name}");
    let name = match read_username_simple() {
        Ok(name) => name,
        Err(e) => panic!("Coulnd't read the username: {:?}", e),
    };
    println!("{name}");
}
