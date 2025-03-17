// fn main(){
//     let mut _x = 5;
//     let _r = &mut _x;


//     *_r +=1;  //Dereferencing r to get the actual value
//     *_r -=3;
//     //🔹  r is just a reference (like the treasure map).
//     //🔹 *r follows the reference to get the real value inside x

  
//     println!("value of x : {}",_x)
//     println!("value of x : {}",_r)

// }


fn main(){
let mut account : BankAccount = BankAccount{
        owner: "Ali".to_string(),
        balance:1000.0
    };
    //mutable borrow
    account.withdraw(500.0);
    //immutable borrow
    account.check_balance()
}
struct BankAccount{
    owner: String,
    balance:f64
}
impl BankAccount{
    fn withdraw(&mut self,amount:f64){
        println!("Withdrawing {} from the account of {}",amount,self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("The balance is {}",self.balance)
    }
}
