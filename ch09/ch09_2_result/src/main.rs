use std::fs::File;
use std::io::{ErrorKind, self, Read};

fn main() {
    //_recoverable_errors_with_result();
    //_matching_on_different_errors();
    //_unwrap();
    //_expect();
    propagating_errors();
    propagating_errors_shortcut();
    propagating_errors_chain();
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

fn _unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();

    println!("greeting file: {:?}", greeting_file);
}

fn _expect() {

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    println!("greeting file: {:?}", greeting_file);
}

fn propagating_errors() {
    let r = read_username_from_file();

    println!("Result: {:?}", r);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn propagating_errors_shortcut() {
    let r = read_username_from_file_shortcut();

    println!("Result from shortcut: {:?}", r);
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn propagating_errors_chain() {
    let r = read_username_from_file_chain();

    println!("Result from chain: {:?}", r);
}

fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
