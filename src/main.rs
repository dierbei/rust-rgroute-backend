#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use rgroutes::handler;

mod rgroutes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/")]
fn post_index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Access-Token"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().unwrap();

    // rgroutes::handler::list_rgroute().await;

    rocket::build().attach(cors).mount("/", routes![
        handler::list_rgroute,
        handler::get_rgroute,
        handler::delete_rgroute,
        handler::create_rgroute,
        handler::update_rgroute,
    ])
}