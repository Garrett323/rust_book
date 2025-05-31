use crate::chapter15::List::{Cons, Nil};

pub fn run() {
    println!("Chapter 15: Smart Pointers");
    box_type();
}

#[allow(dead_code)]
enum List {
    // Cons(i32, List), // recursive definitions are not allowed
    Cons(i32, Box<List>), // Box is a pointer => known size at compile time
    Nil,
}

#[allow(unused_variables)]
fn box_type() {
    // When to use Box<T> type
    //     • When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    //     • When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    //     • When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    //  !!! stores data in heap NOT on the stack !!!
    let b = Box::new(8);
    println!("b = {}", b);
    let l = Cons(4, Box::new(Cons(8, Box::new(Nil))));
    // dereferencing
    let x = 5;
    let y = &x; // y is a reference to x
    assert_eq!(5, x);
    assert_eq!(5, *y); // the * operator tells the compiler to look at what is store at the
                       // references memory adress
    let x = 5;
    let y = Box::new(x); // Box can be used like a reference
    assert_eq!(5, x);
    assert_eq!(5, *y); // the * operator tells the compiler to look at what is store at the
                       // memory owned by the Box
    let y = MyBox::new(x); // custom smart pointer
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// !!! does not store data on the Heap !!!
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
