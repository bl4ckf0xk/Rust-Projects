/*
 *This is my first program
*/

fn main() {
    // print hello world
    println!("Hello, World!");

    // defined 32bit value and 64bit floating value and 8bit value
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructed the tuple to x, y and z
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five = tup.0;
    let six = tup.2;
    println!("{} {}",five, six);
}

