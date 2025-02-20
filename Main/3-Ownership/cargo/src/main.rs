fn main() {

    // Ownership rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.


    fn a() {
        let x: &str = "hello";
        let y: i32 = 22;
        
        b();
    }

    // x and y are dropped here

    fn b() {
        let x: String = String::from("hello");

    }

    // x is dropped here

    // function b is left first, then function a is left
    // last in first out

    { // s is not valid here, it’s not yet declared
        let s: &str = "hello"; // s is valid from this point forward
    } // this scope is now over, and s is no longer valid

    {
        let s: String = String::from("hello"); // heap memory is allocated
        // automatically freed when s goes out of scope
    }

    let x: i32 = 5;
    let y: i32 = x; // x is copied to y

    let s1: String = String::from("hello");
    let s2: String = s1; // move (not shallow copy) s1 to s2

    // println!("{}", s1); // error: value borrowed here after move

    let s1: String = String::from("hello");
    let s2: String = s1.clone(); // deep copy s1 to s2

    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
    // println!("{}", s); // error: value borrowed here after move

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
} // some_string is returned and moves out to the calling function

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // a_string is returned and moves out to the calling function

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
} // s is returned and moves out to the calling function


