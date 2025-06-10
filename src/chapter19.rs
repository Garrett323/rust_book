#[allow(dead_code)]

static mut COUNTER: u32 = 0;

pub fn run() {
    println!("Chapter 19: Advanced Features");

    unsafe_rust();
    advanced_traits();
    advanced_types();
    advanced_functions_and_closures();
    macros();
}

fn unsafe_rust() {
    println!("Unsafe rust does not turn off the borrow checker! but allows for");
    println!("1. Derefence raw pointers");
    println!("2. Call unsafe code");
    println!("3. Access or modify a mutable static variable");
    println!("4. Implement an unsafe trait");

    deferencing_raw_pointer();
    abstraction();
    foreign_functions();
    mutable_static();
    unsafe_traits();

    fn deferencing_raw_pointer() {
        #[allow(unused_imports)]
        use std::slice;
        // info about raw pointers
        // - Are allowed to ignore the borrowing rules by having both immutable
        //     and mutable pointers or multiple mutable pointers to the same location
        // - Aren’t guaranteed to point to valid memory
        // - Are allowed to be null
        // - Don’t implement any automatic cleanup
        //
        //-----------------------------------------------
        //
        // raw pointers can be created in safe code
        let mut num = 5;
        // these pointers are valid since we know they reference a valid piece of memory
        // we have a mutable and immutable pointer at the same time
        // this is not allowed with references in safe rust (wouldn't compile)
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // this is a random memory adress, which might not be valid
        let adress = 0x012345usize;
        let _r = adress as *const u32;

        // to dereference a raw pointer we need an usafe block
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        // this is a slice of arbitrary memory => this results in undefined behaviour
        // let _slice: &[u32] = unsafe { slice::from_raw_parts(_r, 10_000) };
    }

    fn abstraction() {
        use std::slice;
        unsafe fn dangerous() {} // marked as unsafe => can only be called in an unsafe block
                                 // can perform unsafe operations within function body without unsafe
                                 // block (entire body is unsafe)
        unsafe {
            dangerous();
        }

        let mut v: Vec<i32> = (1..=6).collect();
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        // this is a safe abstraction for an unsafe action
        fn my_split_at(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr(); // get raw pointer of the slice
            assert!(mid <= len); // this assert now insures that the .add call wil be

            // (&mut slice[..mid], &mut slice[mid..]) // does not compile 2 mut ref at same time
            // borrowing two exclusive parts of the same
            // slice is okay but rust does not understant
            // they are exclusive
            // we as programmer have to assure that manually
            //
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid), // unsafe since it requires the
                    // programmer to assure the ptr
                    // handed over is valid
                    // pointer arithmetic is inherintly usafe as well
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        let (a, b) = my_split_at(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    fn foreign_functions() {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("Absolute value of -3: {}", abs(-3));
        }

        // we can also make rust code accessible to other languages by defining it as below
        #[no_mangle] // such that we know the function name
        pub extern "C" fn call_from_c() {
            println!("Rust code");
        }
    }

    fn mutable_static() {
        static HELLO_WORLD: &str = "Hello"; // static vars always have a static memory adress
                                            // const does not guarante this
        println!("{HELLO_WORLD}");

        unsafe {
            COUNTER += 1;
        }
        unsafe {
            println!("count: {}", COUNTER); // dunno how to fix this warning
        }
    }

    fn unsafe_traits() {
        // Send and Sync are unsafe => compiler cant check if you didnt mess up in a multithreaded
        // scenario
        #[allow(dead_code)]
        unsafe trait Foo {
            // methods go here
        }

        unsafe impl Foo for i32 {
            // methods to implement
        }
    }
}

fn advanced_traits() {
    #[allow(dead_code)]
    pub trait Iterator {
        type Item; // type required by type but left arbitrary until specific implementation is
                   // provided
                   // like here an iterator returns a specific type but the trait is unaware of all possible
                   // types (user defined types are hard to know in advance)

        fn next(&mut self) -> Option<Self::Item>;
    }
    // so why not use generics?
    // we could provide multiple definitions
    // => we could have multiple implementations of the same trait on the same type for different
    // T
    //pub trait Iterator<T> {
    //    fn next(&mut self) -> Option<T>;
    //}

    // nothing prevents two traits to call their methods the same
    // and nothing prevents us from implementing both on a struct
    trait Wizard {
        fn fly(&self);
        fn name() -> String;
    }
    trait Pilot {
        fn fly(&self);
    }
    struct Human {}
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
        fn name() -> String {
            String::from("Gandalf")
        }
    }
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("Waving arms!");
        }

        fn name() -> String {
            String::from("Human")
        }
    }

    let h = Human {};
    // defaults to implementation on struct
    h.fly();
    // use mor specific syntax to call trait methods
    // this works when the method takes a self parameter
    Wizard::fly(&h);
    Pilot::fly(&h);
    // without self parameter
    println!("{}", Human::name()); // this is clear
                                   // println!("{}", Wizard::name()); // multiple type might implement this function, whihc do we
                                   // want?
    println!("{}", <Human as Wizard>::name()); // this way we can specifiy which implementation we
                                               // want to call
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        // requires type to implent the Display trait
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

fn advanced_types() {
    // type alias
    type Kilometer = i32;
    let x: i32 = 4;
    let y: Kilometer = 3;
    println!("{}", x + y); // since we use a type alias both are the same type under the hood
                           // this doesnt provide rigouros type checking as the newtype pattern
                           // but may be usefull to provide shorthand notations for long types

    // never type !
    #[allow(dead_code)]
    fn never_to_return() -> ! {
        // -- snip --
        loop {}
    } // indcates the compiler this function will never return
      // used for match arms that dont return makes the compiler aware that it never returns

    // let s1: str = "Hello";  // not valid rust code since the sizes are unknown at compile time
    // => both steings take a different amount of memory
    // let s2: str = "Im a str";
    // every type with known size at compiler time implemetns the trait SIzed
    // this is also inheritet for other types if all contained types implement Sized
    struct _Test {
        // -> inherits Sized Trait
        _t: u32,
    }
    #[allow(dead_code)]
    fn generics<T: ?Sized>(_t: &T) { // this syntax is only available for SIzed (reads may or not be
                                     // SIzed)
                                     // allows for types that dont implement Sized
    }
}

fn advanced_functions_and_closures() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    // we are passing a function pointer
    // implements all Fn traits and is in rust always passable if a closure is accepted
    // other languages (C) do not support this so when interfacing with other languages we always
    // need to pass a function pointer
    let answer = do_twice(add_one, 5);
    println!("answer: {}", answer);

    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    // constructors can also be passed as function pointer
    let _list_of_values: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // returning closures
    // doesnt compile, cant return traits (they dont have a known size at compile time)
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    //
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    println!("result from closure: {}", returns_closure()(2))
}

fn macros() {
    println!("This is a macro call");

    // #[macro_export] // means macro should be available whenever the crate is brought into scope
    macro_rules! vec {
        ( $( $x:expr )=>* ) => { // similar to match expressions => is the separator token
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    let v = vec![1=> 2=> 3];
    println!("{:?}", v);

    // procedural macros
    // syntax looks like this
    // use proc_macro;
    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();
}
