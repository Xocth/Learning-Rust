struct User { // Defining a struct 
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn main() {
    let mut user1 = User { // Creating an instance of the struct
        email: String::from("bogdan@mail.com"),
        username: String::from("bogdan123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username; // Accessing the fields of the struct
    user1.username = String::from("wallace123");

    let user2 = build_user(
        String::from("kyle@mail.com"),
        String::from("kyle123")); // Using a function to create a struct


    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("james123"),
        ..user2 // Using the rest of the fields from another struct
    };


    // Tuple structs


    struct Color(i32, i32, i32); // Tuple struct

    struct Point(i32, i32, i32); 

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    ); // Using a function to calculate the area of a rectangle

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    ); // Improved readability using a tuple

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    }; // Creating an instance of a struct
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((rect2.width, rect2.height))
    ); // Improved readability using a struct

}

struct Rectangle { // Defining a struct
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
} // Function that calculates the area of a rectangle

fn area2(dimensions: (u32, u32)) -> u32 { 
    dimensions.0 * dimensions.1
} // Function that calculates the area of a rectangle using a tuple

fn build_user(email: String, username: String) -> User { // Function that returns a struct
    User {
        email, // Field init shorthand
        username, 
        active: true,
        sign_in_count: 1,
    }
}
