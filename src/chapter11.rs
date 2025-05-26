#[allow(dead_code)]

pub fn run() {
    println!("Chapter 11: Writing Tests");
    println!("Run 'cargo test' to evaluate this chapter!");
    println!("Tests are by default run in parallel, using 'cargo test -- --test-threads=1' makes them run sequential.");
    println!("To show std::out add 'cargo test -- --show-output'");
    println!("To run irgnored tests run 'cargo test -- --ignored'");
    println!("'cargo test [name]' will run all tests containing [name]");
    println!("*********************************************************************************");
    println!("Unit tests are contained in this file, while integration tests are in 'tests/integration_test.rs'");
    println!("To run integration tests run 'cargo test --test [name]'");
    println!("This only works if we have a src/lib.rs file");
    println!("*********************************************************************************");
}

#[allow(dead_code)]
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)] // tells the compiler to only compile and run when 'cargo test' is called
mod tests {
    // this allows to access all the code in this module
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // testing for errors
    // this test reports success when the code inside panics
    #[test]
    #[should_panic(expected = "This is supposed to fail")] // and the specific error message was given
    fn fail() {
        panic!("Wrong message");
    }

    #[test]
    #[ignore] // this test is ignored unless specifically requested
    fn can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn custom_message() {
        let s = "custom";
        assert!(false, "This is a {} message!", s);
    }

    #[test]
    fn return_result_instead_of_panic() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
