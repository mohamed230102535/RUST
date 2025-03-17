fn main() {
// loop keyword =====================================================
/*
   let mut count = 0;
   let result = loop{
    count += 1;
    if count == 20 {
        break count * 2;
    };
    println!("count: {}", count);
   };
    println!("result: {}", result);
*/

// Loop labels to disambiguate between multiple loop =====================================================
// so u know which one to break out of


 /*  
let mut count = 0;
'counting_up:loop{
    println!("count: {}", count);
    let mut remaining = 10;
    loop{
        println!("remaining: {}", remaining);
        if remaining ==9{
            break;
        }
        if count ==2 {
            break 'counting_up;
        }
        remaining -= 1;
    }
   count += 1;
  
}
*/

// while loop =====================================================


/*
let mut number =3;

while number !=0{
    println!("{}!", number);
    number -= 1;
}
 */

//looping through a collection with for loop
let a = [-10, 20, 30, 40, 50];
for element in &a {
    println!("the value is: {}", element);
}

let sum:i32 = a.iter().sum();
println!("sum: {}", sum);


}

