use rocket::{
    get, routes,
    serde::json::{json, Value},
    Route,
};

pub fn user_friends_routes() -> Vec<Route> {
    routes![get_user_friends]
}

#[get("/<user_id>/friends")]
fn get_user_friends(user_id: u8) -> Value {
    json!([{ "friend_id": user_id }])
}
