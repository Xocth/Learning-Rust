fn main() {
    let mut a = 4; // cannot be changed without mut keyword // assigned value is never read
    let _b: u32 = 5; // unused variable

    a = 5; // Error: cannot assign twice to immutable variable `a`

    println!("a is: {}", a);

    let a = 6; // using let allows us to change the variable value even if it is not immutable
    println!("a is: {}", a);


    {
        let a = 2; // shadowing the variable a
        println!("a is: {}", a);
    }

    let a = a + 1; // a is still 6 because the previous a is shadowed
    println!("a is: {}", a); // a here is 7 not 3

    let x = 4;
    println!("x = {}", x);

    let x = "Hello";
    println!("x = {}", x); // redifining x as a string

    const SECONDS_IN_MINUTE: u32 = 60; // changing the value of a constant will cause an error
    println!("Seconds in a minute: {}", SECONDS_IN_MINUTE);

}
