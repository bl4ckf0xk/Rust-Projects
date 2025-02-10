fn main(){
    let rect: (i32, i32) = (200,500);

    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someusernam@e.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@e.com");
    println!("User email is: {}", user1.email);

    // Create instances by function
    fn build_user(email: String, username:String) -> User{
        User{
            active:true,
            email,
            username,
            sign_in_count:1,
        }
    }

    // Create instances by other instances
    let user2 = User{
        email: String::from("another@m.com"),
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);

    let black = Color(0,0,0);
    let white = Color(255,255,255);

    
}