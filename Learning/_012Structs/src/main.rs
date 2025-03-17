#![allow(warnings)]

fn main() {
    // tuple 
    let rect = (300, 500);

    //struct

    struct book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    };
    struct User {
        active : bool,
        username: String,
        email: String,
        sign_in_count: u64,
    };


    let mut user1 : User = User{
        active: true,
        username: String::from("user1"),
        email: String::from("lol@gmail.com"),
        sign_in_count: 1,
    };

    println!("user1 email: {}", user1.email);

    user1.email = String::from("newlol@gamil.com");

    println!("user1 email: {}", user1.email);

    //returning the struct from a function
    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    //Create instance from other instances 

    let user2=User{
        email: String::from("moh@gmail.com"),
        ..user1 // ya5od el ba2y men user1 zy el active w el sign_in_count
    }

    //Tuple structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    //Unit-like structs
    struct AlwaysEqual;
    let subject= AlwaysEqual;
}