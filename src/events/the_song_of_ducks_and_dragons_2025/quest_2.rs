use std::{
    fmt::Display,
    ops::{AddAssign, DivAssign},
};

use crate::common::downloader::download_notes;

#[derive(Copy, Clone)]
struct ComplexNumber {
    a: i64,
    b: i64,
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.a, self.b)
    }
}

impl AddAssign<&ComplexNumber> for ComplexNumber {
    fn add_assign(&mut self, other: &ComplexNumber) {
        self.a += other.a;
        self.b += other.b;
    }
}

impl DivAssign<&ComplexNumber> for ComplexNumber {
    fn div_assign(&mut self, other: &ComplexNumber) {
        self.a /= other.a;
        self.b /= other.b;
    }
}

impl ComplexNumber {
    fn square(&mut self) {
        let a = self.a;
        let b = self.b;
        self.a = a * a - b * b;
        self.b = 2 * a * b;
    }
}

fn part1() -> anyhow::Result<()> {
    let notes = download_notes(2025, 2, 1)?;
    let (a, b) = notes
        .strip_prefix("A=[")
        .and_then(|s| s.strip_suffix("]"))
        .unwrap()
        .split_once(",")
        .unwrap();
    let a = a.parse::<i64>()?;
    let b = b.parse::<i64>()?;

    let mut r = ComplexNumber { a: 0, b: 0 };
    let a = ComplexNumber { a, b };
    let t = ComplexNumber { a: 10, b: 10 };
    for _ in 0..3 {
        r.square();
        r /= &t;
        r += &a;
    }
    println!("{}", r);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let notes = download_notes(2025, 2, 2)?;
    let (a, b) = notes
        .strip_prefix("A=[")
        .and_then(|s| s.strip_suffix("]"))
        .unwrap()
        .split_once(",")
        .unwrap();
    let a = a.parse::<i64>()?;
    let b = b.parse::<i64>()?;
    let t = ComplexNumber {
        a: 100000,
        b: 100000,
    };

    let mut res = 0;
    for y in (b..=b + 1000).step_by(10) {
        for x in (a..=a + 1000).step_by(10) {
            let mut p = ComplexNumber { a: x, b: y };
            let q = p;
            let mut good = true;
            for _ in 0..100 {
                if p.a.abs() > 1_000_000 || p.b.abs() > 1_000_000 {
                    good = false;
                    break;
                }
                p.square();
                p /= &t;
                p += &q;
            }
            if good {
                res += 1;
            }
        }
    }
    println!("{}", res);
    Ok(())
}

fn part3() -> anyhow::Result<()> {
    let notes = download_notes(2025, 2, 3)?;
    let (a, b) = notes
        .strip_prefix("A=[")
        .and_then(|s| s.strip_suffix("]"))
        .unwrap()
        .split_once(",")
        .unwrap();
    let a = a.parse::<i64>()?;
    let b = b.parse::<i64>()?;
    let t = ComplexNumber {
        a: 100000,
        b: 100000,
    };

    let mut res = 0;
    for y in b..=b + 1000 {
        for x in a..=a + 1000 {
            let mut p = ComplexNumber { a: x, b: y };
            let q = p;
            let mut good = true;
            for _ in 0..100 {
                if p.a.abs() > 1_000_000 || p.b.abs() > 1_000_000 {
                    good = false;
                    break;
                }
                p.square();
                p /= &t;
                p += &q;
            }
            if good {
                res += 1;
            }
        }
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
