// src/days/day1.rs
// This file contains the solution to the advent of code puzzle. 

use std::io;
use std::collections::HashMap;


/// Parses the input of day 1 part 1 and solves the puzzle.
pub fn solve_part1(input_lines: impl Iterator<Item = io::Result<String>>) {
    let mut sum: u32 = 0;
    for line in input_lines {
        match line {
            Ok(content) => {
                let mut first_digit: Option<char> = None;
                let mut last_digit: Option<char> = None;

                // Forward loop
                for c in content.chars() {
                    if c.is_numeric() {
                        first_digit = Some(c);
                        break;
                    }
                }

                // Backward loop
                for c in content.chars().rev() {
                    if c.is_numeric() {
                        last_digit = Some(c);
                        break;
                    }
                }

                if let (Some(first), Some(last)) = (first_digit, last_digit) {
                    let line_number = format!("{}{}", first, last);
                    sum += line_number.parse::<u32>().unwrap();
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
    println!("Sum of day 1 part 1 is: {}", sum);
}


/// Parses the input of day 1 part 2 and solves the puzzle.
pub fn solve_part2(input_lines: impl Iterator<Item = io::Result<String>>) {
    let mut sum: u32 = 0;
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
         "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let mut digit_mapping: HashMap<&str, &str> = HashMap::new();
    digit_mapping.insert("one", "1");
    digit_mapping.insert("two", "2");
    digit_mapping.insert("three", "3");
    digit_mapping.insert("four", "4");
    digit_mapping.insert("five", "5");
    digit_mapping.insert("six", "6");
    digit_mapping.insert("seven", "7");
    digit_mapping.insert("eight", "8");
    digit_mapping.insert("nine", "9");

    for line in input_lines {
        match line {
            Ok(content) => {
                // Check for matching digits
                let mut digits_found: Vec<&str> = Vec::new();
                for i in 0..content.len() {
                    for digit in digits {
                        if content[i..].starts_with(digit) {
                            digits_found.push(digit);
                        }
                    }
                }
                
                let mut first_digit = digits_found.first().unwrap();
                let mut last_digit = digits_found.last().unwrap();

                if digit_mapping.contains_key(first_digit) {
                    first_digit = digit_mapping.get(first_digit).unwrap();
                }

                if digit_mapping.contains_key(last_digit) {
                    last_digit = digit_mapping.get(last_digit).unwrap();
                }

                let line_number = format!("{}{}", first_digit, last_digit);
                println!("{}", line_number);
                sum += line_number.parse::<u32>().unwrap();  
                
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
    println!("Sum of day 1 part 2 is: {}", sum);
}