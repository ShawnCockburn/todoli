use crate::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, sqlite};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(connection: &mut impl MigrationHarness<sqlite::Sqlite>) -> Result<()> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error running migrations: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

pub mod models;
pub mod schema;

pub fn setup() -> Result<()> {
    let db_file = "./todos.db";

    let conn = &mut get_connection(&db_file);

    run_migrations(conn)?;

    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    let db_file = "./todos.db";

    get_connection(&db_file)
}

fn get_connection(db_file: &str) -> SqliteConnection {
    let conn = SqliteConnection::establish(db_file);

    match conn {
        Ok(conn) => {
            return conn;
        }
        Err(e) => {
            println!("Error connecting to database, exiting");

            eprintln!("{}", e);

            std::process::exit(1);
        }
    }
}
