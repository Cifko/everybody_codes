mod quest_1;
mod quest_2;
mod quest_3;
mod quest_4;
mod quest_5;
mod quest_6;

pub fn run(quest: usize, part: usize) -> anyhow::Result<()> {
    match quest {
        1 => quest_1::run(part),
        2 => quest_2::run(part),
        3 => quest_3::run(part),
        4 => quest_4::run(part),
        5 => quest_5::run(part),
        6 => quest_6::run(part),
        _ => anyhow::bail!("Unknown quest {}", quest),
    }
}
