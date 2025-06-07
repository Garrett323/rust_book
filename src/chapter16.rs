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
    shared_state();
    println!("The Send trait lets values be transferred between threads safely (Rc does not implement Send => cannot be transferred)");
    println!("The Sync trait lets values be accessed between threads safely (Rc does not implement Sync => cannot be accessed by more than one thread)");
    println!("While these can be implemented manually, all types that are composed of types with these trait automatically inherit them");
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
    use std::time::Duration;

    // tx = transmitter, rx = receiver
    let (tx, rx) = mpsc::channel();
    // needs to be copied here cuase its moved later on
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("HI");
        tx.send(val).unwrap();
        // println!("Sent: {}", val); // not valid, struct transferred ownership => we cannot use val
        // anmore here
        let vals = vec![
            String::from("from"),
            String::from("the"),
            String::from("Thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    // we can have multiple sender
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("here"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // let received = rx.recv().unwrap(); // blocking receive call use revc_try for non blocking
    for received in rx {
        // is iterable for receiving multiple items
        println!("Got: {}", received);
    }
}

fn shared_state() {
    use std::sync::{Arc, Mutex}; // Arc => atomic Rc
                                 // like Rc but safe to use in a multithreaded env
    use std::thread;

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // blocking call (waits until we have
                                         // the lock)
        *num = 6;
        // MutexGuard gets out of scope => the lock is dropped
    }
    println!("m = {:?}", m);

    //////////////////////////////////////////////////////////77

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        // without the Arc this is not safe => only one owner of the mutex is possible
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
