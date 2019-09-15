#[derive(Debug)]
struct Employee {
    first_name: String,
    last_name: String,
    age: i8,
    married: bool,
    height: f32,
}

fn main() {



let employee_01 = Employee {
    first_name: String::from("abc"),
    last_name: "xyz".to_string(),
    age: 25,
    married: false,
    height: 5.6,
};

let employee_02 = Employee {
    first_name: String::from("ddd"),
    last_name: String::from("zzz"),
    age: 25,
    married: false,
    height: 5.6,
};

//println!("First name is:{}",employee_02.first_name);
//println!("{:?}",employee_01);
let Employee_03 = create_student("Babar".to_string(),"Zaman".to_string(),24,true,5.4);
println!("{:?}",Employee_03);
}


fn create_student (fname:String,lname:String,age:i8,mar:bool,h:f32) -> Employee{
    Employee {
        first_name: fname,
        last_name: lname,
        age: age,
        married: mar,
        height: h,
    }
 
}

//Write a program which makes struct for a car

