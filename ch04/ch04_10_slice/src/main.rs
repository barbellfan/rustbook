fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let s2 = String::from("hello");
    let slice1 = &s2[0..2];
    let slice2 = &s2[..2];

    println!("slices are equal: {} {}", slice1, slice2);

    let s3 = String::from("hello");
    let len = s3.len();
    let slice3 = &s3[3..len];
    let slice4 = &s3[3..];
    println!("These are also equal: {} {}", slice3, slice4);

    let s4 = String::from("hello");
    let len2 = s4.len();
    let slice5 = &s4[0..len2];
    let slice6 = &s4[..];
    println!("Again, equal: {} {}", slice5, slice6);
}

