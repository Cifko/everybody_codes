mod quest_1;
mod quest_2;
mod quest_3;

pub fn run(quest: usize, part: usize) -> anyhow::Result<()> {
    match quest {
        1 => quest_1::run(part)?,
        2 => quest_2::run(part)?,
        3 => quest_3::run(part)?,
        _ => println!("Unknown quest {}", quest),
    }
    Ok(())
}
