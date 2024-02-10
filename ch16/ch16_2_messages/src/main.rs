use std::sync::mpsc;

fn main() {
    // mpsc = Multiple Producer, Single Consumer
    let (tx, rx) = mpsc::channel();
}
