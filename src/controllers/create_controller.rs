use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRequest {
    lorem: String,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateResponse {
    msg: String,
}

#[openapi(tag = "Ranking")]
#[post("/", data = "<request>")]
pub async fn handle(request: Json<CreateRequest>) -> Json<CreateResponse> {
    Json(CreateResponse {
        msg: request.lorem.clone(),
    })
}
