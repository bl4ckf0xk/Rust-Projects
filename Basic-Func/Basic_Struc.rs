fn main() {
    println!("Hello, Rust from Cargo!");

    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer :{}", x);
    println!("Undigned Integer :{}", y);

    let animals: [&str; 2] = ["cat", "dog"];
    println!("Animals are {:?}",animals);

    // tuple
    let my_mix = ("Hey", 23, true);
    println!("My Mix : {:?}", my_mix);

    // Slices
    let number_slices :&[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);

    let book_slices :&[&String] = &[&"IT".to_string(),&"Harry".to_string(),&"ZEN".to_string()];
    println!("Book SLices: {:?}", book_slices);
}
