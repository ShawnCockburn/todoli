use inquire::Text;

use crate::{
    prelude::*,
    storage::{self, models::todo::NewTodo},
};

pub fn handle() -> Result<()> {
    let conn = &mut storage::establish_connection();

    let name = Text::new("New todo name:").prompt();

    match name {
        Ok(name) => storage::models::todo::create_todo(conn, NewTodo { title: name })?,
        Err(_) => println!("An error happened!"),
    }

    Ok(())
}
