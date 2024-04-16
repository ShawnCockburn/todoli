use inquire::Confirm;

use crate::{
    prelude::*,
    storage::{self, models::todo::Todo},
};

fn render_header() -> Result<()> {
    println!("Todos:\n");
    println!(" Status | Title");
    println!(" -------+------------------------");
    Ok(())
}

fn render_footer() -> Result<()> {
    println!(" -------+------------------------");
    println!("\nend of list\n");
    Ok(())
}

fn render_todo(todo: Todo) -> Result<()> {
    let checked_emoji = if todo.completed { "✅" } else { "❌" };
    println!("   {}   | {}", checked_emoji, todo.title);
    Ok(())
}

fn render_todo_list(todos: Vec<Todo>) -> Result<()> {
    if todos.len() == 0 {
        println!("No todos found");
        return Ok(());
    }

    render_header()?;
    for todo in todos {
        render_todo(todo)?;
    }
    render_footer()?;
    Ok(())
}

pub fn handle() -> Result<()> {
    let conn = &mut storage::establish_connection();

    let todos = storage::models::todo::list_todos(conn)?;

    render_todo_list(todos)?;

    let _ = Confirm::new("Press enter to continue")
        .with_default(true)
        .with_default_value_formatter(&|i| match i {
            _ => "".to_string(),
        })
        .prompt();

    Ok(())
}
