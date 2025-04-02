pub fn run() {
    shadowing();
    data_allocation();
    loops_can_return();
    println!("{}", fib(1));
    println!("{}", fib(11));
}

fn shadowing() {
    let x = 2; // immutable variable
               // x = x + 1; // results in a error since x is immutable
    let x = x + 1; // uses shadowing (which is fine)
    let x: f32 = x as f32 / 2.0; // we are even allowed to change the type!
    println!("x: {x}");
}

fn data_allocation() {
    let a = [1, 2, 8]; // allocated on the STACK
    let b = vec![1, 2, 8]; // allocated on the HEAP
    println!("Stack allocated: {:?}", a);
    println!("Heap allocated: {:?}", b);
}

fn loops_can_return() {
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            // break acts like a return from this loop
            break counter * 2;
        }
    };
    println!("{res}")
}

// exercise function
fn fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n - 1 {
        (a, b) = (b, a + b);
    }
    b
}
