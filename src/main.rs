#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::fairing::AdHoc;
use rocket::Config;
use rocket::{
    http::Status,
    serde::json::{json, Value},
    Build, Rocket,
};
use rocket_okapi::mount_endpoints_and_merged_docs;
use std::env;

mod api;
mod controllers;
mod database;
mod modules;

use api::{api_doc, cors, openapi_spec};

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

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket_build() -> Rocket<Build> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or(String::from("8000"))
        .parse::<u16>()
        .unwrap();

    let config = Config {
        port: port.clone(),
        ..Config::debug_default()
    };

    let mut building_rocket = rocket::build()
        .configure(config)
        .attach(AdHoc::on_ignite(
            "Connect to MongoDB cluster",
            |rocket| async {
                match database::mongo_db::connect().await {
                    Ok(database) => rocket.manage(database),
                    Err(error) => {
                        panic!("Cannot connect to MDB instance:: {:?}", error)
                    }
                }
            },
        ))
        .attach(api_doc::run())
        .attach(cors::run(&port))
        .register("/", catchers![not_found])
        .mount("/", routes![index, find, update, delete]);

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let custom_route_spec = (vec![], openapi_spec::run(&port));
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/" => custom_route_spec,
        "/leaderboard" => controllers::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}

#[rocket::main]
async fn main() {
    let launch_result = rocket_build().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
