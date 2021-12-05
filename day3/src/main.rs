#![allow(unused)]
use std::collections::HashSet;
use std::io::{self, BufRead, Read};
extern crate itertools;
use bitstring::BitString;
use itertools::Itertools;

fn part1(input: &mut String) -> i64 {
    let nbits = input.split("\n").next().unwrap().len();
    let mut arr: Vec<i64> = vec![0; nbits];
    for line in input.split("\n") {
        for (idx, c) in itertools::enumerate(line.chars()) {
            match c {
                '1' => arr[idx] += 1,
                '0' => arr[idx] -= 1,
                _ => panic!("Invalid character"),
            }
        }
    }
    let mut gamma = String::new();
    arr.iter()
        .map(|x| if *x > 0 { 1 } else { 0 })
        .for_each(|x| gamma.push_str(&x.to_string()));
    let gamma = i64::from_str_radix(&gamma, 2).unwrap();

    let mut epsilon = String::new();
    arr.iter()
        .map(|x| if *x > 0 { 0 } else { 1 })
        .for_each(|x| epsilon.push_str(&x.to_string()));
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();

    return gamma * epsilon;
}

fn get_bit_counts(set: &mut HashSet<String>, pos: usize) -> (i32, i32) {
    let mut counts = (0,0);
    for str in set.iter() {
        match str.chars().nth(pos).unwrap() {
            '1' => counts.1 += 1,
            '0' => counts.0 += 1,
            _ => panic!("Invalid character"),
        }
    }
    return counts;
}

fn part2(input: &mut String) -> i64 {
    let nbits = input.split("\n").next().unwrap().len();
    let mut set: HashSet<String> = HashSet::from_iter(
        input
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string()),
    );
    for i in 0..nbits {
        let bit_counts = get_bit_counts(&mut set, i);
        let most_common_bit_char = if bit_counts.0 > bit_counts.1 {
            '0'
        } else {
            '1'
        };
        set = HashSet::from_iter(
            set.drain()
                .filter(|x| x.chars().nth(i).unwrap() == most_common_bit_char),
        );
    }
    let oxygen_gen_rate = i64::from_str_radix(set.iter().next().unwrap(), 2).unwrap();

    let mut set: HashSet<String> = HashSet::from_iter(
        input
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string()),
    );
    for i in 0..nbits {
        let bit_counts = get_bit_counts(&mut set, i);
        let least_common_bit_char = if bit_counts.0 <= bit_counts.1 {
            '0'
        } else {
            '1'
        };
        set = HashSet::from_iter(
            set.drain()
                .filter(|x| x.chars().nth(i).unwrap() == least_common_bit_char),
        );
        if set.len() == 1 {
            break;
        }
    }
    let co2_scrub_rate = i64::from_str_radix(set.iter().next().unwrap(), 2).unwrap();
    println!("{} {}", oxygen_gen_rate, co2_scrub_rate);
    return oxygen_gen_rate * co2_scrub_rate;
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input);

    println!("{} {}", part1(&mut input), part2(&mut input));
    return Ok(());
}
