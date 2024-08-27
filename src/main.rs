use rocket::launch;
use rust_for_nodejs_developers::build_server;

#[launch]
fn rocket() -> _ {
    build_server()
}
