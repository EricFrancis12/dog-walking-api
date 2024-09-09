mod models;
mod routes;
mod services;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::{
    booking_route::{
        cancel_booking_by_id, get_all_bookings, get_all_upcoming_bookings, insert_new_booking,
    },
    dog_route::insert_new_dog,
    owner_route::insert_new_owner,
};
use std::io::Result;

use crate::services::db::DB;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello there")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let db = DB::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(insert_new_booking)
            .service(cancel_booking_by_id)
            .service(get_all_bookings)
            .service(get_all_upcoming_bookings)
            .service(insert_new_dog)
            .service(insert_new_owner)
    })
    .bind("localhost:5001")
    .unwrap()
    .run()
    .await
}
