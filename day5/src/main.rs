#![allow(unused)]
use bitstring::BitString;
use float_cmp::{approx_eq, assert_approx_eq};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Read};
use std::ops::Div;

#[derive(Clone, PartialEq, Copy, Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    while (b > 0) {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
impl From<&str> for Line {
    fn from(segment_str: &str) -> Self {
        let mut iter = segment_str.split(" -> ");
        let mut vals_start = iter.next().unwrap().split(',');
        let start = (
            vals_start.next().unwrap().parse().unwrap(),
            vals_start.next().unwrap().parse().unwrap(),
        );

        let mut vals_end = iter.next().unwrap().split(',');
        let end = (
            vals_end.next().unwrap().parse().unwrap(),
            vals_end.next().unwrap().parse().unwrap(),
        );
        Line { start, end }
    }
}
impl Line {
    fn slope(&self) -> (i32, i32) {
        (self.end.0 - self.start.0, self.end.1 - self.start.1)
    }

    fn slope_reduced(&self) -> (i32, i32) {
        let slope = self.slope();
        let g = gcd(slope.0.abs(), slope.1.abs());
        (slope.0 / g, slope.1 / g)
    }

    fn contains(&self, coord: (i32, i32)) -> bool {
        let diff = (coord.0 - self.start.0, coord.1 - self.start.1);
        if diff.0 == 0 && diff.1 == 0 {
            return true;
        }
        let slope = self.slope();
        let (x, y) = (
            (diff.0 as f64).div(slope.0 as f64),
            (diff.1 as f64).div(slope.1 as f64),
        );
        // println!("{:?} {:?} {:?}", self, coord, (x, y));
        match (
            f64::is_infinite(x) || f64::is_nan(x),
            f64::is_infinite(y) || f64::is_nan(y),
        ) {
            (true, true) => false,
            (false, false) => {
                (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y) && approx_eq!(f64, x, y)
            }
            (false, true) => (0.0..=1.0).contains(&x) && diff.1 == slope.1,
            (true, false) => (0.0..=1.0).contains(&y) && diff.0 == slope.0,
        }
    }
    fn get_points(&self) -> HashSet<(i32, i32)> {
        let s = self.slope_reduced();
        let (mut x, mut y) = (self.start.0, self.start.1);
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        loop {
            hs.insert((x, y));
            if (x, y) == self.end {
                break;
            }
            x += s.0;
            y += s.1;
        }
        hs
    }
}

fn parse_input(input: &mut String) -> Vec<Line> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| l.into())
        .collect()
}

fn part1(input: &mut String) -> i64 {
    let lines = parse_input(input);
    let straight_lines: Vec<Line> = lines
        .into_iter()
        .filter(|l| l.start.0 == l.end.0 || l.start.1 == l.end.1)
        .collect();
    let mut intersections: HashMap<(i32, i32), i32> = HashMap::new();
    for (l1) in straight_lines {
        for point in l1.get_points() {
            // println!("{:?} {:?} int at {:?}", l1, l2, point);
            *intersections.entry(point).or_insert(0) += 1;
        }
    }
    intersections.iter().filter(|(&k, &v)| v >= 2).count() as i64
}

fn part2(input: &mut String) -> i64 {
    let lines = parse_input(input);
    let mut intersections: HashMap<(i32, i32), i32> = HashMap::new();
    for (l1) in lines {
        for point in l1.get_points() {
            // println!("{:?} {:?} int at {:?}", l1, l2, point);
            *intersections.entry(point).or_insert(0) += 1;
        }
    }
    intersections.iter().filter(|(&k, &v)| v >= 2).count() as i64
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input);

    println!("{} {}\n", part1(&mut input), part2(&mut input));
    Ok(())
}
