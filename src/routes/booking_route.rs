use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

use crate::{
    models::booking_model::{Booking, BookingRequest},
    services::db::DB,
};

#[get("/bookings/all")]
pub async fn get_all_bookings(db: Data<DB>) -> HttpResponse {
    match db.get_all_bookings().await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/bookings")]
pub async fn get_all_upcoming_bookings(db: Data<DB>) -> HttpResponse {
    match db.get_all_upcoming_bookings().await {
        Ok(full_bookings) => HttpResponse::Ok().json(full_bookings),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/booking")]
pub async fn insert_new_booking(db: Data<DB>, request: Json<BookingRequest>) -> HttpResponse {
    match db
        .insert_new_booking(
            Booking::try_from(BookingRequest {
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                duration_mins: request.duration_mins.clone(),
            })
            .expect("error converting booking request into booking"),
        )
        .await
    {
        Ok(booking) => HttpResponse::Created().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/booking/{booking_id}/cancel")]
pub async fn cancel_booking_by_id(db: Data<DB>, path: Path<(String,)>) -> HttpResponse {
    let booking_id = path.into_inner().0;
    match db.cancel_booking_by_id(booking_id.as_str()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
