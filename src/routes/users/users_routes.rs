use rocket::{
    get, routes,
    serde::json::{json, Value},
    Route,
};

use super::user_friends::user_friends_routes;

pub fn users_routes() -> Vec<Route> {
    [routes![get_users], user_friends_routes()].concat()
}

#[get("/")]
fn get_users() -> Value {
    json!([{ "name": "user" }])
}
