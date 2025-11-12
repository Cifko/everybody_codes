use crate::{common::downloader::download_notes, stories::the_entertainment_hub::EVENT};
use itertools::Itertools;

const QUEST: usize = 1;

fn coin_toss(grid: &Vec<Vec<char>>, start_col: usize, instructions: &Vec<char>) -> usize {
    let mut i = start_col * 2;
    let mut j = 0;
    for g in grid {
        if g[i] == '*' {
            match instructions[j] {
                'L' if i == 0 => i += 1,
                'L' => i -= 1,
                'R' if i == g.len() - 1 => i -= 1,
                'R' => i += 1,
                _ => {}
            }
            j += 1;
        }
    }
    if i + 1 > start_col {
        i + 1 - start_col
    } else {
        0
    }
}

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 1)?;
    let mut lines = notes.lines();
    let mut grid = vec![];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut res = 0;
    for (index, line) in lines.enumerate() {
        res += coin_toss(&grid, index, &line.chars().collect::<Vec<char>>());
    }
    println!("{res}");
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 2)?;
    let mut lines = notes.lines();
    let mut grid = vec![];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let mut res = 0;
    for line in lines {
        res += (0..=grid[0].len() / 2)
            .map(|index| coin_toss(&grid, index, &line.chars().collect::<Vec<char>>()))
            .max()
            .unwrap();
    }
    println!("{res}");
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 3)?;
    let mut lines = notes.lines();
    let mut grid = vec![];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let instructions: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut min = usize::MAX;
    let mut max = 0;
    let time_start = std::time::Instant::now();
    for perm in (0..=grid[0].len() / 2).permutations(instructions.len()) {
        let mut current = 0;
        for (index, &line_index) in perm.iter().enumerate() {
            current += coin_toss(&grid, line_index, &instructions[index]);
        }
        if current < min {
            min = current;
        }
        if current > max {
            max = current;
        }
    }
    println!("{min} {max}");
    println!("Time: {:?}", time_start.elapsed());
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
