#![allow(unused)]
use std::{
    f32::consts::E,
    io::{self, Read, Write},
};

#[derive(Debug, Clone)]
enum HairClr {
    Black,
    Brown,
    Blonde,
    White,
}
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: String,
    hair_color: HairClr,
    height: String,
    weight: String,
}
fn add_person(person: &mut Vec<Person>) {
    let mut name: String = String::new();
    let mut age: String = String::new();
    let mut hair_color: HairClr;
    let mut height: String = String::new();
    let mut weight: String = String::new();
    print!("Enter your name : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Please enter a valid String");

    print!("\nEnter your Age : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut age)
        .expect("Please enter a valid String");
    loop {
        println!("Select your Hair_color : ");
        println!("Enter 1 for Black");
        println!("Enter 2 for Brown");
        println!("Enter 3 for Blonde");
        println!("Enter 4 for White");
        print!("\nYour Choice : ");
        io::stdout().flush().unwrap();

        let mut clr = String::new();

        io::stdin()
            .read_line(&mut clr)
            .expect("Please enter a valid String");
        let clr_to_num: i32 = clr.trim().parse().expect("Invalid input detected");

        if (clr_to_num == 1) {
            hair_color = HairClr::Black;
            break;
        } else if clr_to_num == 2 {
            hair_color = HairClr::Brown;
            break;
        } else if clr_to_num == 3 {
            hair_color = HairClr::Blonde;
            break;
        } else if clr_to_num == 4 {
            hair_color = HairClr::White;
            break;
        } else {
            panic!("Invalid input detected");
        }
    }

    print!("\nEnter your Height : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut height)
        .expect("Please enter a valid String");

    print!("\nEnter your Weight : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut weight)
        .expect("Please enter a valid String");

    let new_person = Person {
        name,
        age,
        hair_color,
        height,
        weight,
    };

    person.push(new_person);

    println!("Person added successfully");
}
fn delete_person(person: &mut Vec<Person>) {
    print!("Enter the Person Indexed Number to Delete : ");
    io::stdout().flush().unwrap();
    let mut del_index = String::new();
    io::stdin()
        .read_line(&mut del_index)
        .expect("Please give a valid input");

    let del_index_to_num: i32 = del_index
        .trim()
        .parse()
        .expect("Please enter a valid number");
    if del_index_to_num >= 0 && (del_index_to_num as usize) < person.len() {
        person.remove(del_index_to_num as usize);
    } else {
        println!("Person not found");
    }
}
fn search_person(person: &mut Vec<Person>) {
    print!("Enter the index of the person to remove : ");
    io::stdout().flush().unwrap();
    let mut ind: String = String::new();
    io::stdin()
        .read_line(&mut ind)
        .expect("Please give a valid number.");

    let ind_to_num: i32 = ind.trim().parse().expect("Please give a valid input");
    if ind_to_num >= 0 && (ind_to_num as usize) < person.len() {
        println!("Searching successfull : {:?}", person[ind_to_num as usize]);
    } else {
        println!("Searching complete, no person found");
    }
}
fn view_all(person: &mut Vec<Person>) {
    println!("All the persons in the list are : {:#?}", person);
}
fn main() {
    let mut persons: Vec<Person> = Vec::new();
    println!("----WELCOME TO PEOPLE MANAGEMENT SYSTEM----");
    loop {
        println!("Press 1 for adding a new Person");
        println!("Press 2 for viewing all the Persons");
        println!("Press 3 for deleting a Person");
        println!("Press 4 for Searching a Person");
        println!("Press 5 to exit");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Please give a valid Input");
        let option = option.trim();
        // println!("You entered : {}", option); // Debugging

        if option == "1" {
            add_person(&mut persons);
        } else if option == "2" {
            view_all(&mut persons);
        } else if option == "3" {
            delete_person(&mut persons);
        } else if option == "4" {
            search_person(&mut persons);
        } else {
            println!("Exiting");
            break;
        }
    }
    println!("Thank you for using my People manager developed in RUST");
    // println!("The vector says: {:#?}", persons);
}
