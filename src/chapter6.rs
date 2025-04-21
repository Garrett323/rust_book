/*
 * HAVING FUN WITH ENUMS
 * and pattern matching too
 */

pub fn run() {
    create();
    with_data();
}

enum MyEnum {
    A,
    B,
}

// enums also can contain information
enum MyEnumButBetter {
    A(u32),
    B(i32),
    C { x: i32, y: i32 }, // this version contains an "anonymous struct"
    D(f32, f32),          // this version contains an tuple
}

impl MyEnumButBetter {
    fn call(&self) {
        // method on enum
    }
}

fn create() {
    let a = MyEnum::A;
    let b = MyEnum::B;
    test(a);
    test(b);
}

fn with_data() {
    let a = MyEnumButBetter::A(30);
    let b = MyEnumButBetter::B(-30);
    let c = MyEnumButBetter::C { x: 20, y: 20 };
    let d = MyEnumButBetter::D(1.0, 1.0);
    a.call();
    b.call();
    c.call();
    d.call();
    // accessing the inner value using match

    match a {
        MyEnumButBetter::A(x) => println!("{x}"),
        _ => {
            println!("Not a valid entry!");
        }
    }
    // if let statement kinda works like a match but can be used if match is to verbose
    if let MyEnumButBetter::B(-30) = b {
        println!("Minus thirty!");
    }
    if let MyEnumButBetter::D(a, b) = d {
        println!("x:{a}, y:{b}");
    }
    if let MyEnumButBetter::D(a, b) = c {
        println!("x:{a}, y:{b}");
    } else {
        println!("Wrong pattern!");
    }
}

fn test(_: MyEnum) {}
