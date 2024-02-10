use std::{sync::mpsc, thread};

fn main() {
    // mpsc = Multiple Producer, Single Consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
