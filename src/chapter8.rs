// Collections!
// These collections (or sometimes called containers) store data of variable size
// Hence they are allocated on the heap
//
#[allow(dead_code)]

pub fn run() {
    println!("Chapter8");
    vectors();
}

enum Container {
    A(i32),
    B(String),
}

#[allow(unused)]
fn vectors() {
    // requires type annotation, since initial vector is empty
    let v: Vec<i32> = Vec::new();
    // type can be inferred from values inside the initial vector
    let v = vec![1, 2, 3];
    // No type annotation required since we fill the vector at compile time with values of a given
    // type, hence type is inferred
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // accessing elemnts
    // without safety checks
    println!("first {}", &v[0]);
    // with safety checks
    match v.get(2) {
        Some(element) => println!("third {}", element),
        None => println!("There is no third element!"),
    };
    for e in &v {
        println!("{e}");
    }
    for e in &mut v {
        *e += 10; // needs to derefence to access the value
        println!("{e}");
    }

    // vectors can only store one type.
    // To work around this we can use a Enum
    let v = vec![Container::A(2), Container::B(String::from("test"))];
} // <- v gets out of scope here, all contained values are dropped at this point
