fn main() {
    println!("Hello, Rust from Rust!");
    hello_world();
    tell_height(13);
}

fn hello_world(){
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

    // Strings
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);
}

fn tell_height(height: i32){
    println!("My height is {}", height);
}