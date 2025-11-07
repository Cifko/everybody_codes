use crate::{common::downloader::download_notes, println_part, println_quest};

fn potion(c: char) -> u64 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2024, 1, 1)?;
    let sum: u64 = notes.chars().fold(0, |acc, x| acc + potion(x));
    println_part!(1, "{}", sum);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2024, 1, 2)?;
    let sum: u64 = notes
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .fold(0, |acc, pair| {
            acc + potion(pair[0]) + potion(pair[1]) + if pair.contains(&'x') { 0 } else { 2 }
        });
    println_part!(2, "{}", sum);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2024, 1, 3)?;
    let sum: u64 = notes
        .lines()
        .map(|line| {
            line.chars()
                .filter(|&c| c.is_alphabetic())
                .collect::<Vec<_>>()
                .chunks(3)
                .fold(0, |acc, triplet| {
                    acc + potion(triplet[0])
                        + potion(triplet[1])
                        + potion(triplet[2])
                        + match triplet.iter().filter(|&&c| c == 'x').count() {
                            1 => 2,
                            2 | 3 => 0,
                            _ => 6,
                        }
                })
        })
        .sum();
    println_part!(3, "{}", sum);
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    println_quest!(1);
    part1()?;
    part2()?;
    part3()?;
    Ok(())
}
