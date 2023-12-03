// src/days/day2.rs
// This file contains the solution to the advent of code puzzle. 

use std::{io, collections::HashMap};


/// Parses the input of day 1 part 1 and solves the puzzle.
pub fn solve_part1(input_lines: impl Iterator<Item = io::Result<String>>) {
    let mut sum: u32 = 0;
    let mut game_sum: u32 = 0;
    let cube_counts: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for line in input_lines {
        match line {
            Ok(content) => {
                let mut game_possible: bool = true;

                // Get the game ID as integer
                let split_result: Vec<&str> = content.split(":").collect();
                let game_id: u32 = split_result.first().unwrap().replace("Game ", "").parse::<u32>().unwrap();
                game_sum += game_id;

                // Get the cube counts
                let cube_string: Vec<&str> = split_result.last().unwrap().split(";").collect();
                for round in cube_string {
                    let draws:  Vec<&str> = round.split(",").collect();
                    for draw in draws {
                        let draw = draw.trim();
                        let draw_split: Vec<&str> = draw.split(" ").collect();
                        let cube_count: u32 = draw_split.first().unwrap().parse::<u32>().unwrap();
                        let cube_color: &str = draw_split.last().unwrap();

                        // Get avilable count of the color
                        let count_avilable: &u32 = cube_counts.get(cube_color).unwrap();
                        if cube_count > *count_avilable && game_possible {
                            game_possible = false;
                            sum += game_id;
                            break;
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
    println!("Result of day 2 part 1 is: {}", game_sum - sum);
}

/// Parses the input of day 1 part 2 and solves the puzzle.
pub fn solve_part2(input_lines: impl Iterator<Item = io::Result<String>>) {
    let mut sum: u32 = 0;
    for line in input_lines {
        match line {
            Ok(content) => {
                // Container for maximal cube counts of a game
                let mut cube_counts: HashMap<&str, u32> = HashMap::from([
                    ("red", 0),
                    ("green", 0),
                    ("blue", 0),
                ]);

                // Get the cube counts
                let split_result: Vec<&str> = content.split(":").collect();
                let cube_string: Vec<&str> = split_result.last().unwrap().split(";").collect();
                for round in cube_string {
                    let draws:  Vec<&str> = round.split(",").collect();
                    for draw in draws {
                        let draw = draw.trim();
                        let draw_split: Vec<&str> = draw.split(" ").collect();
                        let cube_count: u32 = draw_split.first().unwrap().parse::<u32>().unwrap();
                        let cube_color: &str = draw_split.last().unwrap();

                        let max_cube_conut: &u32 = cube_counts.get(cube_color).unwrap();
                        if cube_count > *max_cube_conut {
                            cube_counts.insert(cube_color, cube_count);
                        }
                    }
                }

                // Calculate the power of the game
                let mut power: u32 = 1;
                for val in cube_counts.values() {
                    power *= val;
                }

                sum += power;
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
    println!("Result of day 2 part 2 is: {}", sum);
}