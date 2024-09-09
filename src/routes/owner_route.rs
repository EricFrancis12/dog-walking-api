use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    models::owner_model::{Owner, OwnerRequest},
    services::db::DB,
};

#[post("/owner")]
pub async fn insert_new_owner(db: Data<DB>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .insert_new_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone(),
            })
            .expect("error converting owner request into owner"),
        )
        .await
    {
        Ok(owner) => HttpResponse::Created().json(owner),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
