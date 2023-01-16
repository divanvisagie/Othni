use crate::clients::db;
use crate::repos::futhark::FutharkRepository;
use actix_web::{web, Error, HttpResponse};

struct Rune {
    rune: String,
    latin: String,
}

struct RuneSet {
    name: String,
    runes: Vec<Rune>,
}


pub async fn get_futharks() -> HttpResponse {
    // write input to posgresql database
    let mut conn = db::establish_connection();
    let mut fr = FutharkRepository::new(&mut conn);
    let futharks = fr.get_futharks().unwrap();

    HttpResponse::Ok().json(futharks)
}