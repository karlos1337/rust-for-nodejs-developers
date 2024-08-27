mod routes;

use rocket::{build, Build, Rocket};
use routes::RoutesFairing;

pub fn build_server() -> Rocket<Build> {
    build().attach(RoutesFairing {})
}
