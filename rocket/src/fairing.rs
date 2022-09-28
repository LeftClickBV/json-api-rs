use rocket::fairing::{Fairing, Info, Kind, Result};
use rocket::{Build, Rocket};

use crate::error;

pub struct JsonApiFairing;

#[rocket::async_trait]
impl Fairing for JsonApiFairing {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Ignite,
            name: "JsonApiFairing",
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        Ok(rocket.register("/", error::catchers()))
    }
}
