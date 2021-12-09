#![allow(unused)]
use std::io::{self, BufRead, Read};
extern crate itertools;
use itertools::Itertools;

fn part1(input: &mut String) -> i32 {
    let mut vals = (0, 0);
    for line in input.split('\n') {
        for (direction, amount) in line
            .split_whitespace()
            .tuples()
            .map(|(direction, amount)| (direction, amount.parse::<i32>().unwrap()))
        {
            match direction {
                "forward" => vals.0 += amount,
                "down" => vals.1 += amount,
                "up" => vals.1 -= amount,
                _ => panic!("Unknown direction {}", direction),
            }
        }
    }
    vals.0 * vals.1
}

struct Values {
    x: i32,
    y: i32,
    aim: i32,
}
fn part2(input: &mut String) -> i32 {
    let mut vals = Values { x: 0, y: 0, aim: 0 };
    for line in input.split('\n') {
        for (direction, amount) in line
            .split_whitespace()
            .tuples()
            .map(|(direction, amount)| (direction, amount.parse::<i32>().unwrap()))
        {
            match direction {
                "forward" => {
                    vals.x += amount;
                    vals.y += amount * vals.aim;
                }
                "down" => vals.aim += amount,
                "up" => vals.aim -= amount,
                _ => panic!("Unknown direction {}", direction),
            }
        }
    }
    vals.x * vals.y
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input);

    println!("{} {}", part1(&mut input), part2(&mut input));
    Ok(())
}
