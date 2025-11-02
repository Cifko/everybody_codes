mod quest_1;
mod quest_2;

pub fn run() -> anyhow::Result<()> {
    println!("The Kingdom of Algorithms - 2024");
    quest_1::run()?;
    quest_2::run()?;
    Ok(())
}
