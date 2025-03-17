
fn main() {
    let age :u32 = 4;
    if age >=18{
        println!("You can drive a car");

    }else if age < 10{

        println!("Nigga u can't drive");

    }else{

        println!("You cannot drive a car"); // this is called arm
    }

    let condition = false;
    let number = if condition { 43 }
     else  { 0 }; 
/*
     let number = if condition { 43 }
     else  { 0 };  //error  expected integer, found `&str`
*/

    println!("Number: {number}");
}
