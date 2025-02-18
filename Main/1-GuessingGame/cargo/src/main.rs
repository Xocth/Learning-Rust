use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
    
        println!("You guessed: {}", guess);
    
        // match is a control flow construct that compares a value against a series of patterns and then executes code based on which pattern matches
        match guess.cmp(&secret_number) { // cmp is a method that can be called on anything that can be compared
            Ordering::Less => println!("{}", "Too small!".red()), 
            Ordering::Greater => println!("{}", "Too big!".red()), // red() is a method that can be called on a string to color it red
            Ordering::Equal => {
                println!("{}", "You win!".green()); // green() is a method that can be called on a string to color it green
                break; // break the loop
            }
        }
    }


}
