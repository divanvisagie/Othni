use crate::{clients::db};
use crate::repos::futhark::{FutharkRepository, NewFuthark};
use actix_web::web::Bytes;
use actix_web::{HttpResponse, HttpRequest};


pub async fn get_futharks() -> HttpResponse {

    let mut conn = db::establish_connection();
    let mut repo = FutharkRepository::new(&mut conn);
    let futs = repo.get_futharks().unwrap();

    HttpResponse::Ok().json(futs)
}


pub async fn post_futhark(
    _req: HttpRequest, body: Bytes
) -> HttpResponse {

    let mut conn = db::establish_connection();
    let mut repo = FutharkRepository::new(&mut conn);
  
    let futhark_request = serde_json::from_slice::<NewFuthark>(&body).unwrap();
    let result = repo.save_futhark(futhark_request);

    tracing::info!("Saving futhark: {:?}", &futhark_request);

    HttpResponse::Created().json(&result.unwrap())
}