//shadowing is different from mut, 

fn main() {
    let x = 5; 

    let x = x + 1;  // this is what the compiler will see   // x = 6

    // x = 10;  // this will give an error because x is immutable so u need to use let x = 10; to shadow x 

    {
        let x = x * 2 ;  /// result is 12
        println!("The value of x inside the scope is: {}", x);
    }
    println!("The value of x is: {}", x);  



    let mut spaces = "   ";  // string with 3 spaces
    let spaces = spaces.len(); // number type

    /*
    let spaces = "   ";  // string with 3 spaces
    spaces = spaces.len(); //this will give error because spaces will expect a number type but it is a string type

    */


    println!("The value of spaces is: {}", spaces);
}
