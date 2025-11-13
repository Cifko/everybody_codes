use std::collections::HashSet;

use crate::{common::downloader::download_notes, stories::the_entertainment_hub::EVENT};
use itertools::Itertools;

const QUEST: usize = 3;

#[derive(Debug)]
struct Dice {
    _id: usize,
    faces: Vec<i64>,
    seed: usize,
    pulse: usize,
    roll_number: usize,
    spin: usize,
}

impl Dice {
    fn new(id: usize, faces: Vec<i64>, seed: usize) -> Self {
        Self {
            _id: id,
            faces,
            seed: seed,
            pulse: seed,
            roll_number: 0,
            spin: 0,
        }
    }

    pub fn roll(&mut self) -> i64 {
        self.roll_number += 1;
        let spin = self.roll_number * self.pulse;
        self.spin += spin;
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse = self.pulse + 1 + self.roll_number + self.seed;
        self.faces[self.spin % self.faces.len()]
    }
}

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 1)?;
    let lines = notes.lines();
    let mut dice = vec![];
    for line in lines {
        let numbers: Vec<i32> = regex::Regex::new(r"-?\d+")
            .unwrap()
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();
        dice.push(Dice::new(
            numbers[0] as usize,
            numbers[1..numbers.len() - 1]
                .iter()
                .map(|&n| n as i64)
                .collect(),
            numbers[numbers.len() - 1] as usize,
        ));
    }
    let mut total_points = 0;
    while total_points < 10_000 {
        for d in dice.iter_mut() {
            let roll = d.roll();
            total_points += roll;
        }
    }
    println!("{}", dice[0].roll_number);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 2)?;
    let mut lines = notes.lines();
    let mut dice = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let numbers: Vec<i32> = regex::Regex::new(r"-?\d+")
            .unwrap()
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();
        dice.push(Dice::new(
            numbers[0] as usize,
            numbers[1..numbers.len() - 1]
                .iter()
                .map(|&n| n as i64)
                .collect(),
            numbers[numbers.len() - 1] as usize,
        ));
    }
    let race = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect_vec();
    let mut finished = vec![];
    let mut position = vec![0; dice.len()];
    while finished.len() < dice.len() {
        for (i, die) in dice.iter_mut().enumerate() {
            let roll = die.roll();
            if race.len() > position[i] {
                if roll == race[position[i]] {
                    position[i] += 1;
                    if position[i] == race.len() {
                        finished.push(i);
                    }
                }
            }
        }
    }
    println!(
        "{}",
        finished.iter().map(|i| format!("{}", i + 1)).join(",")
    );
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 3)?;
    let mut lines = notes.lines();
    let mut dice = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let numbers: Vec<i32> = regex::Regex::new(r"-?\d+")
            .unwrap()
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();
        dice.push(Dice::new(
            numbers[0] as usize,
            numbers[1..numbers.len() - 1]
                .iter()
                .map(|&n| n as i64)
                .collect(),
            numbers[numbers.len() - 1] as usize,
        ));
    }
    let mut coins = vec![];
    let mut check_next = HashSet::new();
    for (y, line) in lines.enumerate() {
        let nums = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        for x in 0..nums.len() {
            for d in 0..dice.len() {
                check_next.insert((y, x, d));
            }
        }
        coins.push(nums);
    }
    let mut coins_in_price = HashSet::new();
    while !check_next.is_empty() {
        let rolls = dice.iter_mut().map(|d| d.roll()).collect_vec();
        let mut next_check = HashSet::new();
        for &(y, x, d) in check_next.iter() {
            let roll = rolls[d];
            if coins[y][x] == roll as i32 {
                coins_in_price.insert((y, x));
                // check neighbors
                let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];
                for (di, dj) in directions.iter() {
                    let ni = y as isize + di;
                    let nj = x as isize + dj;
                    if ni >= 0
                        && ni < coins.len() as isize
                        && nj >= 0
                        && nj < coins[0].len() as isize
                    {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if coins[ni][nj] != 0 {
                            next_check.insert((ni, nj, d));
                        }
                    }
                }
            }
        }
        check_next = next_check;
    }
    println!("{}", coins_in_price.len());
    Ok(())
}

pub fn run(part: usize) -> anyhow::Result<()> {
    match part {
        1 => part1(),
        2 => part2(),
        3 => part3(),
        _ => anyhow::bail!("Unknown part {}", part),
    }
}
