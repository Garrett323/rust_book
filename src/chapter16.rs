pub fn run() {
    println!("Chapter 16: Concurrent Programming");
    /*
     * Covered in this chapter:
     * -> How to creat threads
     * -> Message-passing
     * -> Shared-state
     * ->Sync and Send Traits
     */
    println!("Rust offer 'direct-threading' => every language thread is  an OS thread.");
    // this is done to keep the rust runtime as minimal as possible
    threading();
    move_closures_with_threads();
    message_passing();
}

fn threading() {
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        // added handle to access thread later
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    // not all of this output will be displayed
    // as soon as main finished all remaining threads get cleared up

    for i in 1..5 {
        println!("Hi form main thread {i}");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // wait for spawned thread to finish executing
                            // THIS IS A BLOCKING CALL
                            // NO OTHER CODE IS EXECUTED IN THIS THREAD UNTIL IT FINISHED
}

fn move_closures_with_threads() {
    use std::thread;

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // move kw is required to give ownership
        // to the closure
        println!("Here's is a vector: {:?}", v);
    });

    handle.join().unwrap();
    // println!("Here's is a vector: {:?}", v); // moved ownership to closure
}

fn message_passing() {
    use std::sync::mpsc;
    use std::thread; // mpsc => Multiple Producer Single Consumer

    // tx = transmitter, rx = receiver
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("HI");
        tx.send(val).unwrap();
        // println!("Sent: {}", val); // not valid, struct transferred ownership => we cannot use val
        // anmore here
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
