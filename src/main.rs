#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::error::Error;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    // #[serde(rename(serialize = "uid", deserialize = "id"))]
    pub id: i32,
    pub name: String,
}

#[get("/todo")]
fn todo() -> Json<User> {
    Json(User{id: 1, name: String::from("hedui")})
}

#[post("/todo")]
fn postTodo() -> Json<User> {
    Json(User{id: 1, name: String::from("heduipost")})
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<(), Box<dyn Error>> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "jwt-token"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite().mount("/", routes![index, todo, postTodo]).attach(cors).launch();

    Ok(())
}