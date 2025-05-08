#[allow(dead_code)]

pub fn run() {
    // default construction
    let mut user1 = User {
        email: String::from("example.com"),
        name: String::from("bob"),
        active: true,
        sign_in_count: 9,
    };
    println!("{}", user1.active);
    user1.sign_in_count = 10; // requires the struct to be mutable
                              // Make a new struct
                              // and reuse all the values of user1 using the ..user1 shorthand
    #[allow(unused)]
    let user2 = User {
        name: String::from("Alice"),
        ..user1
    };

    #[allow(unused)]
    let c = Color(3, 2, 2);
    let r = Rectangle {
        width: 10,
        height: 2,
    };
    println!("{:?}", r); // print in debug mode
    println!("{}", r.area()); // print in debug mode)
    #[allow(unused)]
    let sq = Rectangle::square(20);
}

#[allow(unused)]
struct Color(i32, i32, i32); //shorthand creation of struct without named fields

#[allow(unused)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // assiciated function
    // also known as static method in object oriented languages
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
