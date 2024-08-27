use rocket::{
    get, launch, routes,
    serde::json::{json, Value},
};

#[get("/")]
fn get_root() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> Value {
    json!([{ "name": "user" }])
}

#[get("/users/<user_id>/friends")]
fn get_user_friends(user_id: u8) -> Value {
    json!([{ "friend_id": user_id }])
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_root, get_users, get_user_friends])
}
