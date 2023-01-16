use actix_web::{web, App, HttpServer};
use handlers::ping::get_ping;
use tracing_actix_web::TracingLogger;

mod handlers;

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
    });


    tracing::info!("App booting...");
    server.bind((IP, PORT))?
    .run()
    .await
}
