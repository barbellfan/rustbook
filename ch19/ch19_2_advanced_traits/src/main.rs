fn main() {
    default_generic_type_params_and_op_overload();
    default_generic_type_params_and_op_overload_2();
    disambiguation();
    disambiguation_2();
    supertraits();
}

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_generic_type_params_and_op_overload() {
    assert_eq!(
        Point { x: 1, y:0 } + Point { x: 2, y: 3},
        Point { x: 3, y: 3}
    );
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn default_generic_type_params_and_op_overload_2() {
    let mi = Millimeters(100);
    let me = Meters(2);

    let added = mi.add(me);
    println!("meters + millimeters: {}", added.0);
}

fn disambiguation() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("Waving arms furiously");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);

    // these two lines do the same thing.
    person.fly();
    Human::fly(&person);
}

fn disambiguation_2() {
    trait  Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name()); // returns 'Spot'
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // returns 'puppy'
}

fn supertraits() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    let p = Point{ x: 5, y: 8 };
    println!("Point:");
    p.outline_print();
}
