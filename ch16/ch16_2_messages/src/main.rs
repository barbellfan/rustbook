use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc = Multiple Producer, Single Consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // can't do this. send() owns val, which prevents thread problems.
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
