use std::{collections::HashMap, fs};
#[derive(Debug, Clone, Copy)]
pub struct WrittenNumber {
    value: u8,
    first_index: usize,
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

pub fn check_for_written_number(input: &str) -> Vec<WrittenNumber> {
    let in_string = input.to_string();
    let mut output: Vec<WrittenNumber> = Vec::new();
    let mut final_out: Vec<WrittenNumber> = Vec::new();

    let written_numbers = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];
    let keys: HashMap<String, u8> = HashMap::from([
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]);
    for c in written_numbers.iter() {
        println!("Erkennende Line: {:?}", c);

        if input.contains(c) {
            println!("Input Line: {:?}", input);

            let value = keys.get(&c.to_string()).unwrap();
            let index = in_string.find(c).unwrap();
            let final_number = WrittenNumber {
                value: *value,
                first_index: index,
            };
            println!("Erkannt: {:?}", final_number);

            output.push(final_number);
        }
    }
    final_out.push(output[0]);
    final_out.push(output[output.len()-1]);
    final_out
}

pub fn day1_2(input: &str) {

    let file_string = fs::read_to_string(input)
        .expect("CANT READ FILE")
        .to_ascii_lowercase();
    let numerics: Vec<&str> = file_string
        .lines()
        .filter(|x| x.chars().any(char::is_numeric))
        .collect();
    println!("Input Text: {:?}", file_string);


    let mut numbers: Vec<String> = Vec::new();
    let mut index: usize = 0;
    let mut number: char = 'a';
    for entry in numerics.iter() {

        let wordnumbers = check_for_written_number(entry);
        let mut num_str: String = String::new();
        for c in entry.chars() {
            if c.is_numeric() {
                number = c;
                index = entry.find(c).unwrap();
                if index < wordnumbers[0].first_index && num_str.len() == 0 {
                    num_str.push(c)
                } else if index > wordnumbers[0].first_index && num_str.len() == 0  {
                    num_str.push_str(&wordnumbers[0].value.to_string())
                }
            }
        }
        if index < wordnumbers[1].first_index {
            num_str.push_str(&wordnumbers[1].value.to_string())
        } else {
            num_str.push(number)
        }
        println!("Num String: {:?}", num_str);

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
    let test_input = "eightwothree";
    let result = check_for_written_number(test_input);
    
    let mut final_out: Vec<WrittenNumber> = Vec::new();
    
    // Check if there's at least one WrittenNumber in the result
    if let Some(last_written_number) = result.last() {
        final_out.push(last_written_number.clone());
    }

    println!("Result: {:?}", final_out);
    
    //println!("Numbers: {:?}", check_for_written_number("eightwothree"));
    //day1_1("Inputs/Day1_1.txt")
    //day1_2("Inputs/Day1_2_example.txt")

}
