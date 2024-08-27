use rocket::{get, routes, Route};

pub fn root_routes() -> Vec<Route> {
    routes![get_root]
}

#[get("/")]
fn get_root() -> &'static str {
    "Hello, world!"
}
