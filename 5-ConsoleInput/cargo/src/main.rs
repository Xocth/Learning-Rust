use std::io; // imports the input/output library

fn main() {
    println!("Hello, world!");

    let mut input = String::new(); // creates a mutable string

    io::stdin().read_line(&mut input).expect("failed to read line"); // reads the input from the user // &mut is a mutable reference
    // this allows us to modify the value of the variable
    // .expect is used to handle any errors that may occur

    println!("You typed: {}", input); // prints the input
}
