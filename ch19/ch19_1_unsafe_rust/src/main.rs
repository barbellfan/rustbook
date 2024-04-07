use std::any::type_name;

fn main() {
    raw_pointers_from_references();
}

fn raw_pointers_from_references()
{
    let mut num = 5;

    // no unsafe needed
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // not in the book, but I wanted to print out the type
    print_type_of(&num); // prints i32
    print_type_of(&r1); // prints *const i32
    print_type_of(&r2); // *mut i32
}

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}
