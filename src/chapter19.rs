static mut COUNTER: u32 = 0;

pub fn run() {
    println!("Chapter 19: Advanced Features");

    unsafe_rust();
    advanced_traits();
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

fn advanced_traits() {}
