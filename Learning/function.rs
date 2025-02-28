fn main() {
    hello_world();
    // tell_height(182);
    human_id("Mohamed",19,"Egy");
    let total_lol= {
        let price = 5;
        let qnt = 10;
        price * qnt
    };
    println!("total is : {}",total_lol);

    let lol:i32 = add(5,5);
    println!("the sum is : {}",lol);

    let weight:f64 = 70.0;
    let height:f64 = 1.82;
    let bmi = calculate_bmi(weight , height);
    println!("BMI is : {:.2}",bmi);
}


fn hello_world(){
    println!("Hello, Mohamed")
}

// fn tell_height(height: u32){
//     println!("My height is {}",height)
// }

fn human_id(name:&str,age:u8,country:&str){
    println!("The name is {} the age is {} and country {}",name,age ,country)
}

fn add(x:i32 ,y:i32) -> i32{    //return type 
    x+y
}

//BMI = weight / height^2

fn calculate_bmi(weight: f64, height: f64) -> f64{
    weight / height.powi(2)
}
