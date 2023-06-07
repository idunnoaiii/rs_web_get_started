use actix_service::Service;
use actix_web::{App, HttpServer};
#[macro_use] extern crate diesel;
extern crate dotenv;

mod json_serialization;
mod to_do;
mod views;
mod schema;
mod models;
mod database;
mod jwt;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
