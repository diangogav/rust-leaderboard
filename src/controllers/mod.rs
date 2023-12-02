use rocket_okapi::{okapi::openapi3::OpenApi, openapi_get_routes_spec, settings::OpenApiSettings};

mod create_controller;
mod get_leaderboard_user_position;
mod get_leaderboard_users;
mod leaderboard_user_update_controller;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_controller::handle, leaderboard_user_update_controller::handle, get_leaderboard_users::handle, get_leaderboard_user_position::handle]
}
