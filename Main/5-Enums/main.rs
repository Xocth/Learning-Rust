enum IpAddrKind { // Defining an Enum
    V4(String), 
    V6(String), // Variants
    V4_1(u8, u8, u8, u8), // Variants
}

enum Message {
    Quit, // Variants
    Move { x: i32, y: i32 }, // Variants
    Write(String), // Variants
    ChangeColor(i32, i32, i32), // Variants
}

impl Message {
    fn some_function() {
        println!("Let's Get Rusty!");
    }
} // Implementing a method on an Enum

struct IpAddr { // Defining a struct
    kind: IpAddrKind,
    address: String,
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct

struct ChangeColorMessage(i32, i32, i32); // tuple struct


fn main() {
    let four = IpAddrKind::V4; // Using an Enum
    let six = IpAddrKind::V6; 

    

    let localhost = IpAddrKind::V4(String::from("127.0.0.1")); // Using an Enum with a value

    let localhost = IpAddrKind::V4_1(127, 0, 0, 1); // Using an Enum with a value


    // Option Enum

    enum Option<T> {
        Some(T),
        None,
    } // Defining an Option Enum that takes a generic type T

    let some_number = Some(5); // Using an Option Enum
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // Using an Option Enum


}

fn route(ip_kind: IpAddrKind) {} // Defining a function that takes an Enum as a parameter

fn value_in_cents(coin: Coin) -> u32 { // Defining a function that takes an Enum as a parameter
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
} // Using a match expression to get the value of a Coin Enum



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
} // Defining an Enum