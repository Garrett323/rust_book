pub fn run() {
    // varibales with known size at compile time which are stored on the stack default to copy
    primitive_copy();
    // varibales with unknown size at compile time which are stored on the heap default to move
    // => we need to call clone explicitly
    complex_copy();
    // same holds for function calls
    let x = 5;
    simple_function(x);
    println!("outside the function: {x}");
    let s = String::from("Hello");
    complex_type(s);
    //println!("outside the function: {s}"); // results in an error s has been moved
    let s = dangle();
    // we can pass it as reference tho
    complex_type_ref(&s);
    println!("outside the function: {s}");
    // you can have multiple read only references
    let r1 = &s;
    let r2 = &s;
    // mutable refs have to be unique!
    // let r3 = mut &s;
}

fn primitive_copy() {
    let x = 5;
    let y = x; // copies the value of x into y
    println!("x: {x}, y: {y}");
}

fn complex_copy() {
    let x = String::from("Hello");
    //let y = x; // moves x into y => x is no longer valid!!
    let y = x.clone(); // copies x
    println!("x: {x}, y: {y}");
}

fn simple_function(x: u32) {
    println!("inside the function: {x}");
}

fn complex_type(s: String) {
    println!("inside the function: {s}");
}

fn complex_type_ref(s: &String) {
    println!("inside the function: {s}");
}

fn dangle() -> String {
    let s = String::from("hello");
    //&s // cannot return pointer to variables local to this function
    // this is due to the rust borrow system
    // The pointer wouldn't be owned
    s
}

fn first_word(s: &String) -> usize {
    // store string as array of chars
    let bytes = s.as_bytes();
    // iterate with an index, we use a ref for the byte
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
