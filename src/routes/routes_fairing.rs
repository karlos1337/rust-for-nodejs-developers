use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    Build, Rocket,
};

use super::{root_routes, users::users_routes};

pub struct RoutesFairing {}
#[rocket::async_trait]
impl Fairing for RoutesFairing {
    fn info(&self) -> Info {
        Info {
            name: "Routes module",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        Ok(rocket
            .mount("/", root_routes())
            .mount("/users", users_routes()))
    }
}
