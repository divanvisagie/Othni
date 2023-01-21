use std::error::Error;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::rune::dsl::*;
use crate::schema::rune;

#[derive(Serialize,Deserialize, Queryable, Debug)]// Model implementation
pub struct Rune {
    pub id: i32,
    name: String,
    latin: String,
    futhark_id: i32,
}

#[derive(Insertable,Serialize,Deserialize, Debug, Clone, Copy)]
#[diesel(table_name = rune)]
pub struct NewRune<'a> {
    name: &'a str,
    latin: &'a str,
    futhark_id: i32
}

pub struct RuneRepository<'a> {
    connection: &'a mut PgConnection
}


impl<'a> RuneRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        Self {
            connection
        }
    }

    pub fn get_runes(&mut self) -> Result<Vec<Rune>, Box<dyn Error>> {
        let results = 
            rune.load::<Rune>(self.connection);
           
       Ok(results?)
    }
    
    pub fn save_rune(&mut self, new_rune: NewRune) -> Result<Rune, Box<dyn Error>> {
 
        let results = 
            diesel::insert_into(rune)
            .values(new_rune)
            .get_result(self.connection);
           

       Ok(results?)
    }
}