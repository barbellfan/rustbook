use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //_recoverable_errors_with_result();
    //_matching_on_different_errors();
    unwrap();
}

fn _recoverable_errors_with_result() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match  greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("greeting file: {:?}", greeting_file);
}

fn _matching_on_different_errors() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    println!("greeting file: {:?}", greeting_file);
}

fn unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();

    println!("greeting file: {:?}", greeting_file);
}
