use crate::common::downloader::download_notes;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 1, 1)?;
    let notes = notes.lines().collect::<Vec<&str>>();
    let names = notes[0].split(",").collect::<Vec<&str>>();
    let moves = notes[2].split(",").collect::<Vec<&str>>();
    let mut index = 0;
    for mv in moves {
        let c = mv.chars().next().unwrap();
        let p = mv[1..].parse::<i64>()?;
        if c == 'L' {
            index = index - p;
        } else {
            index = index + p;
        }
        if index < 0 {
            index = 0;
        } else if index >= names.len() as i64 {
            index = names.len() as i64 - 1;
        }
    }
    println!("{}", names[index as usize % names.len()]);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 1, 2)?;
    let notes = notes.lines().collect::<Vec<&str>>();
    let names = notes[0].split(",").collect::<Vec<&str>>();
    let moves = notes[2].split(",").collect::<Vec<&str>>();
    let mut index = 0;
    for mv in moves {
        let c = mv.chars().next().unwrap();
        let p = mv[1..].parse::<i64>()?;
        if c == 'L' {
            index = index - p;
        } else {
            index = index + p;
        }
    }
    println!("{}", names[index as usize % names.len()]);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 1, 3)?;
    let notes = notes.lines().collect::<Vec<&str>>();
    let mut names = notes[0].split(",").collect::<Vec<&str>>();
    let moves = notes[2].split(",").collect::<Vec<&str>>();
    let l = names.len();
    for mv in moves {
        let c = mv.chars().next().unwrap();
        let p = mv[1..].parse::<usize>()?;
        if c == 'L' {
            names.swap(0, (l - p % l) % l);
        } else {
            names.swap(0, p % l);
        }
    }
    println!("{}", names[0]);
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
