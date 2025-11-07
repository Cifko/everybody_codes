use crate::common::downloader::download_notes;

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
    println!("{}", sum);
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
    println!("{}", sum);
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
    println!("{}", sum);
    Ok(())
}

pub fn run(part: usize) -> anyhow::Result<()> {
    match part {
        1 => part1()?,
        2 => part2()?,
        3 => part3()?,
        _ => println!("Unknown part {}", part),
    }
    Ok(())
}
