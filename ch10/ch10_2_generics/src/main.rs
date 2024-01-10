fn main() {
    generic_in_fn_def();
    generic_in_struct_def();
    generic_in_struct_def_2();
    generic_in_method_def();
    generic_mixup();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_in_fn_def() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

fn generic_in_struct_def() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.01, y: 4.01 };
    //let wont_work = Point { x: 5, y: 4.0 }; // types must match

    println!("integer.x: {}", integer.x);
    println!("integer.y : {}", integer.y);
    println!("float.x: {}", float.x);
    println!("float.y: {}", float.y);
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn generic_in_struct_def_2() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.01, y: 4.01 };
    let integer_and_float = Point2 { x: 5, y: 4.01 };

    println!("integer.x: {}", both_integer.x);
    println!("integer.y : {}", both_integer.y);
    println!("both_float.x: {}", both_float.x);
    println!("both_float.y: {}", both_float.y);
    println!("integer_and_float.x: {}", integer_and_float.x);
    println!("integer_and_float.y: {}", integer_and_float.y);
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_in_method_def() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2 = Point { x: 10.0, y: 20.0 };
    println!("distance: {}", p2.distance_from_origin());
}

struct Point3<X1,Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic_mixup() {
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 {x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
