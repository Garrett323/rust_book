use std::fmt::Display;

pub fn run() {
    println!("Chapter 10 : Generics, Traits and Lifetimes");
    make_code_generic();
    generic_structs();
    traits();
    lifetimes();
}

fn make_code_generic() {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    // We need certain Traits for our implementation to work
    // Since i32 and Char implement these Traits is not required to state them
    // and we can use them explcicitly
    fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0]; // Copy trait makes a copy of the values here
                                   // we copy the list here to iterate over it
        for &item in list.iter() {
            if item > largest {
                largest = item; // using copy again
            }
        }
        largest
    }
    let list_i32 = vec![2, 4, 1, 8, 23];
    let list_chars = vec!['e', 'i', 'o', 'z', 't'];
    println!("{}", largest_i32(&list_i32));
    println!("{}", largest_char(&list_chars));
    // now generic
    println!("{}", largest(&list_i32));
    println!("{}", largest(&list_chars));

    // doesnt require the copy trait since there are no additional allocations
    // We only used references
    fn largest_ref<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = &item;
            }
        }
        largest
    }
    println!("{}", largest_ref(&list_i32));
    println!("{}", largest_ref(&list_chars));
}

struct Point<T> {
    x: T,
    y: T,
}
// in method definition
// impl methods for generic types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// only if we use generic types they need to be declared in the impl block
impl Point<f32> {
    fn distance_to_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}
struct PPoint<T, U> {
    x: T,
    y: U,
}

// we can mix and match generics as we please
// however this might increase code complexity and compile time
impl<T, U> PPoint<T, U> {
    // and use different generic in the method implementation
    fn mixup<V, W>(self, other: PPoint<V, W>) -> PPoint<T, W> {
        PPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// works also with Enums
fn generic_structs() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 0.1, y: 1.2 };
    println!("{}, {}", p1.x, p1.y);
    println!("{}", p1.x());
    println!("{}, {}", p2.x, p2.y);
    println!("{}", p2.distance_to_origin());
    // let p3 = Point { x: 0.1, y: 1 }; // wont work x is f32 and y is i32 but they have to be the
    // same type
    let p3 = PPoint { x: 0.1, y: 1 }; // now it works
    println!("{}, {}", p3.x, p3.y);

    let p1 = PPoint { x: 5, y: 10.4 };
    let p2 = PPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("{}, {}", p3.x, p3.y);
}

//Traits and generic behaviour
//Interfaces of other languades (not one-to-one but similar)
//Note : if neither the trait nor the struct is local to your extern crate
// => you cannot provide and implementation (And need to use wrappers instead)
#[allow(dead_code)]
fn traits() {
    // Every Type implementing this trait needs to provide a custom implementation
    // we just define in a trait which mehtods are implemented
    pub trait Summary {
        fn summarize(&self) -> String; // no default implementation
        fn summarize_or_print_default(&self) -> String {
            // here we give a default implementation
            // If no implementation id procided this will be called
            self.summarize(); // requires summerzie to be implemented by the calling struct
            String::from("Default text.")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // there is no implementation of this function hence we use the default implementation
    println!("1 new tweet: {}", tweet.summarize_or_print_default());

    // we can pass any type that implements the Summary trait
    // multiple traits can be required with the '+' syntax
    // fn notify(item: impl Summary + Display) { // the display trait isnt implemented, hence this
    // wouldnt compile
    fn notify(item: impl Summary) {
        println!("News: {}", item.summarize_or_print_default());
    }
    notify(tweet);
    // the above implementation of notify uses 'syntactic sugar'
    // fn notify_2<T: Summary + Display>(item: T) {
    // This syntax might get complex fast
    fn notify_2<T: Summary>(item: T) {
        // long syntax same as function above
        println!("News: {}", item.summarize_or_print_default());
    }

    // alternative syntax for trait bounds
    fn notify_3<T, U>(t: T, u: U)
    where
        T: Clone + Display, // same as above but easier to read
        U: Copy + Display,
    {
        println!("{}, {}", t, u);
    }
    // Traits can also be returned
    // which is useful but we dont care about right now
    // HOWEVER each function returning a trait can only return ONE specific type
    // it can only return and i32 or f32 not both even if both implement the copy trait
    // We can even implent traits for other traits
    // impl<T: Display> ToString for T {
    // --snip--
    // }
    // like this trait bounds are also valid here
}

fn lifetimes() {
    // lifetimes tell the compiler how long a reference is valid
    // they are used to solve issues like dangling references
    //let r; // var in outer scope                                          +--'a
    //{                                                                     |     +--'b
    //    let x = 5;                                                        |     |
    //    r = &x; // set to reference x                                     |     |
    //} // x is dropped here and no longer valid => r is now a null pointer |     +
    //  // but rust doesnt allow for null pointers => compiler time error   |
    //println!("{}", r);                                                    +
    //fixed version of the code above
    //now x lives long enough
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }
    // will not compile without lifetime param
    // Since we pass in a reference the lifetime is unknown to the compiler
    // we need to reassure that both references have the same lifetime
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz".to_string();
        result = longest(string1.as_str(), string2.as_str()); // since result lifes as long
                                                              // as s2 and s1 there is not a problem
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result); // because string2 doesnt live here anymore
    // this is not valid rust code
    //
    //wont compile returning a dangling reference rust will not let you do this
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     let res = "long".to_string();
    //     return &res;
    // }
    // structs may use lifetime annotations too
    #[allow(dead_code)]
    struct ImportantExerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldnt find '.'");
    #[allow(unused_variables)]
    let i = ImportantExerpt {
        part: first_sentence,
    }; // cannot outlive the owning variable (novel in this case)
       // Rust does some inference on lifetimes
       // so sometimes they are not required to be specified
       // ########################
       // check 'lifetime elision rules' for more information
       // ########################
    let s: &'static str = "Lives the enitre runtime of the program.";
    println!("{s}");
}
