fn main() {
    hello_world();
    // tell_height(182);
    human_id("Mohamed",19,"Egy");
    let total_lol= {
        let price = 5;
        let qnt = 10;
        price * qnt
    };
    println!("total is : {}",total_lol)
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

