use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

use crate::database::mongo_db::MongoDB;
use crate::modules::message::domain::{Message, MessageRepository};
use crate::modules::message::infrastructure::MongodbMessageRepository;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRequest {
    content: String,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateResponse {
    id: String,
}

#[openapi(tag = "Ranking")]
#[post("/", data = "<request>")]
pub async fn handle(
    connection: &State<MongoDB>,
    request: Json<CreateRequest>,
) -> Result<Json<CreateResponse>, Status> {
    let repository = MongodbMessageRepository { connection };

    let message = Message::create(request.content.clone());

    return match repository.save(message.clone()).await {
        Ok(id) => Ok(Json(CreateResponse { id })),
        Err(_) => Err(Status::InternalServerError),
    };
}
