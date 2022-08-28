use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //when we call join on a joinhandle it will wait for the thread to finish!
    //we can put it at the end so it wraps up at the end, or we can put this right after the join handle is created and allow it
    //to be ran first
    handle.join().unwrap();


    let v = vec![1, 2, 3];

    //need to implement move otherwise the borrowed value may be dropped while still holding a reference and the compiler will panic
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // oh no!

    handle.join().unwrap();
}
