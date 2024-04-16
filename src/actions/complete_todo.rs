use std::{clone, iter};

use inquire::MultiSelect;

use crate::{prelude::*, storage};

pub fn handle() -> Result<()> {
    let conn = &mut storage::establish_connection();

    let todos = storage::models::todo::list_todos(conn)?;

    let options = todos
        .iter()
        .map(|todo| todo.title.clone())
        .collect::<Vec<String>>();

    let pre_selected_options = todos
        .iter()
        .enumerate()
        .filter(|(_, todo)| todo.completed)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let ans = MultiSelect::new("Select which todos to complete", options)
        .with_default(&pre_selected_options)
        .prompt();

    match ans {
        Ok(selected_todo_titles) => {
            for selected_title in selected_todo_titles {
                let todo = &todos.iter().find(|todo| todo.title == selected_title);
                if let Some(todo) = todo {
                    storage::models::todo::complete_todo(conn, todo.id)?;
                }
            }
        }
        Err(_) => println!("An error happened!"),
    }

    Ok(())
}
