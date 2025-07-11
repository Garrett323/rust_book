pub fn add_two(x: i32) -> i32 {
    x + 2
}

// Documentation with examples get used as test => write good Documentation = better code
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_book::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// return unit value or some
// object that implement the error trait
pub fn execute(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.filename)?; // since we return a
                                                              // Result we move the error just up
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        // println!("{:?}", args); // first argument is the path of the binary
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = std::env::var("CASE_INSENSITVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|s| s.contains(query))
        .map(|s| s.trim())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    //////////////////////////// Chapter 15: Mock Object ////////////////////////////////////
    use std::cell::RefCell;
    struct MockMessenger {
        // sent_messages: Vec<String>, // we want to modify this field but cannot due to mutability
        // restirctions
        sent_messages: RefCell<Vec<String>>, // Hence we can use a RefCell
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // cannot change method signature due to trait requirements
            // self.sent_messages.push(message.to_string()); // trying to mutate immutable object =>
            // not allowed
            self.sent_messages.borrow_mut().push(message.to_string()); // since we know this
                                                                       // operation is safe we can
                                                                       // use a RefCell to access
                                                                       // the data
        }
    }

    #[test]
    fn it_send_an_over_75_percent() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

//////////////////////////// Chapter 15: Mock Object ////////////////////////////////////
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("Error: you are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent Warning: u used 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: u used 75% of your quota!");
        }
    }
}

//////////////////////////// Chapter 20: Multithreaded Server ////////////////////////////////////
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// # Penics
    ///
    /// The `new` function will panic!("c if n_thread is 0");
    pub fn new(number_of_threads: usize) -> ThreadPool {
        assert!(number_of_threads > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(number_of_threads);
        for id in 0..number_of_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending Terminate!");
        // we need this additional for loop to guarantuee that every worker got ther terminate match message when we call join
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down Worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(std::thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got job; executing.", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} terminating...", id);
                    break;
                }
            }
        }));
        Worker { id, thread }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}
