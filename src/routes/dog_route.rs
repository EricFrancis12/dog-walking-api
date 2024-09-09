use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    models::dog_model::{Dog, DogRequest},
    services::db::DB,
};

#[post("/dog")]
pub async fn insert_new_dog(db: Data<DB>, request: Json<DogRequest>) -> HttpResponse {
    match db
        .insert_new_dog(
            Dog::try_from(DogRequest {
                owner: request.owner.clone(),
                name: request.name.clone(),
                age: request.age.clone(),
                breed: request.breed.clone(),
            })
            .expect("error converting dog request into dog"),
        )
        .await
    {
        Ok(dog) => HttpResponse::Created().json(dog),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
