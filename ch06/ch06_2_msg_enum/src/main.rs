fn main() {
    #[allow(unused)]
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let w = Message::Write(String::from("hello"));
    w.call();

    let q = Message::Quit;
    q.call();

    let m = Message::Move { x: 2, y: 3 };
    m.call();

    let c = Message::ChangeColor(5, 6, 7);
    c.call();
}

