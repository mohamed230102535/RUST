// Error Handling techniques [2 approaches]



 //Approach 1 

 enum Option<T>{ //Define the generic Option type
     Some(T),// Represents a value
     None, // Represents no value
 }

 enum Result<T, E>{ //Define the generic Result type
    Ok(T), // Represents success
    Err(E), // Represents error
}

fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        return Option::None;
    }else {
        return Option::Some(numerator / denominator);
    }
   
   }


fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Result::Err("Cannot divide by zero".to_string());
    }else {
        return Result::Ok(numerator / denominator);
    }
   }



fn main() { 
    let result = divideOption(10.0, 2.0);


    match result {
        Option::Some(value) => println!("Result: {}", value),
        Option::None => println!("Cannot divide by zero"),
    }

    match divideResult(10.0, 0.0) {
       Result::Ok(value) => println!("Result: {}", value),
        Result::Err(message) => println!("Error: {}", message),    
    }

    let x =10.0;
    let y = 0.0;

    let result = x/y;
    println!("Result: {}", result);
}