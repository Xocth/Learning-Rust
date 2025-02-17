fn main() {
    // conditions
    let cond = (2 as f32) <= 2.2;

    println!("cond: {}", cond); // true

    // compound conditions
    let cond2 = true && cond; // true
    println!("{}", cond2);

    let cond2 = false || cond; // true 
    println!("{}", cond2);

    let cond2 = !(true || cond); // false
    println!("{}", cond2);

    // control flow

    let food = "ice cream";

    if food == "cookie"{
        println!("Same!");

    }else if food == "cake"{
        println!("I dislike cake!");

    }else if food == "ice cream"{
        println!("I love ice cream!");

    }else{
        println!("I prefer cookie!");
    }

}
