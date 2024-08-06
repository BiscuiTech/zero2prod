use std::net::TcpListener;

use actix_web::{dev::Server, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        actix_web::App::new()
            .route(
                "/health_check",
                actix_web::web::get().to(crate::routes::health_check),
            )
            .route(
                "/subscriptions",
                actix_web::web::post().to(crate::routes::subscribe),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
