#[macro_use]
extern crate rocket;

mod controllers;
use controllers::create_controller;

use rocket::{
    http::Status,
    serde::json::{json, Value},
};

#[get("/")]
async fn index() -> (Status, Value) {
    (Status::Ok, json!({ "hello": "world"}))
}

#[get("/<id>")]
async fn find(id: i32) -> (Status, Value) {
    (Status::Ok, json!({ "msg": format!("find {}", id)}))
}

#[put("/<id>")]
async fn update(id: i32) -> (Status, Value) {
    (Status::Ok, json!({ "msg": format!("update {}", id)}))
}

#[delete("/<id>")]
async fn delete(id: i32) -> (Status, Value) {
    (Status::Ok, json!({ "msg": format!("delete {}", id)}))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, create_controller::handle, find, update, delete],
    )
}
