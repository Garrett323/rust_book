use std::{thread, time::Duration};

#[allow(dead_code)]

pub fn run() {
    println!("Chapter 13: Iterators and Closures");
    closures();
    capturing();
    iterators();
}

fn iterators() {
    // Iterators are lazy they dont execute anything unless called by a consuming method
    let v = vec![1, 2, 3];
    // v.iter(); // does nothing
    // loops consume iterators
    for val in v.iter() {
        println!("Got {} value", val);
    }

    let mut v1_iter = v.iter();
    // next gets the next element of a iterators
    // calling next also consumes the iterator
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    let total: i32 = v1_iter.sum();
    println!("{}", total); // prints 0 snce we 'sonsumed' the iterato already

    // non-consuming methods
    let plus_one = v.iter().map(|x| x + 1);
    for val in plus_one {
        println!("Got {} value", val);
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating.. slowly..");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[allow(unused_assignments)]
fn capturing() {
    let mut x = 4;
    let equal_to_x = |z| z == x;
    println!("{}", equal_to_x(3));
    println!("{}", x);
    x = 3;
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // move lets the closure take
                                      // owenrship of a environment variable
    equal_to_x(vec![1, 2, 3]);
}

fn closures() {
    let simulated_user_value = 10;
    let simulated_random_number = 7;
    // takes a long time to run thats why its commented out
    // some_function_expensive(simulated_user_value, simulated_random_number);
    // some_function_better(simulated_user_value, simulated_random_number);
    // some_function_closure(simulated_user_value, simulated_random_number);
    some_function_cached(simulated_user_value, simulated_random_number);
}

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
            Some(x) => x,
        }
    }
}

#[allow(dead_code)]
fn some_function_expensive(user_value: u32, random_value: u32) {
    // many calls to an expensive function
    if user_value < 25 {
        println!(
            "Today do {} pushups!",
            simulated_expensive_calculation(user_value)
        );
        println!(
            "Next do {} situps!",
            simulated_expensive_calculation(user_value)
        );
    } else {
        if random_value == 3 {
            println!("Take a rest");
        } else {
            println!(
                "Today run {} minutes!",
                simulated_expensive_calculation(user_value)
            );
        }
    }
}

#[allow(dead_code)]
fn some_function_better(user_value: u32, random_value: u32) {
    // just precompute
    // but may not be needed
    let expensive_results = simulated_expensive_calculation(user_value);
    if user_value < 25 {
        println!("Today do {} pushups!", expensive_results);
        println!("Next do {} situps!", expensive_results);
    } else {
        if random_value == 3 {
            println!("Take a rest");
        } else {
            println!("Today run {} minutes!", expensive_results);
        }
    }
}

#[allow(dead_code)]
fn some_function_closure(user_value: u32, random_value: u32) {
    // now we are using closures nut still calling it a lot
    // we can write a wrapper to only compute is once
    let expensive_closure = |num| simulated_expensive_calculation(num);
    if user_value < 25 {
        println!("Today do {} pushups!", expensive_closure(user_value));
        println!("Next do {} situps!", expensive_closure(user_value));
    } else {
        if random_value == 3 {
            println!("Take a rest");
        } else {
            println!("Today run {} minutes!", expensive_closure(user_value));
        }
    }
}

fn some_function_cached(user_value: u32, random_value: u32) {
    // now we are using closures nut still calling it a lot
    // we can write a wrapper to only compute is once
    let mut expensive_closure = Cache::new(|num| simulated_expensive_calculation(num));
    if user_value < 25 {
        println!("Today do {} pushups!", expensive_closure.value(user_value));
        println!("Next do {} situps!", expensive_closure.value(user_value));
    } else {
        if random_value == 3 {
            println!("Take a rest");
        } else {
            println!("Today run {} minutes!", expensive_closure.value(user_value));
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // filter takes a closure returing a boolean and only keeps the items
    // that return true on the closure
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

#[allow(dead_code)]
struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
