// Primitive data types:

// int , float, bool , char

// Integer : =======================================================
// Signed Integers (+ and -):i8 ,i16 ,i32 ,i64 ,i128
// Unsigned Integers (only -):u8 ,u16 ,u32 ,u64 ,u128

//i32 (32 bits range 2147483647)
//unsigned integers are little bit lager than the 
// signed integers due to the power

// fn main() {
//     let x :i32 = -42;
//     let y :u32 = 100;
//     println!("Signed Integer : {}",x);
//     println!("Unsigned Integer : {}", y);
// }

// Float : =======================================================
// f32 , f64
// Boolean : 
// Char :


// fn main(){
//     let pi : f64 = 3.14;
//     println!("Value of pi = {}",pi);

//     let is_snowing:bool = true;
    
//     println!("Is it snowing? {}",is_snowing);

//     let letter:char = 'a';

//     println!("First letter is = {}", letter);
// }



//Compound Data types
// Arrays , tuples, slices, and strings (slice string)

//Array
//cannot mix int and str
// fn main(){

//     let num: [i32;5] = [1,-2,3,-4,5];

//     println!("the numbers are : {:?}",num);

//     let fruits: [&str;3]=["apple","kiwi","banana"];
    
//     println!("fruits are = {:?}", fruits);
//     println!("1 - {}", fruits[0]);
//     println!("2 - {}", fruits[1]);
//     println!("3 - {}", fruits[2]);

// }

// Tuples

// fn main(){
//     let human:(String,i8, bool) = ("Mohamed".to_string(), 19,false);
//     println!("card :{:?}",human);

//     let mixed_tuples = ("hamda",34,true,[1,2,3,4]);
//     println!("{:?}",mixed_tuples);
// }


//Slices : is contagious sequnece which make it good thing for memory allocation
//and memory efficiency 


// fn main(){
//     let num_slice:&[i32] = &[1,2,3,4,5];
//     println!("{:?}",num_slice);

//     let str_slice:&[&str] = &["mohamed","sayed","ahmed"];
//     println!("{:?}",str_slice);

//     let books_slices:&[String] = &["Python".to_string(),"JAVA".to_string(),"LOL".to_string()];
//     println!("{:?}",books_slices);
// }

//Strings vs string Slices ($str)
//A- Strings [growable, mutable, owned string type]

// fn main(){
//     let mut stone_cold: String = String::from("Hell, ");
//     println!("Stone Cold says : {}",stone_cold);
//     stone_cold.push_str("Yeah!!");
//     println!("Stone Cold says : {}",stone_cold);

// //B- str (Sting Slice)
//     let string: String = String::from("Hello, world!");
//     let slice: &str = &string[0..5]; //&reference
//     println!("slice value: {}",slice)

// }

