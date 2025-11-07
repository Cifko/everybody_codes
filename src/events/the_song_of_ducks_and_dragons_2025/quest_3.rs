use std::collections::{HashMap, HashSet};

use crate::common::downloader::download_notes;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 3, 1)?;
    let crates = notes
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<HashSet<_>>();
    println!("{}", crates.iter().sum::<i64>());
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 3, 2)?;
    let crates = notes
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<HashSet<_>>();
    let mut sorted: Vec<_> = crates.into_iter().collect();
    sorted.sort();
    println!("{}", sorted.iter().take(20).sum::<i64>());
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 3, 3)?;
    let crates =
        notes
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .fold(HashMap::new(), |mut map, n| {
                *map.entry(n).or_insert(0) += 1;
                map
            });
    println!("{}", crates.values().max().unwrap_or(&0));
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
