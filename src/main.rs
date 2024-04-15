#![allow(unused)]

use inquire::{InquireError, Select};

use crate::prelude::*;

mod actions;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let options = vec!["List Todos", "Add Todo", "Complete Todo", "Quit"];

    let ans: core::result::Result<&str, InquireError> =
        Select::new("What would you like to do?", options).prompt();

    match ans {
        Ok("List Todos") => actions::list_todos::list_todos(),
        Ok("Add Todo") => actions::add_todo::add_todo(),
        Ok("Complete Todo") => actions::complete_todo::complete_todo(),
        Ok("Quit") => actions::quit::quit(),
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
        _ => {
            eprintln!("Invalid option selected");
            return Ok(());
        }
    }?;

    Ok(())
}
