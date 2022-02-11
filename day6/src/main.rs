#![allow(unused)]
use bitstring::BitString;
use float_cmp::{approx_eq, assert_approx_eq};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Read};
use std::ops::Div;

fn parse_input(input: &mut String) -> Vec<usize> {
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn part1(input: &mut String, n_days: usize) -> i64 {
    let init_fish = parse_input(input);
    let mut fish = vec![0_i64; 9];
    for f in init_fish {
        fish[f] += 1;
    }
    // need buff to store day 7
    for i in 0..n_days {
        let n_fish = fish[0];
        println!("{:?}", fish);
        fish[0..=6].rotate_left(1);
        fish[6] += fish[7];
        fish[7] = fish[8];
        fish[8] = n_fish;
    }

    let s: i64 = fish.iter().sum();
    println!("{:?} ", fish);

    s
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input);

    println!("{} {}\n", part1(&mut input, 80), part1(&mut input, 256));
    Ok(())
}
