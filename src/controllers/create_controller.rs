use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    lorem: String,
}

#[post("/", data = "<request>")]
pub async fn handle(request: Json<CreateRequest>) -> (Status, Value) {
    (
        Status::Created,
        json!({"msg": format!("created with: {}",request.lorem)}),
    )
}
