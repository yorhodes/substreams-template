use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Mailbox", "abi/mailbox.json")?
        .generate()?
        .write_to_file("src/abi/mailbox.rs")?;

    Ok(())
}
