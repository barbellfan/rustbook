use std::any::type_name;

fn main() {
    raw_pointers_from_references();
    raw_pointer_from_thin_air();
    dereferencing_raw_pointers_in_unsafe_block();
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

fn raw_pointer_from_thin_air()
{
    let address = 0x012345usize;
    let r = address as *const i32;

    // you can print the type
    print_type_of(&r);
    // you can print the address
    println!("r: {:?}", r);
    /*
    unsafe {
        // you can't print the value tho. panic:
        // misaligned pointer dereference: address must be a multiple of 0x4 but is 0x12345
        println!("&r: {:?}", *r);
    }
    */

    /*
    // OK, pick an address that's a multiple of 4
    let address2 = 0x012340usize;
    let r2 = address2 as *const i32;

    unsafe {
        // this doesn't print anything.
        println!("&r2: {:?}", *r2);
    }
    // this line never gets reached. the unsafe line must crash or something.
    println!("done with raw_pointer_from_thin_air()");
    */
}

fn dereferencing_raw_pointers_in_unsafe_block() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}
