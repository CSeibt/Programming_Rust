use std::collections::HashMap;

fn add_member(input: &Vec<&str>, member_list: &mut HashMap<String, String>) {
    if input.len() < 4 {
        println!("Command is incomplete");
    } else {
        member_list.entry(input[1].to_string()).or_insert(input[3].to_string());
    }
}

fn change_member(input: &Vec<&str>, member_list: &mut HashMap<String, String>) {
    if input.len() < 4 {
        println!("Command is incomplete");
    } else {
    member_list.insert(input[1].to_string(), input[3].to_string());
    }
}

fn remove_member(input: &Vec<&str>, member_list: &mut HashMap<String, String>) {
    if input.len() < 2 {
        println!("Command is incomplete");
    } else {
        member_list.remove(&input[1].to_string());
    }
}

fn main() {
    let mut command = String::new();
    let mut company_members = HashMap::new();

    loop {
        println!("Enter your command: ");
        std::io::stdin().read_line(&mut command).unwrap();
        let vec_of_input: Vec<&str> = command.split(" ").collect();
        match vec_of_input[0] {
            "Add" => add_member(&vec_of_input, &mut company_members),
            "Change" => change_member(&vec_of_input, &mut company_members),
            "Remove" => remove_member(&vec_of_input, &mut company_members),
            _ => break,
        }
        command.clear();
    } 
    println!(" ");
    println!("Name: Department");
    println!(" ");
    for (name, department) in &company_members {
        println!("{name}: {department}");
    }
}
