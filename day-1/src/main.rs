//! Advent of Code 2021: Day 1
//!
//! https://adventofcode.com/2021/day/1

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let mut increases = 0;
    let mut previous = None;
    let values = input
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for window in values.as_slice().windows(3) {
        let value = window.iter().sum::<i32>();

        match previous {
            Some(previous_value) => {
                if previous_value < value {
                    increases += 1;
                }
            }
            None => {}
        }
        previous = Some(value);
    }

    println!("{}", increases);
    Ok(())
}
