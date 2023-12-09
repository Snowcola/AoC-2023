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

