mod gui;
//use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl gui::Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("calling draw() for SelectBox: width {} height {} options {}",
        self.width,
        self.height,
        self.options.len());
    }
}

fn main() {
    let screen = gui::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(gui::Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    screen.run();
}
