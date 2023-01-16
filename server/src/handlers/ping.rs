use actix_web::{web, Error, HttpResponse};

pub async fn get_ping(info: web::Path<String>) -> Result<HttpResponse, Error> {
    let name = info.to_string();
    let ret = format!("Hello {}!", name);
    tracing::info!("Ping request received: {}", ret);
    Ok(HttpResponse::Ok().body(ret))
}