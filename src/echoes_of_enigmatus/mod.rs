mod quest_1;
mod quest_2;
mod quest_3;

pub fn run() -> anyhow::Result<()> {
    println!("Echoes of Enigmatus");
    quest_1::run()?;
    quest_2::run()?;
    quest_3::run()?;
    Ok(())
}
