fn main() {
    let x = 2; 
    let y = x;
    // x and y is stored in stack
    example(); // function call is stored in stack

    let string = String::from("Hello, world!"); // string is stored in heap
    // pointer to heap is stored in stack
    
} // x, y, example() is removed from stack in reverse order

fn example() {
    let a = "true";
    let b = false
}