use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use tokio::runtime::Runtime;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust coding")
}

fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Use await here to prevent the main function from exiting prematurely
    Ok(())
}


