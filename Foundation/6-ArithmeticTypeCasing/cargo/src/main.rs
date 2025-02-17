fn main() {
    
    let x: u8 = 255;
    let y: u8 = 10; 

    let z = x / y; // 255 / 10 = 25
    println!("z: {}", z);

    // literal 255 is an integer, but the type is f64


    let x: f64 = 255.0;
    let y: f64 = 10.0; 

    let z = x / y;
    println!("z: {}", z); // 25.5 float

    let x: f64 = 255.0;
    let y: f64 = 10.0; 

    let z = x % y; // 255 % 10 = 5 because 255 = 25 * 10 + 5
    println!("z: {}", z);

    // Type conversions and casting

    let x = 255.0f32; // f32 data type
    let y = 10.0_f32; // f32 data type

    let z = x % y; // 255 % 10 = 5 because 255 = 25 * 10 + 5
    println!("z: {}", z);

    let x = 127_000i64; 
    let y = 10_i64; 

    let z = x / y; // 127000 / 10 = 12700
    println!("z: {}", z);

    let x = 127_000 as i64; 
    let y = 10_i32; 

    let z = x / (y as i64); // this is a cast operation / brackets arent necessary but for clarity
    println!("z: {}", z); // explicit type conversion

    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;

    let z = x as i32 / y; // overflow and gives us a negative number
    println!("z: {}", z); // compiler will not warn us about overflow
    


}

