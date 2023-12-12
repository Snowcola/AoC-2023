use std::fs;

pub fn solve() {
    let file_path = "input/2023/day1.txt";
    let mut sum = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        sum += day1_single(line)
    }
    println!("Day 1: {}", sum)
}

fn day1_single(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .chars()
        .filter_map(
            |c| c.to_digit(10)
        )
        .collect();
    return digits.first().unwrap()*10 + digits.last().unwrap() 
}

pub fn solve_pt2() {
     let file_path = "input/2023/day1.txt";
    let mut sum = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        sum += day1_single(&parse_digits(line).as_str());
    }
    println!("Day 1 part 2: {}", sum);
}

fn parse_digits(input: &str) -> String {
    let parsed = String::from(input);
    parsed
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}


