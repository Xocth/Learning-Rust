fn main() {
    println!("Hello, world!");
    test_one();
    add_numbers(20, 30); // calling function with parameters / expression statement

    let x = 10; // statement
    let x = 2 < 3; // expression / return or gives you a value

    let number = {
        let x = 3;
        x + 1  // using a semi colon will make an error because it will be a statement
    }; // expression
    println!("The number is: {}", number);

    let result = add_numbers_with_return(0, 1);
    println!("The result is: {}", result);

    let result = add_numbers_with_return2(1, 1);
    println!("The 2result is: {}", result);

    let result = add_numbers_with_return3(12, 3);
    println!("The 3result is: {}", result);
}

fn test_one(){ // can be called from main / doesn't need to be declared before main
    println!("test has been called");
}

fn add_numbers(x: i32, y: i32){ // function with parameters
    println!("Sum: {}", x + y);

}

fn add_numbers_with_return(x: i32, y: i32) -> i32{ // function with parameters and return type
    x + y // return value
}

fn add_numbers_with_return2(x: i32, y: i32) -> i32{ // function with parameters and return type
    return x + y; // return value with return keyword
}

fn add_numbers_with_return3(x: i32, y: i32) -> i32{ // function with parameters and return type
    let result = x + y;
    if result > 10 {
        return result - 10; // subtract 10 if result is greater than 10
    }
    result // return value
}

