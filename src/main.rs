mod testing;
use crate::testing::tweet;
use actix_web::{get, middleware, App, HttpRequest, HttpServer};
use env_logger;
use std::env;

#[get("/index")]
async fn index(_req: HttpRequest) -> &'static str {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actic_web=debug,actix_server_info=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(middleware::Logger::default())
            .service(tweet::list)
            // .service(tweet::get)
            .service(tweet::create)
        // .service(tweet::delete)
        // .service(like::list)
        // .service(like::plus_one)
        // .service(like::minus_one)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
