
use crate::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::storage::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::storage::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTodo {
    pub title: String,
}

pub fn list_todos(conn: &mut SqliteConnection) -> Result<Vec<Todo>> {
    use crate::storage::schema::todos::dsl::*;

    match todos.select(Todo::as_select()).load(conn) {
        Ok(found_todos) => Ok(found_todos),
        Err(e) => {
            eprintln!("Error loading todos: {}", e);
            Err(Error::Generic("Error loading todos".to_string()))
        }
    }
}