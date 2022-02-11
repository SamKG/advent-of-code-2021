#![allow(unused)]
use float_cmp::{approx_eq, assert_approx_eq};
use std::io::{self, BufRead, Read};
use std::ops::Div;

fn parse_input(input: &mut String) -> Vec<i64> {
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

fn part1(input: &mut String) -> i64 {
    let crabs = parse_input(input);
    crabs
        .iter()
        .map(|c| crabs.iter().map(|b| (c - b).abs()).sum())
        .min()
        .unwrap()
}

fn summed(x: i64) -> i64 {
    (x * (x + 1)) / 2
}

fn part2(input: &mut String) -> i64 {
    let crabs = parse_input(input);
    let mi = crabs.iter().min().unwrap();
    let ma = crabs.iter().max().unwrap();
    let mut r = (*mi..=*ma);
    r.map(|c| crabs.iter().map(|b| summed((c - b).abs())).sum())
        .min()
        .unwrap()
}
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input);

    println!("{} {}\n", part1(&mut input), part2(&mut input));
    Ok(())
}
