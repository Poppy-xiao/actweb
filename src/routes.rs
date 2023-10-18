use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/get", web::get().to(handlers::get_person));
}
