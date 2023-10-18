use actix_web::{App, HttpServer};
mod config;
mod routes;
mod handlers;
mod models;
mod services;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    // load configuration
    let server_address = "127.0.0.1:8080";

    // create server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::MyMiddleware) // define middleware
            .configure(routes::configure) // configure routes
    })
    .bind(server_address)? // bind server to address
    .run()
    .await
}
