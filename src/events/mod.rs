use anyhow::Ok;

use crate::echoes_of_enigmatus;

mod year_2024_the_kingdom_of_algorithms;

pub fn run() -> anyhow::Result<()> {
    echoes_of_enigmatus::run()?;
    // year_2024_the_kingdom_of_algorithms::run()
    Ok(())
}
