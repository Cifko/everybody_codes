mod common;
mod echoes_of_enigmatus;
mod events;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    events::run()
}
