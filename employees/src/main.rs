#[derive(Debug)]
enum Colors{
    ColorByName(String),
    ColorByCode(u8, u8, u8),            //Red, green, blue
}

struct Employee {
    id: u32,
    name: String,
    department: String,
    salary: f64,
}

impl Employee {
    fn print_employee_info(&self){
        println!("Employee Information:");
        println!("Name: {}", self.name);
        println!("Id: {}", self.id);
        println!("Department: {}", self.department);
        println!("Salary: {}", self.salary);
        println!(" ");
    }
}

fn main() {
    let ceo = Employee {
        id: 1,
        name: String::from("Christoph Seibt"),
        department: String::from("Head"),
        salary: 10000000.0,
    };

    let yellow = Colors::ColorByName(String::from("yellow"));
    let some_turquoise = Colors::ColorByCode(35, 128, 255);

    println!("The first color is: {:?}", yellow);
    println!("The second color is: {:?}", some_turquoise);
    
    ceo.print_employee_info();


}
