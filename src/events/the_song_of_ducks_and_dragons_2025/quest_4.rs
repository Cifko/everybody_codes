use std::collections::{HashMap, HashSet};

use crate::common::downloader::download_notes;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 4, 1)?;
    let cogs = notes
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", 2025 * cogs.first().unwrap() / cogs.last().unwrap());
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 4, 2)?;
    let cogs = notes
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = (10_000_000_000_000 * cogs.last().unwrap() + cogs.first().unwrap() - 1)
        / cogs.first().unwrap();
    println!("{}", n);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 4, 3)?;
    let cogs = notes
        .lines()
        .map(|s| {
            s.split('|')
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut res = 100.0 * cogs[0][0];
    for cog in cogs.iter().skip(1) {
        if cog.len() == 2 {
            res = (res / cog[0]) * cog[1];
        } else {
            res /= cog[0];
        }
    }
    println!("{}", res as i64);
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
