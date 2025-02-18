fn main() {
    let mut x = 5; // mut is short for mutable and allows the value to be changed
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBERS: u32 = 100_000; // constants are always immutable
    println!("The number of subscribers is: {}", SUBSCRIBERS);

    let x = 5;

    let mut x = 6; // shadowing allows you to change the type of the value
    println!("The value of x is: {}", x);

    // Integers

    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)
    let f: u8 = 255; // u8

    // Floating-point numbers

    let g: f32 = 2.0; // f32
    let h: f64 = 3.0; // f64

    // addition

    let sum: i32 = 5 + 10;

    // subtraction

    let difference: f64 = 95.5 - 4.3;

    // multiplication

    let product = 4 * 30;

    // division

    let quotient: f64 = 56.7 / 32.2;

    // remainder

    let remainder: i32 = 43 % 5;

    // Boolean

    let t = true;
    let f: bool = false;

    // Character

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound types

    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);


    my_function();
    arguments_function(5);

}

fn my_function() {
    println!("Function called!");
}

fn arguments_function(x: i32) {
    println!("The value of x in the function is: {}", x);
}