use std::collections::HashSet;

use fancy_regex::Regex;

use crate::{common::downloader::download_notes, println_part, println_quest};

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2024, 2, 1)?;
    let notes = notes.lines().collect::<Vec<&str>>();
    let words = notes[0]
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();
    let text = notes[2];
    let mut total = 0;
    for word in words {
        total += regex::Regex::new(word).unwrap().find_iter(text).count();
    }
    println_part!(1, "{}", total);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2024, 2, 2)?;
    let notes = notes.lines().collect::<Vec<&str>>();
    let words = notes[0]
        .strip_prefix("WORDS:")
        .ok_or_else(|| anyhow::anyhow!("Failed to parse words"))?
        .split(',')
        .collect::<Vec<&str>>();
    let text = notes[2..].join("\n");
    let mut runic_letters = HashSet::new();
    for word in &words {
        let re = Regex::new(&format!(
            "(?=({}|{}))",
            word,
            word.chars().rev().collect::<String>()
        ))?;
        let matches = re.find_iter(&text).collect::<Result<Vec<_>, _>>().unwrap();
        for m in matches {
            for index in m.start()..m.start() + word.len() {
                runic_letters.insert(index);
            }
        }
    }
    println_part!(2, "{}", runic_letters.len());
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2024, 2, 3)?;
    println_part!(3, "{}", 0);
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    println_quest!(2);
    part1()?;
    part2()?;
    part3()?;
    Ok(())
}
