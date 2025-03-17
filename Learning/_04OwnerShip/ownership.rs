fn main(){
//Example 1
    let s1 = String::from("Rust");
    let len = calculate_length(&s1); // &s1 is a reference to the string s1
    //so we can use the value of s1 without 
    //taking ownership of it that what called borrowing 
    println!("The length of '{}' is {}",&s1,len);  
    
//Example 2
    let s2 = s1; //s1 is no longer valid
    // println!("{}",s1) //error: value borrowed here after move
    println!("{}",s2);

//Example 3
     
}

fn calculate_length(s: &String) -> usize{  // usize = unsigned size
    //By using &String, we pass a reference instead of moving ownership
    s.len()
}