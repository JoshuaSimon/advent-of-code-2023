mod days;

use std::fs::File;
use std::io::{self, BufRead, BufReader};


/// Opens a file in read-only mode and returns an iterator over the lines.
fn read_file_lines(filename: &str) -> Result<impl Iterator<Item = io::Result<String>>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

/// Reads the inputs of the current day and solves the puzzle.
fn main() {
    let day: &str = "1";
    let filename: String = format!("inputs/day{}.txt", day);
    let input_lines_part1 = read_file_lines(&filename).expect("Error reading file");
    let input_lines_part2 = read_file_lines(&filename).expect("Error reading file");
    days::day1::solve_part1(input_lines_part1);
    days::day1::solve_part2(input_lines_part2);
}
