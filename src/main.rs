#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::Config;
use rocket::{
    http::Status,
    serde::json::{json, Value},
    Build, Rocket,
};
use rocket_okapi::{
    mount_endpoints_and_merged_docs,
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use std::env;

mod controllers;
mod cors;
mod openapi_spec;

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

fn rocket_build() -> Rocket<Build> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or(String::from("8000"))
        .parse::<u16>()
        .unwrap();

    let mongo_db_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI required");

    print!("{mongo_db_uri}\n {port}\n");

    let config = Config {
        port: port.clone(),
        ..Config::debug_default()
    };

    let mut building_rocket = rocket::build()
        .configure(config)
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .attach(cors::run(&port))
        .mount("/", routes![index, find, update, delete]);

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    let custom_route_spec = (vec![], openapi_spec::run(&port));
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/" => custom_route_spec,
        "/ranking" => controllers::get_routes_and_docs(&openapi_settings),
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
