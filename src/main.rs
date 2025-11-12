use crate::{
    events::{the_kingdom_of_algorithms_2024, the_song_of_ducks_and_dragons_2025},
    stories::echoes_of_enigmatus,
};

mod common;
mod events;
mod stories;

#[derive(clap::Parser)]
#[command(name = "everybody_codes")]
struct Args {
    #[arg(long, required = true)]
    event: usize,
    #[arg(long, required = true)]
    quest: usize,
    #[arg(long, required = true)]
    part: usize,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let args = <Args as clap::Parser>::parse();
    let start_time = std::time::Instant::now();
    let res = match args.event {
        1 => echoes_of_enigmatus::run(args.quest, args.part),
        2024 => the_kingdom_of_algorithms_2024::run(args.quest, args.part),
        2025 => the_song_of_ducks_and_dragons_2025::run(args.quest, args.part),
        _ => anyhow::bail!("Unknown event {}", args.event),
    };
    println!("Time elapsed: {:?}", start_time.elapsed());
    res
}
