fn main() {
    
    //Scalar Types

    let x = 2;// compiler say that x is an integer
    let x: i32 = 2;// we can specify that x is an integer// assign integer that using 32bits. 

    /*

    i8
    i16
    i32
    i64
    i128 // integers that can hold 8, 16, 32, 64, 128 bits

    */

    let x: u32 = 2;// assign unsigned integer that using 32bits. // this cant hold negative values

    /*
    u8; // 0 - 2 ^ 8 - 1 // range 0 - 255
    i8; // -2 ^ 7 - 2 ^ 7 - 1 // range  -128 - 127

    */
    /* 

    f32; f64; // floating point numbers // 32 = single 64 = double precision

    */
    let f: f32 = 10.9; //f64 is default from compiler


    let true_or_false: bool = false; // boolean

    let letter: char = 'x'; // char

    //Compound Types

    let tup: (i32, bool, f32, char) = (1, true, 10.9, 'x'); // tuple // can hold multiple types // immutable
    let tup2: (i8, bool, f32, char) = (1, true, 10.9, 'x'); 
    println!("{}", tup.0); // access tuple value by index using .
    println!("{}", tup.0); // first element
    println!("{}", tup.1); // second element
    println!("{}", tup.2); // third element
    println!("{}", tup.3); // fourth element

    let mut mutup: (i32, bool, char) = (1, true, 's'); // mutable tuple

    mutup.0 = 10; // change value of first element

    println!("{}", mutup.0);

    let arr = [1, 2, 3, 4, 5]; // array // fixed length // same type // immutable
    arr[0]; // access array value by index

    let mut arr = [1, 2, 3, 4, 5]; // mutable array
    arr[4] = 3;
    println!("{}", arr[4]);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // assign type and length
    arr[4] = 3;
    println!("{}", arr[4]);

}
