use crate::{prelude::*, storage};

pub fn handle() -> Result<()> {
    println!("Listing todos");

    let conn = &mut storage::establish_connection();

    let todos = storage::models::todo::list_todos(conn)?;

    for todo in todos {
        println!("{}: {}", todo.id, todo.title);
    }

    Ok(())
}
