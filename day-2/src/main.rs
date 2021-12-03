//! Advent of Code 2021: Day 2
//!
//! https://adventofcode.com/2021/day/2

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let mut depth = 0;
    let mut x_pos = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut word_iter = line.split_whitespace();
        let instruction = word_iter.next().unwrap();
        let value = word_iter.next().unwrap().parse::<i32>().unwrap();

        match instruction {
            "forward" => {
                x_pos += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!(),
        };
    }

    println!("{}", x_pos * depth);
    Ok(())
}
