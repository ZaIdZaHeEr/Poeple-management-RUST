#![allow(unused)]

fn main() {
    let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("The elements of the array are: {}", arr[0]);
    println!("The elements of the array are: {}", arr[1]);
    println!("The elements of the array are: {}", arr[2]);
    println!("The elements of the array are: {}", arr[3]);
    println!("The elements of the array are: {}", arr[4]);
    println!("The elements of the array are: {}", arr[5]);

    println!("All the elements are : {:?}", arr);
    for i in arr {
        println!("{}", i);
    }

    println!("Using the index values");
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    println!("2 + 3 = {}", 2 + 3);

    let a: i32 = 2;
    let b: u32 = 3;
    println!("{0} {1} = {2}", a, b, b + a as u32);

    // TUPLES////////////////////////////////////////////////////
    let person = ("Muhammad Zaid Zaheer", 21, "brown");
    println!("The person info is : {:?}", person);

    let (name, age, color) = person;
    println!("Name : {}, Age : {}, Color : {}", name, age, color);

    let name: String = "Muhammad Zaid Zaheer".to_string();
    let short_name: Vec<&str> = name.split(" ").collect();
    println!("Short name is : {:?}", short_name);

    let name: String = "Muhammad Zaid Zaheer".to_string();
    let short_name = &name[0..8];
    println!("THe short name is : {0}", short_name);
}
