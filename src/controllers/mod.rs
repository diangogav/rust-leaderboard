use rocket_okapi::{okapi::openapi3::OpenApi, openapi_get_routes_spec, settings::OpenApiSettings};

mod create_controller;
mod leaderboard_user_update_controller;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_controller::handle, leaderboard_user_update_controller::handle]
}
