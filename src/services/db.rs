use chrono::Utc;
use core::panic;
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, DateTime},
    error::Error,
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::{env, str::FromStr};

use crate::models::{
    booking_model::{Booking, FullBooking},
    dog_model::Dog,
    owner_model::Owner,
};

pub struct DB {
    booking: Collection<Booking>,
    dog: Collection<Dog>,
    owner: Collection<Owner>,
}

impl DB {
    pub fn new(
        booking: Collection<Booking>,
        dog: Collection<Dog>,
        owner: Collection<Owner>,
    ) -> Self {
        DB {
            booking,
            dog,
            owner,
        }
    }

    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27018/?directConnection=true".to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("dog_walking");

        let booking = db.collection("booking");
        let dog = db.collection("dog");
        let owner = db.collection("owner");

        DB::new(booking, dog, owner)
    }

    pub async fn insert_new_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("error inserting new booking");

        Ok(result)
    }

    pub async fn cancel_booking_by_id(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::from_str(booking_id).expect("error pasring booking _id")
                },
                doc! {
                    "$set": doc! {
                        "cancelled": true
                    }
                },
            )
            .await
            .ok()
            .expect("error canceling booking");

        Ok(result)
    }

    pub async fn get_all_bookings(&self) -> Result<Vec<Booking>, Error> {
        let mut results = self
            .booking
            .find(doc! {})
            .await
            .ok()
            .expect("error getting upcoming bookings");

        let mut bookings: Vec<Booking> = Vec::new();
        while let Some(result) = results.next().await {
            match result {
                Ok(booking) => {
                    bookings.push(booking);
                }
                Err(err) => panic!("error getting booking: {}", err),
            }
        }

        Ok(bookings)
    }

    pub async fn get_all_upcoming_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        let now = Utc::now().into();
        let mut results = self
            .booking
            .aggregate(vec![
                doc! {
                    "$match": {
                        "cancelled": false,
                        "start_time": {
                            "$gte": DateTime::from_system_time(now),
                        },
                    },
                },
                doc! {
                    "$lookup": doc! {
                        "from": "owner",
                        "localField": "owner",
                        "foreignField": "_id",
                        "as": "owner",
                    },
                },
                doc! {
                    "$unwind": doc! {
                        "path": "$owner",
                    },
                },
                doc! {
                    "$lookup": doc! {
                        "from": "dog",
                        "localField": "owner._id",
                        "foreignField": "owner",
                        "as": "dogs",
                    },
                },
            ])
            .await
            .ok()
            .expect("error getting upcoming bookings");

        let mut full_bookings: Vec<FullBooking> = Vec::new();
        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let full_booking: FullBooking =
                        from_document(doc).expect("error converting document to Full Booking");
                    full_bookings.push(full_booking);
                }
                Err(err) => panic!("error getting booking: {}", err),
            }
        }

        Ok(full_bookings)
    }

    pub async fn insert_new_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let result = self
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("error inserting new dog");

        Ok(result)
    }

    pub async fn insert_new_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let result = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("error inserting new owner");

        Ok(result)
    }
}
