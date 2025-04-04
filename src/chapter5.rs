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
    let user2 = User {
        name: String::from("Alice"),
        ..user1
    };

    let c = Color(3, 2, 2);
}

struct Color(i32, i32, i32); //shorthand creation of struct without named fields

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
