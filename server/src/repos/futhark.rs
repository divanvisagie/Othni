use diesel::pg::PGConnection;

struct FutharkRepositiry {
    connection: PGConnection
}


impl FutharkRepositiry {
    pub fn new(connection: PGConnection) -> Self {
        Self {
            connection
        }
    }

    pub fn get_futharks() -> Result<Vec<Futhark>, Error> {
        use crate::schema::futharks::dsl::*;
        let results = futharks
            .load::<Futhark>(&self.connection)
            .expect("Error loading futharks");
        Ok(results)
    }
}