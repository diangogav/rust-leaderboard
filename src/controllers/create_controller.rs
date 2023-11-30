use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;
use uuid::Uuid;

use crate::database::mongo_db::MongoDB;
use crate::modules::leaderboard::domain::{Leaderboard, LeaderboardRepository};
use crate::modules::leaderboard::infrastructure::MongodbLeaderboardRepository;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRequest {
    name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateResponse {
    id: String,
}

#[openapi(tag = "Leaderboard")]
#[post("/", data = "<request>")]
pub async fn handle(
    connection: &State<MongoDB>,
    request: Json<CreateRequest>,
) -> Result<Json<CreateResponse>, Status> {
    let repository = MongodbLeaderboardRepository { connection };

    let message = Leaderboard::create(Uuid::new_v4().to_string(), request.name.to_string());

    return match repository.save(message.clone()).await {
        Ok(id) => Ok(Json(CreateResponse { id })),
        Err(_) => Err(Status::InternalServerError),
    };
}
