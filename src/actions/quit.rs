use crate::prelude::*;

pub fn handle() -> Result<()> {
    println!("Quitting");

    std::process::exit(0);

    Ok(())
}
