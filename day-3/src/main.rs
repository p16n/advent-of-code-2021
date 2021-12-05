//! Advent of Code 2021: Day 3
//!
//! https://adventofcode.com/2021/day/3

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let mut memo = vec![0; input.lines().next().unwrap().len()];

    for line in input.lines() {
        for (i, c) in line.char_indices() {
            match c {
                '0' => memo[i] = memo[i] - 1,
                '1' => memo[i] = memo[i] + 1,
                _ => panic!(),
            };
        }
    }

    let mut gamma = 0b0;
    let mut epsilon = 0b0;

    for i in memo {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if i < 0 {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    println!("{:?}", gamma * epsilon);

    Ok(())
}
