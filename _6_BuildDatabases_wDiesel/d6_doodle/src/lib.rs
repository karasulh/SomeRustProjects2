//diesel setup
//password: admin
//psql -d doodle
//diesel migration generate create_initial_tables
//diesel migration run => it runs up.sql
//diesel migration redo => it runs down.sql
//diesel migration generate add_owner_to_poll
//psql -d doodle -U postgres
//cargo run -- new_user bugra 12345
//cargo run -- view_users
//cargo run -- new_poll "how old are you?" "<5,<10,<15,>=15" -u 1
//cargo run -- new_poll "do you have animals?" "cat,dog,fish,none,other" -u 1
//cargo run -- view_polls
//cargo run -- respond_poll 1 1 3
//cargo run -- respond_poll 2 1 2
//cargo run -- poll_results 1
//cargo run -- poll_results 2

#[macro_use]
extern crate diesel;
use diesel::{PgConnection, Connection};
use diesel::prelude::*;
pub mod schema;
pub mod models;
use std::env;



pub fn create_connection()-> Result<PgConnection,failure::Error>{ //anyhow::Result<PgConnection>{
    dotenv::dotenv().ok();
    Ok(PgConnection::establish("postgres://postgres:admin@localhost/doodle")?)/*&env::var("DATABASE_URL")?)?*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
