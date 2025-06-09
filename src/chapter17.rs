#[allow(dead_code)]

pub fn run() {
    println!("Chapter 17: OOP");
    encapsulation();
    inheritance();
    state_pattern();
    state_pattern_rustified();
}

#[allow(dead_code)]
fn encapsulation() {
    mod avgc {
        use std::vec;

        pub struct AveragedCollection {
            list: Vec<i32>, // still private
            average: f64,   // still private
        }

        impl AveragedCollection {
            pub fn new() -> AveragedCollection {
                AveragedCollection {
                    list: vec![],
                    average: 0.0,
                }
            }
            pub fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            pub fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                // this function is private to the struct and connot be
                // accesed by external users
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }
    }

    let avg = avgc::AveragedCollection::new();
    // avg.average; // cannot be accesed outside of the module avg
    avg.average(); // this is allowed however
}

fn inheritance() {
    // Rust does not provide this functionality
    // but it uses Trait objects instead
    pub trait Draw {
        fn draw(&self);
    }

    // Using generics <T> will gnerate a struct for each specific type but using Trait objects
    // we can pass all types that implement this trait
    pub struct Screen {
        // dyn => dynamically sized => not all objects passed have the same sized
        // but provide the same functionality
        // since we dont know all the objects passed in at compile we need to
        // perfrom 'dynamic dispatch' // looking up the implementation at runtime
        // this adds a performance penalty
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[allow(dead_code)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code that draw a button
        }
    }

    // when implementing a gui library this will allow the user to extend the library and pass
    // their own components (when dont know of at the time we write the library)
    let screen = Screen {
        components: vec![Box::new(Button {
            width: 30,
            height: 25,
            label: String::from("Button"),
        })],
    };
    screen.run();

    // bot all objects can be turned into trait objects
    // two relevant rules
    // 1.   do not return Self
    // 2.   No generics allowed
    // trait Clone {
    // fn clone(&self) -> Self
    // }
    // fn test_cloned(object: dyn Cloned) {
    //      // this function wont compile since cloned violates one of the rules
    // }
}

fn state_pattern() {
    //    1. A blog post starts as an empty draft.
    //    2. When the draft is done, a review of the post is requested.
    //    3. When the post is approved, it gets published.
    //    4. Only published blog posts return content to print, so unapproved posts
    //    can’t accidentally be published.
    //
    let mut post = Post::new();

    post.add_text("I ate salad for lunch");
    assert_eq!("", post.content());
    println!("{}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("{}", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch", post.content());
    println!("{}", post.content());

    struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::from(""),
            }
        }

        fn add_text(&mut self, content: &str) {
            self.content.push_str(content);
        }
        fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }
        fn request_review(&mut self) {
            // we use if let and Option.take() to invalidade the old state
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        fn approve(&mut self) {
            // we use if let and Option.take() to invalidade the old state
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _: &'a Post) -> &'a str {
            ""
        }
    }
    struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            // do not go from Draft to Approved
            self
        }
    }
    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            // do not go from Draft to Approved
            Box::new(Published {})
        }
    }
    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            // do not go from Approved to any state before
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            // do not go from Draft to Approved
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

fn state_pattern_rustified() {
    // idiomatic rust code relies more on the type system
    // to encode values
    // here we are unable to access the content unless
    let mut post = Post::new();
    // returns DraftPost which does not have a content method => we get a compiler error instead of
    // the empty string

    post.add_text("I ate salad for lunch");
    // assert_eq!("", post.content());
    // println!("{}", post.content());

    let post = post.request_review();

    let post = post.approve();
    println!("{}", post.content());

    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    impl Post {
        fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    // this struct is private and cannot be accessed outside the pubiic api
    struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}
