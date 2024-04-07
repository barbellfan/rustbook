use std::any::type_name;

fn main() {
    raw_pointers_from_references();
    raw_pointer_from_thin_air();
    dereferencing_raw_pointers_in_unsafe_block();
    calling_unsafe_code();
    probably_will_crash();
    calling_c();
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

use std::slice;

fn calling_unsafe_code() {
    let mut vals: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let middle = 4;

    println!("{:?}", split_at_mut(&mut vals, middle));
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn probably_will_crash() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    println!("getting values unsafely");
    let _values: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
    println!("done getting values unsafely");

    // this doesn't print anything, and neither does the next line.
    // causes a crash.
    //println!("an unsafe value: {:?}", values[0]);
    //println!("done printing unsafe value");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn calling_c() {
    unsafe {
        println!("The absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
