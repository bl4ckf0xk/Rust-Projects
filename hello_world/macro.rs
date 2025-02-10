macro_rules! greet {
	() => {
		println!("hello, macros!");
	};
}

fn main(){
	greet!();
}