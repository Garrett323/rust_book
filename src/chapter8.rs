// Collections!
// These collections (or sometimes called containers) store data of variable size
// Hence they are allocated on the heap
//

#[allow(unused)]
pub fn run() {
    println!("Chapter8");
    vectors();
    strings();
    hash_maps();
    // exercises
    println!("Exercises");
    let v = vec![2, 4, 5, 6, 9, 12, 4];
    println!("Mean of {:?} is {}", &v, mean(&v));
    println!("Median of {:?} is {}", &v, median(&v));
    println!("Mode of {:?} is {}", &v, mode(&v));
    let mut departments = HashMap::new();
    println!(
        "{:?}",
        text_interface("Add Sally to Marketing".to_string(), &mut departments)
    );
    println!(
        "{:?}",
        text_interface("Marketing".to_string(), &mut departments)
    );
}

#[allow(dead_code)]
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
    // let a = &v[100]; // doesnt exist => throws an error
    let a = v.get(100); // returns an option (None if the item doesnt exist)
    let first = &v[0];
    // v.push(Container::A(12)); // cannot do mutable borrow while there is a non mutable borrow
} // <- v gets out of scope here, all contained values are dropped at this point

#[allow(unused)]
fn strings() {
    // strings are implemented as collection of bytes (chars in C terms)
    // they are UTF-8 encoded
    "This is a string literal, its stored in the binary of the program";
    let mut s = String::new(); // allocates memory for a new empty string
    let s = "String literals are of type str, hence they have all methods of that type".to_string();
    let s = String::from("Another way to init a string");
    let mut s = String::from("Hello,");
    s.push_str(" World"); // strings can be updated if they are mutable
    s.push('!'); // push takes single bytes (chars) as parameter
    let s1 = String::from("A");
    let s2 = String::from("B");
    let s = s1 + &s2; // note s1 has been moved and can no longer be used
    let s1 = String::from("A");
    let s2 = String::from("B");
    let s = format!("{} - {}", s1, s2); // another way to format strings
                                        // let h = s[0]; // rust does not support indexing for strings
                                        // This is because of the UTF-8 encoding
    let hello = "Здравствуйте";
    // let answer = &hello[0]; // should be З but is in fact 208
    // (becasue in utf-8 208 alone is not a valid character)
    // indexing should be done in O(1) but this not really possible with strings hence another
    // minor reason to now allow it
    let answer = &hello[0..2]; // however slicing is allowed, if the range is valid
                               // [0..1] will throw an error since the first char is 2 bytes long
    println!("{answer}");
    println!("Chars:");
    for c in hello.chars() {
        // iterate over chars
        print!("{c} ");
    }
    println!("\nBytes:");
    for b in hello.bytes() {
        // iterate over raw byte values
        print!("{b} ");
    }
    println!("");
}

// hashmaps are not common in systems programming, hence there is less build in support
// and we need to manually import them
use std::collections::HashMap;
#[allow(unused)]
// by default the hashmap uses a cryptographically strong hashing algorithm
// this can be changed in performance critical applications
fn hash_maps() {
    // dictonaries in some other languages
    // I assumestrings people know how they work and this is just rust specific
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 8);
    scores.insert(String::from("Red"), 12);
    // another way to init hashmaps
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 8];
    // type annotation are need to tell the compiler the target data type
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // while constructing a hashmap types the implement the copy trait are copied (e.g. i32) while all other
    // types are moved and hence invalid afterwars (e.g. Strings)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 8);
    scores.insert(String::from("Red"), 12);
    match scores.get(&String::from("Blue")) {
        None => println!("Not a valid entry"),
        Some(x) => println!("Score: {x}"),
    };
    // iterationg over maps can be done using tuple deconstruction
    scores.insert(String::from("Blue"), 40);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 8);
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(30); // not inserted since we already have a blue entry
    println!("{:?}", scores);
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mutable reference to the inserted value
        let count = map.entry(word).or_insert(0);
        *count += 1; // * <- dereference operator to modify the given value
    }
    println!("{:?}", map);
}

/************************************************************************************************************************
* exercises
************************************************************************************************************************/
#[allow(unused)]
fn mean(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for e in numbers {
        sum += e;
    }
    sum as f32 / numbers.len() as f32
}

#[allow(unused)]
fn median(numbers: &Vec<i32>) -> f32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2] as f32
    } else {
        (sorted[sorted.len() / 2] + sorted[sorted.len() / 2 - 1]) as f32 / 2.0
    }
}

#[allow(unused)]
fn mode(numbers: &Vec<i32>) -> i32 {
    let mut lookup: HashMap<_, _> = HashMap::new();
    let mut max_count = 0;
    let mut mode = match numbers.get(0) {
        Some(x) => *x,
        None => panic!("Please provide an array with elements"),
    };
    for e in numbers {
        let count = lookup.entry(e).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = *e;
        }
    }
    return mode;
}

#[allow(unused)]
fn text_interface(
    command: String,
    departments: &mut HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    if command.contains("Add") {
        let pattern: Vec<&str> = command.split(" ").collect();
        let name = pattern[1];
        let dep = pattern.last().unwrap();
        let employees = departments.entry(dep.to_string()).or_insert(Vec::new());
        employees.push(name.to_string());
        employees.sort();
        return None;
    } else {
        if departments.contains_key(&command) {
            return Some(departments[&command].clone());
        } else {
            return None;
        }
    }
}
