use std::collections::{HashMap, HashSet};

use crate::common::downloader::download_notes;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 7, 1)?;
    let lines = notes.lines().collect::<Vec<_>>();
    let names = lines[0].split(',').collect::<Vec<_>>();
    let rules = lines[2..]
        .iter()
        .map(|line| {
            let (c, rest) = line.split_once(" > ").unwrap();
            let choices = rest
                .split(',')
                .map(|s| s.chars().next().unwrap())
                .collect::<HashSet<_>>();
            (c.chars().next().unwrap(), choices)
        })
        .collect::<HashMap<_, _>>();
    for name in names {
        let mut good = true;
        for i in 1..name.len() {
            good &=
                rules[&name.chars().nth(i - 1).unwrap()].contains(&name.chars().nth(i).unwrap());
        }
        if good {
            println!("{}", name);
        }
    }
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 7, 2)?;
    let lines = notes.lines().collect::<Vec<_>>();
    let names = lines[0].split(',').collect::<Vec<_>>();
    let rules = lines[2..]
        .iter()
        .map(|line| {
            let (c, rest) = line.split_once(" > ").unwrap();
            let choices = rest
                .split(',')
                .map(|s| s.chars().next().unwrap())
                .collect::<HashSet<_>>();
            (c.chars().next().unwrap(), choices)
        })
        .collect::<HashMap<_, _>>();
    let mut res = 0;
    for (i, name) in names.iter().enumerate() {
        let mut good = true;
        for i in 1..name.len() {
            good &=
                rules[&name.chars().nth(i - 1).unwrap()].contains(&name.chars().nth(i).unwrap());
        }
        if good {
            res += i + 1;
        }
    }
    println!("{}", res);
    Ok(())
}

fn count_pos(s: char, len: usize, rules: &HashMap<char, HashSet<char>>) -> usize {
    if len > 11 {
        return 0;
    }
    let mut res = if len >= 7 { 1 } else { 0 };
    if rules.contains_key(&s) {
        for next in rules[&s].iter() {
            res += count_pos(*next, len + 1, rules);
        }
    }
    res
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 7, 3)?;
    let lines = notes.lines().collect::<Vec<_>>();
    let names = lines[0].split(',').collect::<Vec<_>>();
    let rules = lines[2..]
        .iter()
        .map(|line| {
            let (c, rest) = line.split_once(" > ").unwrap();
            let choices = rest
                .split(',')
                .map(|s| s.chars().next().unwrap())
                .collect::<HashSet<_>>();
            (c.chars().next().unwrap(), choices)
        })
        .collect::<HashMap<_, _>>();
    let mut good_names = HashSet::new();
    for name in names {
        let mut good = true;
        for i in 1..name.len() {
            good &=
                rules[&name.chars().nth(i - 1).unwrap()].contains(&name.chars().nth(i).unwrap());
        }
        if good {
            good_names.insert(name);
        }
    }
    let good_names_clone = good_names.clone();
    good_names.retain(|name| {
        !good_names_clone
            .iter()
            .any(|other| other != name && name.starts_with(other))
    });
    let mut res = 0;
    for name in good_names {
        res += count_pos(name.chars().last().unwrap(), name.len(), &rules);
    }
    println!("{}", res);
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
