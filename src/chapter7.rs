#[allow(dead_code)]

pub fn run() {
    front_of_house::hosting::add_to_waitlist();
}

fn test() {} // outer

mod front_of_house {
    fn test() {} // inner

    pub mod hosting {
        pub fn add_to_waitlist() {
            // super is used to access namespace above the current
            super::test(); // inner
            super::super::test(); // outer
        }
        #[allow(unused)]
        fn seat_at_table() {}
    }
    mod serving {
        #[allow(unused)]
        fn take_order() {}
        #[allow(unused)]
        fn serve_order() {}
    }
}
