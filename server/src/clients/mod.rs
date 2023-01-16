use diesel::pg::PGConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub mod db {
    pub fn establish_connection() -> PGConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PGConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}