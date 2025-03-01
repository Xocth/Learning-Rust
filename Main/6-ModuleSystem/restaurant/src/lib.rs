mod front_of_house {
    pub mod hosting { // pub makes the module public and can be accessed from outside
        fn add_to_waitlist() {} // This function is private and can't be accessed from outside
     
    }
} // This is a module declaration

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super keyword is used to access the parent module
    }

    fn cook_order() {}
}

// use

use self::front_of_house::hosting; // This is an absolute path

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}