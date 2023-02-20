use std::{sync::Arc};

use crate::app::db::prisma::{PrismaClient};
use actix_web::{get, HttpResponse, Responder, web, post};
use serde::Deserialize;

#[derive(Deserialize)]
struct Playlist {
    name: String,
}

#[get("/playlist")]
pub async fn list(client: web::Data<Arc<PrismaClient>>) -> impl Responder {

    let result = client.playlist().find_many(vec![]).exec().await;
    if let Err(e) = result {
        println!("{:?}", e);
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    HttpResponse::Ok().json(result.unwrap())
}

#[post("/playlist")]
async fn hello(client: web::Data<Arc<PrismaClient>>, body: web::Json<Playlist>) -> impl Responder {

    let created = client.playlist()
    .create(
        body.name.to_string(),
        vec![]
    )
    .exec()
    .await;
    if let Err(e) = created {
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }

    HttpResponse::Ok().json(created.unwrap())
}

#[get("/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}