use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //mpsc stands for "multiple producer, single consumer"
    //means that it has multiple sending ends, but only one recieving end
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        //send takes ownership of passed value. After this function call val is no longer in this thread!
        tx.send(val).unwrap();
    });

    //recv will block the current thread and make it wait until a value is recieved
    //try_recv will try at that point in excution and not wait, returning a Result<T,E>
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //sending multiple values and seeing the receiver wait
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //here treating rx2 as an iterator and will wait until the channel is closed
    //closed = tx closing || rx closing
    for received in rx2 {
        println!("Got: {}", received);
    }

    //below we are using multiple senders and one reciever
    let (tx3, rx3) = mpsc::channel();

    let tx31 = tx3.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx31.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx3 {
        println!("Got: {}", received);
    }
}