use crate::common::{downloader::download_notes, math::lcm};

fn part1() -> anyhow::Result<()> {
    let input = download_notes(1, 3, 1)?;
    let mut res = 0;
    for line in input.lines() {
        if let Some((x_part, y_part)) = line.split_once(' ') {
            let x = x_part
                .strip_prefix("x=")
                .and_then(|s| s.parse::<i64>().ok());
            let y = y_part
                .strip_prefix("y=")
                .and_then(|s| s.parse::<i64>().ok());
            if let (Some(mut x), Some(mut y)) = (x, y) {
                x -= 1;
                y -= 1;
                let p = x + y + 1;
                x = (x + 100) % p;
                y = ((y - 100) % p + p) % p;
                x += 1;
                y += 1;
                res += x + 100 * y;
            }
        }
    }
    println!("{}", res);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let mut delta = 0;
    let mut period = 1;
    for line in download_notes(1, 3, 2)?.lines() {
        if let Some((x_part, y_part)) = line.split_once(' ') {
            let x = x_part
                .strip_prefix("x=")
                .and_then(|s| s.parse::<i64>().ok());
            let y = y_part
                .strip_prefix("y=")
                .and_then(|s| s.parse::<i64>().ok());
            if let (Some(mut x), Some(mut y)) = (x, y) {
                x -= 1;
                y -= 1;
                let p = x + y + 1;
                while (y - delta) % p != 0 {
                    delta += period;
                }
                period = lcm(period, p);
            }
        }
    }
    println!("{}", delta);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let mut delta = 0;
    let mut period = 1;
    for line in download_notes(1, 3, 3)?.lines() {
        if let Some((x_part, y_part)) = line.split_once(' ') {
            let x = x_part
                .strip_prefix("x=")
                .and_then(|s| s.parse::<i64>().ok());
            let y = y_part
                .strip_prefix("y=")
                .and_then(|s| s.parse::<i64>().ok());
            if let (Some(mut x), Some(mut y)) = (x, y) {
                x -= 1;
                y -= 1;
                let p = x + y + 1;
                while (y - delta) % p != 0 {
                    delta += period;
                }
                period = lcm(period, p);
            }
        }
    }
    println!("{}", delta);
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
