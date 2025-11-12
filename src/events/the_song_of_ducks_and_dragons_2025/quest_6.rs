use std::collections::{HashMap, HashSet};

use crate::common::downloader::download_notes;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 6, 1)?;
    let mut cnt_a = 0;
    let mut res = 0;
    for c in notes.chars() {
        match c {
            'A' => cnt_a += 1,
            'a' => res += cnt_a,
            _ => {}
        }
    }
    println!("{}", res);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 6, 2)?;
    let mut cnt_a = 0;
    let mut cnt_b = 0;
    let mut cnt_c = 0;
    let mut res = 0;
    for c in notes.chars() {
        match c {
            'A' => cnt_a += 1,
            'a' => res += cnt_a,
            'B' => cnt_b += 1,
            'b' => res += cnt_b,
            'C' => cnt_c += 1,
            'c' => res += cnt_c,
            _ => {}
        }
    }
    println!("{}", res);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 6, 3)?;
    let notes = notes.repeat(1000);
    let mut cnt_a = 0;
    let mut cnt_b = 0;
    let mut cnt_c = 0;
    let mut res = 0u64;
    let mut index = 0;
    for c in notes.chars().take(999) {
        match c {
            'A' => cnt_a += 1,
            'B' => cnt_b += 1,
            'C' => cnt_c += 1,
            _ => {}
        }
    }
    let chars = notes.chars().collect::<Vec<char>>();
    for c in &chars {
        if index + 1000 < notes.len() {
            let out_c = chars[index + 1000];
            match out_c {
                'A' => cnt_a += 1,
                'B' => cnt_b += 1,
                'C' => cnt_c += 1,
                _ => {}
            }
        }
        if index >= 1001 {
            let in_c = chars[index - 1001];
            match in_c {
                'A' => cnt_a -= 1,
                'B' => cnt_b -= 1,
                'C' => cnt_c -= 1,
                _ => {}
            }
        }
        match c {
            'a' => res += cnt_a,
            'b' => res += cnt_b,
            'c' => res += cnt_c,
            _ => {}
        }
        index += 1;
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
