use inquire::{InquireError, Select};

use crate::prelude::*;

mod actions;
mod error;
mod prelude;
mod storage;
mod utils;

fn render_cli() -> Result<()> {
    println!("Welcome to the Todo App!");
    println!("=========================\n");
    let options = vec!["List Todos", "Add Todo", "Complete Todo", "Quit"];

    loop {
        clearscreen::clear().expect("failed to clear screen");
        let ans: core::result::Result<&str, InquireError> =
            Select::new("What would you like to do?", options.clone()).prompt();

        match ans {
            Ok("List Todos") => actions::list_todos::handle(),
            Ok("Add Todo") => actions::add_todo::handle(),
            Ok("Complete Todo") => actions::complete_todo::handle(),
            Ok("Quit") => actions::quit::handle(),
            Err(e) => {
                eprintln!("Error: {}", e);
                return Ok(());
            }
            _ => {
                eprintln!("Invalid option selected");
                return Ok(());
            }
        }?;
    }
}

fn main() -> Result<()> {
    match storage::setup() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    render_cli()?;

    Ok(())
}
