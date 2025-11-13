use crate::{common::downloader::download_notes, stories::the_entertainment_hub::EVENT};

const QUEST: usize = 2;

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, 1)?;
    let mut index = 0;
    let mut chars = notes.chars();
    while let Some(mut c) = chars.next() {
        while c == ['R', 'G', 'B'][index % 3] {
            c = if let Some(c) = chars.next() {
                c
            } else {
                break;
            };
        }
        index += 1;
    }
    println!("Result: {}", index);
    Ok(())
}

fn next_index_in_shot(index: usize, shot: &Vec<bool>) -> usize {
    let mut next_index = (index + 1) % shot.len();
    while shot[next_index] {
        next_index = (next_index + 1) % shot.len();
    }
    next_index
}

fn solve_part_2_and_3<const M: usize>(part: usize) -> anyhow::Result<()> {
    let notes = download_notes(EVENT, QUEST, part)?;
    let mut index = 0;
    let mut opposite_index = (notes.len() * M) / 2;
    let mut shot = vec![false; notes.len() * M];
    let chars = notes.chars().collect::<Vec<_>>();
    let mut shot_count = 0;
    let mut balloons_left = chars.len() * M;
    loop {
        if balloons_left % 2 == 0 {
            if chars[index % chars.len()] == ['R', 'G', 'B'][shot_count % 3] {
                shot[opposite_index] = true;
                balloons_left -= 1;
                opposite_index = next_index_in_shot(opposite_index, &shot);
            }
        } else {
            opposite_index = next_index_in_shot(opposite_index, &shot);
        }
        shot[index] = true;
        balloons_left -= 1;
        if balloons_left == 0 {
            break;
        }
        index = next_index_in_shot(index, &shot);
        shot_count += 1;
    }
    shot_count += 1;
    println!("{shot_count}");
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    solve_part_2_and_3::<100>(2)
}

fn part3() -> anyhow::Result<()> {
    solve_part_2_and_3::<100000>(3)
}

pub fn run(part: usize) -> anyhow::Result<()> {
    match part {
        1 => part1(),
        2 => part2(),
        3 => part3(),
        _ => anyhow::bail!("Unknown part {}", part),
    }
}
