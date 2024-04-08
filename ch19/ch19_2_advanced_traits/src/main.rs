fn main() {
    default_generic_type_params_and_op_overload();
    default_generic_type_params_and_op_overload_2();
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
