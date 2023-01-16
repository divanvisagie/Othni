use std::error::Error;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::futhark::dsl::*;

use crate::schema::futhark;

#[derive(Serialize,Deserialize, Queryable, Debug)]// Model implementation
pub struct Futhark {
    pub id: i32,
    rune_set: String,
}

#[derive(Insertable, Deserialize, Serialize, Debug, Clone, Copy)]
#[diesel(table_name = futhark)]
pub struct NewFuthark<'a> {
    rune_set: &'a str,
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
        let results = 
            futhark.load::<Futhark>(self.connection);
           
       Ok(results?)
    }
    pub fn save_futhark(&mut self, new_fut: NewFuthark) -> Result<Futhark, Box<dyn Error>> {
 
        let results = 
            diesel::insert_into(futhark)
            .values(new_fut)
            .get_result(self.connection);
           

       Ok(results?)
    }
}