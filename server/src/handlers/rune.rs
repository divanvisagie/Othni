
use actix_web::web::Bytes;
use actix_web::{HttpResponse, HttpRequest};

use crate::clients::db;
use crate::repos::rune::{RuneRepository, NewRune};

pub async fn get_runes() -> HttpResponse {
    
    let mut conn = db::establish_connection();
    let mut repo = RuneRepository::new(&mut conn);
    let runes = repo.get_runes().unwrap();

    HttpResponse::Ok().json(runes)
}

pub async fn post_rune(_req: HttpRequest, body: Bytes) -> HttpResponse {
    let mut conn = db::establish_connection();
    let mut repo = RuneRepository::new(&mut conn);

    let futhark_request = serde_json::from_slice::<NewRune>(&body).unwrap();
    let result = repo.save_rune(futhark_request);
    
    tracing::info!("Saving rune: {:?}", &futhark_request);

    HttpResponse::Created().json(&result.unwrap())
}