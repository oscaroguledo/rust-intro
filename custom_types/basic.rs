// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)] // implements the Debug trait. It prints the struct in a debug-friendly format, like:
struct Person {
    name: String,
    age: u8, //+ve integers only
    email: Option<String>,  // email is now optional
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let email = Some(String::from("fdfd@gmail.com"));
    let peter = Person { name, age, email };
    let john = Person { name: String::from("john"), age:99, email: None };
    // Print debug struct
    println!("{:?}", peter);
    println!("{:?}", john);
}