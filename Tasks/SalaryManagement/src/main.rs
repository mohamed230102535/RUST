use std::io;

fn main() {
    let mut input = String::new();
    let mut multiSalaries : Vec<u32> = Vec::new();
    println!("Salary Management and Tax Deduction");
    println!("==================================");
    println!("Enter '1' for single salary tax calculation.");
    println!("Enter '2' for multiple salaries to calculate the average.");
    println!("Enter the choice: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice: u32 = input.trim().parse().expect("Invalid Number, please try again") ;
    if choice == 1 {
    println!("Enter the base salary: ");
    input.clear();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let baseSalary: u32 = input.trim().parse().expect("Invalid Number, please try again") ;
   
    taxDeducted(baseSalary);
    } else if choice == 2 {
        avgSalaries(multiSalaries);
    } else {
        println!("Invalid choice, please try again");
    }

 
    
}


fn taxDeducted(salary: u32){

    let tax: f32;
    let mut bonus: f32 = 0.0; 
   if salary <= 3000 {

        tax = 0.05;
       
        } else if salary > 3000 && salary <= 7000 {
            tax = 0.10;

               if salary > 5000 {
                bonus = 0.05;
               } else {
                bonus = 0.0;
               }
        } else {

        tax = 0.15;
        }

    
        

        let deducted = (salary as f32) * tax;
        let salaryAfterDeduction = (salary as f32) - deducted;
        println!("Salary Report:");
        println!("==============");
        println!("Base Salary: {}", salary);
        println!("Tax Rate: {:.2}%", tax);
        println!("Tax Deducted: {:.2}", deducted);
        println!("Net Salary After Tax: {:.2}", salaryAfterDeduction); 
        if bonus > 0.0 {
        let bonusAdded = (salary as f32) * bonus;
        let finalSalary = salaryAfterDeduction + bonusAdded;
        println!("Bonus Added: {}", bonus);
        println!("Final Salary: {:.2}", finalSalary);
    }
}

fn avgSalaries(mut salaries: Vec<u32>){
    let mut input = String::new();

    println!("Enter the number of salaries: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let numOfSalaries: u32 = input.trim().parse().expect("Invalid Number, please try again") ;
    let mut count = 1;
    while count <= numOfSalaries {
        println!("Enter salary {} : ", count);  
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            let salaryInput:u32 = input.trim().parse().expect("Invalid Number, please try again") ;
            salaries.push(salaryInput);
            count += 1;
        };

    let mut total =0;
    for salary in &salaries{
        total += salary;
    }
    
    let avg = total/(salaries.len()) as u32;
    println!("The average salary is: {}", avg);
}