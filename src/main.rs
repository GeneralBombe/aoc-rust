use std::{fs, collections::HashMap};

struct written_number {
    Vec
}

pub fn day1_1(input: &str) {

    let file_string = fs::read_to_string(input).expect("CANT READ FILE");
    let numerics: Vec<&str> = file_string
        .lines()
        .filter(|x| x.chars().any(char::is_numeric))
        .collect();

    let mut numbers: Vec<String> = Vec::new();
    for entry in numerics.iter() {
        let mut num_str: String = String::from("");
        for c in entry.chars() {
            if c.is_numeric() {
                num_str.push(c)
            }
        }
        numbers.push(num_str);
    }
    println!("String: {:?}", numbers);
    let mut final_int: Vec<u16> = Vec::new();
    for n in numbers.iter() {
        let firstnum_str = n.chars().nth(0).unwrap();
        let lastnum_str = n.chars().nth(n.len() - 1).unwrap();
        let mut combined_num = String::new();
        combined_num.push(firstnum_str);
        combined_num.push(lastnum_str);
        final_int.push(combined_num.parse::<u16>().unwrap());
    }
    println!("Numbers: {:?}", final_int);

    let sum: u16 = final_int.iter().sum();
    println!("Result: {}", sum);

}

pub fn check_for_written_number(input: &str) -> (u8, u16) {
    let written_numbers = [String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine")];
    let keys = HashMap::from([
        (String::from("one"), 1),
        (String::from("two"), 1),
        (String::from("three"), 1),
        (String::from("four"), 1),
        (String::from("five"), 1),
        (String::from("six"), 1),
        (String::from("seven"), 1),
        (String::from("eight"), 1),
        (String::from("nine"), 9),

    ]);
    for c in written_numbers.iter() {
        if input.contains(c) {
            let value = keys.get(&c.to_string()).copied().unwrap();
           
        }
    }
    (value,3)
}

pub fn day1_2(input: &str) {
    let mut file_string = fs::read_to_string(input).expect("CANT READ FILE").to_ascii_lowercase();
    let numerics: Vec<&str> = file_string
        .lines()
        .filter(|x| x.chars().any(char::is_numeric))
        .collect();

    let mut numbers: Vec<String> = Vec::new();
    for entry in numerics.iter() {
        let mut num_str: String = String::from("");
        for c in entry.chars() {
            if c.is_numeric() {
                num_str.push(c)
            }
        }
        numbers.push(num_str);
    }
    println!("String: {:?}", numbers);
    let mut final_int: Vec<u16> = Vec::new();
    for n in numbers.iter() {
        let firstnum_str = n.chars().nth(0).unwrap();
        let lastnum_str = n.chars().nth(n.len() - 1).unwrap();
        let mut combined_num = String::new();
        combined_num.push(firstnum_str);
        combined_num.push(lastnum_str);
        final_int.push(combined_num.parse::<u16>().unwrap());
    }
    println!("Numbers: {:?}", final_int);

    let sum: u16 = final_int.iter().sum();
    println!("Result: {}", sum);

}

fn main() {
    day1_1("Inputs/Day1_1.txt")
}
