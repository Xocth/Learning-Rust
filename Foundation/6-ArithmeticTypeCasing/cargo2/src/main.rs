use std::io;

fn main() {

    println!("Enter an input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    let int_input: i64 = input.trim().parse().unwrap(); // parse string to integer

    println!("You entered: {}", int_input);

}
