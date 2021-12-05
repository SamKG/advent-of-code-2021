#![allow(unused)]
use bitstring::BitString;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Read};
extern crate ndarray;
use ndarray::Array2;
extern crate itertools;
use itertools::Itertools;

type Grid = Array2<i64>;
type GridMask = Array2<bool>;

#[derive(Clone)]
struct Bingo {
    board: Grid,
    mask: GridMask,
}

impl Bingo {
    fn is_won(&self) -> bool {
        for col in self.mask.columns() {
            if col.iter().all(|&x| x) {
                return true;
            }
        }
        for row in self.mask.rows() {
            if row.iter().all(|&x| x) {
                return true;
            }
        }
        return false;
    }
    fn mark(&mut self, val: i64) {
        for (i, row) in self.board.outer_iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == val {
                    self.mask[(i, j)] = true;
                }
            }
        }
    }
    fn get_unmarked(&self) -> Vec<i64> {
        let mut vals = Vec::new();
        for (i, row) in self.board.outer_iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if !self.mask[(i, j)] {
                    vals.push(*val);
                }
            }
        }
        return vals;
    }
}

fn parse_input(input: &mut String) -> (Vec<i64>, Vec<Bingo>) {
    let mut lines = input.split("\n").filter(|s| !s.is_empty());

    let mut numbers: Vec<i64> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut input_numbers = lines.flat_map(|line| {
        line.split(" ")
            .filter(|c| !c.is_empty())
            .map(|num| num.parse::<i64>().unwrap())
    });

    let boards: Vec<Bingo> = input_numbers
        .chunks(25)
        .into_iter()
        .map(|chunk| {
            let mut bingo = Bingo {
                mask: GridMask::from_elem((5, 5), false),
                board: Grid::from_elem((5, 5), 0),
            };
            for (idx, val) in chunk.enumerate() {
                let (i, j) = (idx / 5, idx % 5);
                bingo.board[[i, j]] = val;
            }
            bingo
        })
        .collect();

    return (numbers, boards);
}

fn part1(input: &mut String) -> i64 {
    let (numbers, mut boards) = parse_input(input);
    for num in numbers {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.is_won() {
                let board_sum: i64 = board.get_unmarked().iter().sum();
                return num * board_sum;
            }
        }
    }
    return 0;
}

fn part2(input: &mut String) -> i64 {
    let (numbers, mut boards) = parse_input(input);
    let mut won_boards: Vec<(i64)> = Vec::new();
    for num in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            board.mark(num);
            if board.is_won() {
                won_boards.push((board.get_unmarked().iter().sum::<i64>() * num));
            }
        }
        boards = boards
            .iter()
            .filter(|board| !board.is_won())
            .cloned()
            .collect();
    }
    return *won_boards.last().unwrap();
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input);

    println!("{} {}", part1(&mut input), part2(&mut input));
    return Ok(());
}
