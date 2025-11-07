use crate::common::downloader::download_notes;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 1, 1)?;
    let notes = notes.lines().collect::<Vec<&str>>();

    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 1, 2)?;
    let notes = notes.lines().collect::<Vec<&str>>();
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 1, 3)?;
    let notes = notes.lines().collect::<Vec<&str>>();
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
