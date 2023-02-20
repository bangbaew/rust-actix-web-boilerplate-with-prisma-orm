use std::env;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};

use app::db::get_prisma_client;

mod app;

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    env_logger::init();
    
    let client = Data::new(Arc::new(get_prisma_client().await));
    
    let listen_port: String = env::var("LISTEN_PORT").expect("$LISTEN_PORT is not set");
    let listen_address: String = ["0.0.0.0", &listen_port].join(":");
    
    println!("Server is listening at {}...", listen_address);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .configure(app::route::setup_routes)
            .wrap(middleware::Logger::default())
    })
    .bind(listen_address)?
    .run()
    .await
}
