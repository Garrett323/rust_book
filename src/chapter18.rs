#[allow(dead_code)]

pub fn run() {
    println!("Chapter 18: Patterns and Matching");
    all_places_patterns();
    pattern_syntax();
    deconstruction();
    match_guards();
}

fn all_places_patterns() {
    let value = Some(3);
    match value {
        Some(x) => println!("{}", x),
        None => println!("None"),
    };

    let favorit_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let experssions can be used seemlessly with normal if-else blocks
    // and do not depend on one another
    if let Some(color) = favorit_color {
        println!("{color}");
    } else if is_tuesday {
        println!("Its tuesday!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple");
        } else {
            println!("Using red");
        }
    } else {
        println!("Blue");
    }
    // if let isnt exhaustive at compile time while match is
    let mut stack = vec![1, 2, 3];
    while let Some(x) = stack.pop() {
        println!("{x}");
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // formally a let statement is also using Patterns
    // let PATTERN = EXPRESSION
    // function parameters are the statement
    fn print_point(&(x, y): &(i32, i32)) {
        println!("x:{x}, y:{y}");
    }
    let p = (3, 4);
    print_point(&p);
}

fn pattern_syntax() {
    let x = 1;
    // literals
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
    // named vars
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50!"),
        // creates new y that shadows outer y // it prints 5 here
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x= {:?}", x),
    }

    println!("at the end x: {:?} y: {:?}", x, y);
    // multi pattern
    let x = 1;
    match x {
        1 | 3 => println!("one or three"),
        2 => println!("two"),
        _ => println!("anything"),
    }
    // ranges
    let x = 1;
    match x {
        1..=3 => println!("one to three"),
        _ => println!("anything"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn deconstruction() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 2, y: 3 };
    let Point { x: a, y: b } = p;
    assert_eq!(2, a);
    assert_eq!(3, b);
    // shorthand notation for above
    // but vars have the same names as struct fields
    let Point { x, y } = p;
    assert_eq!(2, x);
    assert_eq!(3, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neihter axis: {x},{y}"),
    };

    // deconstruction of nested values
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
    // ignoring parts of a pattern
    // ignoring entire values
    let (x, _, z) = (80, 19, 7); // ignoring middle value
    println!("x:{}, z:{}", x, z);
    let _i = 0;
    // starting a var name with _ makes the compiler ignore it
    let s = Some(7);
    if let Some(_x) = s {
        // s gets moved into _x
        println!("{}", _x);
    }
    // println!("{}", s); // not possible s got moved
    let s = Some(7);
    if let Some(_) = s {
        // some code here
    }
    println!("{:?}", s);
    let (x, .., y) = (4, 12, 9, 0, 90);
    println!("ignored all but the first value: {x} and last: {y}");
    // let ( .., second, ..) = (4, 12, 9, 0, 90); // ambiguous deconstruction => compiler error
    let (.., _last) = (4, 12, 9, 0, 90); // be care how the deconstruction really works out
}

fn match_guards() {
    // match guards are additional if statements which also needs to be matched for an match arm to
    // match
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{x}"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    enum Message {
        Hello { id: i32 },
    }
    // Using the @ operator allows to test for a pattern and bin the var at the same time
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
