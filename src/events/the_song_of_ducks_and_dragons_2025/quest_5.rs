use crate::common::downloader::download_notes;

fn sword(nums: &Vec<i64>) -> Vec<Vec<Option<i64>>> {
    let mut sword: Vec<Vec<Option<i64>>> = vec![];
    for &n in nums {
        let mut inserted = false;
        for level in sword.iter_mut() {
            let x = level[1].unwrap();
            if n < x && level[0].is_none() {
                level[0] = Some(n);
                inserted = true;
                break;
            }
            if n > x && level[2].is_none() {
                level[2] = Some(n);
                inserted = true;
                break;
            }
        }
        if !inserted {
            sword.push(vec![None, Some(n), None]);
        }
    }
    sword
}

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 5, 1)?;
    let nums = notes
        .split_once(":")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let sword = sword(&nums);
    let result = sword
        .iter()
        .fold(String::new(), |acc, e| format!("{acc}{}", e[1].unwrap()));
    println!("{}", result);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 5, 2)?;
    let mut result = vec![];
    for line in notes.lines() {
        let nums = line
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let sword = sword(&nums);
        let val = sword
            .iter()
            .fold(String::new(), |acc, e| format!("{acc}{}", e[1].unwrap()));
        result.push(val.parse::<i64>().unwrap());
    }
    println!(
        "{}",
        result.iter().max().unwrap() - result.iter().min().unwrap()
    );
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 5, 3)?;
    let mut result = vec![];
    for line in notes.lines() {
        let (key, nums) = line.split_once(":").unwrap();
        let key = key.parse::<i64>().unwrap();
        let nums = nums
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let sword = sword(&nums);
        let val = sword
            .iter()
            .fold(String::new(), |acc, e| format!("{acc}{}", e[1].unwrap()));
        let sword = sword
            .into_iter()
            .map(|level| {
                let x = level.into_iter().fold(String::new(), |acc, opt| match opt {
                    Some(v) => format!("{acc}{v}"),
                    None => format!("{acc}"),
                });
                x.parse::<i64>().unwrap()
            })
            .collect::<Vec<_>>();
        result.push((val.parse::<i64>().unwrap(), sword, key));
    }
    result.sort();
    let mut res = 0;
    for (i, (_val, _sword, key)) in result.into_iter().rev().enumerate() {
        res += (i as i64 + 1) * key;
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
