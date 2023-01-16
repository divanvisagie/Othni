use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use handlers::ping::get_ping;
use handlers::futhark::get_futharks;

mod handlers;
mod clients;
mod repos;
mod schema;

const IP: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    tracing_subscriber::fmt::init();
   
    // set up a new http server
    let server = HttpServer::new(|| {
        let logger = TracingLogger::default();

        App::new()
        .wrap(logger)
        .route("/ping/{name}", web::get().to(get_ping))	
        .route("/futharks", web::get().to(get_futharks))
    });


    tracing::info!("Starting server at http://{}:{}", IP, PORT);
    server.bind((IP, PORT))?
    .run()
    .await
}
