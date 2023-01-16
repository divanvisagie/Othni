// use crate::clients::db;

struct Rune {
    rune: String,
    latin: String,
}

struct RuneSet {
    name: String,
    runes: Vec<Rune>,
}


fn write_to_database(set: RuneSet) {
    // write input to posgresql database
    // let conn = db::establish_connection();
    
}