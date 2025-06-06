use crate::chapter15::List::{Cons, Nil};
use crate::chapter15::RcList::{RcCons, RcNil};
use std::rc::Rc;

pub fn run() {
    println!("Chapter 15: Smart Pointers");
    box_type();
    deref_coersion();
    dropping();
    reference_counting();
    println!(
        "The RefCell type enures Rusts borrow checker rules at runtime rather than compile time!"
    );
    println!("Check lib.rs to see an example use case.");
    reference_cycles(); // Memory leaks
    weak_references_and_trees();
}

#[allow(dead_code)]
enum List {
    // Cons(i32, List), // recursive definitions are not allowed
    Cons(i32, Box<List>), // Box is a pointer => known size at compile time
    Nil,
}

#[allow(dead_code)]
enum RcList {
    // Cons(i32, List), // recursive definitions are not allowed
    RcCons(i32, Rc<RcList>), // Box is a pointer => known size at compile time
    RcNil,
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

fn deref_coersion() {
    fn hello(name: &str) {
        println!("{}", name);
    }

    let name = String::from("Hans");
    hello(&name);
    let name = MyBox::new(String::from("Hans"));
    hello(&name);
}

struct CustomPointer {
    data: String,
}
impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping pointer with data: {}", self.data);
    }
}

fn dropping() {
    let _p = CustomPointer {
        data: String::from("Test"),
    };
    let _p2 = CustomPointer {
        data: String::from("Test2"),
    };
    // calling drop early is not allowed
    // _p.drop() // will give a compier error
    std::mem::drop(_p); // use this instead
    println!("Created pointer");
}

fn reference_counting() {
    // allows multiple owners for a single piece of memory
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let _b = Cons(3, Box::new(a)); // a gets moved into b => we are not able to use a
                                   // afterwards anymore

    // let c = Cons(7, Box::new(a)); // compiler error
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("rc after creating a {}", Rc::strong_count(&a));
    let _b = RcCons(3, Rc::clone(&a)); // a gets moved into b => we are not able to use a
    println!("rc after creating b {}", Rc::strong_count(&a));
    {
        let _c = RcCons(7, Rc::clone(&a)); // compiler error
        println!("rc after creating c {}", Rc::strong_count(&a));
    }
    println!("rc after c out of scope {}", Rc::strong_count(&a));
}

use crate::chapter15::CycleList::{Cons as CCons, Nil as CNil};
use std::cell::RefCell;

#[derive(Debug)]
enum CycleList {
    Cons((), RefCell<Rc<CycleList>>),
    Nil,
}

impl CycleList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            CCons(_, item) => Some(item),
            CNil => None,
        }
    }
}

fn reference_cycles() {
    let a = Rc::new(CCons((), RefCell::new(Rc::new(CNil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(CCons((), RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

fn weak_references_and_trees() {
    use std::rc::Weak;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
