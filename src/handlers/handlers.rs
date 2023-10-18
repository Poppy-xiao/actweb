use actix_web::{HttpResponse, Responder};
use crate::services;

pub async fn get_person() -> impl Responder {
    let person = services::get_person().await;
    HttpResponse::Ok().json(person) // returns a JSON response
}
