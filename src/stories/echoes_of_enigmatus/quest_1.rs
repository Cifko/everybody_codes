use std::collections::HashMap;

use crate::common::downloader::download_notes;

fn eni1(n: u64, e: u64, m: u64) -> u64 {
    let mut r = 1;
    let mut result = String::new();
    for _ in 0..e {
        r = (r * n) % m;
        result = format!("{}{}", r, result);
    }
    result.parse::<u64>().unwrap()
}

fn part1() -> anyhow::Result<()> {
    let input1 = download_notes(1, 1, 1)?;
    let mut max_res = 0;
    for line in input1.lines() {
        let mut a = 0u64;
        let mut b = 0u64;
        let mut c = 0u64;
        let mut x = 0u64;
        let mut y = 0u64;
        let mut z = 0u64;
        let mut m = 0u64;

        for part in line.split_whitespace() {
            let (key, value) = part.split_once('=').unwrap();
            let val = value.parse::<u64>().unwrap();
            match key {
                "A" => a = val,
                "B" => b = val,
                "C" => c = val,
                "X" => x = val,
                "Y" => y = val,
                "Z" => z = val,
                "M" => m = val,
                _ => {}
            }
        }

        let res = eni1(a, x, m) + eni1(b, y, m) + eni1(c, z, m);
        if res > max_res {
            max_res = res;
        }
    }
    println!("{}", max_res);
    Ok(())
}

fn eni2(n: u64, e: u64, m: u64) -> u64 {
    let mut r = 1;
    let mut result = Vec::new();
    let mut ex = e - 5;
    let mut nn = n;
    while ex > 0 {
        if ex & 1 == 1 {
            r = (r * nn) % m;
        }
        ex >>= 1;
        nn = nn * nn % m;
    }
    for _ in 0..5 {
        r = (r * n) % m;
        result.push(r);
    }
    result.reverse();
    result
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn part2() -> anyhow::Result<()> {
    let input2 = download_notes(1, 1, 2)?;
    let mut max_res = 0u64;
    for line in input2.lines() {
        let mut a = 0u64;
        let mut b = 0u64;
        let mut c = 0u64;
        let mut x = 0u64;
        let mut y = 0u64;
        let mut z = 0u64;
        let mut m = 0u64;

        for part in line.split_whitespace() {
            let (key, value) = part.split_once('=').unwrap();
            let val = value.parse::<u64>().unwrap();
            match key {
                "A" => a = val,
                "B" => b = val,
                "C" => c = val,
                "X" => x = val,
                "Y" => y = val,
                "Z" => z = val,
                "M" => m = val,
                _ => {}
            }
        }

        let res = eni2(a, x, m) + eni2(b, y, m) + eni2(c, z, m);
        if res > max_res {
            max_res = res;
        }
    }
    println!("{}", max_res);
    Ok(())
}

fn eni3(n: u64, e: u64, m: u64) -> u64 {
    let mut v = HashMap::new();
    let mut rem = vec![1u64];
    let mut r = 1;
    let mut res = 0;
    v.insert(1, 0);
    for i in 1..=e {
        r = (r * n) % m;
        if v.contains_key(&r) {
            let cycle_start = v[&r];
            let cycle_length = i - cycle_start;
            let cycle_sum = rem[cycle_start as usize..].iter().sum::<u64>();
            res += (e - (i - 1)) / cycle_length * cycle_sum;
            res += rem[cycle_start as usize..(cycle_start + (e - (i - 1)) % cycle_length) as usize]
                .iter()
                .sum::<u64>();
            break;
        }
        v.insert(r, i);
        rem.push(r);
        res += r;
    }
    res
}

fn part3() -> anyhow::Result<()> {
    let input3 = download_notes(1, 1, 3)?;
    let mut max_res = 0u64;
    for line in input3.lines() {
        let mut a = 0u64;
        let mut b = 0u64;
        let mut c = 0u64;
        let mut x = 0u64;
        let mut y = 0u64;
        let mut z = 0u64;
        let mut m = 0u64;

        for part in line.split_whitespace() {
            let (key, value) = part.split_once('=').unwrap();
            let val = value.parse::<u64>().unwrap();
            match key {
                "A" => a = val,
                "B" => b = val,
                "C" => c = val,
                "X" => x = val,
                "Y" => y = val,
                "Z" => z = val,
                "M" => m = val,
                _ => {}
            }
        }

        let res = eni3(a, x, m) + eni3(b, y, m) + eni3(c, z, m);
        if res > max_res {
            max_res = res;
        }
    }
    println!("{}", max_res);
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
