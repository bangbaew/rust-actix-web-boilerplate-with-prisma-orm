use crate::app::api::playlist::*;
use actix_web::web;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service((index,list,hello));
}