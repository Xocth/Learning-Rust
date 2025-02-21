#[derive(Debug)]
// Define a struct named `Rectangle` with two fields: `width` and `height`
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Implement a method named `area` for the `Rectangle` struct
    // This method calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        } // Method that checks if a rectangle can hold another rectangle
    }

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
} // Implementing an associated function for the `Rectangle` struct

fn main() {
    // Create an instance of `Rectangle` with width 30 and height 50
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the area of the rectangle using the `area` method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );                          

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    }; 

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect3 = Rectangle::square(25); // Using the associated function to create a square rectangle

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2)); // Using the `can_hold` method to check if a rectangle can hold another rectangle
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

}
