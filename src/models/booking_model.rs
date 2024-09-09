use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::{error::Error, time::SystemTime};

use super::{dog_model::Dog, owner_model::Owner};

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration_mins: u8,
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_mins: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBooking {
    pub _id: ObjectId,
    pub owner: Owner,
    pub dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration_mins: u8,
    pub cancelled: bool,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn Error>;

    fn try_from(item: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time)
            .map_err(|err| format!("error parsing start time: {}", err))?
            .with_timezone(&chrono::Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).expect("error parsing owner"),
            start_time: DateTime::from(chrono_datetime),
            duration_mins: 0,
            cancelled: false,
        })
    }
}
