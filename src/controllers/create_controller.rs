use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

use crate::database::models::mongo_message::MongoMessage;
use crate::database::mongo_db::MongoDB;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRequest {
    content: String,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateResponse {
    msg: String,
}

#[openapi(tag = "Ranking")]
#[post("/", data = "<request>")]
pub async fn handle(
    connection: &State<MongoDB>,
    request: Json<CreateRequest>,
) -> Result<Json<CreateResponse>, Status> {
    let new_message = MongoMessage {
        _id: ObjectId::new(),
        content: request.content.clone(),
    };

    let user_collection = connection.database.collection::<MongoMessage>("messages");

    return match user_collection.insert_one(&new_message, None).await {
        Ok(_) => Ok(Json(CreateResponse {
            msg: new_message.content,
        })),
        Err(_) => Err(Status::InternalServerError),
    };
}
