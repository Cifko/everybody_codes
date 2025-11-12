mod quest_1;

const EVENT: usize = 2;

pub fn run(quest: usize, part: usize) -> anyhow::Result<()> {
    match quest {
        1 => quest_1::run(part)?,
        _ => println!("Unknown quest {}", quest),
    }
    Ok(())
}
