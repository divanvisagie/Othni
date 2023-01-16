use std::error::Error;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable)]// Model implementation
pub struct Futhark {
    pub id: i32,
    rune_set: String,
}

pub struct FutharkRepository<'a> {
  connection: &'a mut PgConnection
}


impl<'a> FutharkRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        Self {
            connection
        }
    }

    pub fn get_futharks(&mut self) -> Result<Vec<Futhark>, Box<dyn Error>> {
        use crate::schema::futhark::dsl::*;
        
        let results = 
            futhark.load::<Futhark>(self.connection);
           

       Ok(results?)
    }
}