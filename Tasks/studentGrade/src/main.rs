use std::io;


fn main() {
let mut input = String::new();
let mut studentsGradeList: Vec<u32> = Vec::new();
        println!("Student Grades Analysis and Performance Classification");
        println!("======================================================");
        println!("Enter the number of students: ");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let numOfStudentsInput: u32 = input.trim().parse().expect("Invalid Number, please try again") ;


        let mut count = 1;
        while count <= numOfStudentsInput {
            println!("Enter grade for student {} (1 - 100): ", count);  
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
                let studnetGradeInput:u32 = input.trim().parse().expect("Invalid Number, please try again") ;
                studentsGradeList.push(studnetGradeInput);
                count += 1;
            };
            println!("The grades entered are: {:?}", studentsGradeList);
            avgGradeAndClass(studentsGradeList);

}

fn avgGradeAndClass(Grades: Vec<u32>){
    let mut total =0;
    for grade in &Grades{

        total += grade;

    }

    let avg = total/(Grades.len()) as u32;

    println!("The average grade is: {}", avg);

    if avg >= 75 {

        println!("Class Performance: Excellent performance");

    } else if avg >= 50 {

        println!("Class Performance: Average performance");

    } else {

        println!("Class Performance: Needs improvement");

    }
    
    let num_students = Grades.len();
    if num_students >= 1 && num_students <= 9 {

        println!("Class Size: Small class");

    } else if num_students >= 10 && num_students <= 30 {

        println!("Class Size: Medium class");

    } else {println!("Class Size:Large class");}
    
}   